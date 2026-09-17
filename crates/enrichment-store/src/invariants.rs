//! One bounded native admission relation, with the rule carried by each witness.
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions::string::expr_fn::concat,
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
        self.branches.push(
            frame
                .select(vec![
                    lit(rule).alias("rule"),
                    lit(stage).alias("stage"),
                    // A diagnostic label is deliberately plain text, not an identity domain.
                    concat(vec![lit(""), key]).alias("witness_id"),
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
}
