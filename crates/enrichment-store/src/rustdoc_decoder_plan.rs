//! Native decoder admission and complete receipt validation. The worker only parses bytes.
use crate::{
    control_jobs::Grant, immutable_definitions::Definitions, native_catalog, runtime::QueryRuntime,
};
use datafusion::{
    common::{DataFusionError, Result, ScalarValue},
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::{Artifact, arrow_model::expressions::literal},
    execution::rustdoc_decoder::{self, Effect, Report, Request, StreamReceipt},
    native_union::NativeStruct,
    producer::rustdoc::facts::Fact,
};
use std::path::{Path, PathBuf};

pub(crate) struct Prepared {
    pub parent: Grant,
    pub value: Effect,
    _binding: enrichment_core::delta_reference::DeltaVersionRef,
}

pub(crate) async fn request(
    runtime: &QueryRuntime,
    artifact: &Artifact,
    root: &Path,
) -> Result<Request> {
    let session = runtime.session();
    let frame = crate::native_catalog::batch(
        &session,
        "rustdoc_decoder_plan",
        Artifact::batch(std::slice::from_ref(artifact))?,
    )?
    .select(vec![
        lit(rustdoc_decoder::PROTOCOL).alias("protocol"),
        col("artifact_id"),
        col("sha256"),
        col("size_bytes").alias("bytes"),
        literal(&root.to_owned())?.alias("root"),
        lit(rustdoc_decoder::DEADLINE_SECONDS).alias("deadline_seconds"),
    ])?;
    let request = runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("decoder request missing"))?;
    admit_request(runtime, &request).await?;
    Ok(request)
}

pub(crate) async fn admit_request(runtime: &QueryRuntime, request: &Request) -> Result<()> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "decoder_request",
        Request::batch(std::slice::from_ref(request))?,
    )?;
    let frame = session.table("decoder_request").await?;
    if let Some(violations) = enrichment_core::native_schema::intrinsic_violations(
        frame,
        &arrow::datatypes::Schema::new(Request::fields()),
    )? {
        runtime
            .require_empty(
                violations.select(vec![lit("request").alias("witness")])?,
                "decoder_request_contract",
                "decoder_admission",
            )
            .await?;
    }
    runtime.require_empty(session.sql("SELECT 'decoder_root' AS witness FROM decoder_request WHERE NOT starts_with(root,'/') OR contains(root,chr(0)) OR octet_length(root)>4096").await?,
        "decoder_root", "decoder_admission").await
}

pub(crate) async fn prepare(
    parent: Grant,
    request: Request,
    executable: PathBuf,
) -> Result<Prepared> {
    parent.check().await?;
    let runtime = parent.store().runtime();
    admit_request(runtime, &request).await?;
    let pin = parent.store().pin().await?;
    let session = pin.session(runtime).await?;
    let selected = session
        .sql(
            r#"
        SELECT c.grant_id,c.job_id,c.policy_id FROM state.records.claims c
          JOIN state.records.commands d ON c.job_id=d.job_id
          WHERE c.grant_id=$1 AND c.ecosystem='rust' AND d.arguments.kind='resolve'
    "#,
        )
        .await?
        .with_param_values(datafusion::common::ParamValues::List(vec![
            parent.id().parameter(),
        ]))?;
    let definitions = Definitions::<enrichment_core::identity::RustdocDecoderEffectId>::new(
        parent.control(),
        runtime.clone(),
    );
    let (id, binding) = definitions
        .retain_plan(
            selected.select(vec![
                col("grant_id"),
                col("job_id"),
                col("policy_id"),
                literal(&executable)?.alias("executable"),
                literal(&Vec::<String>::new())?.alias("argv"),
                literal(&std::collections::BTreeMap::<String, String>::new())?.alias("environment"),
                literal(&PathBuf::from("/"))?.alias("cwd"),
                lit(rustdoc_decoder::MEMORY_BYTES).alias("memory_bytes"),
                lit(rustdoc_decoder::CONTROL_BYTES as u64).alias("report_bytes"),
                lit(rustdoc_decoder::STDERR_BYTES as u64).alias("stderr_bytes"),
                lit(crate::runtime::DEFINITION_REVISION).alias("implementation"),
                literal(&request)?.alias("request"),
            ])?,
            vec![crate::retention::Dependency::Artifact {
                artifact_id: request.artifact_id.clone(),
            }],
        )
        .await?;
    let value = runtime
        .records(
            definitions
                .read(&id, &binding)
                .await?
                .drop_columns(&["effect_id"])?,
            1,
        )
        .await?
        .pop()
        .ok_or_else(|| invalid("retained decoder effect missing"))?;
    parent.check().await?;
    Ok(Prepared {
        parent,
        value,
        _binding: binding,
    })
}

