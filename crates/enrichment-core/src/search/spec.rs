//! Eligibility is a contract shared by the reference evaluator and native query lowering.

use serde::{Deserialize, Serialize};

/// A searchable symbol field. Bonus factors are deliberately absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchField {
    Path,
    Name,
    Signature,
    Summary,
    Docs,
    Subject,
    Text,
}

impl SearchField {
    /// Name in the store's canonical search projection.
    #[must_use]
    pub fn column(self) -> &'static str {
        match self {
            Self::Path => "path",
            Self::Name => "name",
            Self::Signature => "signature",
            Self::Summary => "doc_summary",
            Self::Docs => "docs",
            Self::Subject => "subject",
            Self::Text => "text",
        }
    }
}

/// One native-lowerable match operator; patterns are literals, not SQL wildcards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchOp {
    Equals,
    Prefix,
    Contains,
    Suffix,
}

/// A candidate clause is a column, operator and a normalized literal.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Clause {
    pub field: SearchField,
    pub op: MatchOp,
    pub literal: String,
}

/// The finite query contract. Namespace/kind filters compose outside lexical eligibility.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SearchSpec {
    pub version: String,
    pub query: String,
    pub tokens: Vec<String>,
    pub symbol_clauses: Vec<Clause>,
    pub fragment_clauses: Vec<Clause>,
}

impl SearchSpec {
    /// Matching/scoring changes invalidate cursors and cached query plans.
    pub const VERSION: &'static str = "lexical/2";

    /// Exact matches remain eligible even when tokenization yields no long tokens.
    #[must_use]
    pub fn new(query: &str) -> Self {
        let query = query.trim().to_lowercase();
        let tokens = super::tokenize(&query);
        let mut symbol_clauses = Vec::new();
        let mut fragment_clauses = Vec::new();
        if !query.is_empty() {
            for field in [SearchField::Path, SearchField::Name] {
                symbol_clauses.push(Clause {
                    field,
                    op: MatchOp::Equals,
                    literal: query.clone(),
                });
            }
            for separator in ["::", "."] {
                symbol_clauses.push(Clause {
                    field: SearchField::Path,
                    op: MatchOp::Suffix,
                    literal: format!("{separator}{query}"),
                });
            }
            fragment_clauses.push(Clause {
                field: SearchField::Subject,
                op: MatchOp::Equals,
                literal: query.clone(),
            });
        }
        for token in tokens
            .iter()
            .filter(|t| !t.contains("::") && !t.contains('.'))
        {
            symbol_clauses.push(Clause {
                field: SearchField::Name,
                op: MatchOp::Prefix,
                literal: token.clone(),
            });
            for field in [
                SearchField::Path,
                SearchField::Signature,
                SearchField::Summary,
                SearchField::Docs,
            ] {
                symbol_clauses.push(Clause {
                    field,
                    op: MatchOp::Contains,
                    literal: token.clone(),
                });
            }
            for field in [SearchField::Subject, SearchField::Text] {
                fragment_clauses.push(Clause {
                    field,
                    op: MatchOp::Contains,
                    literal: token.clone(),
                });
            }
        }
        Self {
            version: Self::VERSION.into(),
            query,
            tokens,
            symbol_clauses,
            fragment_clauses,
        }
    }

    /// Reference evaluation for tests and specialized score eligibility.
    #[must_use]
    pub fn matches(clauses: &[Clause], value: impl Fn(SearchField) -> Option<String>) -> bool {
        clauses.iter().any(|c| {
            value(c.field).is_some_and(|v| {
                let v = v.to_lowercase();
                match c.op {
                    MatchOp::Equals => v == c.literal,
                    MatchOp::Prefix => v.starts_with(&c.literal),
                    MatchOp::Contains => v.contains(&c.literal),
                    MatchOp::Suffix => v.ends_with(&c.literal),
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_matching_does_not_depend_on_token_length() {
        let spec = SearchSpec::new("X");
        assert!(SearchSpec::matches(&spec.symbol_clauses, |f| (f
            == SearchField::Path)
            .then(|| "pkg::x".into())));
        assert!(SearchSpec::matches(&spec.fragment_clauses, |f| (f
            == SearchField::Subject)
            .then(|| "X".into())));
        assert!(!SearchSpec::matches(
            &SearchSpec::new(" ").symbol_clauses,
            |_| Some("anything".into())
        ));
    }

    #[test]
    fn summary_and_literal_pattern_characters_are_searchable() {
        let spec = SearchSpec::new("summaryonly");
        assert!(SearchSpec::matches(&spec.symbol_clauses, |f| (f
            == SearchField::Summary)
            .then(|| "SummaryOnly documentation".into())));
        let spec = SearchSpec::new("a_%");
        assert!(SearchSpec::matches(&spec.fragment_clauses, |f| (f
            == SearchField::Subject)
            .then(|| "a_%".into())));
        assert!(!SearchSpec::matches(&spec.fragment_clauses, |_| None));
    }
}
