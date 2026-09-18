//! Local object-store durability for native Delta operations.
//!
//! Delegate conditional creation, multipart I/O and exact reads to object_store. A successful
//! mutation is acknowledged only after its file and directory chain have been synchronized.
//! Failure after visibility is an unknown acknowledgement, never an automatic write retry.
use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt, stream::BoxStream};
use object_store::{
    CopyOptions, GetOptions, GetResult, GetResultPayload, ListResult, MultipartUpload, ObjectMeta,
    ObjectStore, PutMultipartOptions, PutOptions, PutPayload, PutResult, RenameOptions, Result,
    UploadPart, local::LocalFileSystem, path::Path,
};
use std::{fmt, fs::File, path::PathBuf, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) struct DurableLocalStore {
    inner: Arc<LocalFileSystem>,
    metadata: Arc<crate::native_cache::MetadataCache>,
    executor: Arc<crate::runtime::NativeExecutor>,
    pool: Arc<dyn datafusion::execution::memory_pool::MemoryPool>,
}
impl DurableLocalStore {
    pub(crate) fn new(
        metadata: Arc<crate::native_cache::MetadataCache>,
        executor: Arc<crate::runtime::NativeExecutor>,
        pool: Arc<dyn datafusion::execution::memory_pool::MemoryPool>,
    ) -> Self {
        Self {
            inner: Arc::new(LocalFileSystem::new()),
            metadata,
            executor,
            pool,
        }
    }
    async fn synchronize(&self, location: &Path, file: bool) -> Result<()> {
        let path = self.inner.path_to_filesystem(location)?;
        synchronize(self.executor.clone(), path, file).await
    }

