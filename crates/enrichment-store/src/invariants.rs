//! One bounded native admission relation, with the rule carried by each witness.
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    logical_expr::Expr,
    prelude::lit,
};

/// One diagnostic boundary for both individual and combined invariant checks. The
/// native field owns the representation; consumers never assume every key is text.
pub(crate) fn witness(frame: DataFrame, stage: &str, limit: usize) -> Result<DataFrame> {
    if frame.schema().fields().len() != 1 {
        return Err(crate::preparation::InvariantFailure::contract(
            "invariant query must select one identity column",
            stage,
            Vec::new(),
        ));
    }
    let (qualifier, field) = frame.schema().qualified_field(0);
    let key = Expr::Column(datafusion::common::Column::new(
        qualifier.cloned(),
        field.name(),
    ));
    let value = enrichment_core::native_id::diagnostic(field, key)?;
    frame
        .select(vec![value.alias("witness_id")])?
        .limit(0, Some(limit))
}

/// Compilation only: all predicates, joins and violation selection execute in DataFusion.
#[derive(Default)]
pub(crate) struct Invariants {
    branches: Vec<DataFrame>,
}

impl Invariants {
    pub fn push(&mut self, frame: DataFrame, rule: &str, stage: &str) -> Result<()> {
        let frame = witness(frame, stage, 1)?;
        self.branches.push(frame.select(vec![
            lit(rule).alias("rule"),
            lit(stage).alias("stage"),
            // A diagnostic label is deliberately plain text, not an identity domain.
            datafusion::prelude::col("witness_id"),
        ])?);
        Ok(())
    }

