//! `evidence.search`: bounded lexical search over a snapshot (blueprint §7, `search_evidence`;
//! §7.1 search requirements; §7.3 budgets).
//!
//! Filter by context and snapshot first, then rank with recorded factors, fold re-exports, and
//! page with a cursor bound to the snapshot, the query digest and the sort. Fewer complete
//! hits are returned rather than a cut one when the byte budget binds. An empty page states
//! which sources were searched: absence from the index is not proof of absence (gate C01).

use std::collections::BTreeSet;

use enrichment_core::evidence::{EvidenceFragment, EvidenceKind, FragmentKind};
use enrichment_core::request::SearchRequest;
use enrichment_core::search::{self, Cursor, CursorError, FragmentHit, SymbolHit};
use enrichment_core::wire::data::{HitKind, ScoreFactor, SearchData, SearchHit};
use enrichment_core::wire::{
    Coverage, Envelope, ErrorCode, Evidence, EvidenceClass, Freshness, Pagination,
    SourceVersionMatch,
};

use super::common::{self, evidence_from_fragment};
use crate::envelope::{self, IntoPartial, Research};
use crate::service::Service;

/// The evidence families a search can be limited to.
pub const FAMILIES: &[&str] = &[
    "api",
    "docs",
    "examples",
    "release_notes",
    "features",
    "source",
];

const SORT: &str = "score";

enum Ranked {
    Symbol(SymbolHit),
    Fragment(FragmentHit),
}

impl Ranked {
    fn score(&self) -> u32 {
        match self {
            Self::Symbol(h) => h.score,
            Self::Fragment(h) => h.score,
        }
    }

    fn key(&self) -> String {
        match self {
            Self::Symbol(h) => format!("0{}", h.symbol.path),
            Self::Fragment(h) => format!("1{}{}", h.fragment.subject, h.fragment.fragment_id),
        }
    }
}

