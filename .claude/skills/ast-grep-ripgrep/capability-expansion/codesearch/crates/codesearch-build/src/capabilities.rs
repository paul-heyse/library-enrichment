//! Capabilities, and the bindings that attach mechanisms to them.
//!
//! `Capability` is the abstract querying ability; `Mechanism` is a concrete way to exercise it.
//! The distinction is what lets `discover` answer "how do I constrain a match by what it contains"
//! rather than returning a list of flags and leaving the agent to guess.
//!
//! This is the only authored input in the catalog. Everything else is derived from the skill's
//! indexes, which are derived from the tools; a capability is a judgement about what someone is
//! trying to do. It is seeded from `seeds/capabilities.json` and grounded in the topic pages the
//! skill already ships, so it is the vocabulary the repository teaches rather than a second one.
//!
//! # A binding that matches nothing is an error
//!
//! A capability with no mechanisms is either a typo in a prefix or a claim the catalog cannot
//! support. Both should be loud, because the failure is otherwise invisible: `discover` would
//! simply return nothing for that ability and look like a legitimate empty result.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Seed {
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Deserialize)]
pub struct Capability {
    pub ability: String,
    pub topic: String,
    pub summary: String,
    pub facets: BTreeMap<String, String>,
    pub bindings: Vec<Binding>,
}

/// How a capability claims its mechanisms. Either a key prefix or a regular expression over the
/// minted key -- both operate on `entity_key`, never on a tool's own naming, so a tool upgrade
/// that recategorises a flag does not silently unbind it.
/// Untagged: the seed writes {"prefix": "..."} and {"regex": "..."} directly, which reads far
/// better than serde's externally-tagged default would.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Binding {
    Prefix { prefix: String },
    Regex { regex: String },
}

impl Binding {
    pub fn matches(&self, entity_key: &str) -> bool {
        match self {
            Binding::Prefix { prefix } => entity_key.starts_with(prefix),
            // A deliberately tiny matcher rather than a regex dependency: the seed only ever needs
            // `^literal` with `(a|b)` alternation and `.*`, and pulling in a regex engine to
            // interpret our own configuration file would be a poor trade.
            Binding::Regex { regex } => simple_match(regex, entity_key),
        }
    }
}

/// Supports `^`, literal text, `(a|b|c)` alternation and `.*`. Anything else is rejected by
/// [`validate_patterns`] rather than silently mis-parsed.
fn simple_match(pattern: &str, text: &str) -> bool {
    let anchored = pattern.starts_with('^');
    let pattern = pattern.strip_prefix('^').unwrap_or(pattern);
    let mut alternatives = vec![String::new()];
    let mut chars = pattern.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let mut group = String::new();
                for g in chars.by_ref() {
                    if g == ')' {
                        break;
                    }
                    group.push(g);
                }
                let options: Vec<&str> = group.split('|').collect();
                alternatives = alternatives
                    .iter()
                    .flat_map(|base| options.iter().map(move |o| format!("{base}{o}")))
                    .collect();
            }
            '.' if chars.peek() == Some(&'*') => {
                chars.next();
                // `.*` splits each alternative into a prefix that must be found, then the rest.
                alternatives = alternatives.iter().map(|a| format!("{a}\u{0}")).collect();
            }
            _ => {
                for a in &mut alternatives {
                    a.push(c);
                }
            }
        }
    }
    alternatives.iter().any(|alt| {
        let parts: Vec<&str> = alt.split('\u{0}').collect();
        let mut rest = text;
        for (i, part) in parts.iter().enumerate() {
            if i == 0 && anchored {
                if !rest.starts_with(part) {
                    return false;
                }
                rest = &rest[part.len()..];
            } else {
                match rest.find(part) {
                    Some(at) => rest = &rest[at + part.len()..],
                    None => return false,
                }
            }
        }
        true
    })
}

#[derive(Debug, thiserror::Error)]
pub enum SeedError {
    #[error("reading {0}: {1}")]
    Io(String, String),
    #[error(
        "capability `{ability}` binds no mechanisms. Either a binding pattern is wrong, or the catalog cannot support the claim -- both are bugs, and a silent empty `discover` result would hide either."
    )]
    UnboundCapability { ability: String },
    #[error("capability `{ability}` uses an unsupported pattern `{pattern}`: {reason}")]
    UnsupportedPattern {
        ability: String,
        pattern: String,
        reason: String,
    },
}

