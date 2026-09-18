//! Reconcile declared writer rows after native process identity proves physical exit.
//! Current Delta state supplies the exact binding; no transaction-marker lifetime
//! or guessed commit acknowledgement is treated as durable output authority.
use super::*;

enrichment_core::native_struct! { struct Resolution {
    pending: Vec<Dependency> => Rule::SequenceBounds { min: 1, max: 1024 },
    replacements: Vec<Dependency> => Rule::SequenceBounds { min: 0, max: 1 },
} }

async fn candidates(session: &SessionContext) -> Result<datafusion::dataframe::DataFrame> {
    session.sql("SELECT DISTINCT dependency FROM (SELECT unnest(dependencies) AS dependency FROM state.records.cleanup_obligations WHERE physical_released AND NOT settled) WHERE dependency.kind='pending_row'").await
}

pub(super) async fn missing_candidate(
    session: &SessionContext,
) -> Result<datafusion::dataframe::DataFrame> {
    native_catalog::work(
        session,
        "incomplete_candidates",
        candidates(session).await?.into_view(),
    )?;
    session.sql("SELECT 'pending_writer' AS witness FROM incomplete_request r LEFT ANTI JOIN incomplete_candidates c ON r.dependency=c.dependency WHERE r.dependency.kind='pending_row' UNION ALL SELECT 'invalid_dependency' AS witness FROM incomplete_request WHERE dependency.kind<>'pending_row'").await
}

/// Only physically completed pending rows can be disregarded when removing an
/// uncommitted table. Even a completed exact table dependency remains protected.
pub(super) async fn protected(
    session: &SessionContext,
) -> Result<datafusion::dataframe::DataFrame> {
    session.sql("SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed UNION ALL SELECT unnest(dependencies) AS dependency FROM state.records.retention_leases WHERE NOT released UNION ALL SELECT dependency FROM (SELECT physical_released,unnest(dependencies) AS dependency FROM state.records.cleanup_obligations WHERE NOT physical_released OR NOT settled) WHERE NOT physical_released OR dependency.kind<>'pending_row'").await
}

