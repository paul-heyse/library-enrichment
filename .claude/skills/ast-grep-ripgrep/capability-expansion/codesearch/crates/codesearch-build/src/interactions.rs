//! Interactions: how one option changes what another does.
//!
//! # Derive a column, author a sentence
//!
//! Evidence 08 called `Interaction` "entirely new", and that is true of the *table*. It is not
//! true of all of its rows, and the split is worth being precise about:
//!
//! - **`regex.tsv.reachable` is a column.** Twenty-five of its forty-four rows say `requires -P`
//!   and one says `default-only`. Each is an engine precondition on a construct, already machine
//!   readable, and deriving it means the interaction cannot drift from the index that states it.
//! - **`unreachable.tsv.why` is a sentence.** "Both flags bound the default engine only. Under
//!   `-P` they do not constrain PCRE2 at all" is an interaction in prose, and turning it into
//!   atoms is a reading. So it is authored in [`seeds/interactions.json`], where it can be
//!   reviewed as a judgement rather than hidden inside a parser.
//!
//! Authored rows carry the same discipline the capability seed does: an interaction naming a
//! mechanism the catalog does not hold, an operator nobody defined, or an assertion that does not
//! exist is a **build error**, not a row that quietly matches nothing.

use std::collections::{BTreeMap, BTreeSet};

use codesearch_bridge::identity;
use codesearch_bridge::interaction::OPS;
use serde::Deserialize;

use crate::index::Row;

/// The interaction kinds §7.1 defines. Closed: a kind nobody defined is a typo, and a typo that
/// lands as data is a claim the catalog cannot interpret.
pub const KINDS: &[&str] = &[
    "requires",
    "enables",
    "has_no_effect_unless",
    "changes_meaning_of",
    "overrides",
    "accumulates_with",
    "conflicts_with",
    "consumes_binding_from",
    "equivalent_under",
];

/// The ordering vocabulary. `ordering_requirement` is a column rather than an assumption because
/// ast-grep rule order matters when metavariables and relational rules interact, and CLI argument
/// order can change the effective configuration. A model without it asserts a commutativity the
/// surfaces do not have.
pub const ORDERINGS: &[&str] = &["none", "before", "after", "adjacent"];

#[derive(Debug, Deserialize)]
pub struct Seed {
    #[serde(default)]
    pub interactions: Vec<Authored>,
}

#[derive(Debug, Deserialize)]
pub struct Authored {
    pub key: String,
    pub kind: String,
    pub affected: String,
    pub effect: String,
    #[serde(default = "no_ordering")]
    pub ordering_requirement: String,
    #[serde(default)]
    pub assertions: Vec<String>,
    /// Disjuncts, each a conjunction of atoms. `[[a, b], [c]]` is `(a AND b) OR c`.
    pub terms: Vec<Vec<Atom>>,
}

fn no_ordering() -> String {
    "none".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Atom {
    pub facet: String,
    pub op: String,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub value_list: Vec<String>,
    #[serde(default)]
    pub negated: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum InteractionError {
    #[error("reading {0}: {1}")]
    Io(String, String),
    #[error("parsing {0}: {1}")]
    Parse(String, String),
    #[error(
        "interaction `{key}` affects `{affected}`, which this catalog does not hold. An interaction about a mechanism that does not exist can never be evaluated, so it is refused rather than written."
    )]
    UnknownAffected { key: String, affected: String },
    #[error("interaction `{key}` uses `{value}`, which is not one of: {allowed}")]
    UnknownVocabulary {
        key: String,
        value: String,
        allowed: String,
    },
    #[error(
        "interaction `{key}` cites assertion `{assertion}`, which does not exist. Evidence handles are checked because an unresolvable one reads exactly like evidence."
    )]
    UnknownAssertion { key: String, assertion: String },
    #[error("interaction `{key}` has no terms, so it can never evaluate to anything")]
    NoTerms { key: String },
}

/// One interaction, flattened for the batch builders.
#[derive(Debug)]
pub struct Flat {
    pub key: String,
    pub kind: String,
    pub affected: String,
    pub effect: String,
    pub ordering: String,
    pub assertions: Vec<String>,
    pub terms: Vec<Vec<Atom>>,
}

pub fn load(path: &std::path::Path) -> Result<Seed, InteractionError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| InteractionError::Io(path.display().to_string(), e.to_string()))?;
    serde_json::from_str(&text)
        .map_err(|e| InteractionError::Parse(path.display().to_string(), e.to_string()))
}

