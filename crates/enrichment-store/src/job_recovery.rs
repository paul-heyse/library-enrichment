//! Read-only recovery is a native join of the retained command, publication, attempt and
//! result. Producer logs are diagnostic bytes, never an alternative semantic authority.
use crate::{
    control::ControlSnapshot,
    native_catalog,
    repository::{EvidenceRepository, OpenedSnapshot},
    runtime::QueryRuntime,
};
use datafusion::{
    common::Result,
    functions::core::expr_ext::FieldAccessor,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    evidence::catalog::JobPublication,
    native_union::{Cell, NativeStruct, Rule},
    operation::{Arguments, jobs::JobSnapshot, results::ResultRecord},
};
use std::sync::Arc;

#[cfg(test)]
mod tests;

enrichment_core::native_struct! { struct Requested {
    job_id: enrichment_core::identity::JobId => Rule::Text,
    arguments: Arguments => Rule::Text,
} }

/// Keep the captured catalog and exact snapshot protection alive through final byte delivery.
pub struct RecoveredJob {
    pub publication: JobPublication,
    pub catalog: Arc<ControlSnapshot>,
    _snapshot: OpenedSnapshot,
    _result: crate::result_catalog::CapturedResult,
}

pub struct RecoveredComparison {
    pub publication: enrichment_core::evidence::catalog::ComparisonPublication,
    pub catalog: Arc<ControlSnapshot>,
    _before: OpenedSnapshot,
    _after: OpenedSnapshot,
    _result: crate::result_catalog::CapturedResult,
}

