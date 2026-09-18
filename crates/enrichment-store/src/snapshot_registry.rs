//! Accounted, exact Delta state shared by namespace owners. Native caches own eviction;
//! a bounded, weak per-namespace coordinator owns cold loads and incremental refreshes.
//! No cache value carries read protection, effect authority, or the QueryRuntime.
use datafusion::{
    common::TableReference,
    error::{DataFusionError, Result},
    execution::{
        cache::{Cache, CacheKey, CacheValue, default_cache::DefaultCache},
        memory_pool::{MemoryConsumer, MemoryPool, MemoryReservation},
    },
};
use deltalake::DeltaTable;
use enrichment_core::{
    config::NativeCachePolicy,
    wire::status::{NativeCacheCounters, NativeCacheFamily},
};
use std::{
    collections::HashMap,
    ops::Deref,
    path::Path,
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicU64, Ordering},
    },
};
use tokio::sync::{Mutex as AsyncMutex, OwnedMutexGuard, OwnedSemaphorePermit, Semaphore};

/// An immutable captured table and the reservation shared by all its consumers.
#[derive(Clone, Debug)]
pub struct LoadedTable {
    table: DeltaTable,
    namespace: Namespace,
    memory: Arc<SnapshotMemory>,
    bytes: usize,
    cacheable: bool,
}
impl Deref for LoadedTable {
    type Target = DeltaTable;
    fn deref(&self) -> &DeltaTable {
        &self.table
    }
}
impl LoadedTable {
    pub(crate) fn into_parts(self) -> (DeltaTable, Arc<SnapshotMemory>) {
        (self.table, self.memory)
    }
    pub(crate) fn namespace(&self) -> &Namespace {
        &self.namespace
    }
    pub(crate) fn table_clone(&self) -> DeltaTable {
        self.table.clone()
    }
    pub(crate) fn memory(&self) -> Arc<SnapshotMemory> {
        self.memory.clone()
    }
    pub(crate) fn cacheable(&self) -> bool {
        self.cacheable
    }
}

type Usage = Arc<Mutex<enrichment_core::wire::status::NativeCacheOwnership>>;

/// The native reservation travels with the actual snapshot/provider allocation. Its phase
/// changes once after capture. Failed loads and last physical readers release it through Drop.
#[derive(Debug)]
pub(crate) struct SnapshotMemory {
    memory: MemoryReservation,
    usage: Usage,
    bytes: usize,
    captured: bool,
}
impl SnapshotMemory {
    pub(crate) fn capture(mut self, bytes: usize) -> Result<Self> {
        self.memory.try_resize(bytes)?;
        {
            let mut usage = self
                .usage
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            usage.in_flight_bytes -= self.bytes;
            usage.live_value_bytes += bytes;
        }
        self.bytes = bytes;
        self.captured = true;
        Ok(self)
    }
}
impl Drop for SnapshotMemory {
    fn drop(&mut self) {
        let mut usage = self
            .usage
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.captured {
            usage.live_value_bytes -= self.bytes;
        } else {
            usage.in_flight_bytes -= self.bytes;
        }
    }
}

struct Resident(Arc<SnapshotMemory>);
impl Resident {
    fn new(memory: Arc<SnapshotMemory>) -> Self {
        memory
            .usage
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .resident_value_bytes += memory.bytes;
        Self(memory)
    }
}
impl Drop for Resident {
    fn drop(&mut self) {
        self.0
            .usage
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .resident_value_bytes -= self.0.bytes;
    }
}
#[derive(Clone)]
struct SnapshotEntry {
    table: LoadedTable,
    _resident: Arc<Resident>,
}
impl CacheValue for SnapshotEntry {
    fn size(&self) -> usize {
        self.table.bytes
    }
}
impl CacheValue for LoadedTable {
    fn size(&self) -> usize {
        self.bytes
    }
}

