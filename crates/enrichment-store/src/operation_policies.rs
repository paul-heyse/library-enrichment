//! Operation policy is one specialization of the shared immutable definition owner.
pub(crate) type Policies =
    crate::immutable_definitions::Definitions<enrichment_core::identity::OperationPolicyId>;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::ControlStore, runtime::QueryRuntime};
    use datafusion::prelude::col;
    use enrichment_core::config::Config;
    #[tokio::test]
    async fn exact_policy_versions_survive_new_configuration_and_reject_wrong_identity() {
        let root = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default()).unwrap();
        let policies = Policies::new(
            ControlStore::open(root.path(), runtime.clone()).unwrap(),
            runtime.clone(),
        );
        let first = Config::default();
        let (first_id, before) = policies.retain(&first).await.unwrap();
        assert_eq!(
            (first_id.clone(), before.clone()),
            policies.retain(&first).await.unwrap()
        );
        let mut changed = first;
        changed.policy.enabled_profiles = vec!["static".into()];
        let (_, after) = policies.retain(&changed).await.unwrap();
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
                &crate::retention::TableSelection {
                    source: enrichment_core::delta_reference::DeltaVersionRef {
                        table: enrichment_core::delta_reference::DeltaTableRef {
                            table_uri: "operation_policies".into(),
                            table_id: after.table.table_id.clone(),
                            contract_id: after.table.contract_id.clone(),
                        },
                        version: after.version,
                    },
                    row: None,
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
        wrong.table.table_id = "another-table".into();
        assert!(reopened.read(&first_id, &wrong).await.is_err());
        wrong = before;
        wrong.table.contract_id = enrichment_core::identity::SchemaContractId::try_from(
            "schema_contract_eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .to_owned(),
        )
        .unwrap();
        assert!(reopened.read(&first_id, &wrong).await.is_err());
        runtime.close_diagnostics().await.unwrap();
    }
}
