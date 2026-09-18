//! A process supplies bounded facts; native expressions choose the probe payload and outcome.
use std::ops::Not;

use crate::runtime::QueryRuntime;
use datafusion::{
    common::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::execution::ExecutionPayload,
    execution::VerificationCapture,
    native_union::{Cell, NativeStruct, Rule},
    producer::RunOutcome,
    wire::{Coverage, EvidenceClass, JobState, Outcome},
};

enrichment_core::native_struct! { pub struct LoweredProbe {
    payload: ExecutionPayload => Rule::Text,
    state: JobState => Rule::Text,
    evidence_class: EvidenceClass => Rule::Text,
    run_outcome:RunOutcome => Rule::Text,
    summary:String => Rule::Text,
    coverage:Coverage => Rule::Text,
    limitations:Vec<String> => Rule::Sequence,
    outcome:Outcome => Rule::Text,
} }

pub async fn lower(runtime: &QueryRuntime, capture: &VerificationCapture) -> Result<LoweredProbe> {
    lower_at(runtime, capture, false).await
}

/// Called by the physical supervisor after every owned process and cleanup owner has joined.
/// This observes settled cleanup without changing the original captured process facts.
pub async fn settled(
    runtime: &QueryRuntime,
    capture: &VerificationCapture,
) -> Result<LoweredProbe> {
    lower_at(runtime, capture, true).await
}

async fn lower_at(
    runtime: &QueryRuntime,
    capture: &VerificationCapture,
    settled: bool,
) -> Result<LoweredProbe> {
    use datafusion::{functions::string::expr_fn::concat, functions_nested::expr_fn::array_concat};
    use enrichment_core::evidence::arrow_model::expressions::{literal, record};
    let frame = crate::native_catalog::batch(
        &runtime.session(),
        "probe_plan",
        VerificationCapture::batch(std::slice::from_ref(capture))?,
    )?;
    let frame = frame.with_column(
        "last",
        datafusion::functions_nested::expr_fn::array_element(col("observations"), lit(-1i64)),
    )?;
    runtime
        .require_empty(
            frame
                .clone()
                .filter(
                    col("last")
                        .is_null()
                        .or(datafusion::functions_nested::expr_fn::array_length(col(
                            "observations",
                        ))
                        .gt(lit(64u64)))
                        .or(datafusion::functions_nested::expr_fn::array_length(col(
                            "input_artifacts",
                        ))
                        .gt(lit(4098u64))),
                )?
                .select(vec![col("job_id")])?,
            "probe_capture_bounds",
            "probe_normalization",
        )
        .await?;
    let success = col("last")
        .field("end")
        .eq(lit("exited"))
        .and(col("last").field("exit_code").eq(lit(0i32)));
    let state = datafusion::logical_expr::when(
        col("last").field("end").eq(lit("cancelled")),
        lit("cancelled"),
    )
    .when(
        success
            .clone()
            .and(col("last").field("cleanup_confirmed").or(lit(settled))),
        lit("succeeded"),
    )
    .when(success.clone(), lit("partial"))
    .otherwise(lit("failed"))?;
    let mode = col("request").field("mode");
    let class =
        datafusion::logical_expr::when(mode.clone().eq(lit("compile")), lit("compiler_derived"))
            .when(
                mode.clone().eq(lit("typecheck")),
                lit("typechecker_observed"),
            )
            .otherwise(lit("runtime_observed"))?;
    let snippet = datafusion::functions::string::expr_fn::concat(vec![
        lit("art_"),
        datafusion::functions::encoding::expr_fn::encode(
            datafusion::functions::crypto::expr_fn::sha256(col("request").field("snippet")),
            lit("hex"),
        ),
    ]);
    let payload = ExecutionPayload::usage_probe_expression(mode, snippet, col("last"))?;
    let run_outcome =
        datafusion::logical_expr::when(success, lit("succeeded")).otherwise(lit("failed"))?;
    let limitations=array_concat(vec![literal(&vec![
        "Only this agent-supplied snippet was checked; no complete compatibility or assertion-coverage claim.".to_owned(),
        "Synthetic consumer: project files, project lock, private configuration and native system dependencies were not imported.".to_owned(),
    ])?,datafusion::logical_expr::when(lit(settled),literal(&vec!["All owned execution cleanup settled before this result was published; process observations retain each initial cleanup result.".to_owned()])?)
    .when(col("last").field("cleanup_confirmed").not(),literal(&vec!["Removal of the owned execution container was not confirmed; the cleanup supervisor is still retrying and new execution admission may be quarantined.".to_owned()])?)
    .otherwise(literal(&Vec::<String>::new())?)?]);
    let frame = frame
        .with_column("state", state)?
        .with_column("run_outcome", run_outcome)?
        .with_column("limitations", limitations)?;
    let summary=datafusion::logical_expr::when(col("state").eq(lit("succeeded")).and(lit(settled)),lit("The isolated consumer probe succeeded; its scoped result is retained in the published snapshot."))
        .when(col("state").eq(lit("succeeded")),lit("The requested isolated consumer probe completed successfully within its recorded scope."))
        .when(col("state").eq(lit("partial")),lit("The probe completed, but removal of its execution container was not confirmed."))
        .otherwise(lit("The isolated consumer probe did not succeed; read the recorded process outcome and logs."))?;
    let missing = datafusion::logical_expr::when(
        col("state").eq(lit("succeeded")),
        literal(&Vec::<String>::new())?,
    )
    .when(
        col("state").eq(lit("partial")),
        literal(&vec!["confirmed execution container removal".to_owned()])?,
    )
    .otherwise(literal(&vec!["successful consumer probe".to_owned()])?)?;
    let coverage = record(
        &Coverage::data_type(),
        &[
            (
                "scope",
                concat(vec![
                    col("request").field("mode"),
                    lit(" of one supplied consumer snippet"),
                ]),
            ),
            ("indexed", literal(&Vec::<String>::new())?),
            ("missing", missing),
            ("limitations", col("limitations")),
            (
                "assessments",
                literal(&Vec::<enrichment_core::wire::evidence::ScopeAssessment>::new())?,
            ),
        ],
    )?;
    let error=enrichment_core::wire::ErrorDetail {
        code:enrichment_core::wire::ErrorCode::VerificationFailed,
        message:"The isolated consumer probe did not succeed; read the recorded process outcome and logs.".into(),
        retryable:false,
        next_action:"Correct the snippet or environment using the retained diagnostics, then submit a new probe.".into(),
        diagnostic:enrichment_core::wire::Diagnostic::for_error(enrichment_core::wire::ErrorCode::VerificationFailed,"Correct the snippet or environment using the retained diagnostics, then submit a new probe.".into()),
    };
    let outcome = datafusion::logical_expr::when(
        col("state").eq(lit("succeeded")),
        literal(&Outcome::Ok { job: None })?,
    )
    .when(
        col("state").eq(lit("partial")),
        literal(&Outcome::Partial { job: None })?,
    )
    .otherwise(literal(&Outcome::Error { job: None, error })?)?;
    runtime
        .records::<LoweredProbe>(
            frame.select(vec![
                payload.alias("payload"),
                col("state"),
                class.alias("evidence_class"),
                col("run_outcome"),
                summary.alias("summary"),
                coverage.alias("coverage"),
                col("limitations"),
                outcome.alias("outcome"),
            ])?,
            1,
        )
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("native probe lowering missing".into()))
}
