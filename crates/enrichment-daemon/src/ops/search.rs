//! Native planned lexical search with complete-key pagination and bounded wire rendering.
use super::common;
use crate::{
    envelope::{self, IntoPartial, Research},
    service::Service,
};
use enrichment_core::{
    evidence::{EvidenceKind, FragmentKind},
    request::SearchRequest,
    search::{self, page::SearchCursor, spec::SearchSpec},
    wire::{
        Coverage, Envelope, ErrorCode, Evidence, Freshness, Pagination, SourceVersionMatch,
        data::{ScoreFactor, SearchData, SearchHit},
    },
};
use enrichment_store::search_plan::{self, RankedEvidence, SearchOptions};

pub const FAMILIES: &[&str] = &[
    "api",
    "docs",
    "examples",
    "release_notes",
    "features",
    "source",
];

pub async fn search(service: &Service, request: SearchRequest) -> Envelope {
    let query = request.query.trim().to_owned();
    let tokens = search::tokenize(&query);
    if query.is_empty() || query.len() > 65536 || tokens.len() > 64 {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            "Search needs a nonempty query of at most 64 terms and 65536 bytes",
            "Use a focused query, then browse or paginate.",
            false,
        );
    }
    let mut kinds = request.kinds.clone().unwrap_or_else(|| {
        FAMILIES
            .iter()
            .filter(|k| **k != "source")
            .map(|k| (*k).into())
            .collect()
    });
    kinds.sort();
    kinds.dedup();
    if kinds.iter().any(|k| !FAMILIES.contains(&k.as_str())) {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            "Unknown evidence family",
            format!("Use any of: {}", FAMILIES.join(", ")),
            false,
        );
    }
    let opened =
        match common::open_context(service, &request.context_id, request.snapshot_id.as_deref())
            .await
        {
            Ok(value) => value,
            Err(e) => return *e,
        };
    let area = match request
        .area
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| opened.reader.area(s))
        .transpose()
    {
        Ok(value) => value,
        Err(e) => return common::query_error(&e),
    };
    let scope = format!("{}:{}", opened.context.context_id, opened.snapshot_id);
    let budget = common::byte_budget(service, request.max_bytes);
    let spec = SearchSpec::new(&query);
    let digest = enrichment_core::canonical::digest_hex(&serde_json::json!([
        spec,
        kinds,
        area,
        request.max_items,
        budget
    ]));
    let cursor = match request
        .cursor
        .as_deref()
        .map(|c| SearchCursor::decode(c, &scope, &digest))
        .transpose()
    {
        Ok(value) => value,
        Err(e) => {
            return envelope::error(
                ErrorCode::InvalidCursor,
                e.to_string(),
                "Start again without a cursor for this exact context, snapshot, query and filters.",
                false,
            );
        }
    };
    let offset = cursor.as_ref().map_or(0, |c| c.returned_before);
    let want = |family| kinds.iter().any(|k| k == family);
    let mut fragment_kinds = Vec::new();
    if want("docs") {
        fragment_kinds.extend([FragmentKind::DocText, FragmentKind::ReadmeSection]);
    }
    if want("examples") {
        fragment_kinds.push(FragmentKind::Example);
    }
    if want("release_notes") {
        fragment_kinds.push(FragmentKind::ChangelogSection);
    }
    if want("features") {
        fragment_kinds.push(FragmentKind::FeatureDefinition);
    }
    let limit = service.config.limits.search_results.clamp(1, 1024);
    let page = match search_plan::page(
        opened.reader.session(),
        opened.reader.runtime(),
        &spec,
        &SearchOptions {
            include_api: want("api"),
            fragment_kinds,
            area,
            page_size: request.max_items.map_or(limit, |n| n.clamp(1, limit)),
            after: cursor.map(|c| c.after),
        },
    )
    .await
    {
        Ok(value) => value,
        Err(e) => return common::query_error(&e.into()),
    };
    if offset > page.total {
        return envelope::error(
            ErrorCode::InvalidCursor,
            "Cursor position exceeds the pinned result count",
            "Start again without a cursor.",
            false,
        );
    }
    let source_requested = want("source");
    let searched: Vec<String> = kinds
        .iter()
        .filter(|k| k.as_str() != "source")
        .cloned()
        .collect();
    let mut keys = Vec::new();
    let mut hits = Vec::new();
    let mut evidence = Vec::new();
    for row in page.rows {
        keys.push(row.key.clone());
        let (hit, citation) = match render(row, service.config.limits.excerpt_characters) {
            Ok(value) => value,
            Err(e) => return common::store_error(&e),
        };
        hits.push(hit);
        evidence.push(citation);
    }
    let mut data = SearchData {
        query: query.clone(),
        tokens,
        kinds,
        area: request.area,
        hits,
        scoring: search::FACTORS
            .iter()
            .map(|(name, points)| ScoreFactor {
                name: (*name).into(),
                points: *points,
            })
            .collect(),
        searched: searched.clone(),
        offset,
    };
    let manifest = opened.reader.manifest();
    let mut missing: std::collections::BTreeSet<_> = manifest
        .missing
        .iter()
        .map(|k| k.as_str().to_owned())
        .collect();
    if source_requested {
        missing.insert(EvidenceKind::SourceExcerpts.as_str().into());
    }
    let mut limitations = vec!["Lexical matching within the pinned evidence scope; an empty result does not establish that a capability is absent.".into()];
    if source_requested {
        limitations.push("Source excerpts are selected through inspect_symbol at source depth; full source text is not indexed for lexical search.".into());
    }
    let research = Research {
        summary: format!(
            "{} matches for `{query}` in {}; {} results already returned.",
            page.total,
            searched.join(", "),
            offset
        ),
        data: common::to_object(&data),
        coverage: Coverage {
            scope: format!("{} in snapshot {}", searched.join(", "), opened.snapshot_id),
            indexed: searched.into_iter().collect(),
            missing: missing.clone(),
            limitations,
        },
        freshness: Freshness {
            registry_checked_at: None,
            latest_verified: false,
            source_version_match: if manifest.crate_version.as_deref()
                == Some(opened.release.key.version.as_str())
            {
                SourceVersionMatch::Exact
            } else {
                SourceVersionMatch::Unknown
            },
        },
        context_id: Some(opened.context.context_id.to_string()),
        snapshot_id: Some(opened.snapshot_id.to_string()),
        evidence,
        artifacts: vec![],
    };
    let mut result = research.ok_with_pagination(Pagination {
        returned: 0,
        total_matches: Some(page.total),
        truncated: false,
        next_cursor: None,
    });
    loop {
        let returned = data.hits.len() as u64;
        let more = page.has_more || page.total > offset + returned;
        let next_cursor = if more && let Some(key) = keys.get(data.hits.len().saturating_sub(1)) {
            match SearchCursor::new(
                scope.clone(),
                digest.clone(),
                offset + returned,
                key.clone(),
            )
            .encode()
            {
                Ok(value) => Some(value),
                Err(e) => return common::store_error(&e),
            }
        } else {
            None
        };
        result.pagination = Pagination {
            returned,
            total_matches: Some(page.total),
            truncated: more,
            next_cursor,
        };
        result.data = common::to_object(&data);
        if !missing.is_empty() {
            result = result.into_partial();
        }
        if common::json_size(&result) <= budget || data.hits.len() <= 1 {
            break;
        }
        data.hits.pop();
        result.evidence.pop();
    }
    // An oversized first hit is complete in an immutable overflow artifact, including its
    // continuation. It is never silently skipped and cannot trap the caller on an empty page.
    common::enforce_budget(service, result, request.max_bytes)
}