/// Run a search.
pub async fn search(service: &Service, request: SearchRequest) -> Envelope {
    let query = request.query.trim().to_owned();
    if query.is_empty() {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            "`query` must not be empty",
            "Pass a symbol name, a qualified path, or a few words describing the capability.",
            false,
        );
    }
    let mut kinds: Vec<String> = request.kinds.clone().unwrap_or_else(|| {
        FAMILIES
            .iter()
            .filter(|f| **f != "source")
            .map(|s| (*s).to_owned())
            .collect()
    });
    kinds.sort();
    kinds.dedup();
    if let Some(unknown) = kinds.iter().find(|k| !FAMILIES.contains(&k.as_str())) {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            format!("`{unknown}` is not an evidence kind"),
            format!("Use any of: {}.", FAMILIES.join(", ")),
            false,
        );
    }

    let opened =
        match common::open_context(service, &request.context_id, request.snapshot_id.as_deref())
            .await
        {
            Ok(opened) => opened,
            Err(envelope) => return *envelope,
        };
    let scope = opened.snapshot_id.to_string();
    let digest = search::query_digest(&query, &kinds);
    let offset = match &request.cursor {
        None => 0,
        Some(cursor) => match Cursor::decode(cursor, &scope, &digest, SORT) {
            Ok(cursor) => cursor.offset,
            Err(err) => {
                let next = match err {
                    CursorError::Malformed => "Start again without a cursor.",
                    CursorError::Mismatch { .. } => {
                        "A cursor is valid only for the same snapshot, query, kinds and sort. \
                         Start again without a cursor."
                    }
                };
                return envelope::error(ErrorCode::InvalidCursor, err.to_string(), next, false);
            }
        },
    };

    let tokens = search::tokenize(&query);
    let want = |family: &str| kinds.iter().any(|k| k == family);
    let mut searched: Vec<String> = Vec::new();
    let mut ranked: Vec<Ranked> = Vec::new();

    if want("api") {
        searched.push(EvidenceKind::PublicApi.as_str().to_owned());
        let mut candidates = match opened.reader.symbols_matching_any(&tokens).await {
            Ok(c) => c,
            Err(err) => return common::query_error(&err),
        };
        if let Ok(exact) = opened.reader.symbols_at(&query).await {
            candidates.extend(exact);
        }
        if let Some(last) = query.rsplit("::").next()
            && let Ok(suffix) = opened.reader.symbols_ending_with(last).await
        {
            candidates.extend(suffix);
        }
        candidates.sort_by(|a, b| a.symbol_id.cmp(&b.symbol_id));
        candidates.dedup_by(|a, b| a.symbol_id == b.symbol_id);
        ranked.extend(
            search::rank_symbols(&candidates, &query, &tokens)
                .into_iter()
                .map(Ranked::Symbol),
        );
    }

    let mut fragment_kinds: Vec<FragmentKind> = Vec::new();
    if want("docs") {
        searched.push(EvidenceKind::Documentation.as_str().to_owned());
        fragment_kinds.extend([FragmentKind::DocText, FragmentKind::ReadmeSection]);
    }
    if want("examples") {
        searched.push(EvidenceKind::Examples.as_str().to_owned());
        fragment_kinds.push(FragmentKind::Example);
    }
    if want("release_notes") {
        searched.push(EvidenceKind::ReleaseNotes.as_str().to_owned());
        fragment_kinds.push(FragmentKind::ChangelogSection);
    }
    if want("features") {
        searched.push(EvidenceKind::DocumentationBuildConfig.as_str().to_owned());
        fragment_kinds.push(FragmentKind::FeatureDefinition);
    }
    if !fragment_kinds.is_empty() {
        let fragments: Vec<EvidenceFragment> = match opened
            .reader
            .fragments_matching_any(&tokens, Some(&fragment_kinds))
            .await
        {
            Ok(f) => f,
            Err(err) => return common::query_error(&err),
        };
        ranked.extend(
            search::rank_fragments(&fragments, &query, &tokens)
                .into_iter()
                .map(Ranked::Fragment),
        );
    }
    let source_requested = want("source");

    ranked.sort_by(|a, b| {
        b.score()
            .cmp(&a.score())
            .then_with(|| a.key().cmp(&b.key()))
    });
    let total = ranked.len() as u64;

    // Page under the item and byte budgets: complete hits only.
    let page_size = request
        .max_items
        .map_or(service.config.limits.search_results, |m| {
            m.clamp(1, service.config.limits.search_results)
        });
    let budget = common::byte_budget(service, request.max_bytes);
    let excerpt_chars = service.config.limits.excerpt_characters;
    let source_version_match = if opened.reader.manifest().crate_version.as_deref()
        == Some(opened.release.key.version.as_str())
    {
        SourceVersionMatch::Exact
    } else {
        SourceVersionMatch::Unknown
    };

    let mut hits: Vec<SearchHit> = Vec::new();
    let mut evidence: Vec<Evidence> = Vec::new();
    let mut used = 2048usize;
    let mut next_offset = offset;
    let mut truncated = false;
    for item in ranked.iter().skip(offset as usize) {
        if hits.len() >= page_size {
            truncated = true;
            break;
        }
        let (hit, ev) = render(
            service,
            item,
            source_version_match,
            excerpt_chars,
            &opened.reader.manifest().crate_name,
        );
        let size = common::json_size(&hit) + common::json_size(&ev);
        if used + size > budget && !hits.is_empty() {
            truncated = true;
            break;
        }
        used += size;
        hits.push(hit);
        evidence.push(ev);
        next_offset += 1;
    }
    let remaining = total.saturating_sub(next_offset);
    truncated = truncated || remaining > 0;
    let next_cursor =
        (remaining > 0).then(|| Cursor::new(&scope, &digest, SORT, next_offset).encode());

    let mut limitations = vec![
        "Lexical matching over the documented build; absence from these results is not \
         evidence that the crate lacks a capability (see `coverage.indexed`)."
            .to_owned(),
    ];
    if source_requested {
        limitations.push(
            "Source text is not indexed for search; use `inspect_symbol` with depth=source \
             for a specific symbol."
                .to_owned(),
        );
    }
    if hits.is_empty() {
        limitations.push(format!(
            "No match for `{query}` within {}. Try a broader term, a different evidence kind, \
             or `library_overview` to browse.",
            searched.join(", ")
        ));
    }
    let mut missing: BTreeSet<String> = BTreeSet::new();
    if source_requested {
        missing.insert(EvidenceKind::SourceExcerpts.as_str().to_owned());
    }
    for kind in &opened.reader.manifest().missing {
        missing.insert(kind.as_str().to_owned());
    }

    let data = SearchData {
        query: query.clone(),
        tokens,
        kinds,
        hits,
        scoring: search::FACTORS
            .iter()
            .map(|(name, points)| ScoreFactor {
                name: (*name).to_owned(),
                points: *points,
            })
            .collect(),
        searched: searched.clone(),
        offset,
    };
    let returned = data.hits.len() as u64;
    let coverage = Coverage {
        scope: format!(
            "{} in snapshot {} of {} {}",
            searched.join(", "),
            scope,
            opened.release.key.package,
            opened.release.key.version
        ),
        indexed: searched.iter().cloned().collect(),
        missing: missing.clone(),
        limitations,
    };
    let mut env = Research {
        summary: format!(
            "{total} match(es) for `{query}` in {}; returning {returned} from offset {offset}.",
            searched.join(", ")
        ),
        data: common::to_object(&data),
        coverage,
        freshness: Freshness {
            registry_checked_at: None,
            source_version_match,
            latest_verified: false,
        },
        context_id: Some(opened.context.context_id.to_string()),
        snapshot_id: Some(scope.clone()),
        evidence,
        artifacts: Vec::new(),
    }
    .ok_with_pagination(Pagination {
        returned,
        total_matches: Some(total),
        truncated,
        next_cursor,
    });
    if !missing.is_empty() {
        env = env.into_partial();
    }
    env
}

