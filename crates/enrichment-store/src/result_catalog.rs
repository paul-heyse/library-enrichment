//! Native retained-result sections and dependency closure. JSON is only the delivery encoding.
use crate::{
    BlobStore,
    control::{ControlSnapshot, ControlStore, Table},
    runtime::QueryRuntime,
};
use arrow::datatypes::{Schema, SchemaRef};
use datafusion::{
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::evidence::arrow_model::cells::RowSet;
use enrichment_core::evidence::{Artifact, arrow_model::acquisitions};
use std::sync::Arc;

use enrichment_core::{
    native_union::NativeStruct,
    operation::results::{ResultReference, ResultSection, RetainedResult},
};
pub(crate) fn schema() -> SchemaRef {
    Arc::new(Schema::new(RetainedResult::fields()))
}

pub(crate) fn retained_root(
    record: &RetainedResult,
    sequence: u64,
) -> crate::retention::RetentionRoot {
    use crate::retention::{Dependency, RetentionRoot, TableVersion};
    let mut dependencies = record
        .references
        .iter()
        .map(|reference| Dependency::Artifact {
            artifact_id: reference.artifact_id.clone(),
        })
        .collect::<Vec<_>>();
    dependencies.push(Dependency::Artifact {
        artifact_id: record.result_artifact_id.clone(),
    });
    dependencies.push(Dependency::Table {
        value: TableVersion {
            table_uri: format!("result_{}", record.version.tool),
            table_id: record.version.table_id.clone(),
            version: record.version.version,
            contract_id: record.version.contract_id.clone(),
            cohort_id: None,
        },
    });
    RetentionRoot {
        root_id: record.result_artifact_id.clone(),
        dependencies,
        removed: false,
        sequence,
    }
}

impl ControlStore {
    /// Admit the verified physical encoder's section/reference facts before publishing a result.
    pub async fn retain_result(
        &self,
        runtime: &QueryRuntime,
        blobs: &BlobStore,
        artifact: &Artifact,
    ) -> Result<()> {
        self.require_write()?;
        let owned = blobs.clone();
        let descriptor = artifact.clone();
        let (index, base) = runtime
            .blocking(move || -> std::io::Result<_> {
                let mut file = owned.capture(&descriptor, crate::result::MAX_BYTES)?;
                crate::result::index(&mut file, descriptor.size_bytes)
            })
            .await??;
        let receipts = index
            .references
            .iter()
            .map(|reference| reference.receipt.clone())
            .chain(std::iter::once(artifact.clone()))
            .collect::<Vec<_>>();
        let receipt_batch = crate::artifact_catalog::encode(&receipts)?;
        let admission = runtime.session();
        crate::native_catalog::work(
            &admission,
            "result_receipt_inputs",
            crate::artifact_catalog::flatten(admission.read_batch(receipt_batch.clone())?)?
                .into_view(),
        )?;
        runtime.require_empty(admission.sql("SELECT 'result_receipt_input' AS witness FROM result_receipt_inputs HAVING count(*)>1023 OR sum(size_bytes)>536870912").await?,
            "result_receipt_input_budget", "result_retention").await?;
        let owned = blobs.clone();
        runtime
            .blocking(move || -> std::io::Result<()> {
                for receipt in receipts {
                    owned.verify(&receipt, 512 * 1024 * 1024)?;
                }
                Ok(())
            })
            .await??;
        let retention = crate::retention::RetentionStore::new(self.clone(), runtime.clone());
        let obligation = retention
            .create_obligation(
                artifact.artifact_id.clone(),
                vec![crate::retention::Dependency::TableScope {
                    table_uri: format!("result_{}", index.record.data.kind()),
                }],
            )
            .await?;
        let record = RetainedResult {
            result_artifact_id: artifact.artifact_id.clone(),
            body_base: base,
            version: crate::result_relations::retain(
                &self.delta_namespace(),
                &artifact.artifact_id,
                &index.record,
            )
            .await?,
            sections: index
                .sections
                .iter()
                .map(|(name, window)| ResultSection {
                    name: name.clone(),
                    start: window.start,
                    end: window.end,
                })
                .collect(),
            references: index
                .references
                .iter()
                .map(|reference| ResultReference {
                    artifact_id: reference.receipt.artifact_id.clone(),
                    media_type: reference.receipt.media_type.clone(),
                })
                .collect(),
        };
        let root = retained_root(&record, 0);
        retention
            .release_obligation(&obligation, root.dependencies.clone())
            .await?;
        let batch = RetainedResult::batch(&[record])?;
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(runtime).await?;
            crate::native_catalog::work(
                &session,
                "issued_result",
                session.read_batch(batch.clone())?.into_view(),
            )?;
            crate::native_catalog::work(
                &session,
                "issued_artifacts",
                pin.artifacts(runtime)
                    .await?
                    .union(crate::artifact_catalog::flatten(
                        session.read_batch(receipt_batch.clone())?,
                    )?)?
                    .into_view(),
            )?;
            runtime.require_empty(session.sql("WITH refs AS (SELECT unnest(references) AS child FROM issued_result) SELECT child.artifact_id FROM refs LEFT ANTI JOIN issued_artifacts a ON child.artifact_id=a.artifact_id AND child.media_type=a.media_type LIMIT 1").await?, "result_artifact_reference", "result_retention").await?;
            let missing = runtime.execute(session.sql("SELECT n.* FROM issued_result n LEFT ANTI JOIN state.records.retained_results r ON n.result_artifact_id=r.result_artifact_id").await?).await?;
            if missing.rows == 0 {
                retention.settle_selected(&obligation).await?;
                return Ok(());
            }
            let mut records = missing
                .batches
                .into_iter()
                .map(|b| (Table::RetainedResults, b))
                .collect::<Vec<_>>();
            records.push((Table::ArtifactReceipts, receipt_batch.clone()));
            records.push((
                Table::RetentionRoots,
                crate::retention::RetentionRoot::batch(&[crate::retention::RetentionRoot {
                    sequence: pin.generation() + 1,
                    ..root.clone()
                }])?,
            ));
            if self
                .commit_native(
                    pin.generation(),
                    records,
                    vec![format!("result/{}", artifact.artifact_id)],
                )
                .await?
                .is_some()
            {
                retention.settle_selected(&obligation).await?;
                return Ok(());
            }
        }
        Err(DataFusionError::Execution(
            "result retention conflict bound exceeded".into(),
        ))
    }
}

