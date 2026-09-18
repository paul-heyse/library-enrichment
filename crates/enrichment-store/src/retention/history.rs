//! Durable native reconstruction bounds. A warm snapshot never grants historical admission.
use super::*;

pub(super) async fn below_floor(
    session: &SessionContext,
    input: &str,
) -> Result<datafusion::dataframe::DataFrame> {
    session.sql(&format!("WITH incoming AS (SELECT unnest(dependencies) AS dependency FROM {input}), versions AS (SELECT coalesce(dependency.table.value.source.table.table_uri,dependency.cdf_window.value.table.table_uri) AS table_uri,coalesce(dependency.table.value.source.table.table_id,dependency.cdf_window.value.table.table_id) AS table_id,native_coalesce(dependency.table.value.source.table.contract_id,dependency.cdf_window.value.table.contract_id) AS contract_id,coalesce(dependency.table.value.source.version,dependency.cdf_window.value.start) AS version FROM incoming) SELECT DISTINCT v.table_uri AS witness FROM versions v JOIN state.records.maintenance_runs m ON v.table_uri=m.selection.table.source.table.table_uri AND v.table_id=m.selection.table.source.table.table_id AND v.contract_id=m.selection.table.source.table.contract_id WHERE m.selection.reclaim AND v.version<m.selection.decision.log_floor")).await
}

pub(super) async fn admission_rules(
    invariants: &mut crate::invariants::Invariants,
    session: &SessionContext,
) -> Result<()> {
    invariants.push(session.sql("SELECT run_id AS witness FROM state.records.maintenance_runs WHERE selection IS NOT NULL AND (selection.table.source.table.table_uri<>table_uri OR selection.table.row IS NOT NULL OR selection.decision.log_floor>selection.table.source.version OR (selection.reclaim AND NOT selection.decision.vacuum_allowed) OR selection.log_cutoff>selection.observed_at)").await?, "maintenance_selected_scope", "retention")?;
    invariants.push(session.sql("SELECT r.run_id AS witness FROM state.records.maintenance_runs r JOIN state.history.maintenance_runs h ON r.run_id=h.run_id WHERE h.selection IS NOT NULL AND r.selection IS DISTINCT FROM h.selection").await?, "maintenance_selection_immutable", "retention")?;
    Ok(())
}