    pub fn plan(self) -> Result<DataFrame> {
        let mut branches = self.branches.into_iter();
        let first = branches.next().ok_or_else(|| {
            DataFusionError::Internal("empty invariant admission definition".into())
        })?;
        // Let DataFusion schedule the native branches under the shared runtime's
        // resource policy. Success must exhaust the union; one violation is enough
        // to refuse admission, with its declared rule and witness intact.
        branches
            .try_fold(first, DataFrame::union)?
            .limit(0, Some(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn union_admission_preserves_rule_and_witness() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let session = runtime.session();
        let empty = session.sql("SELECT 'unseen' AS id WHERE false").await?;
        let mut valid = Invariants::default();
        valid.push(empty.clone(), "first", "native")?;
        valid.push(empty.clone(), "second", "native")?;
        runtime.admit(valid).await?;
        let mut invalid = Invariants::default();
        invalid.push(empty, "first", "native")?;
        invalid.push(
            session.sql("SELECT 'symbol_λ' AS id").await?,
            "second",
            "reference",
        )?;
        let error = runtime.admit(invalid).await.unwrap_err();
        let DataFusionError::External(error) = error else {
            panic!("expected a structured invariant failure: {error}");
        };
        let failure = error
            .downcast_ref::<crate::preparation::InvariantFailure>()
            .unwrap();
        assert_eq!(failure.rule, "second");
        assert_eq!(failure.stage, "reference");
        assert_eq!(failure.affected_ids, ["symbol_λ"]);
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn binary_domain_witnesses_share_bounded_native_projection() -> Result<()> {
        use datafusion::prelude::col;
        use enrichment_core::{
            identity::{AttemptId, InterestId, JobId},
            native_union::{NativeStruct, Rule},
        };
        enrichment_core::native_struct! { struct Ids {
            job: JobId => Rule::Text,
            attempt: AttemptId => Rule::Text,
            interest: InterestId => Rule::Text,
            absent: Option<AttemptId> => Rule::Text,
        } }
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let ids = Ids {
            job: "job_00112233445566778899aabbccddeeff"
                .to_owned()
                .try_into()
                .unwrap(),
            attempt: "attempt_00112233445566778899aabbccddeeff"
                .to_owned()
                .try_into()
                .unwrap(),
            interest: "interest_00112233445566778899aabbccddeeff"
                .to_owned()
                .try_into()
                .unwrap(),
            absent: None,
        };
        let frame = crate::native_catalog::batch(
            &runtime.session(),
            "domain_witnesses",
            Ids::batch(&vec![ids; 10])?,
        )?;
        for name in ["job", "attempt", "interest"] {
            let selected = frame.clone().select(vec![col(name)])?;
            runtime
                .require_empty(selected.clone().filter(lit(false))?, "empty", "typed")
                .await?;
            let error = runtime
                .require_empty(selected.clone(), "declared_rule", "typed")
                .await
                .unwrap_err();
            let DataFusionError::External(error) = error else {
                panic!("structured invariant failure expected");
            };
            let failure = error
                .downcast_ref::<crate::preparation::InvariantFailure>()
                .unwrap();
            assert_eq!(failure.rule, "declared_rule");
            assert_eq!(
                failure.affected_ids,
                vec![format!("{name}_00112233445566778899aabbccddeeff"); 8]
            );
            let mut combined = Invariants::default();
            combined.push(selected, "combined_rule", "typed")?;
            let error = runtime.admit(combined).await.unwrap_err();
            let DataFusionError::External(error) = error else {
                panic!("structured invariant failure expected");
            };
            let failure = error
                .downcast_ref::<crate::preparation::InvariantFailure>()
                .unwrap();
            assert_eq!(
                failure.affected_ids,
                [format!("{name}_00112233445566778899aabbccddeeff")]
            );
        }
        let fallback = enrichment_core::evidence::arrow_model::expressions::coalesce(vec![
            col("absent"),
            col("attempt"),
        ])?;
        let selected = frame.clone().select(vec![fallback.alias("selected")])?;
        let result = runtime.execute(witness(selected, "typed", 1)?).await?;
        assert_eq!(
            crate::preparation::witnesses(&result.batches),
            ["attempt_00112233445566778899aabbccddeeff"]
        );
        let incompatible = enrichment_core::evidence::arrow_model::expressions::coalesce(vec![
            col("absent"),
            col("job"),
        ])?;
        match frame.clone().select(vec![incompatible.alias("selected")]) {
            Err(_) => {}
            Ok(incompatible) => assert!(runtime.execute(incompatible).await.is_err()),
        }
        let session = runtime.session();
        crate::native_catalog::work(&session, "selection_values", frame.clone().into_view())?;
        for expression in [
            "native_coalesce(absent, attempt)",
            "native_coalesce(NULL, absent, attempt)",
            "native_coalesce(absent, attempt, NULL)",
        ] {
            let selected = session
                .sql(&format!(
                    "SELECT {expression} AS selected FROM operation.work.selection_values"
                ))
                .await?;
            enrichment_core::native_analysis::compatible(
                frame.schema().field_with_unqualified_name("attempt")?,
                selected.schema().field_with_unqualified_name("selected")?,
                "SQL coalesce alias",
            )?;
            let result = runtime.execute(witness(selected, "typed", 1)?).await?;
            assert_eq!(
                crate::preparation::witnesses(&result.batches),
                ["attempt_00112233445566778899aabbccddeeff"]
            );
        }
        assert!(
            session
                .sql("SELECT native_coalesce(absent, job) FROM operation.work.selection_values")
                .await
                .is_err(),
            "equal UUID bytes cannot change an identity domain"
        );
        let absent = runtime
            .execute(witness(frame.select(vec![col("absent")])?, "typed", 1)?)
            .await?;
        assert_eq!(
            absent.batches[0].column(0).null_count(),
            1,
            "missing identity cannot invent a prefix-only witness"
        );
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn admission_propagates_native_read_errors() -> Result<()> {
        use arrow::datatypes::{DataType, Field, Schema};
        use datafusion::prelude::{CsvReadOptions, col};

        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let session = runtime.session();
        let path = root.path().join("invalid.csv");
        std::fs::write(&path, "id\ninvalid_integer\n")?;
        let schema = Schema::new(vec![Field::new("id", DataType::Int64, false)]);
        let invalid = session
            .read_csv(
                path.to_str().unwrap(),
                CsvReadOptions::new().schema(&schema),
            )
            .await?
            .filter(col("id").gt(lit(0i64)))?;
        // An empty earlier branch cannot hide a failing native scan. Successful
        // admission must exhaust every branch, including its final stream error.
        let mut read_failure = Invariants::default();
        read_failure.push(
            session.sql("SELECT 'empty' AS id WHERE false").await?,
            "empty",
            "test",
        )?;
        read_failure.push(invalid, "must_read", "test")?;
        let error = runtime.admit(read_failure).await.unwrap_err();
        assert!(error.to_string().contains("invalid_integer"), "{error}");
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
