//! Plan fragments: composable steps, and the evidence a recall claim needs.
//!
//! # `preserving` requires a confirmed assertion
//!
//! This is the rule the module exists for. `preserving` asserts
//! `final_match(x) implies prefilter_selects(x)` -- a claim about behaviour that cannot be
//! established by inspecting a fragment's input and output types, however suggestive they look.
//! So it must be carried by a `BehaviorAssertion`, and specifically by a **`confirmed`** one:
//! a probe that ran and whose control came out the other way.
//!
//! A `recorded` verdict is the tool's own word for it. That is enough to catalogue a mechanism and
//! not enough to license a recall guarantee, because the failure it would hide -- a chain quietly
//! dropping candidates -- is the exact failure the lattice exists to surface.
//!
//! # Assertions are also cited by heuristic fragments
//!
//! Not every citation supports a claim; some establish a limit. `rg --json` is `heuristic` because
//! probes P022 and P024 **confirm** that ignored and hidden files are skipped by default, so the
//! step does not see every file it was nominally handed. The evidence is what makes the honest
//! value heuristic rather than what excuses it.

use std::collections::BTreeSet;

use serde::Deserialize;

/// The five typed units a fragment moves between. Closed: a unit nobody defined would join to
/// nothing, which reads exactly like a chain that legitimately cannot be composed.
pub const UNITS: &[&str] = &[
    "file_set",
    "file_content",
    "line_set",
    "node_set",
    "capture_set",
];

/// What a fragment leaves behind for the next step.
pub const COORDINATES: &[&str] = &["preserved", "remapped", "lost"];

/// What a fragment needs from the step before it.
///
/// `complete_enclosing_syntax` is the proposal's warning made into a value: a structural stage
/// cannot run on fragments of lines, so it does not join to anything that lost or remapped its
/// coordinates.
pub const SCOPES: &[&str] = &[
    "none",
    "whole_file",
    "complete_enclosing_syntax",
    "original_coordinates",
];

#[derive(Debug, Deserialize)]
pub struct Seed {
    #[serde(default)]
    pub fragments: Vec<Authored>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Authored {
    pub key: String,
    pub name: String,
    pub accepted_input: String,
    pub produced_output: String,
    pub scope_assumption: String,
    pub coordinate_preservation: String,
    pub recall: String,
    #[serde(default = "no_ordering")]
    pub ordering_requirement: String,
    #[serde(default)]
    pub mechanisms: Vec<String>,
    #[serde(default)]
    pub assertions: Vec<String>,
    #[serde(default)]
    pub unresolved_obligations: Vec<String>,
}

fn no_ordering() -> String {
    "none".to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum FragmentError {
    #[error("reading {0}: {1}")]
    Io(String, String),
    #[error("parsing {0}: {1}")]
    Parse(String, String),
    #[error("fragment `{key}` uses `{value}`, which is not one of: {allowed}")]
    UnknownVocabulary {
        key: String,
        value: String,
        allowed: String,
    },
    #[error(
        "fragment `{key}` names mechanism `{mechanism}`, which this catalog does not hold. A step nobody can invoke is not a plan."
    )]
    UnknownMechanism { key: String, mechanism: String },
    #[error(
        "fragment `{key}` cites assertion `{assertion}`, which does not exist. An unresolvable evidence handle reads exactly like evidence."
    )]
    UnknownAssertion { key: String, assertion: String },
    #[error(
        "fragment `{key}` claims recall `preserving` with no CONFIRMED assertion behind it. `preserving` asserts that everything the final step would have matched survives this one, which is a claim about behaviour -- it cannot be read off the fragment's input and output types. Cite a confirmed probe, or say `heuristic` and put the condition in `unresolved_obligations`."
    )]
    UnbackedPreserving { key: String },
}

pub fn load(path: &std::path::Path) -> Result<Seed, FragmentError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| FragmentError::Io(path.display().to_string(), e.to_string()))?;
    serde_json::from_str(&text)
        .map_err(|e| FragmentError::Parse(path.display().to_string(), e.to_string()))
}

fn check(key: &str, value: &str, allowed: &[&str]) -> Result<(), FragmentError> {
    if allowed.contains(&value) {
        return Ok(());
    }
    Err(FragmentError::UnknownVocabulary {
        key: key.to_string(),
        value: value.to_string(),
        allowed: allowed.join(", "),
    })
}

