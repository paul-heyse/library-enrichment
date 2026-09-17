//! Native immutable definitions, shared by policy and physical operation consumers.
use crate::{
    control::ControlStore,
    native_delta::{DeltaStore, StorageContract, transaction_conflict},
    retention::{Dependency, ProtectionKind, RetentionStore, TableVersion},
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
    retention: RetentionStore,
}
impl Definitions {
    pub(crate) fn new(
        control: ControlStore,
        runtime: QueryRuntime,
        table: &'static str,
        key: Key,
        id_column: &'static str,
    ) -> Self {
        Self {
            delta: control.delta_namespace(),
            retention: RetentionStore::new(control, runtime.clone()),
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
    pub(crate) async fn retain<T: NativeStruct + Clone + Send + Sync + 'static>(
        &self,
        value: &T,
    ) -> Result<Binding> {
        let input = self
            .runtime
            .session()
            .read_batch(T::batch(std::slice::from_ref(value))?)?;
        Ok(self.retain_plan(input, vec![]).await?.1)
    }

    /// Native callers retain a bounded record without decoding facts into Rust objects.
    /// The exact materialized Arrow row is both hashed and written, so volatile inputs
    /// cannot change between identity selection and Delta publication.
    pub(crate) async fn retain_plan(
        &self,
        input: DataFrame,
        dependencies: Vec<Dependency>,
    ) -> Result<(String, Binding)> {
        let definitions = self.clone();
        self.runtime
            .spawn(async move { Box::pin(definitions.retain_owned(input, dependencies)).await })
            .await
            .map_err(|error| DataFusionError::External(Box::new(error)))?
    }

    async fn retain_owned(
        &self,
        input: DataFrame,
        mut dependencies: Vec<Dependency>,
    ) -> Result<(String, Binding)> {
        enrichment_core::native_schema::check_input(input.schema().as_arrow(), &self.key.schema())?;
        let captured = self.runtime.execute(input.limit(0, Some(2))?).await?;
        if captured.rows != 1 {
            return Err(invalid(
                "native definition requires exactly one bounded record",
            ));
        }
        let input = self.runtime.session().read_batches(captured.batches)?;
        if let Some(violations) =
            enrichment_core::native_schema::intrinsic_violations(input.clone(), &self.key.schema())?
        {
            self.runtime
                .require_empty(
                    violations.select(vec![lit(self.table).alias("definition")])?,
                    "native_definition_fields",
                    "definition_capture",
                )
                .await?;
        }
        enrichment_core::native_struct! { struct Identity { id: String => enrichment_core::native_union::Rule::Text } }
        let selected: Identity = self
            .runtime
            .records(
                input
                    .clone()
                    .select(vec![self.key.expression().alias("id")])?,
                1,
            )
            .await?
            .pop()
            .ok_or_else(|| invalid("native definition identity absent"))?;
        let id = selected.id;
        let mut pending = dependencies.clone();
        pending.push(Dependency::TableScope {
            table_uri: self.table.into(),
        });
        let obligation = self
            .retention
            .create_obligation(id.clone(), pending)
            .await?;
        let binding = self.retain_inner(&id, input).await?;
        dependencies.push(self.dependency(&id, &binding));
        self.retention
            .publish_root(
                &obligation,
                format!("definition/{}/{}/{id}", binding.table_id, binding.version),
                dependencies,
            )
            .await?;
        Ok((id, binding))
    }

    fn dependency(&self, id: &str, binding: &Binding) -> Dependency {
        Dependency::Definition {
            table: TableVersion {
                table_uri: self.table.into(),
                table_id: binding.table_id.clone(),
                version: binding.version,
                contract_id: binding.contract_id.clone(),
                cohort_id: None,
            },
            definition_id: id.into(),
        }
    }

    async fn retain_inner(&self, id: &str, input: DataFrame) -> Result<Binding> {
        let contract = self.contract()?;
        self.delta.prepare_root(self.table)?;
        for _ in 0..16 {
            let table = self
                .delta
                .open_or_create(self.table, &contract, false, &[])
                .await?;
            let session = self.runtime.session();
            let current = session
                .read_table(self.delta.provider(&table, &contract).await?)?
                .filter(col(self.id_column).eq(lit(id)))?;
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
            let input = input
                .clone()
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
        let definitions = self.clone();
        let id = id.to_owned();
        let binding = binding.clone();
        self.runtime
            .spawn(async move { Box::pin(definitions.read_owned(&id, &binding)).await })
            .await
            .map_err(|error| DataFusionError::External(Box::new(error)))?
    }

    async fn read_owned(&self, id: &str, binding: &Binding) -> Result<DataFrame> {
        let contract = self.contract()?;
        let protection = self
            .retention
            .enroll(
                id.into(),
                ProtectionKind::Query,
                vec![self.dependency(id, binding)],
            )
            .await?;
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
        let session = self.runtime.session();
        let provider = crate::leases::protected_provider(
            self.delta.provider(&table, &contract).await?,
            protection,
            &session,
        )?;
        let frame = session
            .read_table(provider)?
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
