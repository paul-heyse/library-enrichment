//! One native contract for immutable cohorts in purpose-specific Delta tables.
use crate::native_delta::{DeltaStore, StorageContract, transaction_conflict};
use arrow::datatypes::{Schema, SchemaRef};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::evidence::snapshot::DeltaBinding;
use std::sync::Arc;

pub(crate) fn contract(schema: SchemaRef) -> Result<StorageContract> {
    let mut fields = vec![Arc::new(enrichment_core::native_union::field::<
        enrichment_core::identity::CohortId,
    >(
        "cohort_id",
        enrichment_core::native_union::Rule::Text,
    ))];
    fields.extend(schema.fields().iter().cloned());
    StorageContract::new(Arc::new(Schema::new_with_metadata(
        fields,
        schema.metadata().clone(),
    )))
}

impl DeltaStore {
    pub(crate) async fn append_cohort(
        &self,
        name: &str,
        relation: &str,
        cohort: &enrichment_core::identity::CohortId,
        contract: &StorageContract,
        input: DataFrame,
        rows: u64,
    ) -> Result<DeltaBinding> {
        self.prepare_root(name)?;
        let columns = std::iter::once(lit(cohort).alias("cohort_id"))
            .chain(
                input
                    .schema()
                    .fields()
                    .iter()
                    .map(|field| col(field.name())),
            )
            .collect::<Vec<_>>();
        let input = input.select(columns)?;
        for _ in 0..16 {
            let table = self.open_or_create(name, contract, true, &[]).await?;
            match self.append(table, contract, input.clone(), vec![]).await {
                Ok(table) => {
                    return Ok(DeltaBinding {
                        source: super::native_delta::capture_version(name, &table, contract)?,
                        relation: relation.into(),
                        cohort_id: *cohort,
                        rows,
                    });
                }
                Err(error) if transaction_conflict(&error) => continue,
                Err(error) => return Err(error),
            }
        }
        Err(invalid("cohort append conflict retry limit"))
    }

    pub(crate) async fn cohort(
        &self,
        binding: &DeltaBinding,
        contract: &StorageContract,
    ) -> Result<DataFrame> {
        if &binding.source.table.contract_id != contract.identity() {
            return Err(invalid("cohort contract changed"));
        }
        let table = self
            .load(
                &binding.source.table.table_uri,
                Some(binding.source.version),
            )
            .await?;
        if table.snapshot().map_err(external)?.metadata().id() != binding.source.table.table_id {
            return Err(invalid("cohort table identity changed"));
        }
        view(
            &self.session(),
            self.provider(&table, contract).await?,
            binding,
            contract,
        )
    }
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}

pub(crate) fn view(
    session: &datafusion::prelude::SessionContext,
    provider: Arc<dyn datafusion::catalog::TableProvider>,
    binding: &DeltaBinding,
    contract: &StorageContract,
) -> Result<DataFrame> {
    session
        .read_table(provider)?
        .filter(col("cohort_id").eq(lit(binding.cohort_id)))?
        .select(
            contract
                .semantic_schema()
                .fields()
                .iter()
                .filter(|f| f.name() != "cohort_id")
                .map(|f| col(f.name()))
                .collect::<Vec<_>>(),
        )
}
