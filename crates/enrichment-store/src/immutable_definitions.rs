//! Native immutable definitions, shared by policy and physical operation consumers.
use crate::{
    control::ControlStore,
    native_delta::{DeltaStore, StorageContract, transaction_conflict},
    retention::{Dependency, ProtectionKind, RetentionStore, TableSelection},
    runtime::QueryRuntime,
};
use arrow::datatypes::Schema;
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::evidence::arrow_model::cells::RowSet;
use enrichment_core::identity::DefinitionId;
use enrichment_core::native_union::NativeStruct;
use enrichment_core::native_union::{Rule, field};
use std::sync::Arc;

use enrichment_core::delta_reference::DeltaVersionRef;
#[derive(Clone)]
pub(crate) struct Definitions<D: DefinitionId> {
    definition: std::marker::PhantomData<D>,
    delta: DeltaStore,
    runtime: QueryRuntime,
    retention: RetentionStore,
}
impl<D: DefinitionId> Definitions<D> {
    pub(crate) fn new(control: ControlStore, runtime: QueryRuntime) -> Self {
        Self {
            delta: control.delta_namespace(),
            retention: RetentionStore::new(control, runtime.clone()),
            runtime,
            definition: std::marker::PhantomData,
        }
    }
    fn row_key(id: &D) -> crate::retention::RowKey {
        crate::retention::RowKey {
            column: D::COLUMN.into(),
            value: id.row_value(),
        }
    }
    fn contract() -> Result<StorageContract> {
        let mut fields = D::KEY.schema().fields().to_vec();
        fields.push(Arc::new(field::<D>(D::COLUMN, Rule::Text)));
        StorageContract::new(Arc::new(Schema::new(fields)))
    }
    pub(crate) async fn retain(&self, value: &D::Record) -> Result<(D, DeltaVersionRef)> {
        let input = crate::native_catalog::batch(
            &self.runtime.session(),
            "immutable_definitions",
            D::Record::batch(std::slice::from_ref(value))?,
        )?;
        self.retain_plan(input, vec![]).await
    }

    /// Native callers retain a bounded record without decoding facts into Rust objects.
    /// The exact materialized Arrow row is both hashed and written, so volatile inputs
    /// cannot change between identity selection and Delta publication.
    pub(crate) async fn retain_plan(
        &self,
        input: DataFrame,
        dependencies: Vec<Dependency>,
    ) -> Result<(D, DeltaVersionRef)> {
        let definitions = self.clone();
        self.runtime
            .spawn(async move { Box::pin(definitions.retain_owned(input, dependencies)).await })
            .await
            .map_err(|error| DataFusionError::External(Box::new(error)))?
    }

    /// Pure native admission shared by capture and its no-storage development oracle.
    async fn capture(runtime: &QueryRuntime, input: DataFrame) -> Result<(D, DataFrame)> {
        enrichment_core::native_schema::check_input(input.schema().as_arrow(), &D::KEY.schema())?;
        let captured = runtime.execute(input.limit(0, Some(2))?).await?;
        if captured.rows != 1 {
            return Err(invalid(
                "native definition requires exactly one bounded record",
            ));
        }
        let input = crate::native_catalog::captured_batches(
            &runtime.session(),
            "immutable_definitions",
            captured.batches,
        )?;
        if let Some(violations) =
            enrichment_core::native_schema::intrinsic_violations(input.clone(), &D::KEY.schema())?
        {
            runtime
                .require_empty(
                    violations.select(vec![lit(D::TABLE).alias("definition")])?,
                    "native_definition_fields",
                    "definition_capture",
                )
                .await?;
        }
        let selected = runtime
            .execute(
                input
                    .clone()
                    .select(vec![D::KEY.identity()?.alias(D::COLUMN)])?,
            )
            .await?;
        let batch = selected
            .batches
            .iter()
            .find(|batch| batch.num_rows() == 1)
            .ok_or_else(|| invalid("native definition identity absent"))?;
        enrichment_core::native_schema::check_input(
            batch.schema().as_ref(),
            &Schema::new(vec![field::<D>(D::COLUMN, Rule::Text)]),
        )?;
        let rows = RowSet::batch(batch)?;
        let id = D::decode(rows.row(0), D::COLUMN)?;
        Ok((id, input))
    }

    async fn retain_owned(
        &self,
        input: DataFrame,
        mut dependencies: Vec<Dependency>,
    ) -> Result<(D, DeltaVersionRef)> {
        let (id, input) = Self::capture(&self.runtime, input).await?;
        let mut pending = dependencies.clone();
        pending.push(crate::retention::pending_row(
            D::TABLE,
            &Self::contract()?,
            Self::row_key(&id),
        )?);
        let obligation = self
            .retention
            .create_obligation(id.to_string(), pending)
            .await?;
        let binding = self.retain_inner(&id, input).await?;
        dependencies.push(Self::dependency(&id, &binding));
        self.retention
            .publish_root(
                &obligation,
                format!(
                    "definition/{}/{}/{id}",
                    binding.table.table_id, binding.version
                ),
                dependencies,
            )
            .await?;
        Ok((id, binding))
    }