/// A comparison retains two independent exact input protections through byte delivery.
pub async fn comparison(
    repository: &EvidenceRepository,
    blobs: &crate::BlobStore,
    requested: &JobSnapshot,
) -> Result<Option<RecoveredComparison>> {
    let runtime = &repository.runtime;
    let catalog = repository.catalog.pin().await?;
    let Some(publication) = catalog
        .comparison_publication(runtime, &requested.job_id)
        .await?
    else {
        return Ok(None);
    };
    publication
        .validate()
        .map_err(datafusion::common::DataFusionError::Execution)?;
    let before = repository
        .open_snapshot(catalog.clone(), &publication.before_snapshot_id)
        .await?;
    let after = repository
        .open_snapshot(catalog.clone(), &publication.after_snapshot_id)
        .await?;
    let session = catalog.session(runtime).await?;
    command(
        runtime,
        &session,
        &requested.job_id,
        &requested.specification,
    )
    .await?;
    let selected = session.sql(r#"
        SELECT p.*,c.arguments FROM recovery_command c
        JOIN state.records.comparison_publications p ON p.job_id=c.job_id
        JOIN state.records.snapshots b ON b.snapshot_id=p.before_snapshot_id AND b.context_id=p.before_context_id
        JOIN state.records.snapshots a ON a.snapshot_id=p.after_snapshot_id AND a.context_id=p.after_context_id
        WHERE c.arguments.compare IS NOT NULL
    "#).await?;
    native_catalog::work(
        &session,
        "recovery_comparison",
        selected.clone().into_view(),
    )?;
    require(
        runtime,
        &session,
        "SELECT 'comparison_scope' AS witness FROM recovery_comparison HAVING count(*)<>1",
        "comparison_recovery_scope",
    )
    .await?;
    let request = enrichment_core::evidence::arrow_model::expressions::record(
        &enrichment_core::request::ResearchRequest::data_type(),
        &[
            ("method", lit("compare_releases")),
            (
                "compare_releases",
                col("arguments").field("compare").field("request"),
            ),
        ],
    )?;
    let key = enrichment_core::native_key::Key::ResearchInvocation;
    let declared = datafusion::functions::string::expr_fn::concat(vec![
        lit(format!("{}_", key.prefix())),
        col("request_digest"),
    ]);
    runtime
        .require_empty(
            selected
                .filter(key.bind(vec![request])?.not_eq(declared))?
                .select(vec![col("job_id")])?,
            "comparison_request_preimage",
            "job_recovery",
        )
        .await?;
    let declared = catalog
        .retained_result(runtime, &publication.delivery.artifact_id)
        .await?;
    let result = catalog.capture_result(&declared).await?;
    catalog
        .result_dependencies(runtime, blobs, &publication.delivery)
        .await?;
    crate::result_plan::admit_comparison(runtime, &publication, result.record.clone()).await?;
    Ok(Some(RecoveredComparison {
        publication,
        catalog,
        _before: before,
        _after: after,
        _result: result,
    }))
}

/// This does not claim a job, consult current execution permission or rerun a producer.
/// A missing publication is distinct from a malformed or revoked retained publication.
pub async fn recover(
    repository: &EvidenceRepository,
    blobs: &crate::BlobStore,
    requested: &JobSnapshot,
) -> Result<Option<RecoveredJob>> {
    let runtime = &repository.runtime;
    let catalog = repository.catalog.pin().await?;
    let Some(publication) = catalog.job_publication(runtime, &requested.job_id).await? else {
        return Ok(None);
    };
    publication
        .validate()
        .map_err(datafusion::common::DataFusionError::Execution)?;
    let snapshot = repository
        .open_snapshot(catalog.clone(), &publication.snapshot_id)
        .await?;
    let session = catalog.session(runtime).await?;
    command(
        runtime,
        &session,
        &requested.job_id,
        &requested.specification,
    )
    .await?;
    let selected = session
        .sql(
            r#"
        SELECT p.*,c.arguments,t.resolution,s.publication AS manifest,
               a.inputs,a.acquisitions,a.producer_binding_id,a.log
        FROM recovery_command c
        JOIN state.records.job_publications p ON p.job_id=c.job_id
        JOIN state.records.job_transitions t ON t.job_id=c.job_id
        JOIN state.records.snapshots s ON s.snapshot_id=p.snapshot_id AND s.context_id=p.context_id
        JOIN state.records.attempts a ON a.snapshot_id=p.snapshot_id AND a.attempt_id=p.attempt_id
    "#,
        )
        .await?;
    native_catalog::work(&session, "recovery_scope", selected.into_view())?;
    require(
        runtime,
        &session,
        "SELECT 'missing_or_ambiguous_scope' AS witness FROM recovery_scope HAVING count(*)<>1",
        "recovery_scope",
    )
    .await?;

    let retained = catalog
        .retained_result(runtime, &publication.delivery.artifact_id)
        .await?;
    let result = catalog.capture_result(&retained).await?;
    catalog
        .result_dependencies(runtime, blobs, &publication.delivery)
        .await?;
    crate::result_plan::admit_job(runtime, &publication, result.record.clone()).await?;
    native_catalog::work(
        &session,
        "recovery_result",
        crate::native_catalog::batch(
            &session,
            "job_recovery",
            ResultRecord::batch(std::slice::from_ref(&result.record))?,
        )?
        .into_view(),
    )?;
    admit(runtime, &session).await?;
    Ok(Some(RecoveredJob {
        publication,
        catalog,
        _snapshot: snapshot,
        _result: result,
    }))
}

/// Compare the defining native values themselves. A caller-supplied job key cannot stand
/// in for its preimage, and JSON object ordering/formatting is not an identity discriminator.
async fn command(
    runtime: &QueryRuntime,
    session: &SessionContext,
    job_id: &enrichment_core::identity::JobId,
    arguments: &Arguments,
) -> Result<()> {
    native_catalog::work(
        session,
        "recovery_requested",
        crate::native_catalog::batch(
            session,
            "job_recovery",
            Requested::batch(&[Requested {
                job_id: *job_id,
                arguments: arguments.clone(),
            }])?,
        )?
        .into_view(),
    )?;
    let commands = session
        .table("state.records.commands")
        .await?
        .filter(col("job_id").eq(lit(job_id)))?;
    native_catalog::work(session, "recovery_command", commands.into_view())?;
    require(
        runtime,
        session,
        "SELECT 'missing_or_ambiguous_command' AS witness FROM recovery_command HAVING count(*)<>1",
        "recovery_command",
    )
    .await?;
    let values = session.sql("SELECT c.arguments AS retained,r.arguments AS requested FROM recovery_command c JOIN recovery_requested r ON c.job_id=r.job_id").await?;
    let encode = enrichment_core::native_identity::canonical_bytes(
        "enrichment/recovery-arguments/1",
        vec![Arc::new(arrow::datatypes::Field::new(
            "arguments",
            Arguments::data_type(),
            false,
        ))]
        .into(),
    );
    runtime
        .require_empty(
            values
                .filter(
                    encode
                        .call(vec![col("retained")])
                        .not_eq(encode.call(vec![col("requested")])),
                )?
                .select(vec![lit("command_preimage_mismatch").alias("witness")])?,
            "recovery_command_preimage",
            "job_recovery",
        )
        .await
}

async fn require(
    runtime: &QueryRuntime,
    session: &SessionContext,
    sql: &str,
    rule: &str,
) -> Result<()> {
    runtime
        .require_empty(session.sql(sql).await?, rule, "job_recovery")
        .await
}

/// The same finite rule graph serves every producer-backed publication. Each optional payload
/// is selected by its native operation tag; NULL is a refusal for every required value.
async fn admit(runtime: &QueryRuntime, session: &SessionContext) -> Result<()> {
    for (name, sql) in [
        (
            "recovery_acquisitions",
            "SELECT unnest(acquisitions) AS artifact FROM recovery_scope",
        ),
        (
            "recovery_inputs",
            "SELECT unnest(native_map_entries(inputs)) AS input FROM recovery_scope",
        ),
        (
            "recovery_outputs",
            "SELECT unnest(result_artifact_ids) AS artifact_id FROM recovery_scope",
        ),
        (
            "recovery_input_artifacts",
            "SELECT DISTINCT a.artifact.artifact_id AS artifact_id FROM recovery_acquisitions a JOIN recovery_inputs i ON a.artifact.sha256=i.input.value",
        ),
        (
            "recovery_roles",
            "SELECT i.input.key AS role,a.artifact FROM recovery_inputs i JOIN recovery_acquisitions a ON i.input.value=a.artifact.sha256",
        ),
    ] {
        native_catalog::work(session, name, session.sql(sql).await?.into_view())?;
    }
    for (rule, sql) in [
        (
            "recovery_operation",
            "SELECT job_id FROM recovery_scope WHERE NOT coalesce(CASE kind WHEN 'resolve' THEN arguments.resolve IS NOT NULL WHEN 'inspect' THEN arguments.inspect IS NOT NULL WHEN 'verify' THEN arguments.verify IS NOT NULL ELSE false END,false)",
        ),
        (
            "recovery_output_closure",
            "SELECT r.artifact_id FROM recovery_outputs r LEFT ANTI JOIN recovery_input_artifacts a ON r.artifact_id=a.artifact_id",
        ),
        (
            "recovery_output_nonempty_unique",
            "SELECT 'output_count' AS witness FROM recovery_outputs HAVING count(*)=0 OR count(*)>64 OR count(*)<>count(DISTINCT artifact_id)",
        ),
        (
            "recovery_resolution",
            r#"SELECT s.job_id FROM recovery_scope s CROSS JOIN recovery_result r WHERE s.kind='resolve' AND (
            s.resolution IS NULL OR (s.resolution.attempt_id IS DISTINCT FROM s.attempt_id)
            OR (s.resolution.context_id IS DISTINCT FROM s.context_id)
            OR (s.resolution.release_id IS DISTINCT FROM s.manifest.metadata.release_id)
            OR (s.resolution.environment_id IS DISTINCT FROM s.manifest.metadata.environment_id)
            OR (s.result_artifact_ids IS DISTINCT FROM make_array(s.resolution.result_artifact_id))
            OR (r.data.resolve_library.context.context_id IS DISTINCT FROM s.context_id)
            OR (r.data.resolve_library.release.release_id IS DISTINCT FROM s.resolution.release_id)
            OR (r.data.resolve_library.environment.environment_id IS DISTINCT FROM s.resolution.environment_id))"#,
        ),
        (
            "recovery_resolution_input_closure",
            r#"WITH expected AS (SELECT unnest(resolution.input_artifact_ids) AS artifact_id FROM recovery_scope WHERE kind='resolve'), missing AS (SELECT e.artifact_id FROM expected e LEFT ANTI JOIN recovery_input_artifacts a ON e.artifact_id=a.artifact_id), extra AS (SELECT a.artifact_id FROM recovery_input_artifacts a LEFT ANTI JOIN expected e ON a.artifact_id=e.artifact_id WHERE EXISTS (SELECT 1 FROM recovery_scope WHERE kind='resolve')) SELECT * FROM missing UNION ALL SELECT * FROM extra"#,
        ),
        (
            "recovery_verification_scope",
            r#"SELECT s.job_id FROM recovery_scope s CROSS JOIN recovery_result r WHERE s.kind='verify' AND (
            (s.arguments.verify.request.snapshot_id IS DISTINCT FROM r.data.verify_usage.source_snapshot_id)
            OR (s.arguments.verify.request.context_id IS DISTINCT FROM r.data.verify_usage.source_context_id)
            OR (s.arguments.verify.request.mode IS DISTINCT FROM r.data.verify_usage.mode)
            OR (s.arguments.verify.request.profile IS DISTINCT FROM r.data.verify_usage.profile)
            OR (r.data.verify_usage.environment.environment_id IS DISTINCT FROM s.manifest.metadata.environment_id)
            OR (r.data.verify_usage.derived_context.context_id IS DISTINCT FROM s.context_id)
            OR (s.result_artifact_ids IS DISTINCT FROM make_array(r.data.verify_usage.result_artifact_id))
            OR cardinality(r.data.verify_usage.observations)=0)"#,
        ),
        (
            "recovery_verification_roles",
            r#"WITH required AS (SELECT unnest(make_array('snippet','dependency-lock','result')) AS role FROM recovery_scope WHERE kind='verify') SELECT r.role FROM required r LEFT JOIN recovery_roles a ON a.role=r.role GROUP BY r.role HAVING count(a.role)<>1"#,
        ),
        (
            "recovery_verification_artifacts",
            r#"SELECT a.role FROM recovery_roles a CROSS JOIN recovery_result r CROSS JOIN recovery_scope s WHERE s.kind='verify' AND CASE a.role WHEN 'snippet' THEN (a.artifact.artifact_id IS DISTINCT FROM r.data.verify_usage.snippet_artifact_id) WHEN 'dependency-lock' THEN (a.artifact.artifact_id IS DISTINCT FROM r.data.verify_usage.lock_artifact_id) WHEN 'result' THEN (a.artifact.artifact_id IS DISTINCT FROM r.data.verify_usage.result_artifact_id) ELSE false END"#,
        ),
        (
            "recovery_inspection_outputs",
            r#"WITH observations AS (SELECT unnest(r.data.inspect_symbol.execution_observations) AS observation FROM recovery_result r CROSS JOIN recovery_scope s WHERE s.kind='inspect'), missing AS (SELECT o.artifact_id FROM recovery_outputs o LEFT ANTI JOIN observations e ON e.observation.source.artifact_id=o.artifact_id WHERE EXISTS (SELECT 1 FROM recovery_scope WHERE kind='inspect')), extra AS (SELECT e.observation.source.artifact_id AS artifact_id FROM observations e LEFT ANTI JOIN recovery_outputs o ON o.artifact_id=e.observation.source.artifact_id) SELECT * FROM missing UNION ALL SELECT * FROM extra"#,
        ),
    ] {
        require(runtime, session, sql, rule).await?;
    }
    verify_payload(runtime, session).await
}

