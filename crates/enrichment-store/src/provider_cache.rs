//! Typed immutable provider ingredients. Cached state contains no session, lease, effects,
//! codec bytes or query runtime. Every preparation binds current native handles/protection.
use crate::{
    leases::ReadProtection,
    native_delta::{DeltaStore, LoadedTable, StorageContract, verify_contract},
    runtime::DEFINITION_REVISION,
    snapshot_registry::Namespace,
};
use datafusion::{
    catalog::TableProvider,
    common::TableReference,
    error::{DataFusionError, Result},
    execution::{
        cache::{Cache, CacheKey, CacheValue, default_cache::DefaultCache},
        memory_pool::{MemoryConsumer, MemoryReservation},
    },
};
use enrichment_core::{evidence::snapshot::DeltaBinding, native_digest::Sha256Digest};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Key {
    table: TableReference,
    namespace: Namespace,
    source: enrichment_core::delta_reference::DeltaVersionRef,
    cohort: enrichment_core::identity::CohortId,
    definition: &'static str,
    options: Arc<crate::session_witness::Witness>,
}
impl CacheKey for Key {
    fn size(&self) -> usize {
        size_of::<Self>()
            + self.table.to_string().len()
            + self.namespace.size()
            + self.source.table.table_id.capacity()
            + self.source.table.table_uri.capacity()
    }
    fn table_ref(&self) -> Option<&TableReference> {
        Some(&self.table)
    }
}
struct Ingredients {
    table: LoadedTable,
    descriptor_digest: Option<Sha256Digest>,
    bytes: usize,
    _key_memory: MemoryReservation,
}

use crate::read_admission::PreparedRead;
#[derive(Clone)]
struct Descriptor(Arc<Ingredients>);
impl CacheValue for Descriptor {
    fn size(&self) -> usize {
        self.0.bytes
    }
}

pub(crate) struct ProviderCache(DefaultCache<Key, Descriptor>, AtomicU64, AtomicU64);
impl ProviderCache {
    pub(crate) fn invalidate(&self, namespace: &Namespace) -> Result<()> {
        self.0.drop_table_entries(namespace.reference())
    }
    pub(crate) fn clear(&self) {
        self.0.clear();
    }
    pub(crate) fn counters(&self) -> enrichment_core::wire::status::NativeCacheCounters {
        enrichment_core::wire::status::NativeCacheCounters {
            family: enrichment_core::wire::status::NativeCacheFamily::ImmutableProviders,
            entries: self.0.len(),
            limit_bytes: self.0.cache_limit(),
            occupied_bytes: Some(self.0.memory_used()),
            ownership: None,
            activity: enrichment_core::wire::status::NativeCacheActivity {
                hits: Some(self.1.load(Ordering::Relaxed)),
                misses: Some(self.2.load(Ordering::Relaxed)),
                ..Default::default()
            },
        }
    }
    pub(crate) fn new(bytes: usize) -> Self {
        Self(
            DefaultCache::new(bytes).with_name("delta-provider-ingredients"),
            AtomicU64::new(0),
            AtomicU64::new(0),
        )
    }
}