/// Engine preconditions derived from `regex.tsv`'s `reachable` column.
///
/// `requires -P` and `default-only` are the same shape of fact about different engines, so both
/// become a `requires` interaction with one atom. Inventing two kinds for one relation would be a
/// distinction the data does not make.
fn derived(indexes: &BTreeMap<&'static str, Vec<Row>>) -> Vec<Flat> {
    let mut out = Vec::new();
    for row in indexes.get("regex").into_iter().flatten() {
        let construct = &row["construct"];
        let (tool, engine, why) = match row["reachable"].as_str() {
            "requires -P" => (
                "pcre2",
                "pcre2",
                "matches only when ripgrep is run with -P, which selects the PCRE2 engine",
            ),
            "default-only" => (
                "rg",
                "default",
                "matches only under the default Rust regex engine; -P selects PCRE2, which does \
                 not support it",
            ),
            // `yes` needs no precondition and `no` is negative space, not an interaction.
            _ => continue,
        };
        let affected = identity::mechanism_key(tool, "pattern", construct);
        out.push(Flat {
            key: format!("ix:engine/{construct}"),
            kind: "requires".to_string(),
            affected,
            effect: why.to_string(),
            ordering: "none".to_string(),
            assertions: Vec::new(),
            terms: vec![vec![Atom {
                facet: "engine".to_string(),
                op: "eq".to_string(),
                value: Some(engine.to_string()),
                value_list: Vec::new(),
                negated: false,
            }]],
        });
    }
    out
}

