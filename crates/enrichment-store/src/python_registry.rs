//! Python registry decisions expressed as native filters, UNNEST, joins and top-k.
use crate::{registry::rows, runtime::QueryRuntime};
use arrow::{
    array::StringArray,
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::{
    common::ScalarValue, datasource::MemTable, error::Result, prelude::SessionContext,
};
use enrichment_core::{
    producer::python::{DistributionFile, facts},
    request::ResolveRequest,
};
use serde::Deserialize;
use std::{collections::BTreeMap, sync::Arc};

fn session(runtime: &QueryRuntime) -> SessionContext {
    let session = runtime.session();
    session.register_udf(enrichment_core::native_version::pep440_value());
    session.register_udf(enrichment_core::native_version::pep440_matches());
    session
}

/// Compile all marker decisions into one native plan over explicit environment/extra facts.
/// Keep the selected ordinals relational so callers join the requirement facts natively.
pub async fn active_requirement_plan(
    runtime: &QueryRuntime,
    requirements: &[enrichment_core::producer::python::requirements::Requirement],
    environment: &enrichment_core::producer::python::requirements::MarkerEnvironment,
    extras: &[String],
) -> Result<datafusion::dataframe::DataFrame> {
    use datafusion::{
        functions_nested::expr_fn::make_array,
        logical_expr::expr_fn::when,
        prelude::{col, lit},
    };
    use enrichment_core::producer::python::requirements::MarkerEnvironment;
    if requirements.len() > 4096 {
        return Err(datafusion::error::DataFusionError::ResourcesExhausted(
            "requirement command bound".into(),
        ));
    }
    let session = session(runtime);
    if requirements.is_empty() {
        return session.read_batch(RecordBatch::new_empty(Arc::new(Schema::new(vec![
            Field::new("index", DataType::UInt64, false),
        ]))));
    }
    session.register_batch(
        "marker_environment",
        crate::control_jobs::encode(
            MarkerEnvironment::schema(),
            std::slice::from_ref(environment),
        )?,
    )?;
    let extra_values = if extras.is_empty() {
        vec![String::new()]
    } else {
        extras.to_vec()
    };
    session.register_batch(
        "marker_extras",
        RecordBatch::try_from_iter(vec![(
            "extra",
            Arc::new(StringArray::from(extra_values)) as arrow::array::ArrayRef,
        )])?,
    )?;
    let environment = session.sql("SELECT *, concat(split_part(python_full_version, '.', 1), '.', split_part(python_full_version, '.', 2)) AS python_version \
        FROM marker_environment CROSS JOIN marker_extras").await?;
    let choices = requirements
        .iter()
        .enumerate()
        .map(|(index, requirement)| {
            when(requirement.marker_predicate(), lit(index as u64))
                .otherwise(lit(ScalarValue::UInt64(None)))
        })
        .collect::<Result<Vec<_>>>()?;
    environment
        .select(vec![make_array(choices).alias("index")])?
        .unnest_columns(&["index"])?
        .filter(col("index").is_not_null())?
        .distinct()?
        .sort(vec![col("index").sort(true, false)])
}

/// Only the finite acquisition command list crosses back to the network mechanism.
pub async fn ordered_versions(
    runtime: &QueryRuntime,
    versions: &[String],
    allow_prerelease: bool,
    limit: usize,
) -> Result<Vec<String>> {
    let session = session(runtime);
    let schema = Arc::new(Schema::new(vec![Field::new(
        "version",
        DataType::Utf8,
        false,
    )]));
    session.register_batch(
        "versions",
        RecordBatch::try_new(schema, vec![Arc::new(StringArray::from(versions.to_vec()))])?,
    )?;
    #[derive(Deserialize)]
    struct Version {
        version: String,
    }
    let frame = session
        .sql(
            "WITH parsed AS (SELECT version, pep440_value_v1(version) AS parsed FROM versions) \
        SELECT version FROM parsed WHERE parsed IS NOT NULL AND ($1 OR NOT parsed['prerelease']) \
        ORDER BY parsed['precedence'] DESC, version ASC",
        )
        .await?
        .with_param_values(vec![ScalarValue::Boolean(Some(allow_prerelease))])?
        .limit(0, Some(limit))?;
    let selected: Vec<Version> = rows(runtime, frame, limit).await?;
    Ok(selected.into_iter().map(|row| row.version).collect())
}

/// One selected immutable artifact; the native query decides both version and file.
#[derive(Debug, Deserialize)]
pub struct Selected {
    pub version: String,
    pub file: DistributionFile,
}

/// Optional constraints and wheel-only policy share the exact same registry relation/plan.
pub async fn select(
    runtime: &QueryRuntime,
    releases: &BTreeMap<String, Vec<DistributionFile>>,
    request: &ResolveRequest,
    constraint: Option<&str>,
    wheels_only: bool,
) -> Result<Option<Selected>> {
    let session = session(runtime);
    session.register_table(
        "python_files",
        Arc::new(MemTable::try_new(
            facts::schema(),
            vec![facts::decode(releases, 1024)?],
        )?),
    )?;
    let frame = session.sql(r#"
        WITH parsed AS (
            SELECT *, pep440_value_v1(version) AS parsed FROM python_files
        ), environment AS (
            SELECT pep440_value_v1(CAST($2 AS VARCHAR))['release'][1] AS major,
                   pep440_value_v1(CAST($2 AS VARCHAR))['release'][2] AS minor,
                   CAST($3 AS VARCHAR) AS target
        ), eligible AS (
            SELECT version, file, parsed['precedence'] AS precedence FROM parsed
            WHERE parsed IS NOT NULL
              AND ((CAST($1 AS VARCHAR) IS NOT NULL AND parsed['precedence'] = pep440_value_v1($1)['precedence'])
                OR (CAST($1 AS VARCHAR) IS NULL AND ($5 OR NOT parsed['prerelease'])))
              AND ($4 OR NOT file['yanked'])
              AND (file['requires_python'] IS NULL OR CAST($2 AS VARCHAR) IS NULL
                   OR pep440_matches_v1(file['requires_python'], $2))
              AND (CAST($6 AS VARCHAR) IS NULL OR pep440_matches_v1($6, version))
              AND regexp_like(map_extract(file['digests'], 'sha256')[1], '^[0-9a-fA-F]{64}$')
        ), wheel_parts AS (
            SELECT *, regexp_replace(file['filename'], '\.whl$', '') AS stem
            FROM eligible WHERE file['packagetype'] = 'bdist_wheel'
                AND regexp_like(file['filename'], '^[^-]+-[^-]+(-[^-]+)?-[^-]+-[^-]+-[^-]+\.whl$')
        ), interpreters AS (
            SELECT version, file, precedence, stem,
                unnest(string_to_array(split_part(stem, '-', -3), '.')) AS interpreter FROM wheel_parts
        ), abis AS (
            SELECT *, unnest(string_to_array(split_part(stem, '-', -2), '.')) AS abi FROM interpreters
        ), platforms AS (
            SELECT *, unnest(string_to_array(split_part(stem, '-', -1), '.')) AS platform FROM abis
        ), wheels AS (
            SELECT DISTINCT version, file['filename'] AS filename FROM platforms CROSS JOIN environment
            WHERE (abi = 'none' AND platform = 'any' AND (
                (minor IS NULL AND interpreter = 'py3') OR
                (minor IS NOT NULL AND (interpreter = concat('py', major) OR interpreter = concat('py', major, minor)))))
             OR (minor IS NOT NULL AND target IS NOT NULL AND platform = target AND (
                (interpreter = concat('cp', major, minor) AND (abi = interpreter OR abi = 'none' OR abi = 'abi3'))
                OR (abi = 'abi3' AND major = 3 AND starts_with(interpreter, 'cp3')
                    AND try_cast(substr(interpreter, 4) AS BIGINT) <= minor)))
        ), candidates AS (
            SELECT eligible.version, eligible.file, eligible.precedence,
                CASE WHEN wheels.version IS NOT NULL THEN 0 ELSE 1 END AS rank
            FROM eligible LEFT JOIN wheels
                ON eligible.version = wheels.version AND eligible.file['filename'] = wheels.filename
            WHERE wheels.version IS NOT NULL OR (NOT $7 AND eligible.file['packagetype'] = 'sdist'
                AND (ends_with(eligible.file['filename'], '.tar.gz') OR ends_with(eligible.file['filename'], '.zip')))
        )
        SELECT version, file FROM candidates ORDER BY precedence DESC, rank ASC, file['filename'] ASC, version ASC LIMIT 1
    "#).await?.with_param_values(vec![
        ScalarValue::Utf8(request.version.clone()), ScalarValue::Utf8(request.python_version.clone()),
        ScalarValue::Utf8(request.target.clone()), ScalarValue::Boolean(Some(request.allow_yanked)),
        ScalarValue::Boolean(Some(request.allow_prerelease)), ScalarValue::Utf8(constraint.map(str::to_owned)),
        ScalarValue::Boolean(Some(wheels_only)),
    ])?;
    let mut selected = rows(runtime, frame, 1).await?;
    Ok(selected.pop())
}