impl ControlSnapshot {
    /// The immutable result identity selects one typed recovery header in the captured catalog.
    pub async fn retained_result(
        &self,
        runtime: &QueryRuntime,
        id: &str,
    ) -> Result<RetainedResult> {
        let session = self.session(runtime).await?;
        let selected = session
            .table("state.records.retained_results")
            .await?
            .filter(col("result_artifact_id").eq(lit(id)))?;
        runtime
            .records::<RetainedResult>(selected, 1)
            .await?
            .pop()
            .ok_or_else(|| {
                DataFusionError::Execution("result has no native retained declaration".into())
            })
    }
    /// Reconstruct the complete native result from its captured purpose-specific Delta version.
    pub async fn result_record(
        &self,
        declared: &RetainedResult,
    ) -> Result<enrichment_core::operation::results::ResultRecord> {
        let protection = self
            .protect(
                declared.result_artifact_id.clone(),
                crate::retention::ProtectionKind::Query,
                retained_root(declared, 0).dependencies,
            )
            .await?;
        crate::result_relations::read(
            &self.delta,
            &declared.result_artifact_id,
            &declared.version,
            protection,
        )
        .await
    }
    /// Native recursive closure, with explicit cycle/depth/cardinality/byte refusal. Selection
    /// uses this captured catalog; the byte driver only verifies the resulting finite inventory.
    pub async fn result_dependencies(
        &self,
        runtime: &QueryRuntime,
        blobs: &BlobStore,
        root: &Artifact,
    ) -> Result<Vec<Artifact>> {
        let session = self.session(runtime).await?;
        let selected = session
            .table("state.records.retained_results")
            .await?
            .filter(col("result_artifact_id").eq(lit(root.artifact_id.clone())))?;
        if runtime
            .execute(selected.clone().limit(0, Some(2))?)
            .await?
            .rows
            != 1
        {
            return Err(DataFusionError::Execution(
                "result is not enrolled in captured native retention".into(),
            ));
        }
        crate::native_catalog::work(&session, "selected_result", selected.into_view())?;
        crate::native_catalog::work(
            &session,
            "issued_artifacts",
            self.artifacts(runtime).await?.into_view(),
        )?;
        let closure = session.sql("WITH RECURSIVE edges AS (SELECT result_artifact_id,unnest(references) AS child FROM state.records.retained_results), walk AS (SELECT result_artifact_id AS id, make_array(result_artifact_id) AS path, 0 AS depth, false AS cycle FROM selected_result UNION ALL SELECT e.child.artifact_id, array_append(w.path,e.child.artifact_id), w.depth+1, array_has(w.path,e.child.artifact_id) FROM walk w JOIN edges e ON w.id=e.result_artifact_id WHERE w.depth<64 AND NOT w.cycle) SELECT * FROM walk").await?;
        crate::native_catalog::work(&session, "result_closure", closure.into_view())?;
        runtime
            .require_empty(
                session
                    .sql("SELECT id FROM result_closure WHERE cycle OR depth=64 LIMIT 1")
                    .await?,
                "result_dependency_depth_cycle",
                "result_retention",
            )
            .await?;
        runtime.require_empty(session.sql("SELECT w.id FROM result_closure w LEFT ANTI JOIN issued_artifacts a ON w.id=a.artifact_id LIMIT 1").await?,"result_dependency_present", "result_retention").await?;
        let selected = session.sql("WITH candidates AS (SELECT a.*, row_number() OVER (PARTITION BY a.artifact_id ORDER BY a.source_uri,a.retrieved_at,a.media_type,a.kind) AS position FROM issued_artifacts a JOIN (SELECT DISTINCT id FROM result_closure) c ON a.artifact_id=c.id) SELECT * EXCLUDE (position) FROM candidates WHERE position=1").await?;
        crate::native_catalog::work(&session, "closure_artifacts", selected.clone().into_view())?;
        runtime.require_empty(session.sql("SELECT 'artifact_closure_budget' AS witness FROM closure_artifacts HAVING count(*)>1024 OR sum(size_bytes)>536870912").await?,"result_dependency_budget","result_retention").await?;
        let output = runtime.execute(selected).await?;
        let mut artifacts = Vec::new();
        for batch in output.batches {
            let rows = RowSet::batch(&batch)?;
            for i in 0..batch.num_rows() {
                artifacts.push(acquisitions::decode_one(rows.row(i))?);
            }
        }
        let owned = blobs.clone();
        let root_id = root.artifact_id.clone();
        runtime
            .blocking(move || -> std::io::Result<Vec<Artifact>> {
                for artifact in &artifacts {
                    owned.verify(artifact, 512 * 1024 * 1024)?;
                }
                Ok(artifacts
                    .into_iter()
                    .filter(|a| a.artifact_id != root_id)
                    .collect())
            })
            .await?
            .map_err(Into::into)
    }

