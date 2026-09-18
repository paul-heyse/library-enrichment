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
use enrichment_core::evidence::Artifact;
use enrichment_core::evidence::arrow_model::cells::RowSet;
use std::sync::Arc;

use enrichment_core::{
    native_union::NativeStruct,
    operation::results::{ResultReference, ResultSection, RetainedResult},
};
enrichment_core::native_struct! { struct SnapshotReference { snapshot_id: enrichment_core::identity::SnapshotId => enrichment_core::native_union::Rule::Text } }

async fn snapshot_references(
    runtime: &QueryRuntime,
    record: &enrichment_core::operation::results::ResultRecord,
) -> Result<Vec<enrichment_core::identity::SnapshotId>> {
    use enrichment_core::{native_union::Domain, operation::results::ResultRecord};
    let session = runtime.session();
    let frame = crate::native_catalog::batch(
        &session,
        "result_catalog",
        ResultRecord::batch(std::slice::from_ref(record))?,
    )?
    .with_column("result_owner", lit("result"))?;
    let Some(references) = crate::field_admission::identity_values(
        frame,
        &Schema::new(ResultRecord::fields()),
        "result_owner",
        Domain::Snapshot,
    )?
    else {
        return Ok(Vec::new());
    };
    Ok(runtime
        .records::<SnapshotReference>(
            references
                .select(vec![col("value").alias("snapshot_id")])?
                .sort(vec![col("snapshot_id").sort(true, false)])?,
            32,
        )
        .await?
        .into_iter()
        .map(|r| r.snapshot_id)
        .collect())
}
pub(crate) fn schema() -> SchemaRef {
    Arc::new(Schema::new(RetainedResult::fields()))
}

/// Native result values and their exact physical dependency protection share one lifetime.
/// A delivery owner retains this through its final file read, not only the Delta row decode.
pub struct CapturedResult {
    pub record: enrichment_core::operation::results::ResultRecord,
    _protection: crate::leases::ReadProtection,
}

