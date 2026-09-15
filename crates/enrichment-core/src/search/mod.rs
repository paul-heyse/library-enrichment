//! Deterministic lexical search and pagination cursors (blueprint §7.1, §7.3).
//!
//! Exact qualified names outrank fuzzy matches: an exact path, then a path suffix, then a name
//! prefix, then tokens in the signature, then tokens in the documentation. Every hit carries
//! the factors that scored it, so a caller sees *why* something ranked where it did instead of
//! an unexplained number. No embeddings; nothing here needs a model.
//!
//! Near-duplicate re-exports are folded: hits are grouped by definition, the definition's own
//! path is preferred, and the other public paths are listed on the hit as `also_at`.
//!
//! A cursor binds the scope (snapshot), the query digest, the sort and the offset, plus a
//! checksum over all four. A cursor presented against a different query or snapshot is
//! refused with `INVALID_CURSOR` rather than producing an inconsistent page.

use serde::{Deserialize, Serialize};

use crate::canonical;
#[cfg(test)]
use crate::evidence::Symbol;
pub mod page;
pub mod row_page;

/// Lower-case tokens of length two or more, split on anything that is not a word character.
/// A `::`-qualified path stays a single token as well, so exact-path matching sees it whole.
#[must_use]
pub fn tokenize(query: &str) -> Vec<String> {
    let lower = query.trim().to_lowercase();
    let mut tokens: Vec<String> = lower
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|t| t.len() >= 2)
        .map(str::to_owned)
        .collect();
    if lower.contains("::") || lower.contains('.') {
        tokens.insert(0, lower.clone());
    }
    let mut seen = std::collections::BTreeSet::new();
    tokens.retain(|token| seen.insert(token.clone()));
    tokens
}

/// One scoring factor that fired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Factor {
    /// Stable factor name.
    pub name: &'static str,
    /// Points contributed.
    pub points: u32,
}

/// The scoring table, in rank order. Exposed so a response can carry the legend.
pub const FACTORS: &[(&str, u32)] = &[
    ("exact_path", 1000),
    ("path_suffix", 800),
    ("name_exact", 700),
    ("name_prefix", 400),
    ("path_token", 200),
    ("signature_token", 120),
    ("summary_token", 80),
    ("docs_token", 40),
    ("subject_exact", 600),
    ("subject_token", 150),
    ("text_token", 50),
    ("definition_path", 5),
];

fn points(name: &str) -> u32 {
    FACTORS
        .iter()
        .find(|(n, _)| *n == name)
        .map_or(0, |(_, p)| *p)
}

fn factor(name: &'static str) -> Factor {
    Factor {
        name,
        points: points(name),
    }
}

/// Score one symbol against a query. `None` when nothing matched.
#[must_use]
#[cfg(test)]
fn score_symbol(symbol: &Symbol, query: &str, tokens: &[String]) -> Option<(u32, Vec<Factor>)> {
    score_symbol_fields(
        SymbolScoreFields {
            path: &symbol.path,
            name: &symbol.name,
            signature: symbol.signature.as_deref(),
            summary: symbol.doc_summary.as_deref(),
            docs: symbol.docs.as_deref(),
            is_reexport: symbol.is_reexport,
        },
        query,
        tokens,
    )
}

/// Only the borrowed fields needed by the pure scoring kernel; no producer or wire DTO.
#[derive(Debug, Clone, Copy)]
pub struct SymbolScoreFields<'a> {
    pub path: &'a str,
    pub name: &'a str,
    pub signature: Option<&'a str>,
    pub summary: Option<&'a str>,
    pub docs: Option<&'a str>,
    pub is_reexport: bool,
}

/// Score already selected fields. Matching remains independently lowerable to native filters.
#[must_use]
pub fn score_symbol_fields(
    symbol: SymbolScoreFields<'_>,
    query: &str,
    tokens: &[String],
) -> Option<(u32, Vec<Factor>)> {
    let q = query.trim().to_lowercase();
    let path = symbol.path.to_lowercase();
    let name = symbol.name.to_lowercase();
    let mut factors = Vec::new();

    if !q.is_empty() && path == q {
        factors.push(factor("exact_path"));
    } else if !q.is_empty()
        && (path.ends_with(&format!("::{q}")) || path.ends_with(&format!(".{q}")))
    {
        factors.push(factor("path_suffix"));
    } else if !q.is_empty() && name == q {
        factors.push(factor("name_exact"));
    }

    let mut name_prefix = false;
    let mut path_token = false;
    let mut signature_token = false;
    let mut summary_token = false;
    let mut docs_token = false;
    for token in tokens
        .iter()
        .filter(|t| !t.contains("::") && !t.contains('.'))
    {
        if name.starts_with(token.as_str()) {
            name_prefix = true;
        }
        if path.contains(token.as_str()) {
            path_token = true;
        }
        if symbol
            .signature
            .is_some_and(|s| s.to_lowercase().contains(token.as_str()))
        {
            signature_token = true;
        }
        if symbol
            .summary
            .is_some_and(|s| s.to_lowercase().contains(token.as_str()))
        {
            summary_token = true;
        }
        if symbol
            .docs
            .is_some_and(|s| s.to_lowercase().contains(token.as_str()))
        {
            docs_token = true;
        }
    }
    if name_prefix
        && !factors
            .iter()
            .any(|f| f.name == "name_exact" || f.name == "exact_path")
    {
        factors.push(factor("name_prefix"));
    }
    if path_token {
        factors.push(factor("path_token"));
    }
    if signature_token {
        factors.push(factor("signature_token"));
    }
    if summary_token {
        factors.push(factor("summary_token"));
    }
    if docs_token {
        factors.push(factor("docs_token"));
    }
    if factors.is_empty() {
        return None;
    }
    if !symbol.is_reexport {
        factors.push(factor("definition_path"));
    }
    let score = factors.iter().map(|f| f.points).sum();
    Some((score, factors))
}