fn render(
    row: RankedEvidence,
    excerpt_chars: usize,
) -> Result<(SearchHit, Evidence), serde_json::Error> {
    let evidence_id = format!(
        "ev_{}",
        enrichment_core::canonical::digest_hex(&serde_json::json!([
            row.key.candidate_id,
            row.source
        ]))
    );
    let value = serde_json::to_value(&row.source.locator)?;
    let locator = match value {
        serde_json::Value::Object(value) => value,
        _ => {
            return Err(serde::ser::Error::custom(
                "typed locator must encode as an object",
            ));
        }
    };
    let excerpt = common::truncate(&row.excerpt, excerpt_chars);
    let citation = Evidence {
        evidence_id: evidence_id.clone(),
        subject: row.key.subject.clone(),
        artifact_id: row.source.artifact_id.clone(),
        source_uri: row
            .source
            .source_uri
            .clone()
            .unwrap_or_else(|| common::artifact_uri_for(&row.source.artifact_id)),
        locator,
        evidence_class: row.source.evidence_class,
        source_version_match: row.source.source_version_match,
        producer: row.source.extractor,
        producer_version: row.source.extractor_version,
        excerpt: excerpt.clone(),
    };
    Ok((
        SearchHit {
            hit: row.hit,
            score: row.key.score,
            factors: row.factors,
            evidence_id,
            path: row.path,
            symbol_kind: row.symbol_kind,
            signature: row.signature,
            also_at: row.also_at,
            deprecated: row.deprecated,
            fragment_kind: row.fragment_kind,
            subject: Some(row.key.subject),
            excerpt,
        },
        citation,
    ))
}
