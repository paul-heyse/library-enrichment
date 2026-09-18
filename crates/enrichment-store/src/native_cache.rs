//! Native cache construction and immutable policy. Eviction is DataFusion's mechanism;
//! retention/read/effect authority remains outside caches.
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::cache::{
    Cache, CacheEntryInfo,
    cache_manager::{CacheManagerConfig, CachedFileMetadataEntry},
    default_cache::DefaultCache,
};
use enrichment_core::{
    config::NativeCachePolicy,
    wire::status::{NativeCacheCounters, NativeCacheFamily},
};
use object_store::path::Path;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

pub(crate) struct MetadataCache {
    values: DefaultCache<Path, CachedFileMetadataEntry>,
    hits: AtomicU64,
    misses: AtomicU64,
    invalidations: AtomicU64,
}
impl std::fmt::Debug for MetadataCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetadataCache")
            .field("counters", &self.counters())
            .finish()
    }
}
impl MetadataCache {
    pub(crate) fn new(bytes: usize) -> Self {
        Self {
            values: DefaultCache::new(bytes).with_name("parquet_metadata"),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            invalidations: AtomicU64::new(0),
        }
    }
    pub(crate) fn config(self: &Arc<Self>) -> CacheManagerConfig {
        CacheManagerConfig::default()
            .with_file_statistics_cache_limit(0)
            .with_list_files_cache_limit(0)
            .with_metadata_cache_limit(self.values.cache_limit())
            .with_file_metadata_cache(Some(self.clone()))
    }
    pub(crate) fn invalidate(&self, path: &Path) {
        self.remove(path);
    }
    pub(crate) fn clear(&self) {
        Cache::clear(self);
    }
    pub(crate) fn counters(&self) -> NativeCacheCounters {
        NativeCacheCounters {
            family: NativeCacheFamily::FileMetadata,
            entries: self.values.len(),
            limit_bytes: self.values.cache_limit(),
            occupied_bytes: Some(self.values.memory_used()),
            ownership: None,
            activity: enrichment_core::wire::status::NativeCacheActivity {
                hits: Some(self.hits.load(Ordering::Relaxed)),
                misses: Some(self.misses.load(Ordering::Relaxed)),
                invalidations: Some(self.invalidations.load(Ordering::Relaxed)),
                ..Default::default()
            },
        }
    }
}

/// Observe lookups without cloning a cache inventory. A hit means a value was returned;
/// the reader must still validate ObjectMeta and supported metadata type. Eviction, TTL,
/// size calculation and retained values remain the upstream DefaultCache implementation.
impl Cache<Path, CachedFileMetadataEntry> for MetadataCache {
    fn get(&self, key: &Path) -> Option<CachedFileMetadataEntry> {
        let result = self.values.get(key);
        if result.is_some() {
            &self.hits
        } else {
            &self.misses
        }
        .fetch_add(1, Ordering::Relaxed);
        result
    }
    fn put(&self, key: &Path, value: CachedFileMetadataEntry) -> Option<CachedFileMetadataEntry> {
        self.values.put(key, value)
    }
    fn remove(&self, key: &Path) -> Option<CachedFileMetadataEntry> {
        self.invalidations.fetch_add(1, Ordering::Relaxed);
        self.values.remove(key)
    }
    fn contains_key(&self, key: &Path) -> bool {
        self.values.contains_key(key)
    }
    fn len(&self) -> usize {
        self.values.len()
    }
    fn clear(&self) {
        self.invalidations.fetch_add(1, Ordering::Relaxed);
        self.values.clear();
    }
    fn name(&self) -> String {
        self.values.name()
    }
    fn cache_limit(&self) -> usize {
        self.values.cache_limit()
    }
    fn update_cache_limit(&self, bytes: usize) {
        self.values.update_cache_limit(bytes);
    }
    fn cache_ttl(&self) -> Option<std::time::Duration> {
        self.values.cache_ttl()
    }
    fn update_cache_ttl(&self, ttl: Option<std::time::Duration>) {
        self.values.update_cache_ttl(ttl);
    }
    fn drop_table_entries(&self, table: &datafusion::common::TableReference) -> Result<()> {
        self.values.drop_table_entries(table)
    }
    fn list_entries(
        &self,
    ) -> datafusion::common::HashMap<Path, CacheEntryInfo<CachedFileMetadataEntry>> {
        self.values.list_entries()
    }
}
pub(crate) fn validate(policy: &NativeCachePolicy) -> Result<()> {
    if policy.snapshot_entry_bytes == 0
        || policy.control_checkpoint_interval == 0
        || policy.checkpoint_interval == 0
    {
        return Err(DataFusionError::Configuration(
            "snapshot entry and checkpoint bounds must be positive".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plan19_native_metadata_owner_disables_unused_families() -> Result<()> {
        let cache = Arc::new(MetadataCache::new(4096));
        let manager =
            datafusion::execution::cache::cache_manager::CacheManager::try_new(&cache.config())?;
        assert!(manager.get_file_statistic_cache().is_none());
        assert!(manager.get_list_files_cache().is_none());
        assert_eq!(cache.counters().occupied_bytes, Some(0));
        assert_eq!(manager.get_file_metadata_cache().cache_limit(), 4096);
        Ok(())
    }

    #[test]
    fn metadata_lookup_observations_do_not_claim_validation_or_live_allocation() -> Result<()> {
        use datafusion::execution::cache::cache_manager::{CacheManager, FileMetadata};
        struct Metadata;
        impl FileMetadata for Metadata {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn memory_size(&self) -> usize {
                128
            }
            fn extra_info(&self) -> datafusion::common::HashMap<String, String> {
                Default::default()
            }
        }
        let cache = Arc::new(MetadataCache::new(4096));
        let manager = CacheManager::try_new(&cache.config())?;
        let reader = manager.get_file_metadata_cache();
        let path = Path::from("owned/table/data.parquet");
        let metadata = Arc::new(Metadata);
        let witness = Arc::downgrade(&metadata);
        let object = object_store::ObjectMeta {
            location: path.clone(),
            size: 100,
            last_modified: "2026-09-17T00:00:00Z".parse().unwrap(),
            e_tag: None,
            version: None,
        };
        assert!(reader.get(&path).is_none());
        reader.put(
            &path,
            CachedFileMetadataEntry::new(object.clone(), metadata),
        );
        let held = reader.get(&path).expect("resident metadata");
        assert!(held.is_valid_for(&object));
        let mut changed = object;
        changed.size += 1;
        assert!(!held.is_valid_for(&changed));
        assert_eq!(cache.counters().activity.hits, Some(1));
        assert_eq!(cache.counters().activity.misses, Some(1));
        reader.update_cache_limit(0);
        assert_eq!(cache.counters().occupied_bytes, Some(0));
        assert!(
            witness.upgrade().is_some(),
            "reader still owns the evicted value"
        );
        assert!(
            cache.counters().ownership.is_none(),
            "resident size is not live allocation"
        );
        assert!(reader.get(&path).is_none());
        cache.invalidate(&path);
        assert_eq!(cache.counters().activity.invalidations, Some(1));
        drop(held);
        assert!(witness.upgrade().is_none());
        Ok(())
    }
}
