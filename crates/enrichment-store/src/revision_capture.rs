//! Revision source facts are retained before coverage or evidence selection consumes them.
use crate::{control::ControlStore, immutable_definitions::Definitions, runtime::QueryRuntime};
use datafusion::{common::Result, functions::core::expr_ext::FieldAccessor, prelude::col};
use enrichment_core::{
    native_key::Key,
    native_union::NativeStruct,
    operation::{
        retention::Dependency,
        sources::{RevisionCapture, RevisionReceipt},
    },
    producer::revision::RevisionInputs,
};

pub async fn retain(
    control: ControlStore,
    runtime: &QueryRuntime,
    source: RevisionCapture,
) -> Result<RevisionReceipt> {
    let session = runtime.session();
    let input = session.read_batch(RevisionCapture::batch(std::slice::from_ref(&source))?)?;
    crate::native_catalog::work(&session, "revision_capture", input.clone().into_view())?;
    runtime.require_empty(session.sql("SELECT 'revision_source_binding' AS witness FROM revision_capture WHERE inputs.archive_sha256<>archive.sha256 OR archive_response.status<>200 OR commit_response.status<>200 OR NOT regexp_like(tree,'^[a-fA-F0-9]{40}$') OR NOT regexp_like(identity.commit,'^[a-f0-9]{40}$') OR archive_response.retrieved_at<>archive.retrieved_at OR commit_response.retrieved_at<>commit.retrieved_at").await?, "revision_source_binding", "source_capture").await?;
    control
        .retain_artifacts(runtime, &[source.archive.clone(), source.commit.clone()])
        .await?;
    let definitions = Definitions::new(
        control,
        runtime.clone(),
        "revision_captures",
        Key::RevisionCapture,
        "capture_id",
    );
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