fn render(
    service: &Service,
    item: &Ranked,
    source_version_match: SourceVersionMatch,
    excerpt_chars: usize,
    crate_name: &str,
) -> (SearchHit, Evidence) {
    match item {
        Ranked::Symbol(hit) => {
            let s = &hit.symbol;
            let excerpt = s
                .signature
                .clone()
                .or_else(|| s.doc_summary.clone())
                .unwrap_or_else(|| format!("{} {}", s.kind.as_str(), s.path));
            let evidence_id = format!("ev_{}", &s.symbol_id[4..]);
            let evidence = Evidence {
                evidence_id: evidence_id.clone(),
                evidence_class: EvidenceClass::StaticallyExtracted,
                subject: s.path.clone(),
                artifact_id: String::new(),
                source_uri: format!(
                    "library-evidence://contexts/{crate_name}/symbols/{}",
                    s.symbol_id
                ),
                locator: serde_json::json!({
                    "kind": "symbol", "path": s.path, "rustdoc_id": s.producer_local_id,
                    "span_file": s.span_file, "span_line": s.span_line
                })
                .as_object()
                .cloned()
                .unwrap_or_default(),
                source_version_match,
                producer: enrichment_core::producer::rustdoc::PRODUCER.to_owned(),
                producer_version: enrichment_core::producer::rustdoc::NORMALIZER_VERSION.to_owned(),
                excerpt: common::truncate(&excerpt, excerpt_chars),
            };
            (
                SearchHit {
                    hit: HitKind::Symbol,
                    score: hit.score,
                    factors: factors(&hit.factors),
                    evidence_id,
                    path: Some(s.path.clone()),
                    symbol_kind: Some(s.kind),
                    signature: s.signature.clone(),
                    also_at: hit.also_at.clone(),
                    deprecated: s.deprecated.is_some(),
                    fragment_kind: None,
                    subject: None,
                    excerpt: common::truncate(
                        s.doc_summary.as_deref().unwrap_or(""),
                        excerpt_chars,
                    ),
                },
                evidence,
            )
        }
        Ranked::Fragment(hit) => {
            let f = &hit.fragment;
            let evidence = evidence_from_fragment(service, f, source_version_match, excerpt_chars);
            (
                SearchHit {
                    hit: HitKind::Fragment,
                    score: hit.score,
                    factors: factors(&hit.factors),
                    evidence_id: evidence.evidence_id.clone(),
                    path: None,
                    symbol_kind: None,
                    signature: None,
                    also_at: Vec::new(),
                    deprecated: false,
                    fragment_kind: Some(f.kind),
                    subject: Some(f.subject.clone()),
                    excerpt: common::truncate(&f.text, excerpt_chars),
                },
                evidence,
            )
        }
    }
}

fn factors(list: &[search::Factor]) -> Vec<ScoreFactor> {
    list.iter()
        .map(|f| ScoreFactor {
            name: f.name.to_owned(),
            points: f.points,
        })
        .collect()
}
