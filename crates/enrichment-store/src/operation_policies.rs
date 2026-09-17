//! Exact-version effective configuration over the shared immutable native definition contract.
use crate::immutable_definitions::Binding;
use crate::{control::ControlStore, immutable_definitions::Definitions, runtime::QueryRuntime};
use datafusion::{dataframe::DataFrame, error::Result};
use enrichment_core::{config::Config, native_key::Key};
#[derive(Clone)]
pub(crate) struct Policies(Definitions);
impl Policies {
    pub(crate) fn new(control: ControlStore, runtime: QueryRuntime) -> Self {
        Self(Definitions::new(
            control,
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
            ControlStore::open(root.path(), runtime.clone()).unwrap(),
            runtime.clone(),
        );
        let first = Config::default();
        let first_id = Key::OperationPolicy.record(&first).unwrap();
        let before = policies.retain(&first).await.unwrap();
        assert_eq!(before, policies.retain(&first).await.unwrap());
        let mut changed = first;
        changed.policy.enabled_profiles = vec!["static".into()];
        let after = policies.retain(&changed).await.unwrap();
        assert!(after.version > before.version);
        let reopened = Policies::new(
            ControlStore::open(root.path(), runtime.clone()).unwrap(),
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
        let retention = crate::retention::RetentionStore::new(
            ControlStore::open(root.path(), runtime.clone()).unwrap(),
            runtime.clone(),
        );
        let maintenance = retention
            .claim_maintenance(
                "operation_policies".into(),
                "definition-retention-oracle".into(),
            )
            .await
            .unwrap();
        let protected = retention
            .maintenance_decision(
                &maintenance,
                &crate::retention::TableVersion {
                    table_uri: "operation_policies".into(),
                    table_id: after.table_id.clone(),
                    version: after.version,
                    contract_id: after.contract_id.clone(),
                    cohort_id: None,
                },
            )
            .await
            .unwrap();
        assert!(protected.keep_versions.contains(&before.version));
        assert!(protected.keep_versions.contains(&after.version));
        retention
            .finish_maintenance(&maintenance, true)
            .await
            .unwrap();
        let mut wrong = before.clone();
        wrong.table_id = "another-table".into();
        assert!(reopened.read(&first_id, &wrong).await.is_err());
        wrong = before;
        wrong.contract_id = "another-schema".into();
        assert!(reopened.read(&first_id, &wrong).await.is_err());
        runtime.close_diagnostics().await.unwrap();
    }
}
