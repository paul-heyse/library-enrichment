//! One native environment rule serves both new consumer preparation and retained selection.
use crate::{control::ControlSnapshot, native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::Result,
    functions::core::expr_ext::FieldAccessor,
    prelude::{col, lit},
};
use enrichment_core::{
    execution::environment::Target,
    identity::{Context, Ecosystem, Environment},
    native_union::{NativeStruct, Rule},
};
use std::ops::Not;

enrichment_core::native_struct! { struct Declared {
    environment: Environment => Rule::Text,
    target: Target => Rule::Text,
} }
enrichment_core::native_struct! { struct Parent {
    context: Context => Rule::Text,
    normalizer_version: String => Rule::NonEmpty,
} }

fn accepted() -> datafusion::logical_expr::Expr {
    use datafusion::functions_nested::expr_fn::array_has;
    let declared = col("environment");
    let target = col("target");
    declared
        .clone()
        .field("toolchain")
        .is_null()
        .or(declared
            .clone()
            .field("toolchain")
            .eq(target.clone().field("toolchain")))
        .or(array_has(
            target.clone().field("toolchain_aliases"),
            declared.clone().field("toolchain"),
        ))
        .and(
            declared
                .clone()
                .field("target")
                .is_null()
                .or(declared
                    .clone()
                    .field("target")
                    .eq(target.clone().field("target")))
                .or(array_has(
                    target.field("target_aliases"),
                    declared.field("target"),
                )),
        )
}

pub async fn admit(
    runtime: &QueryRuntime,
    environment: &Environment,
    ecosystem: Ecosystem,
    image: &str,
) -> Result<Target> {
    let target = Target::consumer(ecosystem, image);
    let frame = crate::native_catalog::batch(
        &runtime.session(),
        "environment_plan",
        Declared::batch(&[Declared {
            environment: environment.clone(),
            target: target.clone(),
        }])?,
    )?;
    runtime
        .require_empty(
            frame
                .filter(
                    datafusion::functions::core::expr_fn::coalesce(vec![accepted(), lit(false)])
                        .not(),
                )?
                .select(vec![
                    lit("unsupported_consumer_environment").alias("witness"),
                ])?,
            "consumer_environment",
            "process_preparation",
        )
        .await?;
    Ok(target)
}

/// Filter scope and target properties before opening any derived snapshot. Actual static
/// input equality is then a relational comparison over the two admitted snapshot providers.
pub async fn children(
    runtime: &QueryRuntime,
    catalog: &ControlSnapshot,
    parent: &Context,
    environment: &Environment,
    ecosystem: Ecosystem,
    image: &str,
    normalizer: &str,
) -> Result<Vec<Context>> {
    let session = catalog.session(runtime).await?;
    native_catalog::input(
        &session,
        "environment_parent",
        Parent::batch(&[Parent {
            context: parent.clone(),
            normalizer_version: normalizer.into(),
        }])?,
    )?;
    let declaration = crate::native_catalog::batch(
        &session,
        "environment_plan",
        Declared::batch(&[Declared {
            environment: environment.clone(),
            target: Target::consumer(ecosystem, image),
        }])?,
    )?
    .filter(accepted())?;
    native_catalog::work(&session, "environment_scope", declaration.into_view())?;
    let frame=session.sql(r#"
        SELECT c.* FROM state.records.contexts c
        JOIN state.records.environments e ON c.environment_id=e.environment_id
        JOIN state.records.selections selected ON selected.context_id=c.context_id
        JOIN state.records.snapshots s ON s.snapshot_id=selected.snapshot_id
        CROSS JOIN environment_parent p CROSS JOIN environment_scope d
        WHERE c.parent_context_id=p.context.context_id AND c.release_id=p.context.release_id
          AND s.publication.metadata.normalizer_version=p.normalizer_version
          AND e.resolution='resolved' AND e.lock_digest IS NOT NULL
          AND e.toolchain=d.target.toolchain AND e.target=d.target.target
          AND (d.environment.features IS NULL OR e.features=d.environment.features)
          AND (d.environment.default_features IS NULL OR e.default_features=d.environment.default_features)
        ORDER BY c.context_id LIMIT 65
    "#).await?;
    runtime.records(frame, 64).await
}

/// Compare complete digest/locator pairs without decoding sets into Rust.
pub(crate) async fn same_static_inputs(
    runtime: &QueryRuntime,
    left: datafusion::dataframe::DataFrame,
    right: datafusion::dataframe::DataFrame,
) -> Result<bool> {
    let session = runtime.session();
    for (name, frame) in [
        ("left_static_inputs", left.clone()),
        ("right_static_inputs", right.clone()),
    ] {
        crate::native_catalog::work(&session, name, frame.into_view())?;
        runtime
            .require_empty(
                session
                    .sql(&format!(
                        "SELECT '{name}' AS witness FROM {name} HAVING count(*)>8192"
                    ))
                    .await?,
                "static_input_bound",
                "inspection_selection",
            )
            .await?;
    }
    let difference = left
        .clone()
        .except_distinct(right.clone())?
        .union(right.except_distinct(left)?)?
        .limit(0, Some(1))?;
    Ok(runtime.execute(difference).await?.rows == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn plan19_environment_admission_and_paired_static_inputs() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        for ecosystem in [Ecosystem::Python, Ecosystem::Rust] {
            let target = admit(&runtime, &Environment::unspecified(), ecosystem, "image").await?;
            assert!(target.toolchain.ends_with(";image"));
            let resolved = Environment::resolved(
                target.toolchain,
                target.target,
                vec![],
                None,
                "a".repeat(64),
            );
            admit(&runtime, &resolved, ecosystem, "image").await?;
            assert!(
                admit(&runtime, &resolved, ecosystem, "changed-image")
                    .await
                    .is_err()
            );
            let unsupported = Environment::declared(Some("unsupported-target".into()), None, None);
            assert!(
                admit(&runtime, &unsupported, ecosystem, "image")
                    .await
                    .is_err()
            );
        }
        admit(
            &runtime,
            &Environment::python(Some("3.14".into()), Some("linux".into()), None),
            Ecosystem::Python,
            "image",
        )
        .await?;
        assert!(
            admit(
                &runtime,
                &Environment::python(Some("3.13".into()), None, None),
                Ecosystem::Python,
                "image"
            )
            .await
            .is_err()
        );
        let session = runtime.session();
        let left = session
            .sql("SELECT * FROM (VALUES ('a','uri1'),('b','uri2')) AS inputs(sha256,source_uri)")
            .await?;
        let reordered = session.sql("SELECT * FROM (VALUES ('b','uri2'),('a','uri1'),('a','uri1')) AS inputs(sha256,source_uri)").await?;
        let unpaired = session
            .sql("SELECT * FROM (VALUES ('a','uri2'),('b','uri1')) AS inputs(sha256,source_uri)")
            .await?;
        assert!(same_static_inputs(&runtime, left.clone(), reordered).await?);
        assert!(!same_static_inputs(&runtime, left.clone(), unpaired).await?);
        assert!(!same_static_inputs(&runtime, left.clone(), left.filter(lit(false))?).await?);
        runtime.close_diagnostics().await
    }
}