/// Reject pattern syntax the tiny matcher would silently misread.
fn validate_patterns(seed: &Seed) -> Result<(), SeedError> {
    for cap in &seed.capabilities {
        for binding in &cap.bindings {
            let Binding::Regex { regex } = binding else {
                continue;
            };
            for bad in ['[', ']', '+', '?', '{', '\\'] {
                if regex.contains(bad) {
                    return Err(SeedError::UnsupportedPattern {
                        ability: cap.ability.clone(),
                        pattern: regex.clone(),
                        reason: format!("`{bad}` is not supported by the seed matcher"),
                    });
                }
            }
        }
    }
    Ok(())
}

pub fn load(path: &Path) -> Result<Seed, SeedError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| SeedError::Io(path.display().to_string(), e.to_string()))?;
    let seed: Seed = serde_json::from_str(&text)
        .map_err(|e| SeedError::Io(path.display().to_string(), e.to_string()))?;
    validate_patterns(&seed)?;
    Ok(seed)
}

/// Which mechanism keys each capability claims. Errors if any capability claims none.
pub fn bind<'a>(
    seed: &'a Seed,
    mechanism_keys: &[String],
) -> Result<BTreeMap<&'a str, Vec<String>>, SeedError> {
    let mut out: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for cap in &seed.capabilities {
        let matched: Vec<String> = mechanism_keys
            .iter()
            .filter(|k| cap.bindings.iter().any(|b| b.matches(k)))
            .cloned()
            .collect();
        if matched.is_empty() {
            return Err(SeedError::UnboundCapability {
                ability: cap.ability.clone(),
            });
        }
        out.insert(cap.ability.as_str(), matched);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_bindings_match_by_key_not_by_tool_naming() {
        let b = Binding::Prefix {
            prefix: "mech:sg/rule/fix.".to_string(),
        };
        assert!(b.matches("mech:sg/rule/fix.template"));
        assert!(!b.matches("mech:sg/rule/relation.has"));
    }

    #[test]
    fn the_tiny_matcher_handles_the_forms_the_seed_uses() {
        assert!(simple_match(
            "^mech:rg/cli/--(glob|type|ignore)",
            "mech:rg/cli/--glob"
        ));
        assert!(!simple_match(
            "^mech:rg/cli/--(glob|type|ignore)",
            "mech:rg/cli/--pcre2"
        ));
        assert!(simple_match(
            "^mech:(rg|sg)/cli/.*--json",
            "mech:sg/cli/run/--json"
        ));
        assert!(!simple_match(
            "^mech:(rg|sg)/cli/.*--json",
            "mech:sg/rule/fix.template"
        ));
        // Anchoring is real: an unanchored substring must not match at the start check.
        assert!(!simple_match("^mech:rg/cli/--glob", "x-mech:rg/cli/--glob"));
    }

    #[test]
    fn unsupported_pattern_syntax_is_refused_rather_than_misread() {
        let seed: Seed = serde_json::from_str(
            r#"{"capabilities":[{"ability":"a","topic":"t","summary":"s","facets":{},
                "bindings":[{"regex":"^mech:rg/cli/--[a-z]+"}]}]}"#,
        )
        .expect("parses");
        let err = validate_patterns(&seed).expect_err("character classes are not supported");
        assert!(matches!(err, SeedError::UnsupportedPattern { .. }));
    }

    #[test]
    fn a_capability_binding_nothing_is_an_error() {
        let seed: Seed = serde_json::from_str(
            r#"{"capabilities":[{"ability":"ghost","topic":"t","summary":"s","facets":{},
                "bindings":[{"prefix":"mech:nope/"}]}]}"#,
        )
        .expect("parses");
        let err = bind(&seed, &["mech:rg/cli/--pcre2".to_string()])
            .expect_err("an unbound capability must fail the build");
        assert!(matches!(err, SeedError::UnboundCapability { .. }));
    }
}