impl DeltaStore {
    fn ingredient_key(&self, binding: &DeltaBinding) -> Result<Key> {
        let session = self.session();
        self.location(&binding.source.table.table_uri)?;
        let namespace = Namespace::read(&self.root.join(&binding.source.table.table_uri))?;
        Ok(Key {
            table: namespace.reference().clone(),
            namespace,
            source: binding.source.clone(),
            cohort: binding.cohort_id,
            definition: DEFINITION_REVISION,
            options: crate::session_witness::Witness::get(&session.state()),
        })
    }
    fn remember_ingredients(
        &self,
        key: Key,
        table: LoadedTable,
        descriptor_digest: Option<Sha256Digest>,
    ) -> Result<()> {
        if !table.cacheable() {
            return Ok(());
        }
        let memory = MemoryConsumer::new("provider-ingredients")
            .register(&self.session().runtime_env().memory_pool);
        memory.try_grow(key.size() + size_of::<Ingredients>())?;
        let bytes = size_of::<Ingredients>() + table.size();
        self.runtime.descriptors.0.put(
            &key,
            Descriptor(Arc::new(Ingredients {
                table,
                _key_memory: memory,
                descriptor_digest,
                bytes,
            })),
        );
        Ok(())
    }
    pub(crate) fn descriptor_verified(
        &self,
        binding: &DeltaBinding,
        contract: &StorageContract,
        digest: &Sha256Digest,
    ) -> Result<bool> {
        if &binding.source.table.contract_id != contract.identity() {
            return Err(invalid("provider ingredient contract mismatch"));
        }
        let key = self.ingredient_key(binding)?;
        Ok(self
            .runtime
            .descriptors
            .0
            .get(&key)
            .is_some_and(|value| value.0.descriptor_digest.as_ref() == Some(digest)))
    }
    pub(crate) fn remember_descriptor(
        &self,
        binding: &DeltaBinding,
        contract: &StorageContract,
        table: LoadedTable,
        digest: Sha256Digest,
    ) -> Result<()> {
        if &binding.source.table.contract_id != contract.identity() {
            return Err(invalid("provider ingredient contract mismatch"));
        }
        let key = self.ingredient_key(binding)?;
        if table.namespace() != &key.namespace {
            return Err(invalid(
                "persisted provider namespace changed during decode",
            ));
        }
        self.remember_ingredients(key, table, Some(digest))
    }
    pub(crate) async fn immutable_provider(
        &self,
        binding: &DeltaBinding,
        contract: &StorageContract,
        protection: &ReadProtection,
    ) -> Result<Arc<dyn TableProvider>> {
        let admitted = self
            .prepare_immutable_read(binding, contract, protection)
            .await?;
        self.immutable_provider_admitted(binding, contract, admitted)
            .await
    }
    pub(crate) async fn prepare_immutable_read(
        &self,
        binding: &DeltaBinding,
        contract: &StorageContract,
        protection: &ReadProtection,
    ) -> Result<PreparedRead> {
        self.prepare_exact_read(&binding.selection(), contract, protection)
            .await
    }

    pub(crate) async fn immutable_provider_admitted(
        &self,
        binding: &DeltaBinding,
        contract: &StorageContract,
        admitted: PreparedRead,
    ) -> Result<Arc<dyn TableProvider>> {
        let session = self.session();
        if &binding.source.table.contract_id != contract.identity() {
            return Err(invalid("provider ingredient contract mismatch"));
        }
        let key = self.ingredient_key(binding)?;
        admitted.require_selection(&binding.selection())?;
        let table = if let Some(descriptor) = self.runtime.descriptors.0.get(&key) {
            self.runtime.descriptors.1.fetch_add(1, Ordering::Relaxed);
            descriptor.0.table.clone()
        } else {
            self.runtime.descriptors.2.fetch_add(1, Ordering::Relaxed);
            let table = self
                .load(
                    &binding.source.table.table_uri,
                    Some(binding.source.version),
                )
                .await?;
            validate_identity(&table, binding, contract, &session.state())?;
            self.remember_ingredients(key, table.clone(), None)?;
            table
        };
        validate_identity(&table, binding, contract, &session.state())?;
        self.provider_admitted(table, contract, admitted).await
    }
}
fn validate_identity(
    table: &LoadedTable,
    binding: &DeltaBinding,
    contract: &StorageContract,
    session: &dyn datafusion::catalog::Session,
) -> Result<()> {
    if table.version() != Some(binding.source.version)
        || table
            .snapshot()
            .map_err(|error| DataFusionError::External(Box::new(error)))?
            .metadata()
            .id()
            != binding.source.table.table_id
    {
        return Err(invalid(
            "immutable provider table identity/version mismatch",
        ));
    }
    verify_contract(table, contract, session)
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}

#[cfg(test)]
mod tests;
