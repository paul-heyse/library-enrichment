//! Native source admission and preparation selections shared by Cargo and rustdoc drivers.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::common::{DataFusionError, Result};
use enrichment_core::native_union::{NativeStruct, Rule};

enrichment_core::native_struct! { pub struct Path { path: String => Rule::MemberPath } }
enrichment_core::native_struct! { struct Text { text: String => Rule::Text } }
enrichment_core::native_struct! { struct CargoInput {
    package: String => Rule::NonEmpty,
    root: String => Rule::MemberPath,
    features: Vec<String> => Rule::Set,
    default_features: bool => Rule::Text,
} }

/// Decode only syntax in the UDF. Relational predicates decide which sources are admitted.
/// Arbitrary package metadata with words such as `git` or `path` is not Cargo source policy.
async fn cargo_sources(
    runtime: &QueryRuntime,
    manifest: &str,
) -> Result<datafusion::execution::context::SessionContext> {
    if manifest.len() > enrichment_core::native_cargo::MAX_BYTES {
        return Err(DataFusionError::ResourcesExhausted(
            "Cargo manifest byte bound".into(),
        ));
    }
    let session = runtime.session();
    let input = crate::native_catalog::batch(
        &session,
        "preparation_plan",
        Text::batch(&[Text {
            text: manifest.into(),
        }])?,
    )?;
    native_catalog::work(&session, "cargo_input", input.into_view())?;
    let facts = session.sql("SELECT entry.path AS path,entry.kind AS kind,entry.text AS text FROM (SELECT unnest(cargo_toml_entries_v1(text)) AS entry FROM cargo_input)").await?;
    native_catalog::work(&session, "cargo_fields", facts.into_view())?;
    runtime.require_empty(session.sql(r#"
      SELECT array_to_string(path,'/') AS witness FROM cargo_fields WHERE
        path[1] IN ('patch','replace')
        OR (path[1]='workspace' AND path[2] IN ('members','default-members'))
        OR (path[1]='package' AND path[2]='workspace')
        OR (array_element(path,-1) IN ('path','git','registry','registry-index') AND (
          (array_length(path)=3 AND path[1] IN ('dependencies','dev-dependencies','build-dependencies'))
          OR (array_length(path)=5 AND path[1]='target' AND path[3] IN ('dependencies','dev-dependencies','build-dependencies'))
          OR (array_length(path)=4 AND path[1]='workspace' AND path[2]='dependencies')))
        OR (array_element(path,-1)='path' AND (
          (array_length(path)=2 AND path[1]='lib')
          OR (array_length(path)=3 AND path[1] IN ('bin','example','test','bench')))
          AND (kind<>'string' OR starts_with(text,'/') OR contains(text,chr(92)) OR contains(text,':') OR array_has(string_to_array(text,'/'),'..')))
        OR (path[1]='package' AND path[2]='build' AND array_length(path)=2 AND kind='string'
          AND (starts_with(text,'/') OR contains(text,chr(92)) OR contains(text,':') OR array_has(string_to_array(text,'/'),'..')))
    "#).await?, "cargo_source_admission", "producer_preparation").await?;
    Ok(session)
}

enrichment_core::native_struct! { pub struct CargoIdentity {
    package: String => Rule::NonEmpty,
    version: Option<String> => Rule::Text,
    lib_name: String => Rule::NonEmpty,
} }

/// Source identity and the emitted rustdoc filename come from the captured manifest, never
/// an archive-name split or a second independently maintained defaulting rule.
pub async fn cargo_identity(
    runtime: &QueryRuntime,
    manifest: &str,
    package: &str,
    version: Option<&str>,
) -> Result<CargoIdentity> {
    let session = cargo_sources(runtime, manifest).await?;
    let frame=session.sql("SELECT (SELECT text FROM cargo_fields WHERE path=['package','name']) AS package,(SELECT text FROM cargo_fields WHERE path=['package','version']) AS version,coalesce((SELECT text FROM cargo_fields WHERE path=['lib','name']),replace((SELECT text FROM cargo_fields WHERE path=['package','name']),'-','_')) AS lib_name").await?;
    native_catalog::work(&session, "cargo_identity", frame.into_view())?;
    runtime.require_empty(session.sql("SELECT 'cargo_identity' AS witness FROM cargo_identity WHERE (package IS DISTINCT FROM $1) OR ($2 IS NOT NULL AND (version IS DISTINCT FROM $2)) OR lib_name IS NULL OR NOT regexp_like(lib_name,'^[A-Za-z_][A-Za-z0-9_]{0,254}$')").await?.with_param_values(vec![datafusion::common::ScalarValue::from(package),datafusion::common::ScalarValue::Utf8(version.map(str::to_owned))])?,"cargo_source_identity","producer_preparation").await?;
    runtime
        .records::<CargoIdentity>(session.table("cargo_identity").await?, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("Cargo source identity missing".into()))
}

/// Select exactly the producer configuration files that cannot influence the admitted build.
pub async fn cargo_removals(runtime: &QueryRuntime, files: Vec<String>) -> Result<Vec<Path>> {
    let session = runtime.session();
    input_files(&session, files)?;
    let frame = session.sql("SELECT path FROM preparation_files WHERE array_has(string_to_array(path,'/'),'.cargo') OR array_has(['rust-toolchain','rust-toolchain.toml'],array_element(string_to_array(path,'/'),-1)) ORDER BY path").await?;
    runtime.records(frame, 20_000).await
}

fn input_files(
    session: &datafusion::execution::context::SessionContext,
    files: Vec<String>,
) -> Result<()> {
    if files.len() > 20_000 || files.iter().map(String::len).sum::<usize>() > 16_777_216 {
        return Err(DataFusionError::ResourcesExhausted(
            "preparation inventory bound".into(),
        ));
    }
    native_catalog::input(
        session,
        "preparation_files",
        Path::batch(
            &files
                .into_iter()
                .map(|path| Path { path })
                .collect::<Vec<_>>(),
        )?,
    )
}

/// This format boundary uses only admitted ASCII tokens, so quoting adds no hidden escapes.
pub async fn cargo_manifest(
    runtime: &QueryRuntime,
    package: &str,
    root: &str,
    options: &enrichment_core::execution::producer::RustdocOptions,
) -> Result<String> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "cargo_request",
        CargoInput::batch(&[CargoInput {
            package: package.into(),
            root: root.into(),
            features: options.features.clone(),
            default_features: options.default_features,
        }])?,
    )?;
    runtime.require_empty(session.sql(r#"SELECT 'invalid_consumer_manifest' AS witness FROM cargo_request WHERE
      NOT regexp_like(package,'^[A-Za-z0-9_][A-Za-z0-9_-]{0,254}$') OR NOT regexp_like(root,'^[A-Za-z0-9_][A-Za-z0-9_.-]{0,254}$')
      OR cardinality(features)>256 OR array_any_match(features,f -> NOT regexp_like(f,'^[A-Za-z0-9_][A-Za-z0-9_+./?-]{0,254}$'))
    "#).await?,"consumer_manifest_tokens","producer_preparation").await?;
    let frame = session.sql(r#"SELECT concat('[package]',chr(10),'name="enrichment-consumer"',chr(10),'version="0.0.0"',chr(10),'edition="2024"',chr(10),'[workspace]',chr(10),'[dependencies]',chr(10),'"',package,'"={path="source/',root,'",features=[',array_to_string(array_transform(features,f -> concat('"',f,'"')),','),'],default-features=',CASE WHEN default_features THEN 'true' ELSE 'false' END,'}',chr(10)) AS text FROM cargo_request"#).await?;
    Ok(runtime
        .records::<Text>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("consumer manifest missing".into()))?
        .text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn native_preparation_uses_cargo_sections_and_unique_wheel_authority() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        for manifest in [
            "[lib]\npath='src/lib.rs'\n[dependencies]\nserde='1'",
            "[package.metadata.documentation]\npath='../display-only'\ngit='https://example.test'\nmembers=['a']",
            "[target.'cfg(unix)'.dependencies.serde]\nversion='1'",
            "[package]\nname='my-crate'\nbuild='build.rs'\n[[bin]]\nname='tool'\npath='src/bin/tool.rs'",
        ] {
            cargo_sources(&runtime, manifest).await?;
        }
        for manifest in [
            "[dependencies.local]\npath='deps/local'",
            "[target.'cfg(unix)'.dependencies.local]\npath='deps/local'",
            "[build-dependencies.local]\ngit='https://github.com/x/y'",
            "[workspace.dependencies.local]\nregistry='private'",
            "[patch.crates-io]\nserde={path='../serde'}",
            "[workspace]\nmembers=['../other']",
            "[package]\nworkspace='..'",
            "[package]\nbuild='../build.rs'",
            "[lib]\npath='/etc/passwd'",
            "[[bin]]\npath='src/../../main.rs'",
            "[lib]\npath='C:\\main.rs'",
            "[lib]\npath=12",
            "[invalid",
        ] {
            assert!(
                cargo_sources(&runtime, manifest).await.is_err(),
                "{manifest}"
            );
        }
        assert!(
            cargo_sources(
                &runtime,
                &"x".repeat(enrichment_core::native_cargo::MAX_BYTES + 1)
            )
            .await
            .is_err()
        );
        let manifest = "[package]\nname='my-long-crate'\nversion='1.2.3'";
        let identity = cargo_identity(&runtime, manifest, "my-long-crate", Some("1.2.3")).await?;
        assert_eq!(identity.lib_name, "my_long_crate");
        assert!(
            cargo_identity(&runtime, manifest, "other", Some("1.2.3"))
                .await
                .is_err()
        );
        assert!(
            cargo_identity(&runtime, manifest, "my-long-crate", Some("1.2.4"))
                .await
                .is_err()
        );
        let identity = cargo_identity(
            &runtime,
            &format!("{manifest}\n[lib]\nname='explicit_name'"),
            "my-long-crate",
            None,
        )
        .await?;
        assert_eq!(identity.lib_name, "explicit_name");
        let files = [
            ".cargo/config.toml",
            "nested/.cargo/config",
            "rust-toolchain",
            "nested/rust-toolchain.toml",
            "src/not-rust-toolchain",
            "src/lib.rs",
        ];
        let removals =
            cargo_removals(&runtime, files.into_iter().map(str::to_owned).collect()).await?;
        assert_eq!(
            removals.into_iter().map(|row| row.path).collect::<Vec<_>>(),
            [
                ".cargo/config.toml",
                "nested/.cargo/config",
                "nested/rust-toolchain.toml",
                "rust-toolchain"
            ]
        );
        let options = enrichment_core::execution::producer::RustdocOptions {
            target: enrichment_core::execution::producer::RUST_TARGET.into(),
            features: vec!["serde".into(), "other/feature".into()],
            default_features: false,
        };
        let manifest = cargo_manifest(&runtime, "my-crate", "my-crate-1.2.3", &options).await?;
        assert_eq!(
            manifest,
            "[package]\nname=\"enrichment-consumer\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[dependencies]\n\"my-crate\"={path=\"source/my-crate-1.2.3\",features=[\"serde\",\"other/feature\"],default-features=false}\n"
        );
        assert!(
            cargo_manifest(&runtime, "bad\"name", "my-crate-1.2.3", &options)
                .await
                .is_err()
        );
        runtime.close_diagnostics().await
    }
}
