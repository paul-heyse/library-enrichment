//! Artifact reclamation shares native root/lease/obligation authority with Delta rows.
//! Only the final unlink and directory synchronization are physical driver actions.
use super::*;

enrichment_core::native_struct! { struct ArtifactRow { artifact_id: String => Rule::NonEmpty } }

async fn candidates(session: &SessionContext) -> Result<datafusion::dataframe::DataFrame> {
    session.sql("WITH candidates AS (SELECT unnest(dependencies) AS dependency FROM state.records.cleanup_obligations WHERE physical_released AND NOT settled), protected AS (SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed UNION ALL SELECT unnest(dependencies) AS dependency FROM state.records.retention_leases WHERE NOT released UNION ALL SELECT unnest(dependencies) AS dependency FROM state.records.cleanup_obligations WHERE NOT physical_released) SELECT DISTINCT c.dependency.artifact.artifact_id AS artifact_id FROM candidates c LEFT ANTI JOIN protected p ON c.dependency=p.dependency WHERE c.dependency.kind='artifact'").await
}

impl RetentionStore {
    /// Run under the service's exclusive daemon/cache label locks. A native
    /// maintenance fence refuses new artifact enrollment, and the exclusive root
    /// lock additionally refuses captured/read-only bundle readers through unlink.
    pub async fn reclaim_artifacts(&self) -> Result<ArtifactReclamation> {
        let run = self
            .claim_maintenance(
                "@artifacts".into(),
                format!("artifact-maintenance/{}", uuid::Uuid::new_v4()),
            )
            .await?;
        let result = self.reclaim_artifacts_owned(&run).await;
        self.finish_maintenance(&run, result.is_ok()).await?;
        let (report, dependencies) = result?;
        self.settle_dependencies(dependencies).await?;
        Ok(report)
    }

    async fn reclaim_artifacts_owned(
        &self,
        run: &MaintenanceRun,
    ) -> Result<(ArtifactReclamation, Vec<Dependency>)> {
        let rows = {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            self.bind(&session, "artifact_run", run)?;
            self.runtime.require_empty(session.sql("SELECT r.run_id AS witness FROM artifact_run r LEFT ANTI JOIN state.records.maintenance_runs m ON r.run_id=m.run_id AND r.generation=m.generation AND r.process=m.process AND r.table_uri=m.table_uri AND m.state='claimed'").await?,"artifact_cleanup_owner","retention").await?;
            self.runtime
                .records::<ArtifactRow>(candidates(&session).await?, 1024)
                .await?
        };
        let dependencies = rows
            .iter()
            .map(|row| Dependency::Artifact {
                artifact_id: row.artifact_id.clone(),
            })
            .collect();
        let namespace = self.namespace();
        let root = namespace
            .parent()
            .ok_or_else(|| invalid("artifact reclamation root"))?
            .to_owned();
        let report = self
            .runtime
            .blocking(move || {
                let _exclusive = crate::leases::exclusive(&root)?;
                let blobs = crate::blob::BlobStore::read_only(&root)?;
                let mut report = ArtifactReclamation {
                    candidates: rows.len() as u64,
                    removed_files: 0,
                    removed_file_bytes: 0,
                };
                for row in rows {
                    if let Some(bytes) = blobs.remove_unreferenced(&row.artifact_id)? {
                        report.removed_files += 1;
                        report.removed_file_bytes = report
                            .removed_file_bytes
                            .checked_add(bytes)
                            .ok_or_else(|| {
                                std::io::Error::other("artifact removal size overflow")
                            })?;
                    }
                }
                Ok::<_, std::io::Error>(report)
            })
            .await??;
        Ok((report, dependencies))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn plan19_artifact_reclamation_selects_only_unowned_completed_dependencies() -> Result<()>
    {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let session = runtime.session();
        let artifact = |id: &str| Dependency::Artifact {
            artifact_id: id.into(),
        };
        let process = crate::native_process::current()?;
        let done = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_a4c3ed04a95a3da14a9d235c83d868be".to_owned(),
            )
            .unwrap(),
            label: "label".into(),
            process: process.clone(),
            dependencies: vec![
                artifact("orphan"),
                artifact("root"),
                artifact("reader"),
                artifact("writer"),
            ],
            physical_released: true,
            settled: false,
            sequence: 1,
        };
        let live = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_247610f4dedd4ab7247d07dbda19c81c".to_owned(),
            )
            .unwrap(),
            dependencies: vec![artifact("writer")],
            physical_released: false,
            ..done.clone()
        };
        let root = RetentionRoot {
            root_id: "root".into(),
            dependencies: vec![artifact("root")],
            removed: false,
            sequence: 1,
        };
        let lease = RetentionLease {
            lease_id: enrichment_core::identity::RetentionLeaseId::try_from(
                "retention_lease_3d0941964aa3ebdcb00ccef58b1bb399".to_owned(),
            )
            .unwrap(),
            label: "reader".into(),
            process,
            kind: ProtectionKind::Query,
            fence: 1,
            predecessor: 0,
            dependencies: vec![artifact("reader")],
            released: false,
            sequence: 1,
        };
        let records: crate::native_catalog::Tables = [
            (
                "cleanup_obligations".into(),
                crate::native_catalog::batch(
                    &session,
                    "artifacts",
                    CleanupObligation::batch(&[done, live])?,
                )?
                .into_view(),
            ),
            (
                "retention_roots".into(),
                crate::native_catalog::batch(
                    &session,
                    "artifacts",
                    RetentionRoot::batch(&[root])?,
                )?
                .into_view(),
            ),
            (
                "retention_leases".into(),
                crate::native_catalog::batch(
                    &session,
                    "artifacts",
                    RetentionLease::batch(&[lease])?,
                )?
                .into_view(),
            ),
        ]
        .into();
        let catalog = crate::native_catalog::BoundCatalog::default()
            .with_schema(crate::native_catalog::BindingKind::FoldedRecords, records);
        let session = runtime.bound_session(std::collections::BTreeMap::from([(
            "state".into(),
            Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
        )]))?;
        assert_eq!(
            runtime
                .records::<ArtifactRow>(candidates(&session).await?, 8)
                .await?,
            vec![ArtifactRow {
                artifact_id: "orphan".into()
            }]
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
