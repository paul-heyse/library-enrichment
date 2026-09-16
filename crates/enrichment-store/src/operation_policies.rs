//! Exact-version effective configuration over the shared immutable native definition contract.
use crate::immutable_definitions::Binding;
use crate::{immutable_definitions::Definitions, native_delta::DeltaStore, runtime::QueryRuntime};
use datafusion::{dataframe::DataFrame, error::Result};
use enrichment_core::{config::Config, native_key::Key};
#[derive(Clone)]
pub(crate) struct Policies(Definitions);
impl Policies {
    pub(crate) fn new(delta: DeltaStore, runtime: QueryRuntime) -> Self {
        Self(Definitions::new(
            delta,
            runtime,
            "operation_policies",
            Key::OperationPolicy,
            "policy_id",
        ))
    }
    pub(crate) async fn retain(&self, config: &Config) -> Result<Binding> {
        self.0.retain(config).await
    }
    pub(crate) async fn read(&self, id: &str, binding: &Binding) -> Result<DataFrame> {
        self.0.read(id, binding).await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use datafusion::prelude::col;
    #[tokio::test]
    async fn exact_policy_versions_survive_new_configuration_and_reject_wrong_identity() {
        let root = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default()).unwrap();
        let policies = Policies::new(
            DeltaStore::new(&root.path().join("delta"), runtime.clone()).unwrap(),
            runtime.clone(),
        );
        let first = Config::default();
        let first_id = Key::OperationPolicy.value(&first).unwrap();
        let before = policies.retain(&first).await.unwrap();
        assert_eq!(before, policies.retain(&first).await.unwrap());
        let mut changed = first;
        changed.policy.enabled_profiles = vec!["static".into()];
        let after = policies.retain(&changed).await.unwrap();
        assert!(after.version > before.version);
        let reopened = Policies::new(
            DeltaStore::new(&root.path().join("delta"), runtime.clone()).unwrap(),
            runtime.clone(),
        );
        let old = reopened.read(&first_id, &before).await.unwrap();
        let old = runtime
            .execute(
                old.select(vec![
                    datafusion::functions::core::expr_ext::FieldAccessor::field(
                        col("policy"),
                        "enabled_profiles",
                    ),
                ])
                .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(old.rows, 1);
        let profiles = old.batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<arrow::array::ListArray>()
            .unwrap()
            .value(0);
        assert_eq!(
            profiles.len(),
            2,
            "the captured policy retains build even after configuration changes"
        );
        let mut wrong = before.clone();
        wrong.table_id = "another-table".into();
        assert!(reopened.read(&first_id, &wrong).await.is_err());
        wrong = before;
        wrong.contract_id = "another-schema".into();
        assert!(reopened.read(&first_id, &wrong).await.is_err());
    }
}
