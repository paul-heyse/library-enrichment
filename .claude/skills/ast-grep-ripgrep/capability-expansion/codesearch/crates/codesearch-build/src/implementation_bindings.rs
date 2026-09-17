//! The authored join between the capability catalog and the Rust program model.
//!
//! §3.8 calls `catalog.surface_binding` *"the entire answer to how the capability catalog and the
//! Rust program model meet"*, and because both halves mint keys in one grammar (§1.2) it is an
//! ordinary join rather than a bridge. What it is not is **derivable**: nothing the skill ships
//! links a flag to a crate. So these rows are authored, and validated the way every authored seed
//! in this build is -- a binding naming a key the catalog does not hold fails the build.
//!
//! # The roles are not decoration
//!
//! §3.8 gives a role vocabulary, and two of its entries pull apart a distinction that
//! `implemented_by` alone would flatten. `--no-ignore` is not *implemented by* the `ignore` crate;
//! it switches off filtering that crate would otherwise apply. `configured_through` says that, and
//! a reader who needs to know where the behaviour lives is better served by the difference than by
//! a uniform edge.

use std::collections::BTreeSet;

use serde::Deserialize;

/// The roles §3.8 enumerates. Closed, so a typo is an error rather than a new relationship nobody
/// defined.
pub const ROLES: &[&str] = &[
    "declared_by",
    "parsed_by",
    "validated_by",
    "stored_in",
    "normalized_by",
    "configured_through",
    "implemented_by",
    "reported_by",
];

/// What a binding may point at on the object side.
pub const OBJECT_KINDS: &[&str] = &["definition", "package"];

#[derive(Debug, Deserialize)]
pub struct Seed {
    #[serde(default)]
    pub bindings: Vec<Authored>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Authored {
    pub mechanism: String,
    pub object: String,
    pub object_kind: String,
    pub role: String,
    pub precision: String,
    pub observed: String,
    pub evidence_note: String,
}

#[derive(Debug, thiserror::Error)]
pub enum BindingError {
    #[error("reading {0}: {1}")]
    Io(String, String),
    #[error("parsing {0}: {1}")]
    Parse(String, String),
    #[error("binding `{mechanism}` -> `{object}` uses `{value}`, which is not one of: {allowed}")]
    UnknownVocabulary {
        mechanism: String,
        object: String,
        value: String,
        allowed: String,
    },
    #[error(
        "binding names mechanism `{0}`, which this catalog does not hold. A binding to a mechanism nobody can invoke joins to nothing and reads exactly like one that does."
    )]
    UnknownMechanism(String),
    #[error(
        "binding names `{object}`, which no `program.{kind}` row holds. The program model is built from the same skill in the same run, so an unresolvable object here means the key rule and the seed have drifted apart."
    )]
    UnknownObject { object: String, kind: String },
}

pub fn load(path: &std::path::Path) -> Result<Seed, BindingError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| BindingError::Io(path.display().to_string(), e.to_string()))?;
    serde_json::from_str(&text)
        .map_err(|e| BindingError::Parse(path.display().to_string(), e.to_string()))
}