/// Score fragment fields without reconstructing a persistence or wire object.
#[must_use]
pub fn score_fragment_fields(
    subject: &str,
    text: &str,
    query: &str,
    tokens: &[String],
) -> Option<(u32, Vec<Factor>)> {
    let q = query.trim().to_lowercase();
    let subject = subject.to_lowercase();
    let text = text.to_lowercase();
    let mut factors = Vec::new();
    if !q.is_empty() && subject == q {
        factors.push(factor("subject_exact"));
    }
    let mut subject_token = false;
    let mut text_hits = 0u32;
    for token in tokens
        .iter()
        .filter(|t| !t.contains("::") && !t.contains('.'))
    {
        if subject.contains(token.as_str()) {
            subject_token = true;
        }
        if text.contains(token.as_str()) {
            text_hits += 1;
        }
    }
    if subject_token {
        factors.push(factor("subject_token"));
    }
    if text_hits > 0 {
        factors.push(Factor {
            name: "text_token",
            points: points("text_token") * text_hits.min(3),
        });
    }
    if factors.is_empty() {
        return None;
    }
    let score = factors.iter().map(|f| f.points).sum();
    Some((score, factors))
}

/// Digest of what a search asked for, so a cursor can be checked against it.
#[must_use]
#[cfg(test)]
fn query_digest(query: &str, kinds: &[String]) -> String {
    let mut kinds = kinds.to_vec();
    kinds.sort();
    canonical::short_id(
        "q",
        &serde_json::json!({ "query": query.trim().to_lowercase(), "kinds": kinds }),
    )
}

/// A pagination cursor (§7.3): scope, query digest, sort and position, checksummed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cursor {
    /// What is being paged: a snapshot id, or an artifact id.
    pub scope: String,
    /// Digest of the query and filters.
    pub query_digest: String,
    /// The sort in force.
    pub sort: String,
    /// Next position.
    pub offset: u64,
    /// Checksum over the other fields.
    pub check: String,
}

/// Why a cursor was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CursorError {
    /// Not a cursor this service issued.
    #[error("the cursor is not one this service issued")]
    Malformed,
    /// Issued for a different scope, query or sort.
    #[error("the cursor was issued for a different {field}")]
    Mismatch {
        /// Which binding differs.
        field: &'static str,
    },
}

impl Cursor {
    /// Mint a cursor.
    #[must_use]
    pub fn new(scope: &str, query_digest: &str, sort: &str, offset: u64) -> Self {
        Self {
            scope: scope.to_owned(),
            query_digest: query_digest.to_owned(),
            sort: sort.to_owned(),
            offset,
            check: Self::checksum(scope, query_digest, sort, offset),
        }
    }

    fn checksum(scope: &str, query_digest: &str, sort: &str, offset: u64) -> String {
        canonical::digest_hex(&serde_json::json!({
            "contract": "artifact-window/2",
            "scope": scope, "query": query_digest, "sort": sort, "offset": offset
        }))[..16]
            .to_owned()
    }

    /// The opaque wire form.
    /// # Errors
    /// Serialization failure cannot become an empty valid-looking cursor.
    pub fn encode(&self) -> Result<String, serde_json::Error> {
        Ok(format!("artifact2_{}", hex(&serde_json::to_vec(self)?)))
    }

