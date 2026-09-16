//! Purpose-specific Delta evidence tables, captured by exact native version and cohort.
use crate::{
    admission::Relation,
    native_delta::{DeltaStore, StorageContract},
    runtime::QueryRuntime,
};
use datafusion::{
    catalog::TableProvider,
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::evidence::snapshot::DeltaBinding;
use std::{collections::BTreeMap, path::Path, sync::Arc};

#[derive(Clone)]
pub struct EvidenceTables {
    delta: DeltaStore,
}

impl EvidenceTables {
    pub fn new(root: &Path, runtime: QueryRuntime) -> Result<Self> {
        Ok(Self {
            delta: DeltaStore::new(root, runtime)?,
        })
    }

    fn contract(relation: Relation) -> Result<StorageContract> {
        crate::delta_cohort::contract(relation.schema()?)
    }

    /// Write a private cohort with complete native validation and commit semantics.
    /// Publication occurs only when a later control commit selects this returned binding.
    pub async fn append(
        &self,
        relation: Relation,
        cohort: &str,
        input: DataFrame,
        rows: u64,
    ) -> Result<DeltaBinding> {
        self.delta
            .append_cohort(
                &format!("evidence_{}", relation.name()),
                relation.name(),
                cohort,
                &Self::contract(relation)?,
                input,
                rows,
            )
            .await
    }

    pub async fn providers(
        &self,
        bindings: &[DeltaBinding],
    ) -> Result<BTreeMap<Relation, Arc<dyn TableProvider>>> {
        let mut providers = BTreeMap::new();
        for binding in bindings {
            let relation = Relation::ALL
                .into_iter()
                .find(|r| r.name() == binding.relation)
                .ok_or_else(|| invalid("unknown evidence relation"))?;
            let contract = Self::contract(relation)?;
            if binding.table_uri != format!("evidence_{}", relation.name())
                || binding.contract_id != contract.identity()
                || providers.contains_key(&relation)
            {
                return Err(invalid("invalid evidence version vector"));
            }
            let table = self
                .delta
                .load(&binding.table_uri, Some(binding.version))
                .await?;
            if table.snapshot().map_err(external)?.metadata().id() != binding.table_id {
                return Err(invalid("evidence table identity changed"));
            }
            let provider = self.delta.provider(&table, &contract).await?;
            providers.insert(
                relation,
                cohort_view(
                    &self.delta.session(),
                    provider,
                    binding,
                    relation.schema()?.as_ref(),
                )?,
            );
        }
        if providers.len() != Relation::ALL.len() {
            return Err(invalid("incomplete evidence version vector"));
        }
        Ok(providers)
    }

    /// Publication-scoped changes. CDF bounds identify new physical events; native set
    /// differences against both selected cohorts remove unpublished and duplicate facts.
    pub async fn change_plan(
        &self,
        before: &[DeltaBinding],
        after: &[DeltaBinding],
    ) -> Result<DataFrame> {
        use datafusion::logical_expr::JoinType;
        let old = self.providers(before).await?;
        let new = self.providers(after).await?;
        let mut output: Option<DataFrame> = None;
        for relation in Relation::ALL {
            let a = before
                .iter()
                .find(|b| b.relation == relation.name())
                .ok_or_else(|| invalid("missing prior binding"))?;
            let b = after
                .iter()
                .find(|b| b.relation == relation.name())
                .ok_or_else(|| invalid("missing new binding"))?;
            if a.table_id != b.table_id || a.contract_id != b.contract_id || a.version > b.version {
                return Err(invalid(
                    "CDF requires an ordered pair in the same table and schema epoch",
                ));
            }
            let left = self
                .delta
                .session()
                .read_table(Arc::clone(&old[&relation]))?;
            let right = self
                .delta
                .session()
                .read_table(Arc::clone(&new[&relation]))?;
            let key = relation.key();
            let mut inserted = right.clone().except_distinct(left.clone())?;
            let mut removed = left.except_distinct(right.clone())?;
            if b.version > a.version {
                let changes = self
                    .delta
                    .changes(
                        &b.table_uri,
                        &b.table_id,
                        &Self::contract(relation)?,
                        a.version + 1,
                        b.version,
                    )
                    .await?;
                let positive = changes
                    .clone()
                    .filter(
                        col("cohort_id").eq(lit(b.cohort_id.as_str())).and(
                            col("_change_type")
                                .in_list(vec![lit("insert"), lit("update_postimage")], false),
                        ),
                    )?
                    .select(vec![col(key)])?
                    .distinct()?;
                inserted = inserted.join(positive, JoinType::LeftSemi, &[key], &[key], None)?;
                if a.cohort_id == b.cohort_id {
                    let negative = changes
                        .filter(
                            col("cohort_id").eq(lit(a.cohort_id.as_str())).and(
                                col("_change_type")
                                    .in_list(vec![lit("delete"), lit("update_preimage")], false),
                            ),
                        )?
                        .select(vec![col(key)])?
                        .distinct()?;
                    removed = removed.join(negative, JoinType::LeftSemi, &[key], &[key], None)?;
                }
            } else if a.cohort_id == b.cohort_id {
                inserted = inserted.filter(lit(false))?;
                removed = removed.filter(lit(false))?;
            }
            let schema = arrow::datatypes::Schema::new(vec![
                arrow::datatypes::Field::new("relation", arrow::datatypes::DataType::Utf8, false),
                arrow::datatypes::Field::new("key", arrow::datatypes::DataType::Utf8, false),
                arrow::datatypes::Field::new("inserted", arrow::datatypes::DataType::Int64, false),
                arrow::datatypes::Field::new("removed", arrow::datatypes::DataType::Int64, false),
            ]);
            let inserted = crate::native_delta::project(
                inserted.select(vec![
                    lit(relation.name()).alias("relation"),
                    col(key).alias("key"),
                    lit(1_i64).alias("inserted"),
                    lit(0_i64).alias("removed"),
                ])?,
                &schema,
            )?;
            let removed = crate::native_delta::project(
                removed.select(vec![
                    lit(relation.name()).alias("relation"),
                    col(key).alias("key"),
                    lit(0_i64).alias("inserted"),
                    lit(1_i64).alias("removed"),
                ])?,
                &schema,
            )?;
            let delta = inserted.union(removed)?;
            output = Some(match output {
                None => delta,
                Some(previous) => previous.union(delta)?,
            });
        }
        output.ok_or_else(|| invalid("empty evidence relation registry"))
    }
}
fn cohort_view(
    session: &datafusion::prelude::SessionContext,
    provider: Arc<dyn TableProvider>,
    binding: &DeltaBinding,
    schema: &arrow::datatypes::Schema,
) -> Result<Arc<dyn TableProvider>> {
    Ok(session
        .read_table(provider)?
        .filter(col("cohort_id").eq(lit(binding.cohort_id.as_str())))?
        .select(
            schema
                .fields()
                .iter()
                .map(|field| col(field.name()))
                .collect::<Vec<_>>(),
        )?
        .into_view())
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
