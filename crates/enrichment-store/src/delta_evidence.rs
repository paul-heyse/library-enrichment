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

    pub(crate) fn contract(relation: Relation) -> Result<StorageContract> {
        crate::delta_cohort::contract(relation.schema()?)
    }

    /// Write a private cohort with complete native validation and commit semantics.
    /// Publication occurs only when a later control commit selects this returned binding.
    pub async fn append(
        &self,
        relation: Relation,
        cohort: &enrichment_core::identity::CohortId,
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

    /// Compact evidence through the same owned native maintenance path as control.
    /// # Errors
    /// Wrong retention scope, active writers and native maintenance failures refuse.
    pub async fn compact(
        &self,
        relation: Relation,
        retention: &crate::retention::RetentionStore,
    ) -> Result<(u64, deltalake::operations::optimize::Metrics)> {
        self.delta
            .compact(
                &format!("evidence_{}", relation.name()),
                &Self::contract(relation)?,
                retention,
            )
            .await
    }

    /// Native VACUUM/log cleanup under one exact retention generation. Horizons
    /// apply only to unreferenced files; retained versions and DVs stay protected.
    pub async fn reclaim(
        &self,
        relation: Relation,
        retention: &crate::retention::RetentionStore,
    ) -> Result<crate::retention::Reclamation> {
        self.delta
            .reclaim(
                &format!("evidence_{}", relation.name()),
                &Self::contract(relation)?,
                retention,
            )
            .await
    }

    pub async fn providers(
        &self,
        bindings: &[DeltaBinding],
        protection: crate::leases::ReadProtection,
    ) -> Result<BTreeMap<Relation, Arc<dyn TableProvider>>> {
        protection
            .require_tables(&self.delta.root, bindings)
            .await?;
        let mut providers = BTreeMap::new();
        for binding in bindings {
            let relation = Relation::ALL
                .into_iter()
                .find(|r| r.name() == binding.relation)
                .ok_or_else(|| invalid("unknown evidence relation"))?;
            let contract = Self::contract(relation)?;
            if binding.source.table.table_uri != format!("evidence_{}", relation.name())
                || &binding.source.table.contract_id != contract.identity()
                || providers.contains_key(&relation)
            {
                return Err(invalid("invalid evidence version vector"));
            }
            let provider = self
                .delta
                .immutable_provider(binding, &contract, &protection)
                .await?;
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
        retention: &crate::retention::RetentionStore,
    ) -> Result<DataFrame> {
        use datafusion::logical_expr::JoinType;
        if retention.namespace() != self.delta.root {
            return Err(invalid(
                "CDF protection belongs to a different Delta namespace",
            ));
        }
        let protection = retention.enroll_changes(before, after).await?;
        let old = self
            .providers(
                before,
                crate::leases::ReadProtection::Durable(protection.clone()),
            )
            .await?;
        let new = self
            .providers(
                after,
                crate::leases::ReadProtection::Durable(protection.clone()),
            )
            .await?;
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
            if a.source.table.table_id != b.source.table.table_id
                || a.source.table.contract_id != b.source.table.contract_id
                || a.source.version > b.source.version
            {
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
            if b.source.version > a.source.version {
                let changes = self
                    .delta
                    .changes(
                        &enrichment_core::delta_reference::CdfWindow {
                            table: b.source.table.clone(),
                            start: a.source.version + 1,
                            end: b.source.version,
                        },
                        &Self::contract(relation)?,
                    )
                    .await?;
                let positive = cohort_change_keys(changes.clone(), b.cohort_id, key, true)?;
                inserted = inserted.join(positive, JoinType::LeftSemi, &[key], &[key], None)?;
                if a.cohort_id == b.cohort_id {
                    let negative = cohort_change_keys(changes, a.cohort_id, key, false)?;
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
        let output = output.ok_or_else(|| invalid("empty evidence relation registry"))?;
        let session = self.delta.session();
        session.read_table(crate::leases::protected_provider(
            output.into_view(),
            protection,
            &session,
        )?)
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
        .filter(col("cohort_id").eq(lit(binding.cohort_id)))?
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

/// CDF images select native semantic keys only within the admitted cohort.
fn cohort_change_keys(
    changes: DataFrame,
    cohort: enrichment_core::identity::CohortId,
    key: &str,
    additions: bool,
) -> Result<DataFrame> {
    let images = if additions {
        ["insert", "update_postimage"]
    } else {
        ["delete", "update_preimage"]
    };
    changes
        .filter(
            col("cohort_id")
                .eq(lit(cohort))
                .and(col("_change_type").in_list(images.into_iter().map(lit).collect(), false)),
        )?
        .select(vec![col(key)])?
        .distinct()
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        identity::CohortId,
        native_union::{NativeStruct, Rule},
    };
    enrichment_core::native_struct! { struct Change {
        cohort_id: CohortId => Rule::Text,
        key: String => Rule::NonEmpty,
        _change_type: String => Rule::NonEmpty,
    } }
    enrichment_core::native_struct! { struct Key { key: String => Rule::NonEmpty } }
    #[tokio::test]
    async fn plan19_cdf_images_preserve_typed_cohort_selection() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let cohort = CohortId::from_component("00112233445566778899aabbccddeeff").unwrap();
        let other = CohortId::from_component("ffeeddccbbaa99887766554433221100").unwrap();
        let rows = [
            (cohort, "a", "insert"),
            (cohort, "a", "insert"),
            (cohort, "b", "update_postimage"),
            (cohort, "c", "delete"),
            (cohort, "d", "update_preimage"),
            (other, "foreign", "insert"),
            (other, "foreign", "delete"),
        ]
        .into_iter()
        .map(|(cohort_id, key, image)| Change {
            cohort_id,
            key: key.into(),
            _change_type: image.into(),
        })
        .collect::<Vec<_>>();
        let frame =
            crate::native_catalog::batch(&runtime.session(), "cdf_images", Change::batch(&rows)?)?;
        for (additions, expected) in [(true, vec!["a", "b"]), (false, vec!["c", "d"])] {
            let selected = cohort_change_keys(frame.clone(), cohort, "key", additions)?
                .sort(vec![col("key").sort(true, false)])?;
            let keys = runtime.records::<Key>(selected, 2).await?;
            assert_eq!(
                keys.iter().map(|row| row.key.as_str()).collect::<Vec<_>>(),
                expected
            );
        }
        let empty = cohort_change_keys(frame, CohortId::new(), "key", true)?;
        assert!(runtime.records::<Key>(empty, 2).await?.is_empty());
        runtime.close_diagnostics().await
    }
}