    async fn file_result(&self, location: &Path, options: GetOptions) -> Result<GetResult> {
        let store = self.inner.clone();
        let location = location.clone();
        // LocalFileSystem queues metadata work on the current runtime's blocking pool.
        // Select the I/O lane before calling it, including from synchronous kernel callers.
        self.executor
            .physical_context()
            .run(|| {
                self.executor
                    .spawn_on(self.executor.io_handle(), async move {
                        store.get_opts(&location, options).await
                    })
            })
            .await
            .map_err(error)?
    }
}
impl fmt::Display for DurableLocalStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DurableLocalStore")
    }
}
fn error(source: impl std::error::Error + Send + Sync + 'static) -> object_store::Error {
    object_store::Error::Generic {
        store: "DurableLocalStore",
        source: Box::new(source),
    }
}
async fn synchronize(
    executor: Arc<crate::runtime::NativeExecutor>,
    path: PathBuf,
    file: bool,
) -> Result<()> {
    executor
        .physical_context()
        .run(|| {
            executor.spawn_io_blocking(move || {
                if file {
                    File::open(&path)?.sync_all()?;
                }
                for parent in path.ancestors().skip(1) {
                    File::open(parent)?.sync_all()?;
                }
                Ok::<_, std::io::Error>(())
            })
        })
        .await
        .map_err(error)?
        .map_err(error)
}
#[async_trait]
impl ObjectStore for DurableLocalStore {
    async fn put_opts(
        &self,
        location: &Path,
        payload: PutPayload,
        opts: PutOptions,
    ) -> Result<PutResult> {
        self.metadata.invalidate(location);
        let result = self.inner.put_opts(location, payload, opts).await;
        self.metadata.invalidate(location);
        let result = result?;
        self.synchronize(location, true).await?;
        Ok(result)
    }
    async fn put_multipart_opts(
        &self,
        location: &Path,
        opts: PutMultipartOptions,
    ) -> Result<Box<dyn MultipartUpload>> {
        Ok(Box::new(DurableUpload {
            inner: self.inner.put_multipart_opts(location, opts).await?,
            path: self.inner.path_to_filesystem(location)?,
            location: location.clone(),
            metadata: self.metadata.clone(),
            executor: self.executor.clone(),
        }))
    }
    async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult> {
        let mut result = self.file_result(location, options).await?;
        let GetResultPayload::File(mut file, _) = result.payload else {
            return Err(error(std::io::Error::other(
                "local store returned non-file content",
            )));
        };
        let range = result.range.clone();
        let pool = self.pool.clone();
        let executor = self.executor.clone();
        result.payload = GetResultPayload::Stream(
            futures::stream::once(async move {
                executor
                    .physical_context()
                    .run(|| {
                        executor.spawn_io_blocking(move || {
                            crate::owned_bytes::OwnedBytes::read_range(&mut file, range, &pool)
                                .map(bytes::Bytes::from_owner)
                        })
                    })
                    .await
                    .map_err(error)?
                    .map_err(error)
            })
            .boxed(),
        );
        Ok(result)
    }
    async fn get_ranges(
        &self,
        location: &Path,
        ranges: &[std::ops::Range<u64>],
    ) -> Result<Vec<bytes::Bytes>> {
        let result = self.file_result(location, GetOptions::default()).await?;
        let GetResultPayload::File(mut file, _) = result.payload else {
            return Err(error(std::io::Error::other(
                "local store returned non-file content",
            )));
        };
        let ranges = ranges.to_vec();
        let pool = self.pool.clone();
        self.executor
            .physical_context()
            .run(|| {
                self.executor.spawn_io_blocking(move || {
                    ranges
                        .into_iter()
                        .map(|range| {
                            crate::owned_bytes::OwnedBytes::read_range(&mut file, range, &pool)
                                .map(bytes::Bytes::from_owner)
                                .map_err(error)
                        })
                        .collect()
                })
            })
            .await
            .map_err(error)?
    }
    fn delete_stream(
        &self,
        locations: BoxStream<'static, Result<Path>>,
    ) -> BoxStream<'static, Result<Path>> {
        let store = self.clone();
        let invalidation = self.metadata.clone();
        let locations = locations
            .map(move |entry| {
                if let Ok(path) = &entry {
                    invalidation.invalidate(path);
                }
                entry
            })
            .boxed();
        self.inner
            .delete_stream(locations)
            .and_then(move |location| {
                let store = store.clone();
                async move {
                    store.metadata.invalidate(&location);
                    store.synchronize(&location, false).await?;
                    Ok(location)
                }
            })
            .boxed()
    }
    fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>> {
        self.inner.list(prefix)
    }
    fn list_with_offset(
        &self,
        prefix: Option<&Path>,
        offset: &Path,
    ) -> BoxStream<'static, Result<ObjectMeta>> {
        self.inner.list_with_offset(prefix, offset)
    }
    async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult> {
        self.inner.list_with_delimiter(prefix).await
    }
    async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()> {
        self.metadata.invalidate(to);
        let outcome = self.inner.copy_opts(from, to, options).await;
        self.metadata.invalidate(to);
        outcome?;
        self.synchronize(to, true).await
    }
    async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()> {
        self.metadata.invalidate(from);
        self.metadata.invalidate(to);
        let outcome = self.inner.rename_opts(from, to, options).await;
        self.metadata.invalidate(from);
        self.metadata.invalidate(to);
        outcome?;
        self.synchronize(to, true).await?;
        self.synchronize(from, false).await
    }
}
#[derive(Debug)]
struct DurableUpload {
    inner: Box<dyn MultipartUpload>,
    path: PathBuf,
    location: Path,
    metadata: Arc<crate::native_cache::MetadataCache>,
    executor: Arc<crate::runtime::NativeExecutor>,
}
#[async_trait]
impl MultipartUpload for DurableUpload {
    fn put_part(&mut self, data: PutPayload) -> UploadPart {
        self.inner.put_part(data)
    }
    async fn complete(&mut self) -> Result<PutResult> {
        self.metadata.invalidate(&self.location);
        let result = self.inner.complete().await;
        self.metadata.invalidate(&self.location);
        let result = result?;
        synchronize(self.executor.clone(), self.path.clone(), true).await?;
        Ok(result)
    }
    async fn abort(&mut self) -> Result<()> {
        self.inner.abort().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datafusion::execution::memory_pool::{GreedyMemoryPool, MemoryPool};
    use object_store::{GetRange, ObjectStoreExt};

    #[tokio::test]
    async fn object_reads_preserve_native_ranges_and_last_byte_owner_accounting()
    -> datafusion::common::Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(
            &root.path().join("spill"),
            crate::runtime::QueryLimits {
                concurrency: 1,
                native: enrichment_core::config::NativeQueryConfig {
                    blocking_threads: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
        )?;
        let path = root.path().join("input");
        std::fs::write(&path, (0u8..64).collect::<Vec<_>>())?;
        let path = Path::from_filesystem_path(path).unwrap();
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(32));
        let store = DurableLocalStore::new(
            Arc::new(crate::native_cache::MetadataCache::new(128)),
            runtime.executor_owner(),
            pool.clone(),
        );
        let ranges = store.get_ranges(&path, &[2..10, 12..28]).await?;
        assert_eq!(ranges[0].as_ref(), &(2u8..10).collect::<Vec<_>>());
        assert_eq!(pool.reserved(), 24);
        let slice = ranges[0].slice(2..4);
        drop(ranges);
        assert_eq!(slice.as_ref(), &[4, 5]);
        assert_eq!(pool.reserved(), 8);
        drop(slice);
        assert_eq!(pool.reserved(), 0);
        let result = store
            .get_opts(
                &path,
                GetOptions {
                    range: Some(GetRange::Bounded(10..42)),
                    ..Default::default()
                },
            )
            .await?;
        assert_eq!(result.range, 10..42);
        assert_eq!(pool.reserved(), 0);
        let bytes = result.bytes().await?;
        assert_eq!(bytes.as_ref(), &(10u8..42).collect::<Vec<_>>());
        assert_eq!(pool.reserved(), 32);
        assert!(store.get_range(&path, 0..1).await.is_err());
        let retained = bytes.clone();
        drop(bytes);
        assert_eq!(pool.reserved(), 32);
        drop(retained);
        assert_eq!(pool.reserved(), 0);
        assert_eq!(store.head(&path).await?.size, 64);
        assert_eq!(pool.reserved(), 0);
        assert!(
            store
                .get_opts(
                    &path,
                    GetOptions {
                        if_match: Some("wrong-etag".into()),
                        ..Default::default()
                    }
                )
                .await
                .is_err()
        );
        let suffix = store
            .get_opts(
                &path,
                GetOptions {
                    range: Some(GetRange::Suffix(3)),
                    ..Default::default()
                },
            )
            .await?
            .bytes()
            .await?;
        assert_eq!(suffix.as_ref(), &[61, 62, 63]);
        drop(suffix);
        assert!(
            store
                .get_ranges(&path, std::slice::from_ref(&(60..65)))
                .await
                .is_err()
        );
        assert_eq!(pool.reserved(), 0);
        // The only compute blocking thread may be occupied by a synchronous kernel caller.
        // Its file operation must complete on the distinct I/O blocking lane.
        let reader = store.clone();
        let bytes = runtime
            .blocking(move || futures::executor::block_on(reader.get_range(&path, 0..8)))
            .await??;
        assert_eq!(bytes.as_ref(), &[0, 1, 2, 3, 4, 5, 6, 7]);
        drop(bytes);
        assert_eq!(pool.reserved(), 0);
        drop(store);
        runtime.close_diagnostics().await
    }
}