/// Every interaction, derived and authored, validated against what the catalog actually holds.
pub fn collect(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    seed: &Seed,
    mechanism_keys: &BTreeSet<String>,
    assertion_keys: &BTreeSet<String>,
) -> Result<Vec<Flat>, InteractionError> {
    let mut out = derived(indexes);

    for a in &seed.interactions {
        if !KINDS.contains(&a.kind.as_str()) {
            return Err(InteractionError::UnknownVocabulary {
                key: a.key.clone(),
                value: a.kind.clone(),
                allowed: KINDS.join(", "),
            });
        }
        if !ORDERINGS.contains(&a.ordering_requirement.as_str()) {
            return Err(InteractionError::UnknownVocabulary {
                key: a.key.clone(),
                value: a.ordering_requirement.clone(),
                allowed: ORDERINGS.join(", "),
            });
        }
        if !mechanism_keys.contains(&a.affected) {
            return Err(InteractionError::UnknownAffected {
                key: a.key.clone(),
                affected: a.affected.clone(),
            });
        }
        for assertion in &a.assertions {
            if !assertion_keys.contains(assertion) {
                return Err(InteractionError::UnknownAssertion {
                    key: a.key.clone(),
                    assertion: assertion.clone(),
                });
            }
        }
        if a.terms.is_empty() || a.terms.iter().all(Vec::is_empty) {
            return Err(InteractionError::NoTerms { key: a.key.clone() });
        }
        for term in &a.terms {
            for atom in term {
                if !OPS.contains(&atom.op.as_str()) {
                    return Err(InteractionError::UnknownVocabulary {
                        key: a.key.clone(),
                        value: atom.op.clone(),
                        allowed: OPS.join(", "),
                    });
                }
            }
        }
        out.push(Flat {
            key: a.key.clone(),
            kind: a.kind.clone(),
            affected: a.affected.clone(),
            effect: a.effect.clone(),
            ordering: a.ordering_requirement.clone(),
            assertions: a.assertions.clone(),
            terms: a.terms.clone(),
        });
    }

    // Derived rows are validated too. The derivation mints a key from a mechanism it expects to
    // exist, and a regex construct present in the index but absent from the mechanism table would
    // mean the two families had drifted -- which is exactly the class of silent disagreement the
    // whole catalog is built to expose.
    for flat in &out {
        if !mechanism_keys.contains(&flat.affected) {
            return Err(InteractionError::UnknownAffected {
                key: flat.key.clone(),
                affected: flat.affected.clone(),
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn indexes_with(reachable: &str) -> BTreeMap<&'static str, Vec<Row>> {
        let mut row = Row::new();
        row.insert("construct", "lookbehind".to_string());
        row.insert("reachable", reachable.to_string());
        BTreeMap::from([("regex", vec![row])])
    }

    #[test]
    fn requires_p_derives_an_engine_precondition() {
        let flat = derived(&indexes_with("requires -P"));
        assert_eq!(flat.len(), 1);
        assert_eq!(flat[0].kind, "requires");
        assert_eq!(flat[0].affected, "mech:pcre2/pattern/lookbehind");
        assert_eq!(flat[0].terms[0][0].value.as_deref(), Some("pcre2"));
    }

    /// The other engine, same shape. Two kinds for one relation would be a distinction the index
    /// does not make.
    #[test]
    fn default_only_derives_the_same_shape_for_the_other_engine() {
        let flat = derived(&indexes_with("default-only"));
        assert_eq!(flat.len(), 1);
        assert_eq!(flat[0].kind, "requires");
        assert_eq!(flat[0].affected, "mech:rg/pattern/lookbehind");
        assert_eq!(flat[0].terms[0][0].value.as_deref(), Some("default"));
    }

    /// A construct needing no precondition yields no interaction, and one reachable in neither
    /// engine is negative space rather than an interaction. Without this, `derived` would emit a
    /// vacuous row for every construct in the index.
    #[test]
    fn unconditional_and_unreachable_constructs_yield_nothing() {
        assert!(derived(&indexes_with("yes")).is_empty());
        assert!(derived(&indexes_with("no")).is_empty());
    }

    fn seed_with(atom: Atom, affected: &str) -> Seed {
        Seed {
            interactions: vec![Authored {
                key: "ix:test".into(),
                kind: "requires".into(),
                affected: affected.into(),
                effect: "e".into(),
                ordering_requirement: "none".into(),
                assertions: Vec::new(),
                terms: vec![vec![atom]],
            }],
        }
    }

    fn ok_atom() -> Atom {
        Atom {
            facet: "engine".into(),
            op: "eq".into(),
            value: Some("pcre2".into()),
            value_list: Vec::new(),
            negated: false,
        }
    }

    #[test]
    fn an_authored_interaction_about_an_unknown_mechanism_is_refused() {
        let mechanisms = BTreeSet::from(["mech:rg/cli/--pcre2".to_string()]);
        let err = collect(
            &BTreeMap::new(),
            &seed_with(ok_atom(), "mech:rg/cli/--invented"),
            &mechanisms,
            &BTreeSet::new(),
        )
        .expect_err("an interaction about nothing must be refused");
        assert!(matches!(err, InteractionError::UnknownAffected { .. }));

        // The control: the same seed against a mechanism that exists is accepted, so the refusal
        // above is attributable to the missing mechanism rather than to the validation itself.
        assert!(
            collect(
                &BTreeMap::new(),
                &seed_with(ok_atom(), "mech:rg/cli/--pcre2"),
                &mechanisms,
                &BTreeSet::new(),
            )
            .is_ok()
        );
    }

    #[test]
    fn an_undefined_operator_is_refused() {
        let mechanisms = BTreeSet::from(["mech:rg/cli/--pcre2".to_string()]);
        let mut atom = ok_atom();
        atom.op = "approximately".into();
        let err = collect(
            &BTreeMap::new(),
            &seed_with(atom, "mech:rg/cli/--pcre2"),
            &mechanisms,
            &BTreeSet::new(),
        )
        .expect_err("an undefined operator must be refused");
        assert!(matches!(err, InteractionError::UnknownVocabulary { .. }));
    }

    #[test]
    fn a_cited_assertion_must_exist() {
        let mechanisms = BTreeSet::from(["mech:rg/cli/--pcre2".to_string()]);
        let mut seed = seed_with(ok_atom(), "mech:rg/cli/--pcre2");
        seed.interactions[0].assertions = vec!["assert:rg/NOPE".into()];
        let err = collect(
            &BTreeMap::new(),
            &seed,
            &mechanisms,
            &BTreeSet::from(["assert:rg/A001".to_string()]),
        )
        .expect_err("an unresolvable evidence handle must be refused");
        assert!(matches!(err, InteractionError::UnknownAssertion { .. }));

        seed.interactions[0].assertions = vec!["assert:rg/A001".into()];
        assert!(
            collect(
                &BTreeMap::new(),
                &seed,
                &mechanisms,
                &BTreeSet::from(["assert:rg/A001".to_string()]),
            )
            .is_ok()
        );
    }

    #[test]
    fn an_interaction_with_no_terms_is_refused() {
        let mechanisms = BTreeSet::from(["mech:rg/cli/--pcre2".to_string()]);
        let mut seed = seed_with(ok_atom(), "mech:rg/cli/--pcre2");
        seed.interactions[0].terms = Vec::new();
        let err = collect(&BTreeMap::new(), &seed, &mechanisms, &BTreeSet::new())
            .expect_err("a predicate with no literals evaluates to nothing");
        assert!(matches!(err, InteractionError::NoTerms { .. }));
    }
}
