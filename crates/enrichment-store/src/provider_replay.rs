//! Restricted persisted providers for publication-selected search outputs. Native Delta
//! stores the descriptor with its checkpoint; decoded bytes never restore read authority.
use crate::{
    leases::ReadProtection,
    native_delta::{DeltaStore, StorageContract},
    session_witness::Witness,
    snapshot_registry::Namespace,
};
use datafusion::{
    catalog::TableProvider,
    common::{DataFusionError, Result, TableReference},
    execution::memory_pool::MemoryConsumer,
};
use datafusion_proto::logical_plan::LogicalExtensionCodec;
use deltalake::delta_datafusion::{DeltaLogicalCodec, DeltaScanNext};
use enrichment_core::{
    evidence::snapshot::DeltaBinding, native_bytes::NativeBytes, native_digest::Sha256Digest,
    operation::projections::ReadDescriptor,
};
use std::sync::Arc;

pub const CODEC: &str = "delta-immutable-cbor/1";

impl DeltaStore {
    pub(crate) async fn read_descriptor(
        &self,
        binding: &DeltaBinding,
        contract: &StorageContract,
        protection: &ReadProtection,
    ) -> Result<ReadDescriptor> {
        let admitted = self
            .prepare_immutable_read(binding, contract, protection)
            .await?;
        let table = self
            .load(
                &binding.source.table.table_uri,
                Some(binding.source.version),
            )
            .await?;
        let session = self.session();
        admitted.verify_table(&table)?;
        let options = Witness::persisted_options(&session.state())?;
        let namespace = table.namespace().witness()?;
        let config = crate::native_discovery::scan_config(&session.state(), contract);
        let memory = MemoryConsumer::new("persisted-provider-encode")
            .register(&session.runtime_env().memory_pool);
        memory.try_grow(enrichment_core::native_bytes::MAX_BYTES * 3)?;
        let read = protection.clone();
        let (bytes, digest) = self
            .runtime
            .native_read(async move {
                let _owners = (memory, read, admitted);
                let provider = table
                    .table_provider()
                    .with_eager_snapshot(table.snapshot().map_err(external)?.snapshot().clone())
                    .with_scan_config(config)
                    .with_session(Arc::new(session.state()))
                    .build()
                    .await
                    .map_err(external)?;
                let mut bytes = Vec::new();
                DeltaLogicalCodec {}.encode_immutable_provider(&provider, &mut bytes)?;
                let bytes = NativeBytes::new(bytes).map_err(invalid)?;
                let digest = Sha256Digest::hash(bytes.as_slice())?;
                Ok((bytes, digest))
            })
            .await?;
        Ok(ReadDescriptor {
            binding: binding.clone(),
            namespace,
            codec: CODEC.into(),
            definition: crate::runtime::DEFINITION_REVISION.into(),
            options,
            digest,
            bytes,
        })
    }

    pub(crate) async fn replay_provider(
        &self,
        descriptor: &ReadDescriptor,
        binding: &DeltaBinding,
        contract: &StorageContract,
        protection: &ReadProtection,
    ) -> Result<Arc<dyn TableProvider>> {
        protection
            .require_tables(&self.root, std::slice::from_ref(binding))
            .await?;
        self.location(&binding.source.table.table_uri)?;
        let session = self.session();
        if descriptor.codec != CODEC
            || descriptor.definition != crate::runtime::DEFINITION_REVISION
            || descriptor.binding != *binding
            || &binding.source.table.contract_id != contract.identity()
            || descriptor.namespace
                != Namespace::read(&self.root.join(&binding.source.table.table_uri))?.witness()?
            || descriptor.options != Witness::persisted_options(&session.state())?
        {
            return Err(invalid("persisted provider dependency witness changed"));
        }
        let memory = MemoryConsumer::new("persisted-provider-digest")
            .register(&session.runtime_env().memory_pool);
        // Pinned SHAFunc/digest_process borrow the scalar input and allocate a 32-byte
        // result. Reserve the sole payload copy plus bounded invocation descriptors.
        memory.try_grow(descriptor.bytes.as_slice().len() + 4096)?;
        let bytes = descriptor.bytes.clone();
        let read = protection.clone();
        let digest = self
            .runtime
            .native_read(async move {
                let _owners = (memory, read);
                Sha256Digest::hash(bytes.as_slice())
            })
            .await?;
        if descriptor.digest != digest {
            return Err(invalid("persisted provider digest differs from bytes"));
        }
        let admitted = self
            .prepare_immutable_read(binding, contract, protection)
            .await?;
        if self.descriptor_verified(binding, contract, &descriptor.digest)? {
            return self
                .immutable_provider_admitted(binding, contract, admitted)
                .await;
        }
        let memory = self.runtime.snapshots.reserve()?;
        // Restricted CBOR cardinalities, uncompressed IPC bytes and decoded element
        // bounds are enforced before native allocation. This is admission, not RSS.
        let payload = descriptor.bytes.clone();
        let schema = contract.semantic_schema();
        let config = crate::native_discovery::scan_config(&session.state(), contract);
        let task = session.task_ctx();
        let reference = TableReference::bare(binding.source.table.table_uri.clone());
        let read = protection.clone();
        let namespace = admitted.history.namespace.clone();
        let decoded = self
            .runtime
            .native_read(async move {
                let _read = read;
                let provider = DeltaLogicalCodec {}.try_decode_table_provider(
                    payload.as_slice(),
                    &reference,
                    schema,
                    &task,
                )?;
                let scan = provider
                    .downcast_ref::<DeltaScanNext>()
                    .ok_or_else(|| invalid("persisted provider type"))?;
                let snapshot = scan.full_table_snapshot()?;
                let log = admitted.history.table.log_store();
                let mut table =
                    deltalake::DeltaTable::new(log.clone(), scan.snapshot().load_config().clone());
                table.state = Some(deltalake::table::state::DeltaTableState::new(
                    snapshot.as_ref().clone(),
                ));
                admitted.verify_table(&table)?;
                scan.clone().rebind_immutable(log, &config)?;
                Ok((table, memory, admitted))
            })
            .await?;
        let retained = self
            .admit_snapshot(decoded.0, decoded.1, Some(namespace))
            .await?;
        self.remember_descriptor(binding, contract, retained, descriptor.digest)?;
        // Build from the registry's selected shared allocation. A concurrent fill may
        // have won; retaining a different decoded allocation under its charge is invalid.
        self.immutable_provider_admitted(binding, contract, decoded.2)
            .await
    }
}
fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Plan(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
