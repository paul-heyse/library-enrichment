//! Evaluating one literal of an interaction predicate.
//!
//! # The whole model, in two `GROUP BY`s
//!
//! A predicate is stored in disjunctive normal form as ordinary rows: AND within a `term_ord`, OR
//! across distinct `term_ord`s. Evaluating it is
//!
//! ```sql
//! WITH atom AS (SELECT interaction_key, term_ord, atom_truth(...) AS t FROM ...),
//!      term AS (SELECT interaction_key, term_ord, min(t) AS t FROM atom GROUP BY 1, 2)
//! SELECT interaction_key, max(t) AS truth_rank FROM term GROUP BY 1
//! ```
//!
//! `min` then `max`, over the `truth` lattice. There is no interpreter and no expression tree,
//! because Arrow has no recursive type and an interpreter is the procedural code this design
//! exists to avoid.
//!
//! # Kleene `unknown` is not implemented; it falls out
//!
//! An atom whose facet the request does not bind returns `unknown`. `min` carries it through the
//! conjunct, `max` lets a satisfied alternative outrank it, and an interaction finishing at
//! `unknown` is neither `true` nor `false` -- so retrieval surfaces it as an unresolved choice
//! rather than silently treating it as false. The proposal asks for exactly that; the encoding is
//! what provides it.
//!
//! # How the request is carried
//!
//! `bindings` is a single `Utf8` of `facet=value` pairs separated by `;`. That is deliberately
//! humble: the alternative is a map type threaded through a UDF signature, and the request is
//! built by the caller one argument at a time anyway. A facet absent from the string is *unbound*,
//! which is the case the whole three-valued treatment exists for -- so absence is answered
//! `unknown`, never `false`.

use std::sync::Arc;

use arrow::array::{Array, BooleanArray, Int8Array, ListArray, StringArray};
use arrow_schema::DataType;
use datafusion::common::Result as DFResult;
use datafusion::common::{exec_err, plan_err};
use datafusion::logical_expr::{
    ColumnarValue, Documentation, ScalarFunctionArgs, ScalarUDFImpl, Signature, Volatility,
};

use crate::lattice;

/// The operators an atom may use. Closed, because an unrecognised operator must be an error
/// rather than a quiet `unknown` -- a typo that evaluates to "we do not know" is indistinguishable
/// from a genuine gap in the request, and only one of those is a bug.
pub const OPS: &[&str] = &["eq", "ne", "in", "present", "absent", "gte", "lte"];

/// Parse a `facet=value;facet=value` request string, returning the value bound to `facet`.
fn binding<'a>(bindings: &'a str, facet: &str) -> Option<&'a str> {
    bindings.split(';').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        (k.trim() == facet).then(|| v.trim())
    })
}

/// Evaluate one literal, as a rank in the `truth` lattice.
fn evaluate(
    facet: &str,
    op: &str,
    value: Option<&str>,
    value_list: &[String],
    negated: bool,
    bindings: &str,
) -> Result<i8, String> {
    let t = lattice::rank_of("truth", "true").expect("the truth lattice has `true`");
    let f = lattice::rank_of("truth", "false").expect("the truth lattice has `false`");
    let u = lattice::rank_of("truth", "unknown").expect("the truth lattice has `unknown`");

    let bound = binding(bindings, facet);

    // `present` and `absent` ask about the binding itself, so they are total: an unbound facet is
    // a definite answer to them rather than an unknown one.
    let raw = match op {
        "present" => {
            if bound.is_some() {
                t
            } else {
                f
            }
        }
        "absent" => {
            if bound.is_some() {
                f
            } else {
                t
            }
        }
        _ => {
            let Some(actual) = bound else {
                // The facet was never bound. This is the case the three-valued treatment exists
                // for: not false, not true, and it must reach the caller as a choice still open.
                return Ok(u);
            };
            match op {
                "eq" => bool_rank(Some(actual) == value, t, f),
                "ne" => bool_rank(Some(actual) != value, t, f),
                "in" => bool_rank(value_list.iter().any(|v| v == actual), t, f),
                "gte" | "lte" => {
                    // Ordered comparison is numeric or it is nothing. A lexicographic fallback
                    // would silently answer a different question -- the same trap the `Int8`
                    // lattice encoding exists to avoid.
                    let (Ok(lhs), Some(Ok(rhs))) =
                        (actual.parse::<i64>(), value.map(str::parse::<i64>))
                    else {
                        return Ok(u);
                    };
                    bool_rank(if op == "gte" { lhs >= rhs } else { lhs <= rhs }, t, f)
                }
                other => return Err(format!("`{other}` is not an interaction operator")),
            }
        }
    };

    // Negation swaps true and false and leaves unknown alone, which is what Kleene negation is.
    Ok(if negated && raw == t {
        f
    } else if negated && raw == f {
        t
    } else {
        raw
    })
}

