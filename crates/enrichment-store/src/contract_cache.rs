//! Positive validation proofs under one native byte/eviction policy.
//! A miss/error never becomes a cache entry. Physical scope and semantic meaning are witnesses;
//! neither a proof nor its cache residency carries read, effect or retention authority.
use crate::{native_delta::StorageContract, snapshot_registry::Namespace};
use arrow::datatypes::SchemaRef;
use datafusion::{
    common::TableReference,
    error::Result,
    execution::{
        cache::{Cache, CacheKey, CacheValue, default_cache::DefaultCache},
        memory_pool::{MemoryConsumer, MemoryPool, MemoryReservation},
    },
};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Key {
    namespace: Namespace,
    registry: String,
    contract: enrichment_core::identity::SchemaContractId,
    semantic: SchemaRef,
    storage: SchemaRef,
    compiler: &'static str,
}
impl Key {
    pub(crate) fn new(namespace: Namespace, registry: String, contract: &StorageContract) -> Self {
        Self {
            namespace,
            registry,
            contract: contract.identity().clone(),
            semantic: contract.semantic_schema(),
            storage: contract.storage_schema(),
            compiler: crate::runtime::DEFINITION_REVISION,
        }
    }
}
impl CacheKey for Key {
    fn size(&self) -> usize {
        size_of::<Self>()
            + self.namespace.size()
            + self.registry.capacity()
            + [&self.semantic, &self.storage]
                .into_iter()
                .map(|schema| {
                    schema.fields().size()
                        + schema
                            .metadata()
                            .iter()
                            .map(|(key, value)| key.capacity() + value.capacity())
                            .sum::<usize>()
                })
                .sum::<usize>()
    }
    fn table_ref(&self) -> Option<&TableReference> {
        Some(self.namespace.reference())
    }
}
/// Exact immutable scope of a snapshot proof. The descriptor digest includes all version,
/// row/cohort, component, count and coverage declarations; namespace witnesses defeat root reuse.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct EvidenceScope {
    namespaces: Vec<Namespace>,
    descriptor: String,
    runtime: String,
    limits: crate::dataset::WriteLimits,
}
impl EvidenceScope {
    pub(crate) fn new(
        root: &std::path::Path,
        manifest: &enrichment_core::evidence::snapshot::EvidenceManifest,
        descriptor: &str,
        runtime: &crate::runtime::QueryRuntime,
        limits: &crate::dataset::WriteLimits,
    ) -> Result<Self> {
        Ok(Self {
            namespaces: manifest
                .tables
                .iter()
                .map(|table| {
                    Namespace::read(&root.join("delta").join(&table.source.table.table_uri))
                })
                .collect::<Result<_>>()?,
            descriptor: descriptor.into(),
            runtime: runtime.selection_witness().1.clone(),
            limits: limits.clone(),
        })
    }
    fn size(&self) -> usize {
        size_of::<Self>()
            + self.descriptor.capacity()
            + self.runtime.capacity()
            + self.namespaces.iter().map(CacheKey::size).sum::<usize>()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum ValidationKey {
    Blob {
        path: std::path::PathBuf,
        digest: String,
        bytes: u64,
        physical: crate::provider::FileWitness,
    },
    Snapshot(EvidenceScope),
    Attempts {
        evidence: EvidenceScope,
        catalog: Namespace,
        identity: String,
    },
}
impl ValidationKey {
    fn size(&self) -> usize {
        size_of::<Self>()
            + match self {
                Self::Blob { path, digest, .. } => path.as_os_str().len() + digest.capacity(),
                Self::Snapshot(scope) => scope.size(),
                Self::Attempts {
                    evidence,
                    catalog,
                    identity,
                } => evidence.size() + catalog.size() + identity.capacity(),
            }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum ProofKey {
    Storage(Key),
    Validation(ValidationKey),
}
impl CacheKey for ProofKey {
    fn size(&self) -> usize {
        size_of::<Self>()
            + match self {
                Self::Storage(key) => key.size(),
                Self::Validation(key) => key.size(),
            }
    }
    fn table_ref(&self) -> Option<&TableReference> {
        match self {
            Self::Storage(key) => key.table_ref(),
            Self::Validation(_) => None,
        }
    }
}

pub(crate) struct Proof {
    logs: Vec<(String, u64)>,
    bytes: usize,
    _memory: MemoryReservation,
}
impl Proof {
    pub(crate) fn logs(&self) -> &[(String, u64)] {
        &self.logs
    }
}
#[derive(Clone)]
struct Verified(Arc<Proof>);
impl CacheValue for Verified {
    fn size(&self) -> usize {
        self.0.bytes
    }
}
pub(crate) struct Contracts {
    cache: DefaultCache<ProofKey, Verified>,
    pool: Arc<dyn MemoryPool>,
    hits: AtomicU64,
    misses: AtomicU64,
}
impl Contracts {
    pub(crate) fn new(bytes: usize, pool: Arc<dyn MemoryPool>) -> Self {
        Self {
            cache: DefaultCache::new(bytes).with_name("verified-contracts"),
            pool,
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }
    pub(crate) fn contains(&self, key: &Key) -> bool {
        self.get(&ProofKey::Storage(key.clone())).is_some()
    }
    pub(crate) fn admit(&self, key: &Key) -> Result<()> {
        self.put(ProofKey::Storage(key.clone()), vec![])
    }
    pub(crate) fn validation(&self, key: &ValidationKey) -> Option<Arc<Proof>> {
        self.get(&ProofKey::Validation(key.clone()))
    }
    pub(crate) fn admit_validation(
        &self,
        key: ValidationKey,
        logs: Vec<(String, u64)>,
    ) -> Result<()> {
        self.put(ProofKey::Validation(key), logs)
    }
    fn get(&self, key: &ProofKey) -> Option<Arc<Proof>> {
        let found = self.cache.get(key).map(|value| value.0);
        if found.is_some() {
            &self.hits
        } else {
            &self.misses
        }
        .fetch_add(1, Ordering::Relaxed);
        found
    }
    fn put(&self, key: ProofKey, logs: Vec<(String, u64)>) -> Result<()> {
        let bytes = size_of::<Verified>()
            + size_of::<Proof>()
            + logs.capacity() * size_of::<(String, u64)>()
            + logs
                .iter()
                .map(|(digest, _)| digest.capacity())
                .sum::<usize>();
        let memory = MemoryConsumer::new("verified-contract").register(&self.pool);
        memory.try_grow(key.size() + bytes)?;
        self.cache.put(
            &key,
            Verified(Arc::new(Proof {
                _memory: memory,
                logs,
                bytes,
            })),
        );
        Ok(())
    }
    pub(crate) fn invalidate(&self, _: &Namespace) -> Result<()> {
        // An evidence proof can span many tables. Authority invalidation conservatively
        // evicts the bounded shared proof cache; it does not enumerate or copy its entries.
        self.cache.clear();
        Ok(())
    }
    pub(crate) fn clear(&self) {
        self.cache.clear();
    }
    pub(crate) fn counters(&self) -> enrichment_core::wire::status::NativeCacheCounters {
        enrichment_core::wire::status::NativeCacheCounters {
            family: enrichment_core::wire::status::NativeCacheFamily::VerifiedContracts,
            entries: self.cache.len(),
            limit_bytes: self.cache.cache_limit(),
            occupied_bytes: Some(self.cache.memory_used()),
            ownership: None,
            activity: enrichment_core::wire::status::NativeCacheActivity {
                hits: Some(self.hits.load(Ordering::Relaxed)),
                misses: Some(self.misses.load(Ordering::Relaxed)),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::datatypes::{DataType, Field, Schema};
    use datafusion::execution::memory_pool::GreedyMemoryPool;

    #[test]
    fn shared_validation_proofs_bind_physical_scope_and_retain_live_reservations() -> Result<()> {
        let root = tempfile::tempdir()?;
        let path = root.path().join("artifact");
        std::fs::write(&path, b"data")?;
        std::fs::create_dir(root.path().join("_delta_log"))?;
        let namespace = Namespace::read(root.path())?;
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(1024 * 1024));
        let cache = Contracts::new(1024 * 1024, pool.clone());
        let blob = ValidationKey::Blob {
            path: path.clone(),
            digest: "a".repeat(64),
            bytes: 4,
            physical: crate::provider::FileWitness::read(&path)?,
        };
        assert!(cache.validation(&blob).is_none());
        cache.admit_validation(blob.clone(), vec![("b".repeat(64), 5)])?;
        let held = cache.validation(&blob).unwrap();
        assert_eq!(held.logs(), &[("b".repeat(64), 5)]);
        let reserved = pool.reserved();
        assert!(reserved > 0);
        cache.clear();
        assert!(cache.validation(&blob).is_none());
        assert_eq!(
            pool.reserved(),
            reserved,
            "an evicted proof is still physically owned"
        );
        drop(held);
        assert_eq!(pool.reserved(), 0);
        cache.admit_validation(blob.clone(), vec![])?;
        std::fs::rename(&path, root.path().join("old-artifact"))?;
        std::fs::write(&path, b"data")?;
        let replaced = ValidationKey::Blob {
            path: path.clone(),
            digest: "a".repeat(64),
            bytes: 4,
            physical: crate::provider::FileWitness::read(&path)?,
        };
        assert!(cache.validation(&replaced).is_none());
        let scope = EvidenceScope {
            namespaces: vec![namespace.clone()],
            descriptor: "manifest".into(),
            runtime: "runtime/1".into(),
            limits: Default::default(),
        };
        cache.admit_validation(ValidationKey::Snapshot(scope.clone()), vec![])?;
        let mut changed = scope.clone();
        changed.runtime = "runtime/2".into();
        assert!(
            cache
                .validation(&ValidationKey::Snapshot(changed))
                .is_none()
        );
        let mut changed = scope.clone();
        changed.limits.table_rows -= 1;
        assert!(
            cache
                .validation(&ValidationKey::Snapshot(changed))
                .is_none()
        );
        let attempt = ValidationKey::Attempts {
            evidence: scope.clone(),
            catalog: namespace.clone(),
            identity: "catalog:4".into(),
        };
        cache.admit_validation(attempt, vec![])?;
        assert!(
            cache
                .validation(&ValidationKey::Attempts {
                    evidence: scope,
                    catalog: namespace.clone(),
                    identity: "catalog:5".into()
                })
                .is_none()
        );
        cache.invalidate(&namespace)?;
        assert_eq!(pool.reserved(), 0);
        assert!(cache.validation(&blob).is_none());
        Ok(())
    }

    #[test]
    fn plan19_positive_proof_requires_semantic_and_registry_identity() -> Result<()> {
        let root = tempfile::tempdir()?;
        std::fs::create_dir(root.path().join("_delta_log"))?;
        let namespace = Namespace::read(root.path())?;
        let schema = Schema::new(vec![Field::new("value", DataType::Int64, true)]);
        let plain = StorageContract::new(Arc::new(schema.clone()))?;
        let semantic = StorageContract::new(Arc::new(
            schema.with_metadata(
                [("meaning".to_owned(), "changed".to_owned())]
                    .into_iter()
                    .collect(),
            ),
        ))?;
        assert_eq!(plain.storage_schema(), semantic.storage_schema());
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(1024 * 1024));
        let cache = Contracts::new(1024 * 1024, pool.clone());
        let key = Key::new(namespace.clone(), "registry-a".into(), &plain);
        assert!(!cache.contains(&key));
        cache.admit(&key)?;
        assert!(cache.contains(&key));
        assert!(!cache.contains(&Key::new(namespace.clone(), "registry-b".into(), &plain)));
        assert!(!cache.contains(&Key::new(namespace, "registry-a".into(), &semantic)));
        assert!(pool.reserved() > 0);
        std::fs::rename(root.path().join("_delta_log"), root.path().join("old_log"))?;
        std::fs::create_dir(root.path().join("_delta_log"))?;
        let replacement = Key::new(Namespace::read(root.path())?, "registry-a".into(), &plain);
        assert!(!cache.contains(&replacement));
        cache.clear();
        assert_eq!(pool.reserved(), 0);
        Ok(())
    }
}