    /// Parse and check a cursor against what the caller is paging now.
    ///
    /// # Errors
    ///
    /// See [`CursorError`].
    pub fn decode(
        text: &str,
        scope: &str,
        query_digest: &str,
        sort: &str,
    ) -> Result<Self, CursorError> {
        if text.len() > 32768 {
            return Err(CursorError::Malformed);
        }
        let body = text
            .strip_prefix("artifact2_")
            .ok_or(CursorError::Malformed)?;
        let bytes = unhex(body).ok_or(CursorError::Malformed)?;
        let cursor: Self = serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)?;
        if cursor.check
            != Self::checksum(
                &cursor.scope,
                &cursor.query_digest,
                &cursor.sort,
                cursor.offset,
            )
        {
            return Err(CursorError::Malformed);
        }
        if cursor.scope != scope {
            return Err(CursorError::Mismatch {
                field: "snapshot or artifact",
            });
        }
        if cursor.query_digest != query_digest {
            return Err(CursorError::Mismatch {
                field: "query or filters",
            });
        }
        if cursor.sort != sort {
            return Err(CursorError::Mismatch { field: "sort" });
        }
        Ok(cursor)
    }
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(DIGITS[usize::from(b >> 4)] as char);
        out.push(DIGITS[usize::from(b & 0x0f)] as char);
    }
    out
}

pub(crate) fn unhex(text: &str) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return None;
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(text.get(i..i + 2)?, 16).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::SymbolKind;

    fn symbol(path: &str, reexport: bool, docs: &str) -> Symbol {
        let name = path.rsplit("::").next().unwrap_or(path).to_owned();
        Symbol {
            symbol_id: Symbol::symbol_id_for("c", path, SymbolKind::Function, None),
            definition_id: "def_shared".to_owned(),
            path: path.to_owned(),
            name,
            kind: SymbolKind::Function,
            parent_path: None,
            signature: Some(format!("pub fn {path}(size: u32)")),
            doc_summary: Some(docs.to_owned()),
            docs: Some(docs.to_owned()),
            deprecated: None,
            span_file: None,
            span_line: None,
            is_reexport: reexport,
            definition_path: "c::inner::widget".to_owned(),
            defined_in_crate: "c".to_owned(),
            producer_local_id: 1,
            qualifier: None,
            cfg_hints: vec![],
            python: None,
        }
    }

    #[test]
    fn tokens_are_lowercased_and_paths_stay_whole() {
        assert_eq!(tokenize("Async Runtime"), vec!["async", "runtime"]);
        assert_eq!(
            tokenize("enr_fixture::Widget"),
            vec!["enr_fixture::widget", "enr_fixture", "widget"]
        );
        assert!(tokenize("a").is_empty());
    }

    #[test]
    fn exact_paths_have_the_declared_score_advantage() {
        let qualified = symbol("c::inner::widget", false, "A square widget.");
        let alias = symbol("c::widget", true, "A square widget.");
        let query = "c::inner::widget";
        let (score, factors) =
            score_symbol(&qualified, query, &tokenize(query)).expect("qualified match");
        let (alias_score, _) =
            score_symbol(&alias, query, &tokenize(query)).expect("alias token match");
        assert!(score > alias_score);
        assert!(factors.iter().any(|f| f.name == "exact_path"));
    }

    #[test]
    fn docs_only_matches_score_below_name_matches() {
        let by_name = symbol("c::square", false, "nothing relevant");
        let by_docs = symbol("c::other", false, "computes a square");
        let tokens = tokenize("square");
        let (a, _) = score_symbol(&by_name, "square", &tokens).expect("scores");
        let (b, _) = score_symbol(&by_docs, "square", &tokens).expect("scores");
        assert!(a > b);
        assert!(score_symbol(&by_docs, "zzz", &tokenize("zzz")).is_none());
    }

    #[test]
    fn cursors_bind_scope_query_and_sort() {
        let digest = query_digest("Widget", &["api".to_owned()]);
        let cursor = Cursor::new("snap_x", &digest, "score", 12);
        let text = cursor.encode().expect("cursor serializes");
        assert!(text.starts_with("artifact2_"));
        assert_eq!(
            Cursor::decode(&text, "snap_x", &digest, "score")
                .expect("ok")
                .offset,
            12
        );
        assert!(matches!(
            Cursor::decode(&text, "snap_y", &digest, "score"),
            Err(CursorError::Mismatch {
                field: "snapshot or artifact"
            })
        ));
        let other = query_digest("Widget", &["docs".to_owned()]);
        assert!(matches!(
            Cursor::decode(&text, "snap_x", &other, "score"),
            Err(CursorError::Mismatch {
                field: "query or filters"
            })
        ));
        assert!(matches!(
            Cursor::decode("artifact2_zz", "snap_x", &digest, "score"),
            Err(CursorError::Malformed)
        ));
        assert!(matches!(
            Cursor::decode("/etc/passwd", "snap_x", &digest, "score"),
            Err(CursorError::Malformed)
        ));
        // A tampered offset fails the checksum.
        assert!(
            Cursor::decode(
                &text.replacen("artifact2_", "cur_", 1),
                "snap_x",
                &digest,
                "score"
            )
            .is_err()
        );
        let mut tampered = cursor.clone();
        tampered.offset = 99;
        assert!(matches!(
            Cursor::decode(
                &tampered.encode().expect("cursor serializes"),
                "snap_x",
                &digest,
                "score"
            ),
            Err(CursorError::Malformed)
        ));
    }
}
pub mod spec;
