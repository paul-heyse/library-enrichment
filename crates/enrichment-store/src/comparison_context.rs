//! Typed configuration differences and evidence-confounding conditions are native selections.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{common::Result, functions::core::expr_ext::FieldAccessor, prelude::*};
use enrichment_core::{
    evidence::{ObservedConfiguration, arrow_model::expressions::record},
    identity::Environment,
    native_union::{Cell, NativeStruct, Rule},
    wire::data::ConfigurationDifference,
};

enrichment_core::native_struct! {
    struct Inputs {
        before: Environment => Rule::Text,
        after: Environment => Rule::Text,
        before_observed: Option<ObservedConfiguration> => Rule::Text,
        after_observed: Option<ObservedConfiguration> => Rule::Text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn native_configuration_comparison_preserves_unknowns_and_set_meaning() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let before = Environment::unspecified();
        let mut after = before.clone();
        after.target = Some("aarch64-unknown-linux-gnu".into());
        after.features = Some(vec![]);
        let observed = ObservedConfiguration {
            features: vec!["b".into(), "a".into()],
            all_features: false,
            no_default_features: false,
            target: "x86_64-unknown-linux-gnu".into(),
            format_version: 61,
            source: "rustdoc".into(),
        };
        let mut reordered = observed.clone();
        reordered.features.reverse();
        let (differences, confounders) = assess(
            &runtime,
            &before,
            &after,
            &Some(observed),
            &Some(reordered.clone()),
        )
        .await?;
        assert_eq!(differences.len(), 2);
        assert!(differences.iter().any(|value|matches!(value,ConfigurationDifference::Target {before:None,after:Some(value)} if value=="aarch64-unknown-linux-gnu")));
        assert!(differences.iter().any(|value| matches!(
            value,
            ConfigurationDifference::Features {
                before: None,
                after: Some(_)
            }
        )));
        assert!(
            confounders
                .iter()
                .any(|value| value == "before features/extras is unknown")
        );
        let (differences, _) = assess(&runtime, &before, &before, &None, &Some(reordered)).await?;
        assert!(matches!(
            differences.as_slice(),
            [ConfigurationDifference::ObservedConfiguration {
                before: None,
                after: Some(_)
            }]
        ));
        Ok(())
    }
}
enrichment_core::native_struct! {
    struct Difference { difference: ConfigurationDifference => Rule::Text }
}
enrichment_core::native_struct! {
    struct Confounder { message: String => Rule::NonEmpty }
}

/// Generate each independent typed branch from the difference declaration. Canonical native
/// equality respects nested set metadata while preserving unknown versus explicit empty values.
pub async fn assess(
    runtime: &QueryRuntime,
    before: &Environment,
    after: &Environment,
    before_observed: &Option<ObservedConfiguration>,
    after_observed: &Option<ObservedConfiguration>,
) -> Result<(Vec<ConfigurationDifference>, Vec<String>)> {
    let session = runtime.session();
    native_catalog::work(
        &session,
        "configuration_inputs",
        session
            .read_batch(Inputs::batch(&[Inputs {
                before: before.clone(),
                after: after.clone(),
                before_observed: before_observed.clone(),
                after_observed: after_observed.clone(),
            }])?)?
            .into_view(),
    )?;
    let input = session.table("configuration_inputs").await?;
    let kind = ConfigurationDifference::data_type();
    let arrow::datatypes::DataType::Struct(fields) = &kind else {
        unreachable!()
    };
    let mut plans = Vec::new();
    for field in fields.iter().skip(1) {
        let (before, after) = if field.name() == "observed_configuration" {
            (col("before_observed"), col("after_observed"))
        } else {
            (
                col("before").field(field.name()),
                col("after").field(field.name()),
            )
        };
        let arrow::datatypes::DataType::Struct(members) = field.data_type() else {
            unreachable!()
        };
        // Both values use the same field name in the equality contract, not before/after labels.
        let canonical = enrichment_core::native_identity::canonical_bytes(
            format!("configuration-value/1/{}", field.name()),
            vec![members[0].as_ref().clone()].into(),
        );
        let changed = canonical
            .call(vec![before.clone()])
            .not_eq(canonical.call(vec![after.clone()]));
        let payload = record(field.data_type(), &[("before", before), ("after", after)])?;
        let difference = record(
            &kind,
            &[("field", lit(field.name())), (field.name(), payload)],
        )?;
        plans.push(
            input
                .clone()
                .filter(changed)?
                .select(vec![difference.alias("difference")])?,
        );
    }
    let mut plans = plans.into_iter();
    let mut plan = plans.next().expect("finite configuration variants");
    for branch in plans {
        plan = plan.union(branch)?;
    }
    let differences = runtime
        .records::<Difference>(plan, fields.len() - 1)
        .await?
        .into_iter()
        .map(|row| row.difference)
        .collect::<Vec<_>>();
    let messages = session.sql("WITH sides AS (SELECT 'before' AS label, before AS environment FROM configuration_inputs UNION ALL SELECT 'after',after FROM configuration_inputs), missing AS (SELECT label,'toolchain' AS field FROM sides WHERE environment.toolchain IS NULL UNION ALL SELECT label,'target' FROM sides WHERE environment.target IS NULL UNION ALL SELECT label,'features/extras' FROM sides WHERE environment.features IS NULL UNION ALL SELECT label,'dependency resolution' FROM sides WHERE environment.lock_digest IS NULL) SELECT concat(label,' ',field,' is unknown') AS message FROM missing ORDER BY label,field").await?;
    let mut confounders = runtime
        .records::<Confounder>(messages, 8)
        .await?
        .into_iter()
        .map(|row| row.message)
        .collect::<Vec<_>>();
    // Selection of the qualifier is part of the same native difference relation.
    let selected = session.read_batch(Difference::batch(
        &differences
            .iter()
            .cloned()
            .map(|difference| Difference { difference })
            .collect::<Vec<_>>(),
    )?)?;
    let qualifier = selected.limit(0,Some(1))?.select(vec![lit("Environment or observed configuration differs; do not attribute every difference to the release").alias("message")])?;
    confounders.extend(
        runtime
            .records::<Confounder>(qualifier, 1)
            .await?
            .into_iter()
            .map(|row| row.message),
    );
    Ok((differences, confounders))
}