pub(crate) fn retained_root(
    record: &RetainedResult,
    sequence: u64,
) -> crate::retention::RetentionRoot {
    use crate::retention::{Dependency, RetentionRoot, TableSelection};
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
        value: TableSelection {
            source: record.version.source.clone(),
            row: Some(crate::retention::RowKey {
                column: "result_artifact_id".into(),
                value: enrichment_core::identity::RowValue::Text {
                    value: record.result_artifact_id.clone(),
                },
            }),
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
            crate::artifact_catalog::flatten(crate::native_catalog::batch(
                &admission,
                "result_catalog",
                receipt_batch.clone(),
            )?)?
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
                vec![crate::retention::pending_row(
                    &format!("result_{}", index.record.data.kind()),
                    &crate::result_relations::contract(index.record.data.kind())?,
                    crate::retention::RowKey {
                        column: "result_artifact_id".into(),
                        value: enrichment_core::identity::RowValue::Text {
                            value: artifact.artifact_id.clone(),
                        },
                    },
                )?],
            )
            .await?;
        let record = RetainedResult {
            result_artifact_id: artifact.artifact_id.clone(),
            body_base: base,
            snapshots: snapshot_references(runtime, &index.record).await?,
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
                crate::native_catalog::batch(&session, "result_catalog", batch.clone())?
                    .into_view(),
            )?;
            crate::native_catalog::work(
                &session,
                "issued_artifacts",
                pin.artifacts(runtime)
                    .await?
                    .union(crate::artifact_catalog::flatten(
                        crate::native_catalog::batch(
                            &session,
                            "result_catalog",
                            receipt_batch.clone(),
                        )?,
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
        Ok(self.capture_result(declared).await?.record)
    }
    pub async fn capture_result(&self, declared: &RetainedResult) -> Result<CapturedResult> {
        let protection = self
            .protect(
                declared.result_artifact_id.clone(),
                crate::retention::ProtectionKind::Query,
                retained_root(declared, 0).dependencies,
                std::slice::from_ref(&declared.result_artifact_id),
            )
            .await?;
        let record = crate::result_relations::read(
            &self.delta,
            &declared.result_artifact_id,
            &declared.version,
            protection.clone(),
        )
        .await?;
        Ok(CapturedResult {
            record,
            _protection: protection,
        })
    }
    /// Native recursive closure, with explicit cycle/depth/cardinality/byte refusal. Selection
    /// uses this captured catalog; the byte driver only verifies the resulting finite inventory.
    pub async fn result_dependencies(
        &self,
        runtime: &QueryRuntime,
        blobs: &BlobStore,
        root: &Artifact,
    ) -> Result<Vec<Artifact>> {
        self.result_artifact_closure(runtime, blobs, std::slice::from_ref(root), true)
            .await
    }

    /// Verify one native-selected union of retained results and transitive artifacts.
    pub(crate) async fn result_artifacts(
        &self,
        runtime: &QueryRuntime,
        blobs: &BlobStore,
        roots: &[Artifact],
    ) -> Result<Vec<Artifact>> {
        self.result_artifact_closure(runtime, blobs, roots, false)
            .await
    }

    async fn result_artifact_closure(
        &self,
        runtime: &QueryRuntime,
        blobs: &BlobStore,
        roots: &[Artifact],
        exclude_roots: bool,
    ) -> Result<Vec<Artifact>> {
        let session = self.session(runtime).await?;
        crate::native_catalog::work(
            &session,
            "issued_artifacts",
            self.artifacts(runtime).await?.into_view(),
        )?;
        let selected = dependency_plan(runtime, &session, roots).await?;
        let artifacts = runtime.records::<Artifact>(selected, 1024).await?;
        if artifacts.is_empty() {
            return Ok(artifacts);
        }
        let returned = if exclude_roots {
            Some(runtime.records::<Artifact>(session.sql("SELECT a.* FROM closure_artifacts a LEFT ANTI JOIN requested_result_root r ON a.artifact_id=r.artifact_id ORDER BY a.artifact_id").await?,1024).await?)
        } else {
            None
        };
        let root_ids = roots
            .iter()
            .map(|root| root.artifact_id.clone())
            .collect::<Vec<_>>();
        let protection = self
            .protect(
                format!("result-validation/{}", uuid::Uuid::new_v4()),
                crate::retention::ProtectionKind::Query,
                artifacts
                    .iter()
                    .map(|artifact| crate::retention::Dependency::Artifact {
                        artifact_id: artifact.artifact_id.clone(),
                    })
                    .collect(),
                &root_ids,
            )
            .await?;
        let blobs = blobs.clone();
        runtime
            .blocking(move || -> std::io::Result<Vec<Artifact>> {
                let _protection = protection;
                for artifact in &artifacts {
                    blobs.verify(artifact, 512 * 1024 * 1024)?;
                }
                Ok(returned.unwrap_or(artifacts))
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

/// Decisions over a captured native catalog; no blob reads or fresh catalog generations.
async fn dependency_plan(
    runtime: &QueryRuntime,
    session: &datafusion::prelude::SessionContext,
    roots: &[Artifact],
) -> Result<datafusion::dataframe::DataFrame> {
    if roots.len() > 1024 {
        return datafusion::common::exec_err!("result root input bound");
    }
    crate::native_catalog::input(session, "requested_result_root", Artifact::batch(roots)?)?;
    runtime.require_empty(session.sql("SELECT artifact_id AS witness FROM requested_result_root GROUP BY artifact_id HAVING count(DISTINCT sha256)<>1 OR count(DISTINCT size_bytes)<>1 UNION ALL SELECT r.artifact_id FROM requested_result_root r LEFT ANTI JOIN state.records.retained_results t ON r.artifact_id=t.result_artifact_id").await?,"result_root_declaration","result_retention").await?;
    let selected=session.sql("SELECT t.* FROM state.records.retained_results t LEFT SEMI JOIN requested_result_root r ON t.result_artifact_id=r.artifact_id").await?;
    crate::native_catalog::work(session, "selected_result", selected.into_view())?;
    runtime.require_empty(session.sql("SELECT result_artifact_id AS witness FROM selected_result GROUP BY result_artifact_id HAVING count(*)<>1").await?,"result_root_cardinality","result_retention").await?;
    runtime.require_empty(session.sql("SELECT r.artifact_id AS witness FROM requested_result_root r LEFT ANTI JOIN issued_artifacts a ON r.artifact_id=a.artifact_id AND r.sha256=a.sha256 AND r.size_bytes=a.size_bytes").await?,"result_root_content_identity","result_retention").await?;
    let closure=session.sql("WITH RECURSIVE edges AS (SELECT result_artifact_id,unnest(references) AS child FROM state.records.retained_results), walk AS (SELECT result_artifact_id AS id, make_array(result_artifact_id) AS path, 0 AS depth, false AS cycle FROM selected_result UNION ALL SELECT e.child.artifact_id, array_append(w.path,e.child.artifact_id), w.depth+1, array_has(w.path,e.child.artifact_id) FROM walk w JOIN edges e ON w.id=e.result_artifact_id WHERE w.depth<64 AND NOT w.cycle) SELECT * FROM walk").await?;
    crate::native_catalog::work(session, "result_closure", closure.into_view())?;
    runtime
        .require_empty(
            session
                .sql("SELECT id AS witness FROM result_closure WHERE cycle OR depth=64 LIMIT 1")
                .await?,
            "result_dependency_depth_cycle",
            "result_retention",
        )
        .await?;
    runtime.require_empty(session.sql("SELECT w.id AS witness FROM result_closure w LEFT ANTI JOIN issued_artifacts a ON w.id=a.artifact_id LIMIT 1").await?,"result_dependency_present","result_retention").await?;
    runtime.require_empty(session.sql("SELECT a.artifact_id AS witness FROM issued_artifacts a JOIN (SELECT DISTINCT id FROM result_closure) c ON a.artifact_id=c.id GROUP BY a.artifact_id HAVING count(DISTINCT a.sha256)<>1 OR count(DISTINCT a.size_bytes)<>1").await?,"result_dependency_content_identity","result_retention").await?;
    let selected=session.sql("WITH candidates AS (SELECT a.*, row_number() OVER (PARTITION BY a.artifact_id ORDER BY a.source_uri,a.retrieved_at,a.media_type,a.kind) AS position FROM issued_artifacts a JOIN (SELECT DISTINCT id FROM result_closure) c ON a.artifact_id=c.id) SELECT * EXCLUDE (position) FROM candidates WHERE position=1").await?;
    crate::native_catalog::work(session, "closure_artifacts", selected.clone().into_view())?;
    runtime.require_empty(session.sql("SELECT 'artifact_closure_budget' AS witness FROM closure_artifacts HAVING count(*)>1024 OR sum(size_bytes)>536870912").await?,"result_dependency_budget","result_retention").await?;
    selected.sort(vec![col("artifact_id").sort(true, false)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::native_union::{Domain, Rule};
    enrichment_core::native_struct! {struct Edge {artifact_id:String=>Rule::Reference(Domain::Artifact)} }
    enrichment_core::native_struct! {struct Root {
        result_artifact_id:String=>Rule::Reference(Domain::Artifact),references:Vec<Edge> => Rule::Sequence
    } }

    #[tokio::test]
    async fn result_dependency_union_selects_once_and_refuses_cycles_missing_or_conflicting_content()
    -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let artifact = |name: &str| {
            Artifact::describe(
                name.as_bytes(),
                enrichment_core::evidence::ArtifactKind::Other,
                "application/octet-stream",
                &format!("unit://{name}"),
                enrichment_core::native_time::AcquisitionTime::from_micros(1).unwrap(),
            )
        };
        let a = artifact("a");
        let b = artifact("b");
        let child = artifact("child");
        let roots = [
            Root {
                result_artifact_id: a.artifact_id.clone(),
                references: vec![Edge {
                    artifact_id: child.artifact_id.clone(),
                }],
            },
            Root {
                result_artifact_id: b.artifact_id.clone(),
                references: vec![Edge {
                    artifact_id: child.artifact_id.clone(),
                }],
            },
        ];
        let selected = async |records: &[Root],
                              issued: &[Artifact],
                              requested: &[Artifact]|
               -> Result<Vec<Artifact>> {
            let temporary = runtime.session();
            let input =
                crate::native_catalog::batch(&temporary, "result_roots", Root::batch(records)?)?
                    .into_view();
            let session = runtime.bound_session(
                [(
                    "state".into(),
                    Arc::new(crate::native_catalog::BoundCatalog::default().with_schema(
                        crate::native_catalog::BindingKind::FoldedRecords,
                        [("retained_results".into(), input)].into_iter().collect(),
                    )) as Arc<dyn datafusion::catalog::CatalogProvider>,
                )]
                .into_iter()
                .collect(),
            )?;
            crate::native_catalog::input(&session, "issued_artifacts", Artifact::batch(issued)?)?;
            runtime
                .records(dependency_plan(&runtime, &session, requested).await?, 1024)
                .await
        };
        let mut other_origin = child.clone();
        other_origin.source_uri = "unit://second-origin".into();
        let issued = vec![a.clone(), b.clone(), child.clone(), other_origin];
        assert_eq!(
            selected(&roots, &issued, &[a.clone(), b.clone()])
                .await?
                .len(),
            3
        );
        assert!(selected(&roots, &issued, &[]).await?.is_empty());
        assert!(
            selected(&roots, &issued, std::slice::from_ref(&child))
                .await
                .is_err()
        );
        assert!(
            selected(&roots, &issued[..2], std::slice::from_ref(&a))
                .await
                .is_err()
        );
        let mut conflicting = issued.clone();
        conflicting[3].size_bytes += 1;
        assert!(
            selected(&roots, &conflicting, std::slice::from_ref(&a))
                .await
                .is_err()
        );
        let mut wrong = a.clone();
        wrong.sha256 = "0".repeat(64);
        assert!(selected(&roots, &issued, &[wrong]).await.is_err());
        let duplicated = [roots[0].clone(), roots[0].clone()];
        assert!(
            selected(&duplicated, &issued, std::slice::from_ref(&a))
                .await
                .is_err()
        );
        let cyclic = [
            Root {
                result_artifact_id: a.artifact_id.clone(),
                references: vec![Edge {
                    artifact_id: b.artifact_id.clone(),
                }],
            },
            Root {
                result_artifact_id: b.artifact_id.clone(),
                references: vec![Edge {
                    artifact_id: a.artifact_id.clone(),
                }],
            },
        ];
        assert!(
            selected(&cyclic, &issued, std::slice::from_ref(&a))
                .await
                .is_err()
        );
        let large = issued[..3]
            .iter()
            .cloned()
            .map(|mut artifact| {
                artifact.size_bytes = 200 * 1024 * 1024;
                artifact
            })
            .collect::<Vec<_>>();
        assert!(selected(&roots, &large, &large[..2]).await.is_err());
        runtime.close_diagnostics().await
    }
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
