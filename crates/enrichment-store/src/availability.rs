//! Native documented-build/project comparison. Unknown facts stay unknown.
use crate::runtime::QueryRuntime;
use datafusion::common::Result;
use enrichment_core::{
    evidence::{Availability, ObservedConfiguration, RequestedConfiguration},
    identity::Environment,
    native_union::{NativeStruct, Rule},
};

enrichment_core::native_struct! {
struct Input {
    observed: Option<ObservedConfiguration> => Rule::Text,
    features: Vec<String> => Rule::Set,
    features_known: bool => Rule::Text,
    default_features: Option<bool> => Rule::Text,
    target: Option<String> => Rule::Text,
    cfg_hints: Option<Vec<String>> => Rule::Sequence,
}
}

/// All selection and note conditions run in the shared native session.
pub async fn select(
    runtime: &QueryRuntime,
    observed: Option<&ObservedConfiguration>,
    environment: &Environment,
    cfg_hints: Option<&[String]>,
) -> Result<Option<Availability>> {
    let session = runtime.session();
    session.register_batch(
        "availability_input",
        Input::batch(&[Input {
            observed: observed.cloned(),
            features: environment.features.clone(),
            features_known: environment.features_known,
            default_features: environment.default_features,
            target: environment.target.clone(),
            cfg_hints: cfg_hints.map(<[String]>::to_vec),
        }])?,
    )?;
    let requested = session.sql("SELECT observed, cfg_hints, CASE WHEN features_known OR cardinality(features)>0 THEN features ELSE NULL END AS features, default_features,target FROM availability_input").await?;
    let requested = requested.select(vec![
        datafusion::prelude::col("observed"),
        datafusion::prelude::col("cfg_hints"),
        enrichment_core::native_record::record(
            RequestedConfiguration::fields(),
            vec![
                datafusion::prelude::lit("features"),
                datafusion::prelude::col("features"),
                datafusion::prelude::lit("default_features"),
                datafusion::prelude::col("default_features"),
                datafusion::prelude::lit("target"),
                datafusion::prelude::col("target"),
            ],
        )
        .alias("requested"),
    ])?;
    crate::native_catalog::work(&session, "availability_requested", requested.into_view())?;
    let selected = session.sql(r#"
      WITH qualified AS (
        SELECT *, array_except(observed.features,requested.features) AS extra_features
        FROM availability_requested WHERE observed IS NOT NULL AND cfg_hints IS NOT NULL
      )
      SELECT 'project_availability_unverified' AS status,
        observed AS observed_configuration, requested AS requested_configuration,
        array_compact([
          CASE WHEN requested.target IS NOT NULL AND requested.target<>observed.target
            THEN concat('The documentation was built for ',observed.target,'; the project targets ',requested.target,'. Target-gated items may differ.') END,
          CASE WHEN observed.all_features THEN 'The documentation build enabled all features; a project with a narrower feature set may not have every documented item.' END,
          CASE WHEN cardinality(extra_features)>0 THEN concat('The documentation build enabled features the project does not: ',array_to_string(extra_features,', '),'.') END,
          CASE WHEN requested.features IS NULL AND requested.target IS NULL THEN 'No project environment was declared, so availability in the project is unverified.' END,
          CASE WHEN cardinality(cfg_hints)>0 THEN concat('The item carries declared cfg hints (',array_to_string(cfg_hints,'; '),'); these are documentation annotations, not a verified feature predicate.') END
        ]) AS notes
      FROM qualified
    "#).await?;
    Ok(runtime.records(selected, 1).await?.pop())
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::evidence::AvailabilityStatus;

    #[tokio::test]
    async fn native_availability_preserves_unknown_empty_and_observation_scope() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let observed = ObservedConfiguration {
            features: vec!["std".into()],
            all_features: true,
            no_default_features: false,
            target: "x86_64-unknown-linux-gnu".into(),
            format_version: 61,
            source: "rustdoc".into(),
        };
        let unknown = Environment::unspecified();
        let result = select(&runtime, Some(&observed), &unknown, Some(&[]))
            .await?
            .unwrap();
        assert_eq!(
            result.status,
            AvailabilityStatus::ProjectAvailabilityUnverified
        );
        assert!(result.requested_configuration.features.is_none());
        assert_eq!(result.notes.len(), 2);
        assert!(result.notes[1].starts_with("No project environment"));
        let requested = Environment::declared(
            Some("wasm32-unknown-unknown".into()),
            Some(vec![]),
            Some(false),
        );
        let result = select(
            &runtime,
            Some(&observed),
            &requested,
            Some(&["unix".into()]),
        )
        .await?
        .unwrap();
        assert_eq!(result.requested_configuration.features, Some(vec![]));
        assert_eq!(result.notes.len(), 4);
        assert!(result.notes[0].contains("wasm32-unknown-unknown"));
        assert!(result.notes[2].ends_with("std."));
        assert!(result.notes[3].contains("(unix)"));
        assert!(
            select(&runtime, Some(&observed), &requested, None)
                .await?
                .is_none()
        );
        assert!(
            select(&runtime, None, &requested, Some(&[]))
                .await?
                .is_none()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
