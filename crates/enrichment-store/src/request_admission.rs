//! Native durable-request admission over the generated operation arguments.
use crate::runtime::QueryRuntime;
use datafusion::common::Result;
use enrichment_core::{native_union::Cell, operation::Arguments};

pub(crate) async fn validate(
    runtime: &QueryRuntime,
    arguments: &Arguments,
    input_bound: usize,
) -> Result<()> {
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
        length(coalesce(coalesce(verify.request.context_id,inspect.request.context_id),''))=0 OR
        length(coalesce(coalesce(verify.request.snapshot_id,inspect.request.snapshot_id),''))=0)
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
    let inspection = session.sql(r#"
      WITH inspected AS (SELECT inspect.request.execution AS options FROM request_input WHERE kind='inspect'),
      selections AS (SELECT options.methods AS methods,options.snippet AS snippet,options.position AS position,options.runtime AS runtime,options.profile AS profile,$1 AS input_bound FROM inspected)
      SELECT CASE
        WHEN cardinality(methods)>5 OR cardinality(array_distinct(methods))<>cardinality(methods) THEN 'semantic_method_set'
        WHEN snippet IS NOT NULL AND (length(trim(snippet))=0 OR octet_length(snippet)>input_bound) THEN 'snippet_budget'
        WHEN position IS NOT NULL AND (snippet IS NULL OR NOT utf8_position_valid(snippet,position.line,position.byte)) THEN 'utf8_position'
        WHEN runtime IS NOT NULL AND (octet_length(runtime.module)>512 OR cardinality(runtime.attributes)>32
          OR NOT regexp_like(runtime.module,'^[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*(\.[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*)*$')
          OR snippet IS NOT NULL OR cardinality(methods)>0) THEN 'runtime_selection'
        WHEN profile IS DISTINCT FROM CASE WHEN runtime IS NULL THEN 'build' ELSE 'runtime' END THEN 'explicit_execution_profile'
      END AS witness FROM selections
      UNION ALL SELECT 'runtime_attribute' FROM
        (SELECT unnest(runtime.attributes) AS attribute FROM selections) attributes
      WHERE octet_length(attribute)>512 OR NOT regexp_like(attribute,'^[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*$')
    "#).await?.with_param_values(vec![datafusion::common::ScalarValue::UInt64(Some(input_bound as u64))])?;
    runtime
        .require_empty(
            inspection.filter(datafusion::prelude::col("witness").is_not_null())?,
            "inspection_arguments",
            "command_ingress",
        )
        .await?;
    let resolve = session.sql(r#"
      WITH forms AS (
        SELECT resolve.request.ecosystem AS ecosystem,resolve.request.name AS name,resolve.request.version AS version,
          resolve.request.repository AS repository,resolve.request.revision AS revision,resolve.request.package_subdir AS package_subdir,
          coalesce(resolve.request.mode,CASE WHEN resolve.request.version IS NULL THEN 'upstream' ELSE 'project' END) AS mode,
          resolve.request.features AS features,resolve.request.default_features AS default_features,
          resolve.request.python_version AS python_version,resolve.request.extras AS extras
        FROM request_input WHERE kind='resolve'
        UNION ALL
        SELECT compare.request.ecosystem,compare.request.name,unnest([compare.request.from_version,compare.request.to_version]),
          NULL,NULL,NULL,'project',NULL,NULL,NULL,NULL FROM request_input WHERE kind='compare'
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
            "command_ingress",
        )
        .await
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
            context_id: "context".into(),
            snapshot_id: Some("snapshot".into()),
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
            ..Default::default()
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
            release_id: "release".into(),
            environment_id: "environment".into(),
            context_id: "context".into(),
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
