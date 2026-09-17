//! The six epistemic lattices, and the one piece of algebra the whole catalog rests on.
//!
//! Six dimensions that look unrelated in prose turn out to have identical algebra:
//!
//! | lattice      | lowest to highest                                     | asks                          |
//! |--------------|-------------------------------------------------------|-------------------------------|
//! | `precision`  | absent, inexact, exact                                | how well is this fact known?  |
//! | `truth`      | false, unknown, true                                  | does this condition hold?     |
//! | `recall`     | unknown, heuristic, preserving                        | does this step keep all it should? |
//! | `binding`    | unbound, ambiguous, candidate, bound                  | is this native handle mapped? |
//! | `observed`   | not-probed, unknown, recorded, confirmed              | was this checked, and how?    |
//! | `completion` | abandoned, running, partial, complete                 | did the run that wrote this finish? |
//!
//! Composition is `min` and alternation is `max`, for every one of them. Kleene AND is `min`;
//! Kleene OR is `max`; a derivation's precision is the `min` of its inputs'; a plan-fragment
//! chain's recall is the `min` of its steps'. One idiom, five problems, and it is plain SQL:
//!
//! ```sql
//! SELECT parent_id, min(precision_rank) AS precision_rank FROM child GROUP BY parent_id
//! ```
//!
//! # Why the rank is what is stored
//!
//! Each column holds an `Int8` ordinal -- `precision_rank`, not `precision`. Labels are rendered
//! at the projection edge by [`LatticeLabel`], and the mapping lives in the `catalog.lattice`
//! reference table so it is queryable data rather than code.
//!
//! Storing the label would be more legible in raw Parquet and would be wrong, because rank order
//! and lexicographic order are different orders:
//!
//! ```text
//! precision by rank:  absent(0) < inexact(1) < exact(2)
//! precision by bytes: absent    < exact      < inexact
//! ```
//!
//! "precision at least inexact" is a contiguous range over ranks and not reliably one over bytes.
//! It happens to come out contiguous for all five value sets below -- but renaming `exact` to
//! `certain`, or adding a value, would silently stop a filter pruning, with no error and no change
//! to the query plan. Probe PB06 exists because that class of silent optimisation loss is real, so
//! the design does not rely on the coincidence.
//!
//! Storing the ordinal makes every lattice filter a native integer comparison that prunes on Delta
//! min/max statistics with no UDF and no `preimage` at all.
//!
//! # `not-probed` is not `unknown`
//!
//! The `observed` lattice keeps them apart deliberately: nothing was attempted, versus something
//! was attempted and did not settle. The surrounding skill's own reference says of these values
//! that "nothing should be inferred from them in either direction", and a consumer that maps
//! either to `false` destroys the property the catalog exists to provide.

use std::sync::Arc;

use arrow::array::{Int8Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use datafusion::common::Result as DFResult;
use datafusion::common::cast::{as_int8_array, as_string_array};
use datafusion::logical_expr::{
    ColumnarValue, Documentation, ScalarFunctionArgs, ScalarUDFImpl, Signature, Volatility,
};

/// One lattice: a name, and its values ordered lowest to highest.
///
/// The index of a value in `values` **is** its rank. There is no separate rank field, so the
/// ordering and the encoding cannot drift apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LatticeDef {
    pub name: &'static str,
    pub values: &'static [&'static str],
    pub question: &'static str,
}

impl LatticeDef {
    /// The rank of a label, or `None` if the label is not in this lattice.
    pub fn rank_of(&self, label: &str) -> Option<i8> {
        self.values
            .iter()
            .position(|v| *v == label)
            .map(|i| i as i8)
    }

    /// The label for a rank, or `None` if the rank is out of range.
    pub fn label_of(&self, rank: i8) -> Option<&'static str> {
        usize::try_from(rank)
            .ok()
            .and_then(|i| self.values.get(i))
            .copied()
    }

    /// The highest valid rank. This is the upper bound of the CHECK constraint the table carries,
    /// which is why it is derived here rather than written out per table.
    pub fn max_rank(&self) -> i8 {
        (self.values.len() - 1) as i8
    }
}

