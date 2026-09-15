//! Narrow Arrow batch scoring. Native expressions own eligibility; native windows own folding.

use crate::projection::{self, TextColumn};
use arrow::{
    array::{Array, BooleanArray},
    datatypes::DataType,
};
use datafusion::{
    error::{DataFusionError, Result},
    logical_expr::{
        ColumnarValue, Expr, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature, Volatility,
    },
    prelude::{col, lit},
};
use enrichment_core::search::{
    self, SymbolScoreFields,
    spec::{Clause, MatchOp, SearchSpec},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScoreKind {
    Symbol,
    Fragment,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ScoreUdf {
    kind: ScoreKind,
    signature: Signature,
    spec: SearchSpec,
}

/// A request-local pure function whose optimizer identity includes the complete query contract.
#[must_use]
pub fn function(kind: ScoreKind, spec: SearchSpec) -> ScalarUDF {
    let fields = match kind {
        ScoreKind::Symbol => vec![
            DataType::Utf8,
            DataType::Utf8,
            DataType::Utf8,
            DataType::Utf8,
            DataType::Utf8,
            DataType::Boolean,
        ],
        ScoreKind::Fragment => vec![DataType::Utf8, DataType::Utf8],
    };
    ScalarUDF::from(ScoreUdf {
        kind,
        signature: Signature::exact(fields, Volatility::Immutable),
        spec,
    })
}

impl ScalarUDFImpl for ScoreUdf {
    fn name(&self) -> &str {
        match self.kind {
            ScoreKind::Symbol => "evidence_symbol_score_v2",
            ScoreKind::Fragment => "evidence_fragment_score_v2",
        }
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _arguments: &[DataType]) -> Result<DataType> {
        Ok(projection::score::result(&[])?.data_type().clone())
    }

    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        let arrays = args
            .args
            .into_iter()
            .map(|a| a.into_array(args.number_rows))
            .collect::<Result<Vec<_>>>()?;
        let string_count = match self.kind {
            ScoreKind::Symbol => 5,
            ScoreKind::Fragment => 2,
        };
        if arrays.len() != string_count + usize::from(self.kind == ScoreKind::Symbol) {
            return Err(invalid("incorrect score argument count"));
        }
        let strings = arrays[..string_count]
            .iter()
            .map(|a| TextColumn::new(a.as_ref()))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut rows = Vec::with_capacity(args.number_rows);
        match self.kind {
            ScoreKind::Symbol => {
                let reexports = arrays[5]
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .ok_or_else(|| invalid("non-boolean reexport flag"))?;
                for i in 0..args.number_rows {
                    if reexports.is_null(i) {
                        return Err(invalid("null reexport flag"));
                    }
                    let fields = SymbolScoreFields {
                        path: strings[0]
                            .get(i)
                            .ok_or_else(|| invalid("null public path"))?,
                        name: strings[1]
                            .get(i)
                            .ok_or_else(|| invalid("null public name"))?,
                        signature: strings[2].get(i),
                        summary: strings[3].get(i),
                        docs: strings[4].get(i),
                        is_reexport: reexports.value(i),
                    };
                    rows.push(search::score_symbol_fields(
                        fields,
                        &self.spec.query,
                        &self.spec.tokens,
                    ));
                }
            }
            ScoreKind::Fragment => {
                for i in 0..args.number_rows {
                    rows.push(search::score_fragment_fields(
                        strings[0]
                            .get(i)
                            .ok_or_else(|| invalid("null fragment subject"))?,
                        strings[1]
                            .get(i)
                            .ok_or_else(|| invalid("null fragment text"))?,
                        &self.spec.query,
                        &self.spec.tokens,
                    ));
                }
            }
        }
        Ok(ColumnarValue::Array(projection::score::result(&rows)?))
    }
}

/// Lower the canonical eligibility clauses to optimizer-visible literal string expressions.
/// Pattern characters have no wildcard meaning, and absent text cannot establish eligibility.
#[must_use]
pub fn eligibility(clauses: &[Clause]) -> Expr {
    eligibility_columns(clauses, false)
}

/// Domain fragments keep their display label separate from the typed subject reference.
#[must_use]
pub fn fragment_eligibility(clauses: &[Clause]) -> Expr {
    eligibility_columns(clauses, true)
}

fn eligibility_columns(clauses: &[Clause], fragment: bool) -> Expr {
    use datafusion::functions::string::expr_fn::{ends_with, lower, starts_with};
    use datafusion::functions::unicode::expr_fn::strpos;
    clauses
        .iter()
        .map(|clause| {
            let column = clause.field.column();
            let value = lower(col(if fragment && column == "subject" {
                "label"
            } else {
                column
            }));
            let query = lit(&clause.literal);
            match clause.op {
                MatchOp::Equals => value.eq(query),
                MatchOp::Prefix => starts_with(value, query),
                MatchOp::Suffix => ends_with(value, query),
                MatchOp::Contains => strpos(value, query).gt(lit(0i64)),
            }
        })
        .reduce(Expr::or)
        .unwrap_or_else(|| lit(false))
}

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