/// Validate every binding against what both halves of the catalog actually hold.
///
/// Both sides, deliberately. Checking only the mechanism would let a binding point at a definition
/// key that never existed and still build -- and since the object side is the *new* half, that is
/// the side most likely to drift.
pub fn validate(
    seed: &Seed,
    mechanism_keys: &BTreeSet<String>,
    definition_keys: &BTreeSet<String>,
    package_keys: &BTreeSet<String>,
) -> Result<Vec<Authored>, BindingError> {
    for b in &seed.bindings {
        for (value, allowed) in [
            (&b.role, ROLES),
            (&b.object_kind, OBJECT_KINDS),
            (&b.precision, &["absent", "inexact", "exact"] as &[&str]),
            (
                &b.observed,
                &["not-probed", "unknown", "recorded", "confirmed"] as &[&str],
            ),
        ] {
            if !allowed.contains(&value.as_str()) {
                return Err(BindingError::UnknownVocabulary {
                    mechanism: b.mechanism.clone(),
                    object: b.object.clone(),
                    value: value.clone(),
                    allowed: allowed.join(", "),
                });
            }
        }
        if !mechanism_keys.contains(&b.mechanism) {
            return Err(BindingError::UnknownMechanism(b.mechanism.clone()));
        }
        let held = match b.object_kind.as_str() {
            "definition" => definition_keys,
            _ => package_keys,
        };
        if !held.contains(&b.object) {
            return Err(BindingError::UnknownObject {
                object: b.object.clone(),
                kind: b.object_kind.clone(),
            });
        }
    }
    Ok(seed.bindings.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding(mechanism: &str, object: &str, kind: &str, role: &str) -> Authored {
        Authored {
            mechanism: mechanism.into(),
            object: object.into(),
            object_kind: kind.into(),
            role: role.into(),
            precision: "inexact".into(),
            observed: "not-probed".into(),
            evidence_note: "test".into(),
        }
    }

    fn held() -> (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>) {
        (
            BTreeSet::from(["mech:rg/cli/--pcre2".to_string()]),
            BTreeSet::from(["def:grep-pcre2#grep_pcre2::matcher::RegexMatcher".to_string()]),
            BTreeSet::from(["pkg:grep-pcre2@0.1.10".to_string()]),
        )
    }

    /// The rule this module exists for: BOTH ends must resolve.
    #[test]
    fn a_binding_naming_a_key_the_catalog_does_not_hold_is_refused() {
        let (m, d, p) = held();

        let good = Seed {
            bindings: vec![binding(
                "mech:rg/cli/--pcre2",
                "pkg:grep-pcre2@0.1.10",
                "package",
                "implemented_by",
            )],
        };
        assert!(validate(&good, &m, &d, &p).is_ok(), "the control must pass");

        let bad_subject = Seed {
            bindings: vec![binding(
                "mech:rg/cli/--nope",
                "pkg:grep-pcre2@0.1.10",
                "package",
                "implemented_by",
            )],
        };
        assert!(matches!(
            validate(&bad_subject, &m, &d, &p),
            Err(BindingError::UnknownMechanism(_))
        ));

        let bad_object = Seed {
            bindings: vec![binding(
                "mech:rg/cli/--pcre2",
                "pkg:nope@1.0.0",
                "package",
                "implemented_by",
            )],
        };
        assert!(matches!(
            validate(&bad_object, &m, &d, &p),
            Err(BindingError::UnknownObject { .. })
        ));
    }

    /// A definition key checked against the package set would resolve to nothing even when it is
    /// correct, so the kind has to select which set is consulted.
    #[test]
    fn the_object_kind_selects_which_half_is_checked() {
        let (m, d, p) = held();
        let mismatched = Seed {
            bindings: vec![binding(
                "mech:rg/cli/--pcre2",
                "def:grep-pcre2#grep_pcre2::matcher::RegexMatcher",
                "package",
                "implemented_by",
            )],
        };
        assert!(matches!(
            validate(&mismatched, &m, &d, &p),
            Err(BindingError::UnknownObject { .. })
        ));

        let correct = Seed {
            bindings: vec![binding(
                "mech:rg/cli/--pcre2",
                "def:grep-pcre2#grep_pcre2::matcher::RegexMatcher",
                "definition",
                "implemented_by",
            )],
        };
        assert!(validate(&correct, &m, &d, &p).is_ok());
    }

    #[test]
    fn a_role_outside_the_vocabulary_is_refused() {
        let (m, d, p) = held();
        let seed = Seed {
            bindings: vec![binding(
                "mech:rg/cli/--pcre2",
                "pkg:grep-pcre2@0.1.10",
                "package",
                "vibes_from",
            )],
        };
        assert!(matches!(
            validate(&seed, &m, &d, &p),
            Err(BindingError::UnknownVocabulary { .. })
        ));
    }
}
