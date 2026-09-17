//! Native immutable definitions, shared by policy and physical operation consumers.
use crate::{
    native_delta::{DeltaStore, StorageContract, missing_table, transaction_conflict},
    runtime::QueryRuntime,
};
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::native_key::Key;
use enrichment_core::native_union::NativeStruct;
use std::sync::Arc;

pub use enrichment_core::operation::DefinitionBinding as Binding;
#[derive(Clone)]
pub(crate) struct Definitions {
    table: &'static str,
    key: Key,
    id_column: &'static str,
    delta: DeltaStore,
    runtime: QueryRuntime,
}
impl Definitions {
    pub(crate) fn new(
        delta: DeltaStore,
        runtime: QueryRuntime,
        table: &'static str,
        key: Key,
        id_column: &'static str,
    ) -> Self {
        Self {
            delta,
            runtime,
            table,
            key,
            id_column,
        }
    }
    fn contract(&self) -> Result<StorageContract> {
        let mut fields = self.key.schema().fields().to_vec();
        fields.push(Arc::new(Field::new(self.id_column, DataType::Utf8, false)));
        StorageContract::new(Arc::new(Schema::new(fields)))
    }
    pub(crate) async fn retain<T: NativeStruct>(&self, value: &T) -> Result<Binding> {
        let id = self.key.record(value)?;
        let contract = self.contract()?;
        self.delta.prepare_root(self.table)?;
        for _ in 0..16 {
            let table = match self.delta.load(self.table, None).await {
                Ok(table) => table,
                Err(error) if missing_table(&error) => {
                    match self.delta.create(self.table, &contract, false).await {
                        Ok(table) => table,
                        Err(error) if transaction_conflict(&error) => continue,
                        Err(error) => return Err(error),
                    }
                }
                Err(error) => return Err(error),
            };
            let session = self.runtime.session();
            let current = session
                .read_table(self.delta.provider(&table, &contract).await?)?
                .filter(col(self.id_column).eq(lit(&id)))?;
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
                            .filter(col(self.id_column).not_eq(self.key.expression()))?
                            .select(vec![col(self.id_column)])?,
                        "native_definition_identity",
                        "definition_capture",
                    )
                    .await?;
                return binding(&table, &contract);
            }
            let input = session
                .read_batch(T::batch(std::slice::from_ref(value))?)?
                .with_column(self.id_column, self.key.expression())?;
            match self
                .delta
                .append(
                    table,
                    &contract,
                    input,
                    vec![deltalake::kernel::Transaction::new(
                        format!("{}/{id}", self.table),
                        1,
                    )],
                )
                .await
            {
                Ok(table) => return binding(&table, &contract),
                Err(error) if transaction_conflict(&error) => continue,
                Err(error) => return Err(error),
            }
        }
        Err(invalid("native definition conflict bound exceeded"))
    }
    pub(crate) async fn read(&self, id: &str, binding: &Binding) -> Result<DataFrame> {
        let contract = self.contract()?;
        let table = self.delta.load(self.table, Some(binding.version)).await?;
        if binding.contract_id != contract.identity()
            || table
                .snapshot()
                .map_err(|e| DataFusionError::External(Box::new(e)))?
                .metadata()
                .id()
                != binding.table_id
        {
            return Err(invalid(
                "native definition table identity or contract mismatch",
            ));
        }
        let frame = self
            .runtime
            .session()
            .read_table(self.delta.provider(&table, &contract).await?)?
            .filter(col(self.id_column).eq(lit(id)))?;
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
                    .filter(col(self.id_column).not_eq(self.key.expression()))?
                    .select(vec![col(self.id_column)])?,
                "native_definition_identity",
                "definition_binding",
            )
            .await?;
        Ok(frame)
    }
}
fn binding(table: &deltalake::DeltaTable, contract: &StorageContract) -> Result<Binding> {
    Ok(Binding {
        table_id: table
            .snapshot()
            .map_err(|e| DataFusionError::External(Box::new(e)))?
            .metadata()
            .id()
            .to_owned(),
        version: table
            .version()
            .ok_or_else(|| invalid("unloaded native definition"))?,
        contract_id: contract.identity().into(),
    })
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