    pub(crate) fn dependency(id: &D, binding: &DeltaVersionRef) -> Dependency {
        Dependency::Table {
            value: TableSelection {
                source: binding.clone(),
                row: Some(Self::row_key(id)),
            },
        }
    }

    async fn retain_inner(&self, id: &D, input: DataFrame) -> Result<DeltaVersionRef> {
        let contract = Self::contract()?;
        self.delta.prepare_root(D::TABLE)?;
        for _ in 0..16 {
            let table = self
                .delta
                .open_or_create(D::TABLE, &contract, false, &[])
                .await?;
            let session = self.runtime.session();
            let current = session
                .read_table(self.delta.provider(&table, &contract).await?)?
                .filter(
                    crate::retention::selection::Selection::admit(
                        &Self::row_key(id),
                        &contract.semantic_schema(),
                    )?
                    .expression(),
                )?;
            let found = self
                .runtime
                .execute(current.clone().limit(0, Some(2))?)
                .await?;
            if found.rows > 1 {
                return Err(invalid("duplicate immutable native definition"));
            }
            if found.rows == 1 {
                self.runtime
                    .require_empty(
                        current
                            .filter(col(D::COLUMN).not_eq(D::KEY.identity()?))?
                            .select(vec![col(D::COLUMN)])?,
                        "native_definition_identity",
                        "definition_capture",
                    )
                    .await?;
                return crate::native_delta::capture_version(D::TABLE, &table, &contract);
            }
            let input = input.clone().with_column(D::COLUMN, D::KEY.identity()?)?;
            match self
                .delta
                .append(
                    table,
                    &contract,
                    input,
                    vec![deltalake::kernel::Transaction::new(
                        format!("{}/{id}", D::TABLE),
                        1,
                    )],
                )
                .await
            {
                Ok(table) => {
                    return crate::native_delta::capture_version(D::TABLE, &table, &contract);
                }
                Err(error) if transaction_conflict(&error) => continue,
                Err(error) => return Err(error),
            }
        }
        Err(invalid("native definition conflict bound exceeded"))
    }
    pub(crate) async fn read(&self, id: &D, binding: &DeltaVersionRef) -> Result<DataFrame> {
        let definitions = self.clone();
        let id = id.to_owned();
        let binding = binding.clone();
        self.runtime
            .spawn(async move { Box::pin(definitions.read_owned(&id, &binding)).await })
            .await
            .map_err(|error| DataFusionError::External(Box::new(error)))?
    }

    async fn read_owned(&self, id: &D, binding: &DeltaVersionRef) -> Result<DataFrame> {
        let contract = Self::contract()?;
        let protection = self
            .retention
            .enroll(
                id.to_string(),
                ProtectionKind::Query,
                vec![Self::dependency(id, binding)],
            )
            .await?;
        if binding.table.table_uri != D::TABLE {
            return Err(invalid("definition relation differs from declared table"));
        }
        let session = self.runtime.session();
        let selection = TableSelection {
            source: binding.clone(),
            row: Some(Self::row_key(id)),
        };
        let provider = self
            .delta
            .exact_provider(
                &selection,
                &contract,
                &crate::leases::ReadProtection::Durable(protection),
            )
            .await?;
        let frame = session.read_table(provider)?.filter(
            crate::retention::selection::Selection::admit(
                &Self::row_key(id),
                &contract.semantic_schema(),
            )?
            .expression(),
        )?;
        self.runtime
            .require_empty(
                frame
                    .clone()
                    .aggregate(
                        vec![],
                        vec![datafusion::functions_aggregate::expr_fn::count(lit(1)).alias("rows")],
                    )?
                    .filter(col("rows").not_eq(lit(1i64)))?
                    .select(vec![
                        lit("missing_or_ambiguous_definition").alias("witness"),
                    ])?,
                "exact_native_definition",
                "definition_binding",
            )
            .await?;
        self.runtime
            .require_empty(
                frame
                    .clone()
                    .filter(col(D::COLUMN).not_eq(D::KEY.identity()?))?
                    .select(vec![col(D::COLUMN)])?,
                "native_definition_identity",
                "definition_binding",
            )
            .await?;
        Ok(frame)
    }
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{config::Config, identity::*};

