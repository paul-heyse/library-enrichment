//! Revision source facts are retained before coverage or evidence selection consumes them.
use crate::{control::ControlStore, immutable_definitions::Definitions, runtime::QueryRuntime};
use datafusion::{common::Result, functions::core::expr_ext::FieldAccessor, prelude::col};
use enrichment_core::{
    native_union::NativeStruct,
    operation::{
        retention::Dependency,
        sources::{RevisionCapture, RevisionReceipt},
    },
    producer::revision::RevisionInputs,
};

/// Admit the captured commit response before archive acquisition. Source syntax decoding
/// does not decide whether a mutable or mismatched revision is authoritative.
pub async fn commit_tree(
    runtime: &QueryRuntime,
    expected: &str,
    response: enrichment_core::producer::revision::CommitResponse,
) -> Result<String> {
    use enrichment_core::{native_union::Rule, producer::revision::CommitResponse};
    enrichment_core::native_struct! { struct Input {
        expected: String => Rule::NonEmpty,
        response: CommitResponse => Rule::Text,
    } }
    enrichment_core::native_struct! { struct Tree { tree: String => Rule::NonEmpty } }
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "commit_capture",
        Input::batch(&[Input {
            expected: expected.into(),
            response,
        }])?,
    )?;
    runtime.require_empty(session.sql("SELECT 'revision_commit' AS witness FROM commit_capture WHERE response.sha<>expected OR NOT regexp_like(expected,'^[0-9a-f]{40}$') OR NOT regexp_like(response.commit.tree.sha,'^[0-9a-fA-F]{40}$')").await?, "revision_commit", "source_capture").await?;
    runtime
        .records::<Tree>(
            session
                .sql("SELECT lower(response.commit.tree.sha) AS tree FROM commit_capture")
                .await?,
            1,
        )
        .await?
        .pop()
        .map(|row| row.tree)
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("commit capture missing"))
}

pub async fn retain(
    control: ControlStore,
    runtime: &QueryRuntime,
    source: RevisionCapture,
) -> Result<RevisionReceipt> {
    let session = runtime.session();
    let input = crate::native_catalog::batch(
        &session,
        "revision_capture",
        RevisionCapture::batch(std::slice::from_ref(&source))?,
    )?;
    crate::native_catalog::work(&session, "revision_capture", input.clone().into_view())?;
    runtime.require_empty(session.sql("SELECT 'revision_source_binding' AS witness FROM revision_capture WHERE inputs.archive_sha256<>archive.sha256 OR archive_response.status<>200 OR commit_response.status<>200 OR NOT regexp_like(tree,'^[a-fA-F0-9]{40}$') OR NOT regexp_like(identity.commit,'^[a-f0-9]{40}$') OR archive_response.retrieved_at<>archive.retrieved_at OR commit_response.retrieved_at<>commit.retrieved_at").await?, "revision_source_binding", "source_capture").await?;
    control
        .retain_artifacts(runtime, &[source.archive.clone(), source.commit.clone()])
        .await?;
    let definitions =
        Definitions::<enrichment_core::identity::RevisionCaptureId>::new(control, runtime.clone());
    let (capture_id, binding) = definitions
        .retain_plan(
            input,
            vec![
                Dependency::Artifact {
                    artifact_id: source.archive.artifact_id.clone(),
                },
                Dependency::Artifact {
                    artifact_id: source.commit.artifact_id.clone(),
                },
            ],
        )
        .await?;
    let retained = definitions.read(&capture_id, &binding).await?;
    let inputs = retained.clone().select(
        RevisionInputs::fields()
            .iter()
            .map(|field| col("inputs").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )?;
    let extraction = crate::coverage::assess_revision_inputs(runtime, inputs).await?;
    let source = runtime
        .records(retained.drop_columns(&["capture_id"])?, 1)
        .await?
        .pop()
        .ok_or_else(|| {
            datafusion::common::DataFusionError::Execution(
                "retained revision capture missing".into(),
            )
        })?;
    Ok(RevisionReceipt {
        capture_id,
        binding,
        source,
        extraction,
    })
}

pub fn decoder_identity() -> String {
    enrichment_core::canonical::sha256_hex(
        concat!(
            include_str!("../../enrichment-core/src/producer/revision.rs"),
            include_str!("../../enrichment-core/src/producer/revision/inputs.rs"),
            include_str!("../../enrichment-core/src/archive/mod.rs"),
            include_str!("../../../Cargo.lock"),
        )
        .as_bytes(),
    )
}

#[cfg(test)]
mod commit_tests {
    use super::*;
    use enrichment_core::producer::revision::CommitResponse;

    #[tokio::test]
    async fn native_commit_capture_refuses_wrong_or_malformed_identity() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let expected = "a".repeat(40);
        let source: CommitResponse = serde_json::from_value(serde_json::json!({
            "sha": expected, "commit": {"tree": {"sha": "B".repeat(40)}, "message": "ignored"},
            "parents": [],
        }))
        .expect("typed commit fixture");
        assert_eq!(
            commit_tree(&runtime, &expected, source.clone()).await?,
            "b".repeat(40)
        );
        assert!(
            commit_tree(&runtime, &"c".repeat(40), source.clone())
                .await
                .is_err()
        );
        let mut malformed = source;
        malformed.commit.tree.sha = "branch-name".into();
        assert!(commit_tree(&runtime, &expected, malformed).await.is_err());
        assert!(
            serde_json::from_str::<CommitResponse>(r#"{"sha":"missing-tree","commit":{}}"#)
                .is_err()
        );
        runtime.close_diagnostics().await
    }
}
