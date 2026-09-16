//! Local object-store durability for native Delta operations.
//!
//! Delegate conditional creation, multipart I/O and exact reads to object_store. A successful
//! mutation is acknowledged only after its file and directory chain have been synchronized.
//! Failure after visibility is an unknown acknowledgement, never an automatic write retry.
use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt, stream::BoxStream};
use object_store::{
    CopyOptions, GetOptions, GetResult, ListResult, MultipartUpload, ObjectMeta, ObjectStore,
    PutMultipartOptions, PutOptions, PutPayload, PutResult, RenameOptions, Result, UploadPart,
    local::LocalFileSystem, path::Path,
};
use std::{fmt, fs::File, path::PathBuf, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) struct DurableLocalStore {
    inner: Arc<LocalFileSystem>,
}
impl DurableLocalStore {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(LocalFileSystem::new()),
        }
    }
    async fn synchronize(&self, location: &Path, file: bool) -> Result<()> {
        let path = self.inner.path_to_filesystem(location)?;
        synchronize(path, file).await
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
async fn synchronize(path: PathBuf, file: bool) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        if file {
            File::open(&path)?.sync_all()?;
        }
        for parent in path.ancestors().skip(1) {
            File::open(parent)?.sync_all()?;
        }
        Ok::<_, std::io::Error>(())
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
        let result = self.inner.put_opts(location, payload, opts).await?;
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
        }))
    }
    async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult> {
        self.inner.get_opts(location, options).await
    }
    async fn get_ranges(
        &self,
        location: &Path,
        ranges: &[std::ops::Range<u64>],
    ) -> Result<Vec<bytes::Bytes>> {
        self.inner.get_ranges(location, ranges).await
    }
    fn delete_stream(
        &self,
        locations: BoxStream<'static, Result<Path>>,
    ) -> BoxStream<'static, Result<Path>> {
        let store = self.clone();
        self.inner
            .delete_stream(locations)
            .and_then(move |location| {
                let store = store.clone();
                async move {
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
        self.inner.copy_opts(from, to, options).await?;
        self.synchronize(to, true).await
    }
    async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()> {
        self.inner.rename_opts(from, to, options).await?;
        self.synchronize(to, true).await?;
        self.synchronize(from, false).await
    }
}
#[derive(Debug)]
struct DurableUpload {
    inner: Box<dyn MultipartUpload>,
    path: PathBuf,
}
#[async_trait]
impl MultipartUpload for DurableUpload {
    fn put_part(&mut self, data: PutPayload) -> UploadPart {
        self.inner.put_part(data)
    }
    async fn complete(&mut self) -> Result<PutResult> {
        let result = self.inner.complete().await?;
        synchronize(self.path.clone(), true).await?;
        Ok(result)
    }
    async fn abort(&mut self) -> Result<()> {
        self.inner.abort().await
    }
}