    /// CP12 service storage/recovery journey. Compile now; execute after the deletion barrier.
    #[tokio::test]
    async fn typed_definition_unknown_ack_recovery_reclaims_only_unrooted_binary_key() -> Result<()>
    {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let control = ControlStore::open(&directory.path().join("data"), runtime.clone())?;
        let definitions = Definitions::<OperationPolicyId>::new(control.clone(), runtime.clone());
        let contract = Definitions::<OperationPolicyId>::contract()?;
        let first = Config::default();
        let id = OperationPolicyId::from_record(&first);
        let pending = crate::retention::pending_row(
            OperationPolicyId::TABLE,
            &contract,
            Definitions::<OperationPolicyId>::row_key(&id),
        )?;
        let obligation = definitions
            .retention
            .create_obligation("unacknowledged-definition".into(), vec![pending])
            .await?;
        let input = crate::native_catalog::batch(
            &runtime.session(),
            "definition_recovery",
            Config::batch(std::slice::from_ref(&first))?,
        )?;
        let (_, input) = Definitions::<OperationPolicyId>::capture(&runtime, input).await?;
        definitions.retain_inner(&id, input).await?;
        // The writer physically exited, but the acknowledged table binding was lost.
        definitions
            .retention
            .release_obligation(&obligation, obligation.dependencies.clone())
            .await?;
        let mut second = first;
        second.network.max_redirects += 1;
        let (retained, binding) = definitions.retain(&second).await?;
        let reopened = Definitions::<OperationPolicyId>::new(
            ControlStore::open(&directory.path().join("data"), runtime.clone())?,
            runtime.clone(),
        );
        assert_eq!(reopened.retention.reconcile_writers().await?, 1);
        reopened
            .delta
            .reclaim(OperationPolicyId::TABLE, &contract, &reopened.retention)
            .await?;
        let current = reopened.delta.load(OperationPolicyId::TABLE, None).await?;
        let frame = runtime
            .session()
            .read_table(reopened.delta.provider(&current, &contract).await?)?;
        assert_eq!(runtime.execute(frame.clone()).await?.rows, 1);
        assert_eq!(
            runtime
                .execute(frame.filter(col(OperationPolicyId::COLUMN).eq(retained.literal()))?)
                .await?
                .rows,
            1
        );
        assert_eq!(
            runtime
                .execute(reopened.read(&retained, &binding).await?)
                .await?
                .rows,
            1
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[test]
    fn plan19_definition_declarations_own_all_eight_storage_contracts() -> Result<()> {
        fn family<D: DefinitionId>() -> Result<()> {
            let contract = Definitions::<D>::contract()?;
            assert_eq!(D::KEY.schema().fields(), &D::Record::fields());
            let schema = contract.semantic_schema();
            let field = schema.field_with_name(D::COLUMN)?;
            assert_eq!(
                field.data_type(),
                &arrow::datatypes::DataType::FixedSizeBinary(32)
            );
            assert_eq!(
                field.metadata().get("ARROW:extension:metadata"),
                D::metadata().get("ARROW:extension:metadata")
            );
            assert!(enrichment_core::native_schema::required(field));
            let physical = contract.storage_schema();
            let physical = physical.field_with_name(D::COLUMN)?;
            assert_eq!(physical.data_type(), &arrow::datatypes::DataType::Binary);
            assert!(!physical.is_nullable());
            Ok(())
        }
        family::<OperationPolicyId>()?;
        family::<ProcessOperationId>()?;
        family::<ProcessEffectId>()?;
        family::<StaticWorkerEffectId>()?;
        family::<RustdocDecoderEffectId>()?;
        family::<SemanticConversationId>()?;
        family::<RegistryCaptureId>()?;
        family::<RevisionCaptureId>()?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_definition_capture_admits_one_typed_immutable_record() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let config = Config::default();
        let session = runtime.session();
        for records in [vec![], vec![config.clone(), config.clone()]] {
            let input = crate::native_catalog::batch(
                &session,
                "definition_capture",
                Config::batch(&records)?,
            )?;
            assert!(
                Definitions::<OperationPolicyId>::capture(&runtime, input)
                    .await
                    .is_err()
            );
        }
        let input = crate::native_catalog::batch(
            &session,
            "definition_capture",
            Config::batch(std::slice::from_ref(&config))?,
        )?;
        let (id, captured) = Definitions::<OperationPolicyId>::capture(&runtime, input).await?;
        assert_eq!(id, OperationPolicyId::from_record(&config));
        let keyed = captured.with_column(
            OperationPolicyId::COLUMN,
            OperationPolicyId::KEY.identity()?,
        )?;
        let matched = keyed
            .clone()
            .filter(col(OperationPolicyId::COLUMN).eq(id.literal()))?;
        assert_eq!(runtime.execute(matched).await?.rows, 1);
        let corrupted = keyed.with_column(
            OperationPolicyId::COLUMN,
            OperationPolicyId::try_from(format!("policy_{}", "0".repeat(64)))
                .unwrap()
                .literal(),
        )?;
        assert_eq!(
            runtime
                .execute(corrupted.filter(
                    col(OperationPolicyId::COLUMN).not_eq(OperationPolicyId::KEY.identity()?)
                )?)
                .await?
                .rows,
            1
        );
        assert!(
            enrichment_core::native_schema::function_arguments(
                &[field::<OperationPolicyId>("id", Rule::Text).into()],
                &[field::<ProcessEffectId>("id", Rule::Text).into()]
            )
            .is_err()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
