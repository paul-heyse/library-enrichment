//! Current authority for exact reads. Cache residency never replaces history or protection.
use crate::{
    leases::ReadProtection,
    native_delta::{
        DeltaStore, LoadedTable, MetadataTable, StorageContract, capture_version, verify_contract,
    },
    snapshot_registry::Namespace,
};
use datafusion::{
    catalog::TableProvider,
    common::{DataFusionError, Result},
};
use enrichment_core::delta_reference::TableSelection;
use std::sync::Arc;

pub(crate) struct PreparedRead {
    selection: TableSelection,
    namespace: Namespace,
    pub(crate) history: MetadataTable,
    protection: ReadProtection,
}
impl PreparedRead {
    pub(crate) fn verify_table(&self, table: &deltalake::DeltaTable) -> Result<()> {
        self.history.verify_table(table)
    }
    pub(crate) fn require_selection(&self, selection: &TableSelection) -> Result<()> {
        if self.selection != *selection {
            return datafusion::common::plan_err!(
                "exact read admission differs from requested selection"
            );
        }
        Ok(())
    }
}
impl DeltaStore {
    pub(crate) async fn prepare_exact_read(
        &self,
        selection: &TableSelection,
        contract: &StorageContract,
        protection: &ReadProtection,
    ) -> Result<PreparedRead> {
        protection
            .require_selections(&self.root, std::slice::from_ref(selection))
            .await?;
        let source = &selection.source;
        self.location(&source.table.table_uri)?;
        if &source.table.contract_id != contract.identity() {
            return datafusion::common::plan_err!("exact read semantic contract mismatch");
        }
        if let Some(row) = &selection.row {
            crate::retention::selection::Selection::admit(row, &contract.semantic_schema())?;
        }
        let namespace = Namespace::read(&self.root.join(&source.table.table_uri))?;
        self.require_contract(contract).await?;
        let history = self
            .load_metadata(&source.table.table_uri, source.version)
            .await?;
        verify_contract(&history.table, contract, &self.session().state())?;
        if history.namespace != namespace
            || capture_version(&source.table.table_uri, &history.table, contract)? != *source
        {
            return datafusion::common::exec_err!("exact read durable authority changed");
        }
        Ok(PreparedRead {
            selection: selection.clone(),
            namespace,
            history,
            protection: protection.clone(),
        })
    }

    pub(crate) async fn exact_provider(
        &self,
        selection: &TableSelection,
        contract: &StorageContract,
        protection: &ReadProtection,
    ) -> Result<Arc<dyn TableProvider>> {
        let admitted = self
            .prepare_exact_read(selection, contract, protection)
            .await?;
        let table = self
            .load(
                &selection.source.table.table_uri,
                Some(selection.source.version),
            )
            .await?;
        self.provider_admitted(table, contract, admitted).await
    }

    pub(crate) async fn provider_admitted(
        &self,
        table: LoadedTable,
        contract: &StorageContract,
        admitted: PreparedRead,
    ) -> Result<Arc<dyn TableProvider>> {
        if table.namespace() != &admitted.namespace
            || capture_version(&admitted.selection.source.table.table_uri, &table, contract)?
                != admitted.selection.source
        {
            return Err(DataFusionError::Plan(
                "exact provider differs from admitted source".into(),
            ));
        }
        admitted.verify_table(&table)?;
        let session = self.session();
        let provider =
            crate::native_discovery::captured_provider(&session, table, contract.clone()).await?;
        let protected = admitted.protection.provider(provider, &session)?;
        Ok(session.read_table(protected)?.into_view())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        delta_reference::{DeltaTableRef, DeltaVersionRef},
        evidence::snapshot::DeltaBinding,
        identity::{CohortId, SchemaContractId},
        native_union::{NativeStruct, Rule, field},
    };

    #[tokio::test]
    async fn plan19_binary_registry_bounds_compile_without_implicit_coercion() -> Result<()> {
        use arrow::{
            array::BinaryArray,
            datatypes::{DataType, Schema},
            record_batch::RecordBatch,
        };
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        for kind in [DataType::Binary, DataType::LargeBinary] {
            let declared = field::<enrichment_core::native_bytes::NativeBytes>(
                "payload",
                Rule::BinaryBytes { max: 4 },
            )
            .with_data_type(kind.clone());
            let schema = Arc::new(Schema::new(vec![declared]));
            let data =
                BinaryArray::from_vec(vec![b"", &[0, 127, 128, 255], &[0, 127, 128, 255, 0]]);
            let batch =
                RecordBatch::try_new(schema.clone(), vec![arrow::compute::cast(&data, &kind)?])?;
            let frame = crate::native_catalog::batch(&runtime.session(), "registry_bounds", batch)?;
            let violations =
                enrichment_core::native_schema::intrinsic_violations(frame, &schema)?.unwrap();
            assert_eq!(runtime.execute(violations).await?.rows, 1);
        }
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn plan19_exact_reference_fields_survive_native_join_union_and_optimization() -> Result<()>
    {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let make = |digit: &str| DeltaBinding {
            relation: "symbols".into(),
            source: DeltaVersionRef {
                table: DeltaTableRef {
                    table_uri: "evidence_symbols".into(),
                    table_id: "opaque/external".into(),
                    contract_id: SchemaContractId::try_from(format!(
                        "schema_contract_{}",
                        digit.repeat(64)
                    ))
                    .unwrap(),
                },
                version: 3,
            },
            cohort_id: CohortId::from_component(&digit.repeat(32)).unwrap(),
            rows: 0,
        };
        let bindings = [make("1"), make("2")];
        let session = runtime.session();
        crate::native_catalog::work(
            &session,
            "scope_inputs",
            crate::native_catalog::batch(&session, "typed_delta", DeltaBinding::batch(&bindings)?)?
                .into_view(),
        )?;
        let frame = session.sql("WITH scopes AS (SELECT source.table.contract_id AS contract_id,cohort_id FROM scope_inputs), selected AS (SELECT native_coalesce(a.contract_id,b.contract_id) AS contract_id,native_coalesce(a.cohort_id,b.cohort_id) AS cohort_id FROM scopes a FULL JOIN scopes b ON a.cohort_id=b.cohort_id) SELECT * FROM selected UNION ALL SELECT * FROM scopes ORDER BY cohort_id").await?;
        let physical = frame.clone().create_physical_plan().await?;
        let physical_schema = physical.schema();
        let optimized = frame.clone().into_optimized_plan()?;
        for schema in [
            frame.schema().as_arrow(),
            optimized.schema().as_arrow(),
            physical_schema.as_ref(),
        ] {
            for expected in [
                field::<SchemaContractId>("contract_id", Rule::Text),
                field::<CohortId>("cohort_id", Rule::Text),
            ] {
                let actual = schema.field_with_name(expected.name())?;
                assert_eq!(actual.data_type(), expected.data_type());
                assert_eq!(actual.metadata(), expected.metadata());
            }
        }
        enrichment_core::native_struct! { struct Output { contract_id: SchemaContractId => Rule::Text, cohort_id: CohortId => Rule::Text } }
        let rows = runtime.records::<Output>(frame, 4).await?;
        assert_eq!(rows.len(), 4);
        for (row, binding) in
            rows.iter()
                .zip([&bindings[0], &bindings[0], &bindings[1], &bindings[1]])
        {
            assert_eq!(row.contract_id, binding.source.table.contract_id);
            assert_eq!(row.cohort_id, binding.cohort_id);
        }
        runtime.close_diagnostics().await
    }
}