/// Reproduce the canonical normalized probe from the typed retained physical observation.
/// The immutable producer-result bytes remain verified by result dependency admission.
async fn verify_payload(runtime: &QueryRuntime, session: &SessionContext) -> Result<()> {
    use enrichment_core::evidence::execution::ExecutionPayload;
    let input = session.sql("SELECT r.data.verify_usage AS verification,a.artifact.sha256 AS digest FROM recovery_result r CROSS JOIN recovery_scope s JOIN recovery_roles a ON a.role='result' WHERE s.kind='verify'").await?;
    let verification = col("verification");
    let last = datafusion::functions_nested::expr_fn::array_element(
        verification.clone().field("observations"),
        lit(-1i64),
    );
    let payload = ExecutionPayload::usage_probe_expression(
        verification.clone().field("mode"),
        verification.field("snippet_artifact_id"),
        last,
    )?;
    let bytes = enrichment_core::native_identity::canonical_bytes(
        "enrichment/execution-payload/3",
        vec![Arc::new(arrow::datatypes::Field::new(
            "value",
            ExecutionPayload::data_type(),
            false,
        ))]
        .into(),
    )
    .call(vec![payload]);
    let digest = datafusion::functions::encoding::expr_fn::encode(
        datafusion::functions::crypto::expr_fn::sha256(bytes),
        lit("hex"),
    );
    runtime
        .require_empty(
            input
                .filter(digest.not_eq(col("digest")))?
                .select(vec![col("digest")])?,
            "recovery_verification_normalization",
            "job_recovery",
        )
        .await
}