/// Every lattice in the model. Adding one here is the whole cost of adding a dimension: the fold,
/// the constraint bound and the reference table all follow.
pub const LATTICES: &[LatticeDef] = &[
    LatticeDef {
        name: "precision",
        values: &["absent", "inexact", "exact"],
        question: "how well is this fact known?",
    },
    LatticeDef {
        name: "truth",
        values: &["false", "unknown", "true"],
        question: "does this condition hold?",
    },
    LatticeDef {
        name: "recall",
        values: &["unknown", "heuristic", "preserving"],
        question: "does this step keep everything it should?",
    },
    LatticeDef {
        name: "binding",
        values: &["unbound", "ambiguous", "candidate", "bound"],
        question: "is this native handle mapped to a canonical entity?",
    },
    LatticeDef {
        name: "observed",
        values: &["not-probed", "unknown", "recorded", "confirmed"],
        question: "was this checked, and how?",
    },
    // `completion` is the sixth, and the one that is not about a FACT but about the RUN that
    // produced one. §4.1 already called `completion_status` a lattice; making it one costs
    // nothing, because the `Int8` encoding, the derived range constraint and the
    // `catalog.lattice` publication all apply without a line of new machinery.
    //
    // The ordering is the point. `abandoned` is the bottom: a run reaped by a later build is
    // worth strictly less than one still running, because the still-running one may yet finish.
    // `complete` is the top, so "at the top of the completion lattice" is what
    // `projection.visible_run` means, and it asks the reference table rather than a literal.
    //
    // `min` and `max` mean what they mean everywhere else here: the completion of a set of runs
    // taken together is the `min` of theirs -- one abandoned pass makes the whole extraction
    // abandoned, which is exactly the fold every other lattice already performs.
    LatticeDef {
        name: "completion",
        values: &["abandoned", "running", "partial", "complete"],
        question: "did the run that wrote this row finish?",
    },
];

/// Look a lattice up by name.
pub fn lattice(name: &str) -> Option<&'static LatticeDef> {
    LATTICES.iter().find(|l| l.name == name)
}

/// The rank of a label within a named lattice.
pub fn rank_of(lattice_name: &str, label: &str) -> Option<i8> {
    lattice(lattice_name).and_then(|l| l.rank_of(label))
}

/// The label for a rank within a named lattice.
pub fn label_of(lattice_name: &str, rank: i8) -> Option<&'static str> {
    lattice(lattice_name).and_then(|l| l.label_of(rank))
}

/// Schema of the `catalog.lattice` reference table.
pub fn reference_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("lattice", DataType::Utf8, false),
        Field::new("rank", DataType::Int8, false),
        Field::new("label", DataType::Utf8, false),
        Field::new("question", DataType::Utf8, false),
    ]))
}

/// The `catalog.lattice` rows. Publishing the mapping as data is what keeps the stored ordinals
/// interpretable by anything that can query the catalog, including a reader who has never seen
/// this source file.
pub fn reference_batch() -> DFResult<RecordBatch> {
    let mut names = Vec::new();
    let mut ranks = Vec::new();
    let mut labels = Vec::new();
    let mut questions = Vec::new();
    for def in LATTICES {
        for (rank, label) in def.values.iter().enumerate() {
            names.push(def.name);
            ranks.push(rank as i8);
            labels.push(*label);
            questions.push(def.question);
        }
    }
    Ok(RecordBatch::try_new(
        reference_schema(),
        vec![
            Arc::new(StringArray::from(names)),
            Arc::new(Int8Array::from(ranks)),
            Arc::new(StringArray::from(labels)),
            Arc::new(StringArray::from(questions)),
        ],
    )?)
}

/// `lattice_label(lattice_name, rank) -> Utf8` -- renders a stored ordinal for display.
///
/// This is the only lattice function, and it belongs in projections, never in predicates. A
/// filter compares the stored `Int8` directly; putting a function between the column and the
/// comparison is what would cost the pruning this encoding exists to keep.
///
/// An out-of-range rank renders as NULL rather than erroring: the CHECK constraint on every
/// lattice column is what makes that unreachable for stored data, and a display function is the
/// wrong place to discover a violated invariant.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LatticeLabel {
    signature: Signature,
}

impl Default for LatticeLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl LatticeLabel {
    pub fn new() -> Self {
        Self {
            signature: Signature::exact(
                vec![DataType::Utf8, DataType::Int8],
                Volatility::Immutable,
            ),
        }
    }
}

