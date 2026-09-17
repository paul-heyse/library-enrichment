//! One bounded native admission relation, with the rule carried by each witness.
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    logical_expr::Expr,
    prelude::lit,
};

/// Compilation only: all predicates, joins and violation selection execute in DataFusion.
#[derive(Default)]
pub(crate) struct Invariants {
    branches: Vec<DataFrame>,
}

impl Invariants {
    pub fn push(&mut self, frame: DataFrame, rule: &str, stage: &str) -> Result<()> {
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
        let witness = enrichment_core::native_id::diagnostic(field, key)?;
        self.branches.push(
            frame
                .select(vec![
                    lit(rule).alias("rule"),
                    lit(stage).alias("stage"),
                    // A diagnostic label is deliberately plain text, not an identity domain.
                    witness.alias("witness_id"),
                ])?
                .limit(0, Some(1))?,
        );
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