/// Validate every fragment against what the catalog actually holds.
///
/// `confirmed_assertions` is deliberately a separate set from `assertion_keys`: a citation must
/// resolve to *some* assertion, and a `preserving` claim must resolve to a **confirmed** one.
pub fn validate(
    seed: &Seed,
    mechanism_keys: &BTreeSet<String>,
    assertion_keys: &BTreeSet<String>,
    confirmed_assertions: &BTreeSet<String>,
) -> Result<Vec<Authored>, FragmentError> {
    for f in &seed.fragments {
        check(&f.key, &f.accepted_input, UNITS)?;
        check(&f.key, &f.produced_output, UNITS)?;
        check(&f.key, &f.scope_assumption, SCOPES)?;
        check(&f.key, &f.coordinate_preservation, COORDINATES)?;
        check(&f.key, &f.recall, &["unknown", "heuristic", "preserving"])?;
        check(
            &f.key,
            &f.ordering_requirement,
            &["none", "before", "after", "adjacent"],
        )?;

        for mechanism in &f.mechanisms {
            if !mechanism_keys.contains(mechanism) {
                return Err(FragmentError::UnknownMechanism {
                    key: f.key.clone(),
                    mechanism: mechanism.clone(),
                });
            }
        }
        for assertion in &f.assertions {
            if !assertion_keys.contains(assertion) {
                return Err(FragmentError::UnknownAssertion {
                    key: f.key.clone(),
                    assertion: assertion.clone(),
                });
            }
        }
        if f.recall == "preserving"
            && !f
                .assertions
                .iter()
                .any(|a| confirmed_assertions.contains(a))
        {
            return Err(FragmentError::UnbackedPreserving { key: f.key.clone() });
        }
    }
    Ok(seed.fragments.clone())
}

// The composition rule lives in the view, and only there. An earlier draft of this module carried
// a Rust `may_precede` mirroring the join condition in `projection.fragment_edge`; it is gone.
// Two statements of one rule is the drift this design exists to prevent, and the SQL is the copy
// that decides what a caller sees -- so the tests for the rule live against the view, in
// `codesearch-model/tests/projections.rs`, where they exercise the thing that runs.

#[cfg(test)]
mod tests {
    use super::*;

    fn fragment(key: &str, recall: &str, assertions: &[&str]) -> Authored {
        Authored {
            key: key.into(),
            name: key.into(),
            accepted_input: "file_set".into(),
            produced_output: "node_set".into(),
            scope_assumption: "none".into(),
            coordinate_preservation: "preserved".into(),
            recall: recall.into(),
            ordering_requirement: "none".into(),
            mechanisms: Vec::new(),
            assertions: assertions.iter().map(|s| (*s).to_string()).collect(),
            unresolved_obligations: Vec::new(),
        }
    }

    fn sets() -> (BTreeSet<String>, BTreeSet<String>) {
        (
            BTreeSet::from(["assert:sg/A001".to_string(), "assert:rg/P030".to_string()]),
            BTreeSet::from(["assert:sg/A001".to_string()]),
        )
    }

    /// The rule this module exists for.
    #[test]
    fn preserving_needs_a_confirmed_assertion_not_merely_any_assertion() {
        let (all, confirmed) = sets();
        let seed = Seed {
            fragments: vec![fragment("plan:x", "preserving", &["assert:rg/P030"])],
        };
        let err = validate(&seed, &BTreeSet::new(), &all, &confirmed)
            .expect_err("a recorded assertion must not license a preserving claim");
        assert!(matches!(err, FragmentError::UnbackedPreserving { .. }));

        // Two controls, so the refusal is attributable to the verdict and not to the validation.
        let with_confirmed = Seed {
            fragments: vec![fragment("plan:x", "preserving", &["assert:sg/A001"])],
        };
        assert!(validate(&with_confirmed, &BTreeSet::new(), &all, &confirmed).is_ok());

        let as_heuristic = Seed {
            fragments: vec![fragment("plan:x", "heuristic", &["assert:rg/P030"])],
        };
        assert!(
            validate(&as_heuristic, &BTreeSet::new(), &all, &confirmed).is_ok(),
            "a heuristic fragment may cite a recorded assertion -- as evidence of its limit"
        );
    }

    #[test]
    fn a_preserving_claim_with_no_evidence_at_all_is_refused() {
        let (all, confirmed) = sets();
        let seed = Seed {
            fragments: vec![fragment("plan:x", "preserving", &[])],
        };
        assert!(matches!(
            validate(&seed, &BTreeSet::new(), &all, &confirmed),
            Err(FragmentError::UnbackedPreserving { .. })
        ));
    }

    #[test]
    fn a_cited_assertion_must_exist() {
        let (all, confirmed) = sets();
        let seed = Seed {
            fragments: vec![fragment("plan:x", "heuristic", &["assert:rg/NOPE"])],
        };
        assert!(matches!(
            validate(&seed, &BTreeSet::new(), &all, &confirmed),
            Err(FragmentError::UnknownAssertion { .. })
        ));
    }

    #[test]
    fn an_undefined_unit_is_refused() {
        let (all, confirmed) = sets();
        let mut f = fragment("plan:x", "heuristic", &[]);
        f.produced_output = "vibes".into();
        let seed = Seed { fragments: vec![f] };
        assert!(matches!(
            validate(&seed, &BTreeSet::new(), &all, &confirmed),
            Err(FragmentError::UnknownVocabulary { .. })
        ));
    }
}
