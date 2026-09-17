//! Restricted immutable Delta provider descriptors. Cache membership conveys no read
//! authority: every caller supplies current protection and rebuilds runtime handles.
use crate::{
    leases::ReadProtection,
    native_delta::{DeltaStore, StorageContract, verify_snapshot_contract},
    runtime::DEFINITION_REVISION,
};
use datafusion::{
    catalog::TableProvider,
    common::TableReference,
    error::{DataFusionError, Result},
    execution::{
        cache::{Cache, CacheKey, CacheValue, default_cache::DefaultCache},
        memory_pool::{MemoryConsumer, MemoryPool, MemoryReservation},
    },
};
use datafusion_proto::logical_plan::LogicalExtensionCodec;
use deltalake::delta_datafusion::{DeltaLogicalCodec, DeltaScanNext};
use enrichment_core::evidence::snapshot::DeltaBinding;
use std::{io::Write, sync::Arc, time::Duration};

const MAX_DESCRIPTOR_BYTES: usize = 64 * 1024 * 1024;
const CODEC: &str = "delta-immutable-provider/1";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Key {
    table: TableReference,
    root: String,
    table_id: String,
    version: u64,
    cohort: String,
    contract: String,
    definition: &'static str,
    codec: &'static str,
    options: Vec<(String, Option<String>)>,
}
impl CacheKey for Key {
    fn size(&self) -> usize {
        size_of::<Self>()
            + self.table.to_string().len()
            + self.root.capacity()
            + self.table_id.capacity()
            + self.cohort.capacity()
            + self.contract.capacity()
            + self.options.capacity() * size_of::<(String, Option<String>)>()
            + self
                .options
                .iter()
                .map(|(key, value)| key.capacity() + value.as_ref().map_or(0, String::capacity))
                .sum::<usize>()
    }
    fn table_ref(&self) -> Option<&TableReference> {
        Some(&self.table)
    }
}

struct Encoded {
    bytes: Vec<u8>,
    _memory: MemoryReservation,
}
#[derive(Clone)]
struct Descriptor(Arc<Encoded>);
impl CacheValue for Descriptor {
    fn size(&self) -> usize {
        size_of::<Encoded>() + self.0.bytes.capacity()
    }
}

pub(crate) struct ProviderCache(DefaultCache<Key, Descriptor>);
impl ProviderCache {
    pub(crate) fn clear(&self) {
        self.0.clear();
    }
    pub(crate) fn counters(&self) -> enrichment_core::wire::status::NativeCacheCounters {
        enrichment_core::wire::status::NativeCacheCounters {
            family: enrichment_core::wire::status::NativeCacheFamily::ImmutableProviders,
            entries: self.0.len(),
            limit_bytes: self.0.cache_limit(),
            occupied_bytes: Some(self.0.memory_used()),
        }
    }
    pub(crate) fn new(bytes: usize) -> Self {
        Self(DefaultCache::new_with_ttl(bytes, Some(Duration::from_secs(300))).with_name(CODEC))
    }
}