    pub async fn result_section(
        &self,
        runtime: &QueryRuntime,
        id: &str,
        name: &str,
    ) -> Result<Option<crate::result::Window>> {
        let session = self.session(runtime).await?;
        let selected = session
            .table("state.records.retained_results")
            .await?
            .filter(col("result_artifact_id").eq(lit(id)))?;
        crate::native_catalog::work(&session, "selected_result", selected.into_view())?;
        let frame = session.sql("WITH sections AS (SELECT body_base, unnest(sections) AS section FROM selected_result) SELECT body_base+section.start AS start, body_base+section.end AS end, section.name AS name FROM sections").await?.filter(col("name").eq(lit(name)))?.limit(0,Some(2))?;
        let output = runtime.execute(frame).await?;
        if output.rows > 1 {
            return Err(DataFusionError::Execution(
                "ambiguous native result section".into(),
            ));
        }
        for batch in output.batches {
            if batch.num_rows() != 0 {
                let rows = RowSet::batch(&batch)?;
                let r = rows.row(0);
                return Ok(Some(crate::result::Window {
                    start: r.number("start")?,
                    end: r.number("end")?,
                }));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn captured_native_receipts_sections_and_dependency_integrity() {
        let root = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default()).unwrap();
        let catalog = ControlStore::open(root.path(), runtime.clone()).unwrap();
        let blobs = BlobStore::open(root.path()).unwrap();
        let before = catalog.pin().await.unwrap();
        let child = blobs
            .put(b"exact child", |_| {
                Artifact::describe(
                    b"exact child",
                    enrichment_core::evidence::ArtifactKind::Other,
                    "text/plain",
                    "fixture:child",
                    enrichment_core::native_time::AcquisitionTime::try_from(
                        "2026-09-16T00:00:00.000000Z".to_owned(),
                    )
                    .unwrap(),
                )
            })
            .unwrap()
            .acquired;
        let mut answer: enrichment_core::wire::Envelope =
            serde_json::from_str(include_str!("../../../tests/fixtures/wire/ok.fixture.json"))
                .unwrap();
        answer.artifacts = vec![enrichment_core::wire::ArtifactHandle {
            uri: format!("library-evidence://artifacts/{}", child.artifact_id)
                .try_into()
                .unwrap(),
            receipt: child.clone(),
            description: "dependency".into(),
        }];
        answer.summary = "exact retained text".into();
        let (result, _) = crate::result::store(&blobs, &answer, crate::result::JOB_URI).unwrap();
        catalog
            .retain_result(&runtime, &blobs, &result)
            .await
            .unwrap();
        let selected = catalog.pin().await.unwrap();
        assert!(
            before
                .artifact(&runtime, &child.artifact_id)
                .await
                .unwrap()
                .is_none()
        );
        assert_eq!(
            selected
                .artifact(&runtime, &child.artifact_id)
                .await
                .unwrap(),
            Some(child.clone())
        );
        let window = selected
            .result_section(&runtime, &result.artifact_id, "data.text")
            .await
            .unwrap()
            .unwrap();
        use std::io::{Read, Seek};
        let mut file = blobs.capture(&result, crate::result::MAX_BYTES).unwrap();
        file.seek(std::io::SeekFrom::Start(window.start)).unwrap();
        let mut bytes = Vec::new();
        file.take(window.end - window.start)
            .read_to_end(&mut bytes)
            .unwrap();
        assert_eq!(
            serde_json::from_slice::<String>(&bytes).unwrap(),
            "exact retained text"
        );
        std::fs::remove_dir(blobs.root().join(".staging")).unwrap();
        assert_eq!(
            selected
                .result_dependencies(&runtime, &blobs, &result)
                .await
                .unwrap(),
            vec![child.clone()]
        );
        std::fs::write(blobs.path_for(&child.sha256), b"wrong child").unwrap();
        assert!(
            selected
                .result_dependencies(&runtime, &blobs, &result)
                .await
                .is_err()
        );
        assert!(!blobs.root().join(".staging").exists());
    }
}