/// Stable physical incarnation; directory modification time is deliberately excluded
/// because appending a commit changes it. Recreating a root cannot reuse a warm entry.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Namespace {
    root: String,
    reference: TableReference,
    incarnation: Vec<(u64, u64, std::time::SystemTime)>,
}
impl Namespace {
    pub(crate) fn reference(&self) -> &TableReference {
        &self.reference
    }
    pub(crate) fn witness(&self) -> Result<enrichment_core::operation::projections::ReadNamespace> {
        use enrichment_core::operation::projections::{DirectoryIdentity, ReadNamespace};
        Ok(ReadNamespace {
            path: self.root.clone(),
            directories: self
                .incarnation
                .iter()
                .map(|(device, inode, created)| {
                    let created = created
                        .duration_since(std::time::UNIX_EPOCH)
                        .map_err(external)?;
                    Ok(DirectoryIdentity {
                        device: *device,
                        inode: *inode,
                        created_seconds: created.as_secs(),
                        created_nanos: created.subsec_nanos(),
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
    pub(crate) fn read(path: &Path) -> Result<Self> {
        use std::os::unix::fs::MetadataExt;
        let root = std::fs::canonicalize(path)?;
        let mut incarnation = Vec::with_capacity(2);
        for path in [root.clone(), root.join("_delta_log")] {
            let metadata = std::fs::metadata(path)?;
            incarnation.push((metadata.dev(), metadata.ino(), metadata.created()?));
        }
        let root = root.to_string_lossy().into_owned();
        Ok(Self {
            reference: TableReference::bare(root.clone()),
            root,
            incarnation,
        })
    }
}
impl CacheKey for Namespace {
    fn size(&self) -> usize {
        size_of::<Self>()
            + self.root.capacity()
            + self.reference.table().len()
            + self.incarnation.capacity() * size_of::<(u64, u64, std::time::SystemTime)>()
    }
    fn table_ref(&self) -> Option<&TableReference> {
        Some(self.reference())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Key {
    namespace: Namespace,
    id: String,
    version: u64,
}
impl CacheKey for Key {
    fn size(&self) -> usize {
        size_of::<Self>() + self.namespace.size() + self.id.capacity()
    }
    fn table_ref(&self) -> Option<&TableReference> {
        Some(self.namespace.reference())
    }
}
#[derive(Clone)]
struct Head {
    id: String,
    version: u64,
    generation: u64,
    _memory: Arc<MemoryReservation>,
}
impl CacheValue for Head {
    fn size(&self) -> usize {
        size_of::<Self>() + self.id.capacity()
    }
}

type Flights = HashMap<(Namespace, Option<u64>), Weak<AsyncMutex<()>>>;

pub(crate) struct NamespaceEpoch {
    generation: AtomicU64,
    _memory: MemoryReservation,
}
pub(crate) enum FillEpoch {
    Unopened(u64),
    Namespace {
        owner: Arc<NamespaceEpoch>,
        generation: u64,
    },
}
#[derive(Clone, Copy)]
pub(crate) enum Publication {
    /// Incremental loading can adopt a newer checkpoint without changing table version.
    Refreshed,
    /// Restore/write consumers share an already resident equivalent allocation.
    Reuse,
}

pub(crate) struct Registry {
    snapshots: DefaultCache<Key, SnapshotEntry>,
    heads: DefaultCache<Namespace, Head>,
    flights: Mutex<Flights>,
    publication: Mutex<()>,
    epoch: AtomicU64,
    namespace_epochs: Mutex<HashMap<Namespace, Weak<NamespaceEpoch>>>,
    slots: Arc<Semaphore>,
    pool: Arc<dyn MemoryPool>,
    entry_bytes: usize,
    loads: AtomicU64,
    refreshes: AtomicU64,
    unchanged: AtomicU64,
    waits: AtomicU64,
    publications: AtomicU64,
    bypasses: AtomicU64,
    usage: Usage,
}
#[derive(Debug)]
pub(crate) struct Flight {
    _lock: OwnedMutexGuard<()>,
    _slot: OwnedSemaphorePermit,
}
impl Registry {
    pub(crate) fn new(
        policy: &NativeCachePolicy,
        pool: Arc<dyn MemoryPool>,
        concurrency: usize,
    ) -> Self {
        let heads = (policy.snapshots_bytes / 64).min(1024 * 1024);
        Self {
            snapshots: DefaultCache::new(policy.snapshots_bytes - heads)
                .with_name("delta_snapshots"),
            heads: DefaultCache::new(heads).with_name("delta_heads"),
            flights: Mutex::default(),
            publication: Mutex::new(()),
            epoch: AtomicU64::new(0),
            namespace_epochs: Mutex::default(),
            slots: Arc::new(Semaphore::new(concurrency)),
            pool,
            entry_bytes: policy.snapshot_entry_bytes,
            loads: AtomicU64::new(0),
            refreshes: AtomicU64::new(0),
            unchanged: AtomicU64::new(0),
            waits: AtomicU64::new(0),
            publications: AtomicU64::new(0),
            bypasses: AtomicU64::new(0),
            usage: Arc::default(),
        }
    }
    pub(crate) fn reserve(&self) -> Result<SnapshotMemory> {
        let memory = MemoryConsumer::new("delta-snapshot").register(&self.pool);
        memory.try_grow(self.entry_bytes)?;
        self.usage
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .in_flight_bytes += self.entry_bytes;
        Ok(SnapshotMemory {
            memory,
            usage: self.usage.clone(),
            bytes: self.entry_bytes,
            captured: false,
        })
    }
    pub(crate) fn capture(
        &self,
        namespace: Namespace,
        table: DeltaTable,
        memory: SnapshotMemory,
    ) -> Result<LoadedTable> {
        if !table.config.require_files || table.config.skip_stats {
            return Err(invalid(
                "snapshot registry requires full file and statistics capability",
            ));
        }
        let bytes = table
            .snapshot()
            .map_err(external)?
            .snapshot()
            .estimated_owned_heap_size_bytes()
            .saturating_add(size_of::<LoadedTable>())
            .saturating_add(namespace.size())
            .saturating_add(size_of::<Key>())
            .saturating_add(table.snapshot().map_err(external)?.metadata().id().len());
        let memory = memory.capture(bytes)?;
        Ok(LoadedTable {
            table,
            namespace,
            memory: Arc::new(memory),
            bytes,
            cacheable: bytes <= self.entry_bytes,
        })
    }
    pub(crate) async fn flight(
        &self,
        namespace: &Namespace,
        version: Option<u64>,
    ) -> Result<Flight> {
        let slot = self.slots.clone().acquire_owned().await.map_err(external)?;
        let key = (namespace.clone(), version);
        let lock = {
            let mut flights = self
                .flights
                .lock()
                .map_err(|_| invalid("snapshot coordination poisoned"))?;
            flights.retain(|_, value| value.strong_count() != 0);
            if let Some(lock) = flights.get(&key).and_then(Weak::upgrade) {
                lock
            } else {
                let lock = Arc::new(AsyncMutex::new(()));
                flights.insert(key, Arc::downgrade(&lock));
                lock
            }
        };
        let lock = match lock.clone().try_lock_owned() {
            Ok(guard) => guard,
            Err(_) => {
                self.waits.fetch_add(1, Ordering::Relaxed);
                lock.lock_owned().await
            }
        };
        Ok(Flight {
            _lock: lock,
            _slot: slot,
        })
    }
    pub(crate) fn generation(&self, namespace: &Namespace) -> Option<u64> {
        self.heads.get(namespace).map(|head| head.generation)
    }
    pub(crate) fn get(&self, namespace: &Namespace, version: Option<u64>) -> Option<LoadedTable> {
        let head = self.heads.get(namespace)?;
        self.snapshots
            .get(&Key {
                namespace: namespace.clone(),
                id: head.id,
                version: version.unwrap_or(head.version),
            })
            .map(|entry| entry.table)
    }
    /// Caller owns the namespace flight. A delayed writer/historical load never rolls
    /// back the current head. Returning a version does not prove absence of foreign commits.
    pub(crate) fn publish(
        &self,
        namespace: &Namespace,
        table: LoadedTable,
        epoch: FillEpoch,
        publication: Publication,
    ) -> Result<LoadedTable> {
        if table.namespace() != namespace {
            return Err(invalid("snapshot incarnation differs from cache namespace"));
        }
        let _publication = self
            .publication
            .lock()
            .map_err(|_| invalid("snapshot publication poisoned"))?;
        if !self.epoch_current(&epoch) {
            return Err(invalid("snapshot fill crossed cache invalidation"));
        }
        let version = table
            .version()
            .ok_or_else(|| invalid("cannot cache an unloaded table"))?;
        let id = table
            .snapshot()
            .map_err(external)?
            .metadata()
            .id()
            .to_owned();
        let key = Key {
            namespace: namespace.clone(),
            id: id.clone(),
            version,
        };
        let table = match publication {
            Publication::Refreshed => table,
            Publication::Reuse => self.snapshots.get(&key).map_or(table, |entry| entry.table),
        };
        let before = self.heads.get(namespace);
        if before.as_ref().is_some_and(|head| head.id != id) {
            return Err(invalid(
                "Delta table identity changed within one physical incarnation",
            ));
        }
        if table.cacheable() {
            self.snapshots.put(
                &key,
                SnapshotEntry {
                    _resident: Arc::new(Resident::new(table.memory())),
                    table: table.clone(),
                },
            );
        } else {
            self.snapshots.remove(&key);
            self.bypasses.fetch_add(1, Ordering::Relaxed);
        }
        if before.as_ref().is_none_or(|head| head.version <= version) {
            let memory = MemoryConsumer::new("delta-head").register(&self.pool);
            memory.try_grow(namespace.size() + size_of::<Head>() + id.capacity())?;
            self.heads.put(
                namespace,
                Head {
                    id,
                    version,
                    generation: match before {
                        Some(head) => head
                            .generation
                            .checked_add(1)
                            .ok_or_else(|| invalid("snapshot head generation exhausted"))?,
                        None => 0,
                    },
                    _memory: Arc::new(memory),
                },
            );
        }
        Ok(table)
    }
    /// The caller already owns a bounded flight. Weak generation owners live only
    /// as long as fills; unrelated table invalidation cannot reject this publication.
    pub(crate) fn epoch(&self, namespace: Option<&Namespace>) -> Result<FillEpoch> {
        let Some(namespace) = namespace else {
            return Ok(FillEpoch::Unopened(self.epoch.load(Ordering::Acquire)));
        };
        let mut owners = self
            .namespace_epochs
            .lock()
            .map_err(|_| invalid("snapshot generations poisoned"))?;
        owners.retain(|_, owner| owner.strong_count() != 0);
        let owner = match owners.get(namespace).and_then(Weak::upgrade) {
            Some(owner) => owner,
            None => {
                let memory = MemoryConsumer::new("delta-fill-generation").register(&self.pool);
                memory.try_grow(namespace.size() + size_of::<NamespaceEpoch>())?;
                let owner = Arc::new(NamespaceEpoch {
                    generation: AtomicU64::new(0),
                    _memory: memory,
                });
                owners.insert(namespace.clone(), Arc::downgrade(&owner));
                owner
            }
        };
        Ok(FillEpoch::Namespace {
            generation: owner.generation.load(Ordering::Acquire),
            owner,
        })
    }
    fn epoch_current(&self, epoch: &FillEpoch) -> bool {
        match epoch {
            FillEpoch::Unopened(generation) => *generation == self.epoch.load(Ordering::Acquire),
            FillEpoch::Namespace { owner, generation } => {
                *generation == owner.generation.load(Ordering::Acquire)
            }
        }
    }
    pub(crate) fn invalidate(&self, namespace: &Namespace) -> Result<()> {
        let _publication = self
            .publication
            .lock()
            .map_err(|_| invalid("snapshot publication poisoned"))?;
        self.epoch.fetch_add(1, Ordering::AcqRel);
        let mut owners = self
            .namespace_epochs
            .lock()
            .map_err(|_| invalid("snapshot generations poisoned"))?;
        if let Some(owner) = owners.get(namespace).and_then(Weak::upgrade) {
            owner.generation.fetch_add(1, Ordering::AcqRel);
        }
        owners.retain(|_, owner| owner.strong_count() != 0);
        self.heads.remove(namespace);
        self.snapshots.drop_table_entries(namespace.reference())
    }
    pub(crate) fn replay_started(&self, incremental: bool) {
        if incremental {
            &self.refreshes
        } else {
            &self.loads
        }
        .fetch_add(1, Ordering::Relaxed);
    }
    pub(crate) fn unchanged(&self) {
        self.unchanged.fetch_add(1, Ordering::Relaxed);
    }
    pub(crate) fn writer_published(&self) {
        self.publications.fetch_add(1, Ordering::Relaxed);
    }
    pub(crate) fn clear(&self) {
        self.heads.clear();
        self.snapshots.clear();
    }
    pub(crate) fn counters(&self) -> NativeCacheCounters {
        NativeCacheCounters {
            family: NativeCacheFamily::DeltaSnapshots,
            entries: self.snapshots.len(),
            limit_bytes: self.snapshots.cache_limit() + self.heads.cache_limit(),
            occupied_bytes: Some(self.snapshots.memory_used() + self.heads.memory_used()),
            ownership: Some(
                self.usage
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .clone(),
            ),
            activity: enrichment_core::wire::status::NativeCacheActivity {
                cold_loads: Some(self.loads.load(Ordering::Relaxed)),
                incremental_refreshes: Some(self.refreshes.load(Ordering::Relaxed)),
                unchanged_refreshes: Some(self.unchanged.load(Ordering::Relaxed)),
                waits: Some(self.waits.load(Ordering::Relaxed)),
                writer_publications: Some(self.publications.load(Ordering::Relaxed)),
                oversize_bypasses: Some(self.bypasses.load(Ordering::Relaxed)),
                ..Default::default()
            },
        }
    }
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use datafusion::execution::memory_pool::GreedyMemoryPool;

    #[test]
    fn plan19_snapshot_reservations_follow_native_eviction_and_last_reader() -> Result<()> {
        #[derive(Clone)]
        struct ResidentEntry(Arc<Resident>);
        impl CacheValue for ResidentEntry {
            fn size(&self) -> usize {
                self.0.0.bytes
            }
        }
        let root = tempfile::tempdir()?;
        std::fs::create_dir(root.path().join("_delta_log"))?;
        let key = Namespace::read(root.path())?;
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(4096));
        let registry = Registry::new(
            &NativeCachePolicy {
                snapshot_entry_bytes: 1024,
                ..Default::default()
            },
            pool.clone(),
            2,
        );
        let usage = || registry.counters().ownership.unwrap();
        let filling = registry.reserve()?;
        assert_eq!(usage().in_flight_bytes, 1024);
        let captured = Arc::new(filling.capture(768)?);
        assert_eq!(usage().in_flight_bytes, 0);
        assert_eq!(usage().live_value_bytes, 768);
        let cache = DefaultCache::<Namespace, ResidentEntry>::new(4096);
        cache.put(
            &key,
            ResidentEntry(Arc::new(Resident::new(captured.clone()))),
        );
        assert_eq!(usage().resident_value_bytes, 768);
        let physical_reader = captured.clone();
        drop(captured);
        cache.clear();
        assert_eq!(usage().resident_value_bytes, 0);
        assert_eq!(usage().live_value_bytes, 768);
        assert_eq!(pool.reserved(), 768);
        drop(physical_reader);
        assert_eq!(pool.reserved(), 0);
        assert_eq!(usage().live_value_bytes, 0);
        // The entry residency ceiling is not a total workload ceiling; the native pool
        // still has to admit the entire captured reservation for uncached use.
        let oversized = registry.reserve()?.capture(2048)?;
        assert_eq!(pool.reserved(), 2048);
        assert_eq!(usage().live_value_bytes, 2048);
        assert_eq!(usage().resident_value_bytes, 0);
        drop(oversized);
        assert!(registry.reserve()?.capture(8192).is_err());
        assert_eq!(usage().in_flight_bytes, 0);
        assert_eq!(usage().live_value_bytes, 0);
        assert_eq!(pool.reserved(), 0);
        Ok(())
    }

    #[test]
    fn plan19_invalidation_fences_only_the_affected_namespace() -> Result<()> {
        let root = tempfile::tempdir()?;
        for name in ["left", "right"] {
            std::fs::create_dir_all(root.path().join(name).join("_delta_log"))?;
        }
        let left = Namespace::read(&root.path().join("left"))?;
        let right = Namespace::read(&root.path().join("right"))?;
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(65536));
        let registry = Registry::new(&NativeCachePolicy::default(), pool.clone(), 2);
        let a = registry.epoch(Some(&left))?;
        let b = registry.epoch(Some(&right))?;
        registry.invalidate(&left)?;
        assert!(!registry.epoch_current(&a));
        assert!(registry.epoch_current(&b));
        let current = registry.epoch(Some(&left))?;
        assert!(registry.epoch_current(&current));
        assert!(pool.reserved() > 0);
        drop((a, b, current));
        registry.invalidate(&left)?;
        assert_eq!(pool.reserved(), 0);
        assert!(registry.namespace_epochs.lock().unwrap().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn plan19_cancellation_releases_flight_and_reaps_only_dead_coordinates() -> Result<()> {
        let root = tempfile::tempdir()?;
        std::fs::create_dir(root.path().join("_delta_log"))?;
        let namespace = Namespace::read(root.path())?;
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(4096));
        let policy = NativeCachePolicy {
            snapshot_entry_bytes: 1024,
            ..Default::default()
        };
        let registry = Arc::new(Registry::new(&policy, pool.clone(), 2));
        let first = registry.flight(&namespace, None).await?;
        let waiting_registry = registry.clone();
        let waiting_namespace = namespace.clone();
        let waiter =
            tokio::spawn(async move { waiting_registry.flight(&waiting_namespace, None).await });
        while registry.slots.available_permits() != 0 {
            tokio::task::yield_now().await;
        }
        assert!(!waiter.is_finished());
        waiter.abort();
        assert!(waiter.await.unwrap_err().is_cancelled());
        assert_eq!(registry.slots.available_permits(), 1);
        drop(first);
        let next = registry.flight(&namespace, None).await?;
        assert_eq!(registry.flights.lock().unwrap().len(), 1);
        drop(next);
        assert_eq!(registry.slots.available_permits(), 2);
        let reservation = registry.reserve()?;
        assert_eq!(pool.reserved(), 1024);
        drop(reservation);
        assert_eq!(pool.reserved(), 0);
        Ok(())
    }
}