async fn progress(
    session: &SessionContext,
    sequence: u64,
) -> Result<datafusion::dataframe::DataFrame> {
    let remaining = "array_concat(array_except(c.dependencies,r.pending),r.replacements)";
    let changed = session.sql(&format!("SELECT c.obligation_id,c.label,c.process,CASE WHEN array_empty({remaining}) THEN c.dependencies ELSE {remaining} END AS dependencies,c.physical_released,array_empty({remaining}) AS settled,$1 AS sequence FROM state.records.cleanup_obligations c CROSS JOIN write_resolution r WHERE c.physical_released AND NOT c.settled AND arrays_overlap(c.dependencies,r.pending)"))
        .await?.with_param_values(vec![datafusion::common::ScalarValue::UInt64(Some(sequence))])?.alias("writer_output")?;
    // Replacing a dependency constructs a new collection. Bind the declared output
    // fields through native named_struct; control admission validates its row bounds.
    use datafusion::functions::core::expr_ext::FieldAccessor;
    use enrichment_core::{evidence::arrow_model::expressions::record, native_union::Cell};
    let record = record(
        &CleanupObligation::data_type(),
        &CleanupObligation::fields()
            .iter()
            .map(|field| (field.name().as_str(), col(field.name())))
            .collect::<Vec<_>>(),
    )?;
    changed.select(vec![record.alias("resolved")])?.select(
        CleanupObligation::fields()
            .iter()
            .map(|field| col("resolved").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )
}

impl RetentionStore {
    /// Resolve exact predeclared writer keys after physical exit. This method never
    /// retries the writer or publishes an unselected output. Native maintenance can
    /// subsequently delete its rows while preserving every retained old version.
    pub async fn reconcile_writers(&self) -> Result<usize> {
        let pending = {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            self.runtime
                .records::<DependencyRow>(candidates(&session).await?, 1024)
                .await?
        };
        let count = pending.len();
        for record in pending {
            let Dependency::PendingRow {
                table_uri,
                contract_id,
                row,
            } = &record.dependency
            else {
                return Err(invalid("writer recovery selected another dependency kind"));
            };
            let delta = self.control.delta_namespace();
            let root = self
                .namespace()
                .parent()
                .ok_or_else(|| invalid("writer recovery root"))?
                .to_owned();
            let shared = crate::leases::shared(&root)?;
            let resolved = match delta.load(table_uri, None).await {
                Ok(table) => {
                    let snapshot = table
                        .snapshot()
                        .map_err(|error| DataFusionError::External(Box::new(error)))?;
                    if snapshot
                        .metadata()
                        .configuration()
                        .get(crate::native_delta::CONTRACT_PROPERTY)
                        != Some(&contract_id.to_string())
                    {
                        return Err(invalid("interrupted writer contract changed"));
                    }
                    let contract = delta.current_contract(table_uri).await?;
                    if contract.identity() != contract_id {
                        return Err(invalid("interrupted writer contract witness changed"));
                    }
                    super::selection::Selection::admit(row, &contract.semantic_schema())?;
                    Some(Dependency::Table {
                        value: TableSelection {
                            source: crate::native_delta::capture_version(
                                table_uri, &table, &contract,
                            )?,
                            row: Some(row.clone()),
                        },
                    })
                }
                Err(error) if crate::native_delta::missing_table(&error) => {
                    drop(shared);
                    self.reclaim_incomplete(&record.dependency).await?;
                    continue;
                }
                Err(error) => return Err(error),
            };
            let resolution = Resolution {
                pending: vec![record.dependency],
                replacements: resolved.into_iter().collect(),
            };
            self.resolve_writer(&resolution).await?;
        }
        Ok(count)
    }

    async fn reclaim_incomplete(&self, dependency: &Dependency) -> Result<()> {
        let Dependency::PendingRow { table_uri, .. } = dependency else {
            return Err(invalid("incomplete removal requires a pending row"));
        };
        // A previous iteration may have settled every pending row for this table.
        let pending = {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            use datafusion::functions::core::expr_ext::FieldAccessor;
            self.runtime
                .records::<DependencyRow>(
                    candidates(&session).await?.filter(
                        col("dependency")
                            .field("pending_row")
                            .field("table_uri")
                            .eq(lit(table_uri)),
                    )?,
                    1024,
                )
                .await?
        };
        if pending.is_empty() {
            return Ok(());
        }
        let run = self
            .claim_maintenance_for(
                table_uri.clone(),
                format!("incomplete-writer/{}", uuid::Uuid::new_v4()),
                Some(dependency),
            )
            .await?;
        let delta = self.control.delta_namespace();
        let runtime = self.runtime.clone();
        let table_uri = table_uri.clone();
        let root = self
            .namespace()
            .parent()
            .ok_or_else(|| invalid("incomplete removal root"))?
            .to_owned();
        // An interrupted owned task leaves the durable fence claimed. Only a known
        // physical return may finish it. The root guard lives inside that task.
        let result = self
            .runtime
            .native_write(async move {
                Ok(async {
                    let _exclusive = crate::leases::exclusive(&root)?;
                    delta.location(&table_uri)?;
                    let path = delta.root.join(&table_uri);
                    let inventory = crate::owned_tree::Inventory::read(&path)?;
                    let log = delta.log_store(&table_uri)?;
                    if log
                        .is_delta_table_location()
                        .await
                        .map_err(|e| DataFusionError::External(Box::new(e)))?
                    {
                        return Err(invalid("incomplete removal found committed Delta history"));
                    }
                    let namespace = match crate::snapshot_registry::Namespace::read(&path) {
                        Ok(namespace) => Some(namespace),
                        Err(DataFusionError::IoError(error))
                            if error.kind() == std::io::ErrorKind::NotFound =>
                        {
                            None
                        }
                        Err(error) => return Err(error),
                    };
                    if let Some(namespace) = &namespace {
                        runtime.invalidate_namespace(namespace)?;
                    }
                    let store = runtime.session().runtime_env().object_store(
                        datafusion::execution::object_store::ObjectStoreUrl::local_filesystem(),
                    )?;
                    let removed = inventory.remove(store).await;
                    if let Some(namespace) = &namespace {
                        runtime.invalidate_namespace(namespace)?;
                    }
                    removed
                }
                .await)
            })
            .await?;
        let result = match result {
            Ok(()) => {
                self.resolve_writer(&Resolution {
                    pending: pending.into_iter().map(|r| r.dependency).collect(),
                    replacements: vec![],
                })
                .await
            }
            Err(error) => Err(error),
        };
        self.finish_maintenance(&run, result.is_ok()).await?;
        result
    }

    async fn resolve_writer(&self, resolution: &Resolution) -> Result<()> {
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            self.bind(&session, "write_resolution", resolution)?;
            let sequence = pin
                .generation()
                .checked_add(1)
                .ok_or_else(|| invalid("writer recovery generation overflow"))?;
            let output = self
                .runtime
                .execute(progress(&session, sequence).await?)
                .await?;
            if output.rows == 0 {
                return Ok(());
            }
            if output.rows > crate::control::MAX_DELTA_ROWS {
                return Err(invalid("writer recovery row bound"));
            }
            if self
                .control
                .commit_native(
                    pin.generation(),
                    output
                        .batches
                        .into_iter()
                        .map(|batch| (Table::CleanupObligations, batch))
                        .collect(),
                    vec!["retention/writer-recovery".into()],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("writer recovery conflict bound"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn plan19_incomplete_removal_excludes_only_exited_pending_writers() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let session = runtime.session();
        let process = crate::native_process::current()?;
        let pending = |table: &str| Dependency::PendingRow {
            table_uri: table.into(),
            contract_id: enrichment_core::identity::SchemaContractId::try_from(
                "schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf"
                    .to_owned(),
            )
            .unwrap(),
            row: RowKey {
                column: "id".into(),
                value: enrichment_core::identity::RowValue::Text {
                    value: table.into(),
                },
            },
        };
        let exact = Dependency::Table {
            value: TableSelection {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "exact".into(), table_id: "table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, version: 1 }, row: None,},
        };
        let obligation = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_b251994ca8108bbfdac92861ce8d3c5c".to_owned(),
            )
            .unwrap(),
            label: "writer".into(),
            process: process.clone(),
            dependencies: vec![pending("orphan"), exact.clone()],
            physical_released: true,
            settled: false,
            sequence: 1,
        };
        let live = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_247610f4dedd4ab7247d07dbda19c81c".to_owned(),
            )
            .unwrap(),
            dependencies: vec![pending("live")],
            physical_released: false,
            ..obligation.clone()
        };
        let records: crate::native_catalog::Tables = [
            (
                "cleanup_obligations".into(),
                crate::native_catalog::batch(
                    &session,
                    "pending",
                    CleanupObligation::batch(&[obligation, live])?,
                )?
                .into_view(),
            ),
            (
                "retention_roots".into(),
                crate::native_catalog::batch(
                    &session,
                    "pending",
                    RetentionRoot::batch(&[RetentionRoot {
                        root_id: "selected".into(),
                        dependencies: vec![pending("root")],
                        removed: false,
                        sequence: 1,
                    }])?,
                )?
                .into_view(),
            ),
            (
                "retention_leases".into(),
                crate::native_catalog::batch(
                    &session,
                    "pending",
                    RetentionLease::batch(&[RetentionLease {
                        lease_id: enrichment_core::identity::RetentionLeaseId::try_from(
                            "retention_lease_3d0941964aa3ebdcb00ccef58b1bb399".to_owned(),
                        )
                        .unwrap(),
                        label: "reader".into(),
                        process,
                        kind: ProtectionKind::Query,
                        fence: 1,
                        predecessor: 0,
                        dependencies: vec![pending("lease")],
                        released: false,
                        sequence: 1,
                    }])?,
                )?
                .into_view(),
            ),
        ]
        .into();
        let catalog = Arc::new(
            crate::native_catalog::BoundCatalog::default()
                .with_schema(crate::native_catalog::BindingKind::FoldedRecords, records),
        ) as Arc<dyn datafusion::catalog::CatalogProvider>;
        let session = runtime.bound_session(std::collections::BTreeMap::from([(
            "state".into(),
            catalog.clone(),
        )]))?;
        let rows = runtime
            .records::<DependencyRow>(protected(&session).await?, 8)
            .await?;
        assert_eq!(rows.len(), 4);
        for dependency in [exact, pending("live"), pending("root"), pending("lease")] {
            assert!(rows.iter().any(|row| row.dependency == dependency));
        }
        for (table, missing) in [("orphan", 0), ("live", 1), ("unknown", 1)] {
            let session = runtime.bound_session(std::collections::BTreeMap::from([(
                "state".into(),
                catalog.clone(),
            )]))?;
            native_catalog::input(
                &session,
                "incomplete_request",
                DependencyRow::batch(&[DependencyRow {
                    dependency: pending(table),
                }])?,
            )?;
            assert_eq!(
                runtime
                    .execute(missing_candidate(&session).await?)
                    .await?
                    .rows,
                missing
            );
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_pending_rows_resolve_without_releasing_live_writers() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let session = runtime.session();
        let pending = Dependency::PendingRow {
            table_uri: "results".into(),
            contract_id: enrichment_core::identity::SchemaContractId::try_from(
                "schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf"
                    .to_owned(),
            )
            .unwrap(),
            row: RowKey {
                column: "result_artifact_id".into(),
                value: enrichment_core::identity::RowValue::Text {
                    value: "result".into(),
                },
            },
        };
        let resolved = Dependency::Table {
            value: TableSelection {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "results".into(), table_id: "table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, version: 7 }, row: Some(RowKey {
                    column: "result_artifact_id".into(),
                    value: enrichment_core::identity::RowValue::Text {
                        value: "result".into(),
                    },
                }),},
        };
        let complete = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_eebbf6457e46a7f63acdf9b97390f790".to_owned(),
            )
            .unwrap(),
            label: "writer".into(),
            process: crate::native_process::current()?,
            dependencies: vec![pending.clone()],
            physical_released: true,
            settled: false,
            sequence: 1,
        };
        let mut live = complete.clone();
        live.obligation_id = enrichment_core::identity::CleanupObligationId::try_from(
            "cleanup_obligation_247610f4dedd4ab7247d07dbda19c81c".to_owned(),
        )
        .unwrap();
        live.physical_released = false;
        let mut mixed = complete.clone();
        mixed.obligation_id = enrichment_core::identity::CleanupObligationId::try_from(
            "cleanup_obligation_3f8fee624f43b2a9d685353269a0ab3e".to_owned(),
        )
        .unwrap();
        mixed.dependencies.push(Dependency::Artifact {
            artifact_id: "other".into(),
        });
        let records: crate::native_catalog::Tables = std::collections::BTreeMap::from([(
            "cleanup_obligations".into(),
            crate::native_catalog::batch(
                &session,
                "pending",
                CleanupObligation::batch(&[complete, live, mixed])?,
            )?
            .into_view(),
        )]);
        let catalog = Arc::new(
            crate::native_catalog::BoundCatalog::default()
                .with_schema(crate::native_catalog::BindingKind::FoldedRecords, records),
        ) as Arc<dyn datafusion::catalog::CatalogProvider>;
        let session = runtime.bound_session(std::collections::BTreeMap::from([(
            "state".into(),
            catalog.clone(),
        )]))?;
        assert_eq!(
            runtime
                .records::<DependencyRow>(candidates(&session).await?, 8)
                .await?
                .len(),
            1
        );
        for (replacement, expected_settled) in [(Some(resolved.clone()), false), (None, true)] {
            let context = runtime.bound_session(std::collections::BTreeMap::from([(
                "state".into(),
                catalog.clone(),
            )]))?;
            crate::native_catalog::work(
                &context,
                "write_resolution",
                crate::native_catalog::batch(
                    &context,
                    "pending",
                    Resolution::batch(&[Resolution {
                        pending: vec![pending.clone()],
                        replacements: replacement.clone().into_iter().collect(),
                    }])?,
                )?
                .into_view(),
            )?;
            let rows = runtime
                .records::<CleanupObligation>(progress(&context, 2).await?, 8)
                .await?;
            assert_eq!(rows.len(), 2);
            let single = rows
                .iter()
                .find(|row| {
                    row.obligation_id
                        == enrichment_core::identity::CleanupObligationId::try_from(
                            "cleanup_obligation_eebbf6457e46a7f63acdf9b97390f790".to_owned(),
                        )
                        .unwrap()
                })
                .unwrap();
            assert_eq!(single.settled, expected_settled);
            assert_eq!(
                single.dependencies,
                vec![replacement.unwrap_or_else(|| pending.clone())]
            );
            let mixed = rows
                .iter()
                .find(|row| {
                    row.obligation_id
                        == enrichment_core::identity::CleanupObligationId::try_from(
                            "cleanup_obligation_3f8fee624f43b2a9d685353269a0ab3e".to_owned(),
                        )
                        .unwrap()
                })
                .unwrap();
            assert!(!mixed.settled);
            assert!(mixed.dependencies.contains(&Dependency::Artifact {
                artifact_id: "other".into()
            }));
            assert!(
                rows.iter()
                    .all(|row| row.physical_released && row.sequence == 2)
            );
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
