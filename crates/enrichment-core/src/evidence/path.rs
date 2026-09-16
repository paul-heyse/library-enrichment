//! Public lexical paths. Semantic membership edges are a separate relation.

use serde::{Deserialize, Serialize};

use crate::identity::Ecosystem;

/// A lexical path is a language and an ordered list of components, not a delimiter heuristic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "PathParts", into = "PathParts")]
pub struct PublicPath {
    ecosystem: Ecosystem,
    components: Vec<String>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct PathParts {
    ecosystem: Ecosystem,
    components: Vec<String>,
}

impl TryFrom<PathParts> for PublicPath {
    type Error = String;

    fn try_from(value: PathParts) -> Result<Self, Self::Error> {
        Self::new(value.ecosystem, value.components)
    }
}

impl From<PublicPath> for PathParts {
    fn from(value: PublicPath) -> Self {
        Self {
            ecosystem: value.ecosystem,
            components: value.components,
        }
    }
}

impl PublicPath {
    /// Bound both path depth and encoded input size before creating an ancestry index.
    pub const MAX_DEPTH: usize = 64;
    /// Maximum UTF-8 path content, excluding separators.
    pub const MAX_BYTES: usize = 4096;

    /// Construct an exact path. Component punctuation is data, never another parent edge.
    ///
    /// # Errors
    /// Rejects empty, excessive or control-bearing components.
    pub fn new(ecosystem: Ecosystem, components: Vec<String>) -> Result<Self, String> {
        if components.is_empty() || components.len() > Self::MAX_DEPTH {
            return Err("public path depth is outside the admitted bound".into());
        }
        let bytes = components.iter().try_fold(0usize, |n, part| {
            n.checked_add(part.len()).filter(|n| *n <= Self::MAX_BYTES)
        });
        if bytes.is_none()
            || components
                .iter()
                .any(|p| p.is_empty() || p.chars().any(char::is_control))
        {
            return Err("public path has empty, control-bearing or excessive content".into());
        }
        Ok(Self {
            ecosystem,
            components,
        })
    }

    /// Parse an ordinary language-qualified display path; exact unusual components use IDs.
    ///
    /// # Errors
    /// As [`Self::new`].
    pub fn parse(ecosystem: Ecosystem, display: &str) -> Result<Self, String> {
        let separator = match ecosystem {
            Ecosystem::Rust => "::",
            Ecosystem::Python => ".",
        };
        Self::new(
            ecosystem,
            display.split(separator).map(str::to_owned).collect(),
        )
    }

    /// The language whose namespace this path belongs to.
    #[must_use]
    pub fn ecosystem(&self) -> Ecosystem {
        self.ecosystem
    }

    /// Exact ordered components.
    #[must_use]
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// Canonical path identity is independent of the display delimiter.
    #[must_use]
    pub fn id(&self) -> String {
        crate::native_key::Key::PublicPath
            .value(self)
            .expect("declared native public path contract")
    }

    /// Display rendering. Callers use identity to disambiguate literal separator components.
    #[must_use]
    pub fn display(&self) -> String {
        self.components.join(match self.ecosystem {
            Ecosystem::Rust => "::",
            Ecosystem::Python => ".",
        })
    }

    /// Reflexive lexical ancestry, with language and component boundaries preserved.
    #[must_use]
    pub fn is_within(&self, area: &Self) -> bool {
        self.ecosystem == area.ecosystem && self.components.starts_with(&area.components)
    }

    /// Direct lexical parent; an implicit navigation node does not assert a public symbol.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        (self.components.len() > 1).then(|| Self {
            ecosystem: self.ecosystem,
            components: self.components[..self.components.len() - 1].to_vec(),
        })
    }

    /// Bounded reflexive ancestor keys used by the derived namespace-members relation.
    #[must_use]
    pub fn ancestor_ids(&self) -> Vec<String> {
        (1..=self.components.len())
            .map(|end| {
                Self {
                    ecosystem: self.ecosystem,
                    components: self.components[..end].to_vec(),
                }
                .id()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_boundaries_are_authoritative() {
        let nested = PublicPath::parse(Ecosystem::Python, "pkg.a.b").expect("path");
        let literal =
            PublicPath::new(Ecosystem::Python, vec!["pkg".into(), "a.b".into()]).expect("path");
        let area = PublicPath::parse(Ecosystem::Python, "pkg.a").expect("area");
        assert!(nested.is_within(&area));
        assert!(!literal.is_within(&area));
        assert_ne!(nested.id(), literal.id());
        assert_eq!(
            literal.id(),
            "path_451b3acfb83be7f1b578e4f5af75e5c8a3c9ed81d749e7d340949b47d56a50b5"
        );
        assert_eq!(nested.parent(), Some(area));
        assert_eq!(nested.ancestor_ids().len(), 3);
    }

    #[test]
    fn language_and_literal_pattern_characters_are_preserved() {
        let rust = PublicPath::parse(Ecosystem::Rust, "pkg::a_%\\::β").expect("path");
        assert!(rust.is_within(&PublicPath::parse(Ecosystem::Rust, "pkg::a_%\\").expect("area")));
        assert!(
            !rust.is_within(&PublicPath::parse(Ecosystem::Python, "pkg").expect("other language"))
        );
        assert!(
            !rust
                .is_within(&PublicPath::parse(Ecosystem::Rust, "pkg::a").expect("other component"))
        );
    }

    #[test]
    fn deserialization_cannot_bypass_path_bounds() {
        for value in [
            serde_json::json!({"ecosystem":"python","components":[]}),
            serde_json::json!({"ecosystem":"rust","components":["a",""]}),
            serde_json::json!({"ecosystem":"python","components":vec!["x"; 65]}),
        ] {
            assert!(serde_json::from_value::<PublicPath>(value).is_err());
        }
    }
}
