//! Python registry decisions expressed as native filters, UNNEST, joins and top-k.
use crate::{registry::rows, runtime::QueryRuntime};
use arrow::{
    array::StringArray,
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::{common::ScalarValue, dataframe::DataFrame, error::Result};
use enrichment_core::{
    producer::python::{DistributionFile, facts},
    request::ResolveRequest,
};
use std::sync::Arc;

/// A selected file and typed registry info produce one canonical release record.
/// Missing optional source metadata remains NULL; it cannot change content identity.
pub async fn release(
    runtime: &QueryRuntime,
    registry: &str,
    package: &str,
    selected: &Selected,
    info: &Option<enrichment_core::producer::python::registry::ReleaseInfo>,
) -> Result<enrichment_core::identity::Release> {
    use datafusion::{functions::core::expr_ext::FieldAccessor, prelude::col};
    use enrichment_core::{
        evidence::arrow_model::expressions::{literal, record},
        identity::{Ecosystem, Release, ReleaseKey, ReleaseLinks},
        native_key::Key,
        native_union::{Cell, NativeStruct, Rule},
        producer::python::registry::ReleaseInfo,
    };
    enrichment_core::native_struct! { struct Input {
        registry: String => Rule::NonEmpty,
        package: String => Rule::NonEmpty,
        selected: Selected => Rule::Text,
        info: Option<ReleaseInfo> => Rule::Text,
    } }
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "python_release",
        Input::batch(&[Input {
            registry: registry.into(),
            package: package.into(),
            selected: selected.clone(),
            info: info.clone(),
        }])?,
    )?;
    let frame = session.sql("SELECT *,lower(map_extract(selected.file.digests,'sha256')[1]) AS digest,map_extract(info.project_urls,'Documentation')[1] AS documentation FROM python_release").await?;
    crate::native_catalog::work(&session, "release_input", frame.clone().into_view())?;
    runtime.require_empty(session.sql("SELECT 'python_release_digest' AS witness FROM release_input WHERE digest IS NULL OR NOT regexp_like(digest,'^[0-9a-f]{64}$')").await?, "python_release_digest", "registry_selection").await?;
    let key = record(
        &ReleaseKey::data_type(),
        &[
            ("ecosystem", literal(&Ecosystem::Python)?),
            ("registry", col("registry")),
            ("package", col("package")),
            ("version", col("selected").field("version")),
            ("artifact_digest", col("digest")),
        ],
    )?;
    let frame = frame.with_column("key", key)?;
    let identity = Key::Release.identity_expression(
        ReleaseKey::fields()
            .iter()
            .map(|field| col("key").field(field.name()))
            .collect(),
    )?;
    let release = record(
        &Release::data_type(),
        &[
            ("release_id", identity),
            ("key", col("key")),
            (
                "links",
                record(
                    &ReleaseLinks::data_type(),
                    &[
                        ("documentation", col("documentation")),
                        ("homepage", col("info").field("home_page")),
                    ],
                )?,
            ),
            ("license", col("info").field("license")),
            ("yanked", col("selected").field("file").field("yanked")),
        ],
    )?;
    let frame = frame.select(vec![release.alias("release")])?.select(
        Release::fields()
            .iter()
            .map(|field| col("release").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("selected Python release missing"))
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
    use enrichment_core::{
        native_union::NativeStruct, producer::python::requirements::MarkerEnvironment,
    };
    if requirements.len() > 4096 {
        return Err(datafusion::error::DataFusionError::ResourcesExhausted(
            "requirement command bound".into(),
        ));
    }
    let session = runtime.session();
    if requirements.is_empty() {
        return crate::native_catalog::batch(
            &session,
            "python_registry",
            RecordBatch::new_empty(Arc::new(Schema::new(vec![Field::new(
                "index",
                DataType::UInt64,
                false,
            )]))),
        );
    }
    crate::native_catalog::input(
        &session,
        "marker_environment",
        MarkerEnvironment::batch(std::slice::from_ref(environment))?,
    )?;
    let extra_values = if extras.is_empty() {
        vec![String::new()]
    } else {
        extras.to_vec()
    };
    crate::native_catalog::input(
        &session,
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
    versions: DataFrame,
    allow_prerelease: bool,
    limit: usize,
) -> Result<Vec<String>> {
    let session = runtime.session();
    let schema = Arc::new(Schema::new(vec![Field::new(
        "version",
        DataType::Utf8,
        false,
    )]));
    enrichment_core::native_schema::check_input(versions.schema().as_arrow(), &schema)?;
    crate::native_catalog::work(&session, "versions", versions.into_view())?;
    enrichment_core::native_struct! {
    struct Version {
        version: String => enrichment_core::native_union::Rule::Text,
    }
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

enrichment_core::native_struct! {
/// One selected immutable artifact; the native query decides both version and file.
pub struct Selected {
    version: String => enrichment_core::native_union::Rule::Text,
    file: DistributionFile => enrichment_core::native_union::Rule::Text,
}
}

/// Optional constraints and wheel-only policy share the exact same registry relation/plan.
pub async fn select(
    runtime: &QueryRuntime,
    releases: DataFrame,
    request: &ResolveRequest,
    constraint: Option<&str>,
    wheels_only: bool,
) -> Result<Option<Selected>> {
    let session = runtime.session();
    enrichment_core::native_schema::check_input(releases.schema().as_arrow(), &facts::schema())?;
    crate::native_catalog::work(&session, "python_files", releases.into_view())?;
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

#[cfg(test)]
mod source_metadata_tests {
    use super::*;
    use enrichment_core::producer::python::registry::{ReleaseMetadata, Versions};

    #[tokio::test]
    async fn native_release_preserves_optional_metadata_and_canonical_digest_identity() -> Result<()>
    {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let source = serde_json::json!({
            "info": {"license": "MIT", "home_page": null,
                "project_urls": {"Documentation": "https://example.org/docs", "Other": null},
                "external_field": {"preserved": "in artifact"}},
            "urls": [{"filename": "package-1.0-py3-none-any.whl", "packagetype": "bdist_wheel",
                "url": "https://example.org/package.whl", "digests": {"sha256": "A".repeat(64)}, "yanked": true}],
            "ignored": [1, 2, 3],
        });
        let metadata: ReleaseMetadata =
            serde_json::from_value(source).expect("typed registry fixture");
        let selected = Selected {
            version: "1.0".into(),
            file: metadata.urls[0].clone(),
        };
        let full = release(
            &runtime,
            "https://index.example",
            "package",
            &selected,
            &metadata.info,
        )
        .await?;
        assert_eq!(full.release_id, full.key.id());
        assert_eq!(
            full.key.artifact_digest.as_deref(),
            Some("a".repeat(64).as_str())
        );
        assert_eq!(full.license.as_deref(), Some("MIT"));
        assert_eq!(
            full.links.documentation.as_deref(),
            Some("https://example.org/docs")
        );
        assert!(full.links.homepage.is_none());
        assert!(full.yanked);
        let bound = crate::python_distribution::release_binding(
            &runtime,
            &full,
            &["package_api".into(), "package_api".into()],
        )
        .await?;
        assert_eq!(bound.root_module.as_deref(), Some("package_api"));
        assert_eq!(bound.lib_name, bound.root_module);
        assert_eq!(bound.release_id, full.release_id);
        for roots in [vec![], vec!["package_api".into(), "another_package".into()]] {
            let ambiguous =
                crate::python_distribution::release_binding(&runtime, &bound, &roots).await?;
            assert!(ambiguous.root_module.is_none());
            assert!(ambiguous.lib_name.is_none());
            assert_eq!(ambiguous.release_id, full.release_id);
        }
        let missing = release(
            &runtime,
            "https://index.example",
            "package",
            &selected,
            &None,
        )
        .await?;
        assert_eq!(missing.release_id, full.release_id);
        assert!(missing.license.is_none());
        assert_eq!(missing.links, Default::default());
        let mut invalid = selected;
        invalid.file.digests.clear();
        assert!(
            release(
                &runtime,
                "https://index.example",
                "package",
                &invalid,
                &None
            )
            .await
            .is_err()
        );
        assert!(serde_json::from_str::<ReleaseMetadata>(r#"{"urls":null}"#).is_err());
        let versions: Versions =
            serde_json::from_str(r#"{"versions":["1.0","2.0"],"name":"package"}"#)
                .expect("typed versions fixture");
        assert_eq!(versions.versions, ["1.0", "2.0"]);
        runtime.close_diagnostics().await
    }
}