/// Encodes into a bounded buffer with reservation *before* allocation. The value owns
/// this reservation independently of native cache occupancy, including after eviction.
struct Encoder {
    bytes: Vec<u8>,
    memory: MemoryReservation,
    key_bytes: usize,
}
impl Encoder {
    fn new(pool: &Arc<dyn MemoryPool>, key_bytes: usize) -> Result<Self> {
        let memory = MemoryConsumer::new("immutable-provider-encoded").register(pool);
        memory.try_grow(key_bytes + size_of::<Encoded>())?;
        Ok(Self {
            bytes: Vec::new(),
            memory,
            key_bytes,
        })
    }
    fn finish(self) -> Descriptor {
        Descriptor(Arc::new(Encoded {
            bytes: self.bytes,
            _memory: self.memory,
        }))
    }
}
impl Write for Encoder {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let end = self
            .bytes
            .len()
            .checked_add(bytes.len())
            .filter(|size| *size <= MAX_DESCRIPTOR_BYTES)
            .ok_or_else(|| {
                std::io::Error::other("immutable provider descriptor exceeds byte bound")
            })?;
        if end > self.bytes.capacity() {
            let capacity = end.next_power_of_two().min(MAX_DESCRIPTOR_BYTES);
            self.memory
                .try_resize(capacity + self.key_bytes + size_of::<Encoded>())
                .map_err(std::io::Error::other)?;
            self.bytes
                .try_reserve_exact(capacity - self.bytes.len())
                .map_err(std::io::Error::other)?;
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl DeltaStore {
    /// Consume the native logical codec's table hook only. No generic logical-plan,
    /// extension, command, CDF or physical-plan decoding enters this path.
    pub(crate) async fn immutable_provider(
        &self,
        binding: &DeltaBinding,
        contract: &StorageContract,
        protection: &ReadProtection,
    ) -> Result<Arc<dyn TableProvider>> {
        let session = self.session();
        let mut options = session
            .state()
            .config_options()
            .entries()
            .into_iter()
            .map(|option| (option.key, option.value))
            .collect::<Vec<_>>();
        options.sort();
        let key = Key {
            table: TableReference::bare(binding.table_uri.clone()),
            root: self.location(&binding.table_uri)?.to_string(),
            table_id: binding.table_id.clone(),
            version: binding.version,
            cohort: binding.cohort_id.clone(),
            contract: contract.identity().into(),
            definition: DEFINITION_REVISION,
            codec: CODEC,
            options,
        };
        if binding.contract_id != key.contract {
            return Err(invalid("immutable provider semantic contract mismatch"));
        }
        // EvidenceTables admits the complete vector before calling this function. The
        // captured protection is attached on both paths and never enters the cache.
        let provider = if let Some(descriptor) = self.runtime.descriptors.0.get(&key) {
            let memory = MemoryConsumer::new("immutable-provider-decoded")
                .register(&session.runtime_env().memory_pool);
            let budget = descriptor
                .0
                .bytes
                .len()
                .checked_mul(8)
                .and_then(|n| n.checked_add(128 * 1024))
                .ok_or_else(|| invalid("immutable descriptor reservation overflow"))?;
            memory.try_grow(budget)?;
            let decoded = DeltaLogicalCodec {}.try_decode_table_provider(
                &descriptor.0.bytes,
                &key.table,
                contract.semantic_schema(),
                &session.task_ctx(),
            )?;
            let scan = decoded
                .downcast_ref::<DeltaScanNext>()
                .ok_or_else(|| invalid("immutable codec returned an unsupported provider"))?;
            validate_identity(scan, binding, contract, &session.state())?;
            let bound = scan.clone().rebind_immutable(
                self.log_store(&binding.table_uri)?,
                &crate::native_discovery::scan_config(&session.state(), contract),
            )?;
            crate::leases::accounted_provider(Arc::new(bound), Arc::new(memory))
        } else {
            self.require_contract(contract).await?;
            let table = self.load(&binding.table_uri, Some(binding.version)).await?;
            let provider =
                crate::native_discovery::captured_provider(&session, table, contract.clone())
                    .await?;
            let scan = provider
                .downcast_ref::<DeltaScanNext>()
                .ok_or_else(|| invalid("owned factory returned an unsupported provider"))?;
            validate_identity(scan, binding, contract, &session.state())?;
            let mut encoder = Encoder::new(&session.runtime_env().memory_pool, key.size())?;
            DeltaLogicalCodec {}.encode_immutable_provider(provider.as_ref(), &mut encoder)?;
            self.runtime.descriptors.0.put(&key, encoder.finish());
            provider
        };
        // A view is a read-only boundary: rebinding the native LogStore must not expose
        // DeltaScan's raw insertion hook to consumers.
        let protected = protection.clone().provider(provider, &session)?;
        Ok(session.read_table(protected)?.into_view())
    }
}

fn validate_identity(
    scan: &DeltaScanNext,
    binding: &DeltaBinding,
    contract: &StorageContract,
    session: &dyn datafusion::catalog::Session,
) -> Result<()> {
    if scan.snapshot().version() != binding.version
        || scan.snapshot().metadata().id() != binding.table_id
    {
        return Err(invalid(
            "immutable provider table identity/version mismatch",
        ));
    }
    verify_snapshot_contract(scan.snapshot().table_configuration(), contract, session)
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}

#[cfg(test)]
mod tests;