impl ScalarUDFImpl for LatticeLabel {
    fn name(&self) -> &str {
        "lattice_label"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, _arg_types: &[DataType]) -> DFResult<DataType> {
        Ok(DataType::Utf8)
    }

    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> DFResult<ColumnarValue> {
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;
        let names = as_string_array(&arrays[0])?;
        let ranks = as_int8_array(&arrays[1])?;
        let out: StringArray = names
            .iter()
            .zip(ranks.iter())
            .map(|(name, rank)| match (name, rank) {
                (Some(name), Some(rank)) => label_of(name, rank),
                _ => None,
            })
            .collect();
        Ok(ColumnarValue::Array(Arc::new(out)))
    }

    fn documentation(&self) -> Option<&Documentation> {
        static DOC: std::sync::OnceLock<Documentation> = std::sync::OnceLock::new();
        Some(DOC.get_or_init(|| {
            Documentation::builder(
                datafusion::logical_expr::scalar_doc_sections::DOC_SECTION_OTHER,
                "Render a stored lattice ordinal as its label. Display only -- filters compare \
                 the stored Int8 rank directly, which prunes without a function call.",
                "lattice_label('precision', precision_rank)",
            )
            .with_argument("lattice_name", "One of the names in catalog.lattice.")
            .with_argument("rank", "The stored Int8 ordinal.")
            .build()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_and_label_round_trip_for_every_value_of_every_lattice() {
        for def in LATTICES {
            for (expected_rank, label) in def.values.iter().enumerate() {
                let rank = def.rank_of(label).expect("every listed value has a rank");
                assert_eq!(rank, expected_rank as i8, "{}::{label}", def.name);
                assert_eq!(def.label_of(rank), Some(*label));
            }
        }
    }

    #[test]
    fn every_lattice_is_ordered_lowest_to_highest() {
        // The invariant the whole design leans on: index == rank, and the weakest value is 0 so
        // that `coalesce(rank, 0)` is the correct way to treat a missing fact.
        for def in LATTICES {
            assert!(
                def.values.len() >= 2,
                "{} needs at least two values",
                def.name
            );
            assert_eq!(def.rank_of(def.values[0]), Some(0));
            assert_eq!(def.max_rank(), (def.values.len() - 1) as i8);
        }
    }

    #[test]
    fn unknown_names_and_out_of_range_ranks_are_none_not_panics() {
        assert_eq!(rank_of("precision", "definitely-not-a-value"), None);
        assert_eq!(rank_of("not-a-lattice", "exact"), None);
        assert_eq!(label_of("precision", 99), None);
        assert_eq!(label_of("precision", -1), None);
    }

    /// The Kleene case, spelled out because it is the one most likely to be "simplified" later.
    /// An unknown conjunct must not be silently treated as false.
    #[test]
    fn kleene_and_is_min_and_or_is_max() {
        let f = rank_of("truth", "false").unwrap();
        let u = rank_of("truth", "unknown").unwrap();
        let t = rank_of("truth", "true").unwrap();
        assert!(f < u && u < t);

        // AND = min
        assert_eq!(t.min(u), u, "true AND unknown is unknown, not true");
        assert_eq!(u.min(f), f, "unknown AND false is false");
        assert_eq!(t.min(t), t);

        // OR = max
        assert_eq!(f.max(u), u, "false OR unknown is unknown, not false");
        assert_eq!(u.max(t), t, "unknown OR true is true");
        assert_eq!(f.max(f), f);
    }

    /// `not-probed` and `unknown` are different facts and must not collapse.
    #[test]
    fn observed_keeps_not_probed_below_unknown() {
        let not_probed = rank_of("observed", "not-probed").unwrap();
        let unknown = rank_of("observed", "unknown").unwrap();
        let recorded = rank_of("observed", "recorded").unwrap();
        let confirmed = rank_of("observed", "confirmed").unwrap();
        assert!(not_probed < unknown);
        assert!(unknown < recorded);
        assert!(recorded < confirmed);
    }

    /// A chain is as good as its weakest link -- the composition rule, over a mixed set.
    #[test]
    fn composition_takes_the_weakest_link() {
        let ranks = ["exact", "inexact", "exact"]
            .iter()
            .map(|l| rank_of("precision", l).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            ranks.iter().copied().min(),
            rank_of("precision", "inexact"),
            "one inexact input makes the whole derivation inexact"
        );
    }

    #[test]
    fn reference_batch_has_a_row_per_value() {
        let batch = reference_batch().expect("reference batch builds");
        let expected: usize = LATTICES.iter().map(|l| l.values.len()).sum();
        assert_eq!(batch.num_rows(), expected);
        assert_eq!(batch.num_columns(), 4);
    }
}