impl RetentionStore {
    /// Publish the actual table incarnation and library operation bounds before any
    /// DELETE/VACUUM/checkpoint work. Failed/interrupted runs retain this conservative
    /// admission boundary because they may have physically reclaimed some history.
    pub(crate) async fn select_maintenance(
        &self,
        run: &MaintenanceRun,
        selection: &MaintenanceSelection,
    ) -> Result<()> {
        let expected = self.maintenance_decision(run, &selection.table).await?;
        if expected != selection.decision {
            return Err(invalid("maintenance selection changed"));
        }
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            self.bind(&session, "maintenance_owner", run)?;
            self.bind(&session, "maintenance_selection", selection)?;
            self.runtime.require_empty(session.sql("SELECT o.run_id AS witness FROM maintenance_owner o LEFT ANTI JOIN state.records.maintenance_runs r ON o.run_id=r.run_id AND o.generation=r.generation AND o.process=r.process AND o.protected=r.protected AND o.policy=r.policy AND r.state='claimed'").await?, "maintenance_selection_owner", "retention").await?;
            let rows = session.sql("SELECT r.* FROM state.records.maintenance_runs r JOIN maintenance_owner o ON r.run_id=o.run_id").await?;
            let current = self
                .runtime
                .records::<MaintenanceRun>(rows.clone(), 1)
                .await?
                .pop()
                .ok_or_else(|| invalid("maintenance selection label missing"))?;
            if let Some(current) = current.selection {
                return if current == *selection {
                    Ok(())
                } else {
                    Err(invalid("maintenance selection cannot be replaced"))
                };
            }
            let selected = enrichment_core::evidence::arrow_model::expressions::literal(&Some(
                selection.clone(),
            ))?;
            let output =
                self.runtime
                    .execute(rows.with_column("selection", selected)?.with_column(
                        "sequence",
                        lit(
                            pin.generation().checked_add(1).ok_or_else(|| {
                                invalid("maintenance selection sequence overflow")
                            })?,
                        ),
                    )?)
                    .await?;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    output
                        .batches
                        .into_iter()
                        .map(|batch| (Table::MaintenanceRuns, batch))
                        .collect(),
                    vec![format!("maintenance/{}", run.table_uri)],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("maintenance selection conflict bound"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn maintenance_selection_cannot_escape_scope_or_erase_a_recorded_floor() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let original = MaintenanceRun {
            run_id: enrichment_core::identity::MaintenanceRunId::try_from(
                "maintenance_run_acba25512100f80b56fc3ccd14c65be5".to_owned(),
            )
            .unwrap(),
            table_uri: "evidence".into(),
            label: "label".into(),
            process: crate::native_process::current()?,
            generation: 2,
            predecessor: 1,
            policy_id: runtime.retention_policy().identity()?,
            policy: runtime.retention_policy().clone(),
            state: MaintenanceState::Claimed,
            protected: vec![],
            sequence: 3,
            selection: Some(MaintenanceSelection {
                table: TableSelection {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "evidence".into(), table_id: "table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, version: 10 }, row: None,},
                decision: MaintenanceDecision {
                    keep_versions: vec![4],
                    log_floor: 4,
                    vacuum_allowed: true,
                },
                reclaim: true,
                observed_at: enrichment_core::native_time::ObservationTime::from_micros(
                    20_000_000,
                )?,
                log_cutoff: enrichment_core::native_time::ObservationTime::from_micros(10_000_000)?,
            }),
        };
        let check = async |current: MaintenanceRun, history: &[MaintenanceRun]| -> Result<()> {
            let provider = |runs: &[MaintenanceRun]| -> Result<_> {
                Ok(BTreeMap::from([(
                    "maintenance_runs".into(),
                    crate::native_catalog::batch(
                        &runtime.session(),
                        "history",
                        MaintenanceRun::batch(runs)?,
                    )?
                    .into_view(),
                )]))
            };
            let catalog = Arc::new(
                native_catalog::BoundCatalog::default()
                    .with_schema(
                        native_catalog::BindingKind::FoldedRecords,
                        provider(&[current])?,
                    )
                    .with_schema(
                        native_catalog::BindingKind::ValidatedHistory,
                        provider(history)?,
                    ),
            );
            let session = runtime.bound_session(BTreeMap::from([(
                "state".into(),
                catalog as Arc<dyn datafusion::catalog::CatalogProvider>,
            )]))?;
            let mut invariants = crate::invariants::Invariants::default();
            admission_rules(&mut invariants, &session).await?;
            runtime.admit(invariants).await
        };
        check(original.clone(), &[]).await?;
        let mut settled = original.clone();
        settled.state = MaintenanceState::Failed;
        settled.sequence += 1;
        check(settled.clone(), std::slice::from_ref(&original)).await?;
        settled.selection = None;
        assert!(
            check(settled, std::slice::from_ref(&original))
                .await
                .is_err()
        );
        let mut changed = original.clone();
        changed.selection.as_mut().unwrap().decision.log_floor = 3;
        assert!(
            check(changed, std::slice::from_ref(&original))
                .await
                .is_err()
        );
        for (field, change) in [
            ("table", 0),
            ("future_log_floor", 1),
            ("future_cutoff", 2),
            ("vacuum_authority", 3),
        ] {
            let mut changed = original.clone();
            let selection = changed.selection.as_mut().unwrap();
            match change {
                0 => selection.table.source.table.table_uri = "other".into(),
                1 => selection.decision.log_floor = 11,
                2 => {
                    selection.log_cutoff =
                        enrichment_core::native_time::ObservationTime::from_micros(30_000_000)?
                }
                _ => selection.decision.vacuum_allowed = false,
            }
            assert!(check(changed, &[]).await.is_err(), "{field}");
        }
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn plan19_history_admission_is_incarnation_scoped_and_survives_failed_reclamation()
    -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let base = TableSelection {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "evidence".into(), table_id: "table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, version: 10 }, row: None,};
        let selected = MaintenanceSelection {
            table: base.clone(),
            decision: MaintenanceDecision {
                keep_versions: vec![4],
                log_floor: 4,
                vacuum_allowed: true,
            },
            reclaim: true,
            observed_at: enrichment_core::native_time::ObservationTime::from_micros(20_000_000)?,
            log_cutoff: enrichment_core::native_time::ObservationTime::from_micros(10_000_000)?,
        };
        let mut run = MaintenanceRun {
            run_id: enrichment_core::identity::MaintenanceRunId::try_from(
                "maintenance_run_acba25512100f80b56fc3ccd14c65be5".to_owned(),
            )
            .unwrap(),
            table_uri: "evidence".into(),
            label: "label".into(),
            process: crate::native_process::current()?,
            generation: 2,
            predecessor: 1,
            policy_id: runtime.retention_policy().identity()?,
            policy: runtime.retention_policy().clone(),
            state: MaintenanceState::Claimed,
            protected: vec![],
            selection: Some(selected),
            sequence: 3,
        };
        for (state, reclaim) in [
            (MaintenanceState::Claimed, true),
            (MaintenanceState::Failed, true),
            (MaintenanceState::Completed, true),
            (MaintenanceState::Completed, false),
        ] {
            run.state = state;
            run.selection.as_mut().unwrap().reclaim = reclaim;
            let records = BTreeMap::from([(
                "maintenance_runs".into(),
                crate::native_catalog::batch(
                    &runtime.session(),
                    "history",
                    MaintenanceRun::batch(&[run.clone()])?,
                )?
                .into_view(),
            )]);
            let catalog = Arc::new(
                native_catalog::BoundCatalog::default()
                    .with_schema(native_catalog::BindingKind::FoldedRecords, records),
            ) as Arc<dyn datafusion::catalog::CatalogProvider>;
            for (id, version, cdf, refused) in [
                ("table", 3, false, true),
                ("table", 3, true, true),
                ("table", 4, false, false),
                ("table", 4, true, false),
                ("table", 10, false, false),
                ("replacement", 3, false, false),
            ] {
                let session =
                    runtime.bound_session(BTreeMap::from([("state".into(), catalog.clone())]))?;
                let dependency = if cdf {
                    Dependency::CdfWindow {
                        value: enrichment_core::delta_reference::CdfWindow {
                            table: enrichment_core::delta_reference::DeltaTableRef {
                                table_uri: base.source.table.table_uri.clone(),
                                table_id: id.into(),
                                contract_id: base.source.table.contract_id.clone(),
                            },
                            start: version,
                            end: 10,
                        },
                    }
                } else {
                    Dependency::Table {
                        value: TableSelection {
                            source: enrichment_core::delta_reference::DeltaVersionRef {
                                table: enrichment_core::delta_reference::DeltaTableRef {
                                    table_id: id.into(),
                                    ..base.source.table.clone()
                                },
                                version,
                            },
                            ..base.clone()
                        },
                    }
                };
                native_catalog::input(
                    &session,
                    "requested_history",
                    SettledDependencies::batch(&[SettledDependencies {
                        dependencies: vec![dependency],
                    }])?,
                )?;
                let output = runtime
                    .execute(below_floor(&session, "requested_history").await?)
                    .await?;
                assert_eq!(
                    output.rows,
                    usize::from(reclaim && refused),
                    "{state:?}/{reclaim}/{id}/{version}/{cdf}"
                );
            }
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
