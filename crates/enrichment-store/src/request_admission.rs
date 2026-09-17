//! Native durable-request admission over the generated operation arguments.
use crate::runtime::QueryRuntime;
use datafusion::common::Result;
use enrichment_core::{native_union::Cell, operation::Arguments};

/// The generated native contracts admit every research operation, including read-only
/// requests and resources. Durable command admission adds claim/effect preconditions.
pub async fn research(
    runtime: &QueryRuntime,
    request: &enrichment_core::request::ResearchRequest,
    input_bound: usize,
) -> Result<()> {
    let session = runtime.session();
    let values = <enrichment_core::request::ResearchRequest as Cell>::encode(&[Some(request)])?;
    let batch = arrow::record_batch::RecordBatch::from(
        datafusion::common::cast::as_struct_array(&values)?.clone(),
    );
    let schema = batch.schema();
    let frame = session.read_batch(batch)?;
    let input = frame.clone();
    if let Some(invalid) = runtime
        .native_read(
            async move { enrichment_core::native_schema::intrinsic_violations(input, &schema) },
        )
        .await?
    {
        runtime
            .require_empty(
                invalid.select(vec![
                    datafusion::prelude::lit("research_contract").alias("witness"),
                ])?,
                "research_contract",
                "request_admission",
            )
            .await?;
    }
    crate::native_catalog::work(&session, "research_request", frame.into_view())?;
    let invalid = session.sql(r#"
        WITH aspects AS (
            SELECT unnest(inspect_symbol.selection.explicit.aspects) AS aspect FROM research_request
        ), discovery AS (
            SELECT unnest(library_overview.discovery) AS facet FROM research_request
        ), execution AS (
            SELECT inspect_symbol.execution.intent AS intent,
              inspect_symbol.execution.runtime IS NULL AS semantic,
              CASE WHEN inspect_symbol.execution.runtime IS NULL THEN 'semantics' ELSE 'runtime' END AS target
            FROM research_request
        )
        SELECT 'duplicate_inspection_aspect' AS witness FROM aspects GROUP BY aspect.aspect HAVING count(*)>1
        UNION ALL SELECT 'inspection_preview_scope' FROM aspects a JOIN operation.declarations.inspection_aspects d ON a.aspect.aspect=d.aspect
          WHERE a.aspect.max_characters IS NOT NULL AND NOT d.preview
        UNION ALL SELECT 'inspection_cursor_bound' FROM aspects WHERE octet_length(aspect.cursor)>32768
        UNION ALL SELECT 'duplicate_discovery_kind' FROM discovery GROUP BY facet.kind HAVING count(*)>1
        UNION ALL SELECT 'discovery_cursor_bound' FROM discovery WHERE octet_length(facet.cursor)>32768
        UNION ALL SELECT 'runtime_execution_selection' FROM execution
          WHERE intent<>'retained' AND semantic
            AND EXISTS(SELECT 1 FROM aspects WHERE aspect.aspect='runtime')
        UNION ALL SELECT 'execution_aspect_required' FROM execution e LEFT ANTI JOIN aspects a
          ON a.aspect.aspect=e.target WHERE e.intent<>'retained'
    "#).await?;
    runtime
        .require_empty(invalid, "research_selection", "request_admission")
        .await?;
    let inspection = session.sql(r#"
      WITH inspected AS (SELECT inspect_symbol.execution AS options FROM research_request WHERE inspect_symbol.execution IS NOT NULL),
      selections AS (SELECT options.intent AS intent,options.methods AS methods,options.snippet AS snippet,options.position AS position,options.runtime AS runtime,options.profile AS profile,$1 AS input_bound FROM inspected)
      SELECT CASE
        WHEN cardinality(array_distinct(methods))<>cardinality(methods) THEN 'semantic_method_set'
        WHEN snippet IS NOT NULL AND (length(trim(snippet))=0 OR octet_length(snippet)>input_bound) THEN 'snippet_budget'
        WHEN position IS NOT NULL AND (snippet IS NULL OR NOT utf8_position_valid(snippet,position.line,position.byte)) THEN 'utf8_position'
        WHEN runtime IS NOT NULL AND (octet_length(runtime.module)>512 OR cardinality(runtime.attributes)>32
          OR NOT regexp_like(runtime.module,'^[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*(\.[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*)*$')
          OR snippet IS NOT NULL OR cardinality(methods)>0) THEN 'runtime_selection'
        WHEN intent<>'retained' AND profile IS DISTINCT FROM CASE WHEN runtime IS NULL THEN 'build' ELSE 'runtime' END THEN 'explicit_execution_profile'
      END AS witness FROM selections
      UNION ALL SELECT 'runtime_attribute' FROM
        (SELECT unnest(runtime.attributes) AS attribute FROM selections) attributes
      WHERE octet_length(attribute)>512 OR NOT regexp_like(attribute,'^[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*$')
    "#).await?.with_param_values(vec![datafusion::common::ScalarValue::UInt64(Some(input_bound as u64))])?;
    runtime
        .require_empty(
            inspection.filter(datafusion::prelude::col("witness").is_not_null())?,
            "inspection_arguments",
            "request_admission",
        )
        .await?;
    let verification = session.sql(r#"
        SELECT 'verification_input_or_profile' AS witness FROM research_request
        WHERE method='verify_usage' AND (length(trim(verify_usage.snippet))=0 OR octet_length(verify_usage.snippet)>$1
          OR verify_usage.profile IS DISTINCT FROM CASE WHEN verify_usage.mode='runtime' THEN 'runtime' ELSE 'build' END)
    "#).await?.with_param_values(vec![datafusion::common::ScalarValue::UInt64(Some(input_bound as u64))])?;
    runtime
        .require_empty(
            verification,
            "verification_input_or_profile",
            "research_admission",
        )
        .await?;
    let comparison = session.sql(r#"
      SELECT 'comparison_input_form' AS witness FROM research_request
      WHERE method='compare_releases' AND NOT (
        (compare_releases.before_context_id IS NOT NULL AND compare_releases.after_context_id IS NOT NULL
         AND compare_releases.ecosystem IS NULL AND compare_releases.name IS NULL
         AND compare_releases.from_version IS NULL AND compare_releases.to_version IS NULL)
        OR (compare_releases.before_context_id IS NULL AND compare_releases.after_context_id IS NULL
         AND compare_releases.before_snapshot_id IS NULL AND compare_releases.after_snapshot_id IS NULL
         AND compare_releases.ecosystem IS NOT NULL AND compare_releases.name IS NOT NULL
         AND compare_releases.from_version IS NOT NULL AND compare_releases.to_version IS NOT NULL))
    "#).await?;
    runtime
        .require_empty(comparison, "comparison_input_form", "research_admission")
        .await?;
    let resolve = session.sql(r#"
      WITH forms AS (
        SELECT resolve_library.ecosystem AS ecosystem,resolve_library.name AS name,resolve_library.version AS version,
          resolve_library.repository AS repository,resolve_library.revision AS revision,resolve_library.package_subdir AS package_subdir,
          coalesce(resolve_library.mode,CASE WHEN resolve_library.version IS NULL THEN 'upstream' ELSE 'project' END) AS mode,
          resolve_library.features AS features,resolve_library.default_features AS default_features,
          resolve_library.python_version AS python_version,resolve_library.extras AS extras
        FROM research_request WHERE method='resolve_library'
        UNION ALL
        SELECT compare_releases.ecosystem,compare_releases.name,unnest([compare_releases.from_version,compare_releases.to_version]),
          NULL,NULL,NULL,'project',NULL,NULL,NULL,NULL FROM research_request WHERE method='compare_releases' AND compare_releases.before_context_id IS NULL
      ), parsed AS (SELECT *,pep440_value_v1(python_version) AS interpreter FROM forms)
      SELECT CASE
        WHEN NOT coalesce(regexp_like(name,CASE WHEN ecosystem='python' THEN '^[a-zA-Z0-9_.-]+$' ELSE '^[a-zA-Z0-9_-]+$' END),false) THEN 'package_name'
        WHEN version IS NOT NULL AND (CASE WHEN ecosystem='rust' THEN semver_precedence_key_v1(version) IS NULL ELSE pep440_value_v1(version) IS NULL END) THEN 'exact_package_version'
        WHEN mode='revision' AND (version IS NOT NULL OR repository IS NULL OR revision IS NULL
          OR NOT regexp_like(repository,'^https://github.com/[a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+$')
          OR split_part(repository,'/',4) IN ('.','..') OR split_part(repository,'/',5) IN ('.','..') OR ends_with(repository,'.git')
          OR NOT regexp_like(revision,'^[a-fA-F0-9]{40}$')
          OR (coalesce(package_subdir,'')<>'' AND (NOT regexp_like(package_subdir,'^[a-zA-Z0-9_.-]+(/[a-zA-Z0-9_.-]+)*$')
            OR regexp_like(package_subdir,'(^|/)[.]{1,2}(/|$)')))) THEN 'immutable_revision_form'
        WHEN mode<>'revision' AND (repository IS NOT NULL OR revision IS NOT NULL OR package_subdir IS NOT NULL) THEN 'revision_mode_required'
        WHEN ecosystem='rust' AND (python_version IS NOT NULL OR extras IS NOT NULL) THEN 'rust_environment_qualifiers'
        WHEN ecosystem='python' AND (features IS NOT NULL OR default_features IS NOT NULL) THEN 'python_environment_qualifiers'
        WHEN python_version IS NOT NULL AND (interpreter IS NULL OR cardinality(interpreter.release)<2 OR interpreter.prerelease) THEN 'stable_python_interpreter'
      END AS witness FROM parsed
    "#).await?;
    runtime
        .require_empty(
            resolve.filter(datafusion::prelude::col("witness").is_not_null())?,
            "resolution_arguments",
            "research_admission",
        )
        .await?;
    Ok(())
}

pub(crate) async fn validate(
    runtime: &QueryRuntime,
    arguments: &Arguments,
    input_bound: usize,
) -> Result<()> {
    research(runtime, &arguments.clone().into(), input_bound).await?;
    let session = runtime.session();
    let values = Arguments::encode(&[Some(arguments)])?;
    session.register_batch(
        "request_input",
        arrow::record_batch::RecordBatch::from(
            datafusion::common::cast::as_struct_array(&values)?.clone(),
        ),
    )?;
    let scope = session.sql(r#"
      SELECT 'missing_pinned_job_scope' AS witness FROM request_input
      WHERE kind IN ('verify','inspect') AND (
        coalesce(verify.request.context_id,inspect.request.context_id) IS NULL OR
        coalesce(verify.request.snapshot_id,inspect.request.snapshot_id) IS NULL)
      UNION ALL SELECT 'missing_inspection_execution_intent' FROM request_input
      WHERE kind='inspect' AND (inspect.request.execution IS NULL OR inspect.request.execution.intent='retained')
      UNION ALL SELECT 'comparison_acquisition_form' FROM request_input
      WHERE kind='compare' AND (compare.request.ecosystem IS NULL OR compare.request.name IS NULL
        OR compare.request.from_version IS NULL OR compare.request.to_version IS NULL
        OR compare.request.before_context_id IS NOT NULL OR compare.request.after_context_id IS NOT NULL
        OR compare.request.before_snapshot_id IS NOT NULL OR compare.request.after_snapshot_id IS NOT NULL)
    "#).await?;
    runtime
        .require_empty(scope, "durable_request_scope", "command_ingress")
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        identity::Ecosystem,
        policy::ExecutionProfile,
        request::{
            InspectRequest, InspectionIntent, InspectionOptions, ResolveRequest, RuntimeSelection,
        },
    };

    #[tokio::test]
    async fn native_request_rules_preserve_exact_forms_and_utf8_coordinates() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let resolve = ResolveRequest {
            name: "fixture".into(),
            version: Some("1.0.0".into()),
            ..Default::default()
        };
        validate(
            &runtime,
            &Arguments::Resolve {
                request: resolve.clone(),
            },
            32768,
        )
        .await?;
        for request in [
            ResolveRequest {
                name: "a/b".into(),
                ..resolve.clone()
            },
            ResolveRequest {
                version: Some(">=1".into()),
                ..resolve.clone()
            },
            ResolveRequest {
                python_version: Some("3.12".into()),
                ..resolve.clone()
            },
            ResolveRequest {
                ecosystem: Ecosystem::Python,
                version: Some("1.0".into()),
                python_version: Some("3.13rc1".into()),
                ..resolve
            },
        ] {
            assert!(
                validate(&runtime, &Arguments::Resolve { request }, 32768)
                    .await
                    .is_err()
            );
        }
        let mut inspect = InspectRequest {
            context_id: format!("ctx_{}", "1".repeat(64)).try_into().unwrap(),
            snapshot_id: Some(format!("snap_{}", "1".repeat(64)).try_into().unwrap()),
            execution: Some(InspectionOptions {
                intent: InspectionIntent::Rerun,
                profile: Some(ExecutionProfile::Build),
                snippet: Some("xé\r\n".into()),
                position: Some(enrichment_core::evidence::execution::Utf8Position {
                    line: 0,
                    byte: 3,
                }),
                ..Default::default()
            }),
            symbol_path: "sample".into(),
            definition_id: None,
            selection: Default::default(),
            max_bytes: None,
        };
        validate(
            &runtime,
            &Arguments::Inspect {
                request: inspect.clone(),
            },
            32768,
        )
        .await?;
        inspect
            .execution
            .as_mut()
            .unwrap()
            .position
            .as_mut()
            .unwrap()
            .byte = 2;
        assert!(
            validate(
                &runtime,
                &Arguments::Inspect {
                    request: inspect.clone()
                },
                32768
            )
            .await
            .is_err()
        );
        inspect.execution = Some(InspectionOptions {
            intent: InspectionIntent::Rerun,
            profile: Some(ExecutionProfile::Runtime),
            runtime: Some(RuntimeSelection {
                module: "demo.Δ".into(),
                attributes: vec!["symbol".into()],
            }),
            ..Default::default()
        });
        validate(
            &runtime,
            &Arguments::Inspect {
                request: inspect.clone(),
            },
            32768,
        )
        .await?;
        inspect
            .execution
            .as_mut()
            .unwrap()
            .runtime
            .as_mut()
            .unwrap()
            .attributes
            .push("not valid".into());
        assert!(
            validate(&runtime, &Arguments::Inspect { request: inspect }, 32768)
                .await
                .is_err()
        );
        let resolution = crate::control_jobs::Resolution {
            release_id: format!("rel_{}", "1".repeat(64)).try_into().unwrap(),
            environment_id: format!("env_{}", "1".repeat(64)).try_into().unwrap(),
            context_id: format!("ctx_{}", "1".repeat(64)).try_into().unwrap(),
            attempt_id: "attempt".into(),
            input_artifact_ids: vec![format!("art_{}", "a".repeat(64))],
            result_artifact_id: format!("art_{}", "a".repeat(64)),
        };
        crate::operation_policy::resolution(&runtime, &resolution).await?;
        let mut duplicate = resolution.clone();
        duplicate
            .input_artifact_ids
            .push(duplicate.result_artifact_id.clone());
        assert!(
            crate::operation_policy::resolution(&runtime, &duplicate)
                .await
                .is_err()
        );
        let mut missing = resolution;
        missing.result_artifact_id = format!("art_{}", "b".repeat(64));
        assert!(
            crate::operation_policy::resolution(&runtime, &missing)
                .await
                .is_err()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