fn bool_rank(b: bool, t: i8, f: i8) -> i8 {
    if b { t } else { f }
}

/// `atom_truth(facet, op, value, value_list, negated, bindings) -> Int8`
///
/// The only UDF the interaction model needs, and a pure scalar over six arguments. It never
/// appears in a Delta CHECK constraint: PB07 measured that a constraint calling a UDF is stored as
/// SQL text and re-parsed by every future writer, so the table becomes unwritable by any session
/// that has not registered it.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AtomTruth {
    signature: Signature,
}

impl Default for AtomTruth {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomTruth {
    pub fn new() -> Self {
        Self {
            // `any` rather than `exact`, because one argument is a `List<Utf8>` and an exact
            // signature would have to name the child field's spelling and nullability to match.
            // Types are checked at invocation instead, where a mismatch can say which argument.
            signature: Signature::any(6, Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for AtomTruth {
    fn name(&self) -> &str {
        "atom_truth"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, _arg_types: &[DataType]) -> DFResult<DataType> {
        Ok(DataType::Int8)
    }

    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> DFResult<ColumnarValue> {
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;

        // **A string column does not arrive as `Utf8` just because the table says so.** delta-rs
        // sets `datafusion.execution.parquet.schema_force_view_types = true` on the session it
        // builds, so the scan yields `Utf8View` while `assert_fixed_point` quite correctly
        // reports the stored schema as `Utf8`. The two are not in conflict -- one is storage and
        // one is the read path -- but a downcast that assumes the stored type fails at runtime,
        // which is how this was found. Casting accepts either, and `LargeUtf8` besides.
        let as_text = |i: usize, what: &str| -> DFResult<StringArray> {
            let cast = arrow::compute::cast(&arrays[i], &DataType::Utf8).map_err(|e| {
                datafusion::error::DataFusionError::Plan(format!(
                    "atom_truth argument {i} ({what}) is {:?}, which is not string-like: {e}",
                    arrays[i].data_type()
                ))
            })?;
            Ok(cast
                .as_any()
                .downcast_ref::<StringArray>()
                .expect("a cast to Utf8 produces a StringArray")
                .clone())
        };
        let facets = as_text(0, "facet")?;
        let ops = as_text(1, "op")?;
        let values = as_text(2, "value")?;
        let lists = arrays[3].as_any().downcast_ref::<ListArray>();
        let Some(negated) = arrays[4].as_any().downcast_ref::<BooleanArray>() else {
            return plan_err!("atom_truth argument 4 (negated) must be Boolean");
        };
        let bindings = as_text(5, "bindings")?;

        let mut out: Vec<Option<i8>> = Vec::with_capacity(facets.len());
        for i in 0..facets.len() {
            if facets.is_null(i) || ops.is_null(i) {
                out.push(None);
                continue;
            }
            let list: Vec<String> = match lists {
                Some(l) if l.is_valid(i) => {
                    let cell = arrow::compute::cast(&l.value(i), &DataType::Utf8).map_err(|e| {
                        datafusion::error::DataFusionError::Plan(format!(
                            "atom_truth argument 3 (value_list) must hold strings: {e}"
                        ))
                    })?;
                    let strings = cell
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .expect("a cast to Utf8 produces a StringArray");
                    (0..strings.len())
                        .filter(|j| strings.is_valid(*j))
                        .map(|j| strings.value(j).to_string())
                        .collect()
                }
                _ => Vec::new(),
            };
            let rank = evaluate(
                facets.value(i),
                ops.value(i),
                (!values.is_null(i)).then(|| values.value(i)),
                &list,
                !negated.is_null(i) && negated.value(i),
                if bindings.is_null(i) {
                    ""
                } else {
                    bindings.value(i)
                },
            );
            match rank {
                Ok(r) => out.push(Some(r)),
                Err(e) => return exec_err!("{e}"),
            }
        }
        Ok(ColumnarValue::Array(Arc::new(Int8Array::from(out))))
    }

    fn documentation(&self) -> Option<&Documentation> {
        static DOC: std::sync::OnceLock<Documentation> = std::sync::OnceLock::new();
        Some(DOC.get_or_init(|| {
            Documentation::builder(
                datafusion::logical_expr::scalar_doc_sections::DOC_SECTION_OTHER,
                "Evaluate one interaction literal against a request, as a rank in the truth \
                 lattice. A facet the request does not bind answers `unknown`, never `false`.",
                "atom_truth(facet, op, value, value_list, negated, bindings)",
            )
            .with_argument(
                "facet",
                "The request facet, option or context key being tested.",
            )
            .with_argument("op", "One of eq, ne, in, present, absent, gte, lte.")
            .with_argument("value", "The scalar compared against, for eq/ne/gte/lte.")
            .with_argument("value_list", "The domain, for `in`.")
            .with_argument(
                "negated",
                "Kleene negation: swaps true and false, keeps unknown.",
            )
            .with_argument(
                "bindings",
                "The request, as `facet=value` pairs joined by `;`.",
            )
            .build()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rank(label: &str) -> i8 {
        lattice::rank_of("truth", label).expect("a truth value")
    }

    fn eval(facet: &str, op: &str, value: Option<&str>, bindings: &str) -> i8 {
        evaluate(facet, op, value, &[], false, bindings).expect("a known operator")
    }

    #[test]
    fn an_unbound_facet_is_unknown_and_never_false() {
        assert_eq!(
            eval("engine", "eq", Some("pcre2"), "case=insensitive"),
            rank("unknown"),
            "a request that says nothing about the engine has not said the engine is not PCRE2"
        );
        // The control: bind it, and the same atom becomes a definite answer in both directions.
        assert_eq!(
            eval("engine", "eq", Some("pcre2"), "engine=pcre2"),
            rank("true")
        );
        assert_eq!(
            eval("engine", "eq", Some("pcre2"), "engine=default"),
            rank("false")
        );
    }

    /// `present` and `absent` are total. They ask about the binding rather than about its value,
    /// so an unbound facet answers them definitely -- which is why they are the two operators that
    /// do not return `unknown`.
    #[test]
    fn present_and_absent_are_total() {
        assert_eq!(
            eval("engine", "present", None, "engine=pcre2"),
            rank("true")
        );
        assert_eq!(eval("engine", "present", None, "case=on"), rank("false"));
        assert_eq!(eval("engine", "absent", None, "case=on"), rank("true"));
        assert_eq!(
            eval("engine", "absent", None, "engine=pcre2"),
            rank("false")
        );
    }

    #[test]
    fn negation_swaps_true_and_false_and_leaves_unknown_alone() {
        let neg = |bindings: &str| {
            evaluate("engine", "eq", Some("pcre2"), &[], true, bindings).expect("known")
        };
        assert_eq!(neg("engine=pcre2"), rank("false"));
        assert_eq!(neg("engine=default"), rank("true"));
        assert_eq!(
            neg("case=on"),
            rank("unknown"),
            "negating an unknown gives an unknown -- that is Kleene negation, not a missing case"
        );
    }

    #[test]
    fn in_tests_membership_of_the_domain() {
        let list = vec!["auto".to_string(), "always".to_string()];
        let run =
            |bindings: &str| evaluate("color", "in", None, &list, false, bindings).expect("known");
        assert_eq!(run("color=always"), rank("true"));
        assert_eq!(run("color=never"), rank("false"));
        assert_eq!(run("case=on"), rank("unknown"));
    }

    /// An ordered comparison that is not numeric answers `unknown` rather than falling back to
    /// byte order. Rank order and byte order are different orders -- the same fact that decided
    /// the lattice encoding -- so a lexicographic fallback would answer a different question.
    #[test]
    fn ordered_comparison_is_numeric_or_unknown() {
        assert_eq!(eval("depth", "gte", Some("3"), "depth=5"), rank("true"));
        assert_eq!(eval("depth", "lte", Some("3"), "depth=5"), rank("false"));
        assert_eq!(
            eval("depth", "gte", Some("three"), "depth=5"),
            rank("unknown")
        );
    }

    /// An operator nobody defined is an error, not an `unknown`. A typo that evaluates to "we do
    /// not know" is indistinguishable from a genuine gap, and only one of those is a bug.
    #[test]
    fn an_unknown_operator_is_an_error() {
        let err = evaluate("engine", "approximately", None, &[], false, "engine=pcre2")
            .expect_err("an undefined operator must not evaluate");
        assert!(err.contains("approximately"));
        for op in OPS {
            assert!(
                evaluate("engine", op, Some("x"), &[], false, "engine=x").is_ok(),
                "{op} is declared but does not evaluate"
            );
        }
    }

    #[test]
    fn bindings_parse_pairwise_and_tolerate_spacing() {
        assert_eq!(binding("a=1;b=2", "b"), Some("2"));
        assert_eq!(binding(" a = 1 ; b = 2 ", "a"), Some("1"));
        assert_eq!(binding("a=1", "c"), None);
        assert_eq!(binding("", "a"), None);
    }
}