/// No missing/duplicate stream, changed producer, report from another request or malformed
/// value may authorize opening worker output. File/EOS/hash checks remain physical mechanics.
pub(crate) async fn admit_report(
    runtime: &QueryRuntime,
    report: &Report,
    request_digest: &str,
) -> Result<()> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "decoder_report",
        Report::batch(std::slice::from_ref(report))?,
    )?;
    native_catalog::input(
        &session,
        "decoder_streams",
        StreamReceipt::batch(&report.streams)?,
    )?;
    let frame = session.table("decoder_report").await?;
    if let Some(violations) = enrichment_core::native_schema::intrinsic_violations(
        frame,
        &arrow::datatypes::Schema::new(Report::fields()),
    )? {
        runtime
            .require_empty(
                violations.select(vec![lit("report").alias("witness")])?,
                "decoder_report_contract",
                "decoder_output",
            )
            .await?;
    }
    runtime
        .require_empty(
            session
                .sql(
                    r#"
        SELECT 'decoder_identity' AS witness FROM decoder_report
          WHERE request_digest<>$1 OR producer_revision<>$2
        UNION ALL SELECT 'decoder_stream_inventory' AS witness FROM decoder_streams
          HAVING count(*)<>$3 OR count(DISTINCT fact)<>$3
        UNION ALL SELECT 'decoder_header_cardinality' AS witness FROM decoder_streams
          WHERE fact=$4 AND rows<>1
    "#,
                )
                .await?
                .with_param_values(vec![
                    ScalarValue::from(request_digest),
                    crate::runtime::DEFINITION_REVISION.into(),
                    (Fact::ALL.len() as u64).into(),
                    Fact::Header.as_str().into(),
                ])?,
            "decoder_receipt_identity",
            "decoder_output",
        )
        .await
}

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn decoder_native_contract_refuses_incomplete_or_rebound_receipts() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let expected = "b".repeat(64);
        let valid = Report {
            protocol: rustdoc_decoder::PROTOCOL.into(),
            request_digest: expected.clone(),
            producer_revision: crate::runtime::DEFINITION_REVISION.into(),
            streams: Fact::ALL
                .into_iter()
                .map(|fact| StreamReceipt {
                    fact,
                    digest: "a".repeat(64),
                    bytes: 4096,
                    rows: u64::from(fact == Fact::Header),
                })
                .collect(),
        };
        admit_report(&runtime, &valid, &expected).await?;
        let mut changed = valid.clone();
        changed.streams.pop();
        assert!(admit_report(&runtime, &changed, &expected).await.is_err());
        let mut changed = valid.clone();
        changed.streams[1] = changed.streams[0].clone();
        assert!(admit_report(&runtime, &changed, &expected).await.is_err());
        let mut changed = valid.clone();
        changed.streams[0].rows = 0;
        assert!(admit_report(&runtime, &changed, &expected).await.is_err());
        let mut changed = valid.clone();
        changed.streams[1].bytes = 7;
        assert!(admit_report(&runtime, &changed, &expected).await.is_err());
        let mut changed = valid.clone();
        changed.producer_revision = "another_build".into();
        assert!(admit_report(&runtime, &changed, &expected).await.is_err());
        let mut changed = valid.clone();
        changed.protocol = "native-rustdoc-arrow/2".into();
        assert!(admit_report(&runtime, &changed, &expected).await.is_err());
        assert!(
            admit_report(&runtime, &valid, &"c".repeat(64))
                .await
                .is_err()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn decoder_native_request_binds_exact_artifact_and_physical_bounds() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let artifact = Artifact::describe(
            b"input",
            enrichment_core::evidence::ArtifactKind::RustdocJson,
            "application/json",
            "https://example.invalid/input.json",
            enrichment_core::native_time::AcquisitionTime::from_micros(0)?,
        );
        let value = request(
            &runtime,
            &artifact,
            Path::new("/owned/private/rustdoc/fixture"),
        )
        .await?;
        assert_eq!(
            value.input(),
            Path::new("/owned/private/rustdoc/fixture/input.json")
        );
        assert_eq!(value.sha256, artifact.sha256);
        assert_eq!(value.bytes, 5);
        assert_eq!(value.deadline_seconds, 30);
        let mut changed = value.clone();
        changed.artifact_id = format!("art_{}", "a".repeat(64));
        assert!(admit_request(&runtime, &changed).await.is_err());
        let mut changed = value.clone();
        changed.bytes = 0;
        assert!(admit_request(&runtime, &changed).await.is_err());
        let mut changed = value.clone();
        changed.deadline_seconds = 31;
        assert!(admit_request(&runtime, &changed).await.is_err());
        let mut changed = value.clone();
        changed.root = "relative".into();
        assert!(admit_request(&runtime, &changed).await.is_err());
        let encoded = serde_json::to_value(&value).map_err(|e| invalid(&e.to_string()))?;
        assert_eq!(encoded["bytes"], "5");
        assert_eq!(encoded["deadline_seconds"], "30");
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
