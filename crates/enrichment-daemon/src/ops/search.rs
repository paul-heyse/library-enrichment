//! Native planned lexical search with complete-key pagination and bounded wire rendering.
use super::common;
use crate::{
    envelope::{self, Research},
    service::Service,
};
use enrichment_core::{
    request::SearchRequest,
    search::{self, page::SearchCursor, spec::SearchSpec},
    wire::{
        Envelope, ErrorCode, Freshness, Page,
        data::{ScoreFactor, SearchData},
    },
};
use enrichment_store::search_plan::{self, SearchOptions};

pub async fn search(service: &Service, request: SearchRequest) -> Envelope {
    let selection = match enrichment_store::search_policy::select(
        &service.repository.runtime,
        &request,
        service.config.limits.search_results,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "search_selection"),
    };
    let query = selection.query;
    let kinds = selection.kinds;
    let tokens = match enrichment_store::scoring::tokens(&service.repository.runtime, &query).await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "search_tokens"),
    };
    let opened = match common::open_context(
        service,
        &request.context_id,
        request.snapshot_id.as_ref(),
    )
    .await
    {
        Ok(value) => value,
        Err(e) => return *e,
    };
    let area = match selection
        .area
        .as_deref()
        .map(|s| opened.reader.area(s))
        .transpose()
    {
        Ok(value) => value,
        Err(e) => return common::query_error(&e),
    };
    let scope = enrichment_core::search::page::SearchScope {
        context_id: opened.context.context_id.clone(),
        snapshot_id: opened.snapshot_id.clone(),
    };
    let budget = match common::byte_budget(service, request.max_bytes).await {
        Ok(value) => value,
        Err(error) => return common::query_error(&error),
    };
    let spec = SearchSpec::new(&query);
    let digest = enrichment_core::operation::selections::SearchSelection {
        witness: service.selection_witness(),
        spec: spec.clone(),
        kinds: kinds.clone(),
        area: area.clone(),
        max_items: request.max_items,
        max_bytes: budget,
    }
    .identity();
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
    let page = match search_plan::page(
        opened.reader.session(),
        opened.reader.runtime(),
        &spec,
        &SearchOptions {
            include_api: selection.include_api,
            fragment_kinds: selection.fragment_kinds,
            area,
            page_size: selection.page_size,
            after: cursor.map(|c| c.after),
            offset,
        },
        service.config.limits.excerpt_characters,
    )
    .await
    {
        Ok(value) => value,
        Err(e) => return common::query_error(&e.into()),
    };
    let searched = kinds.clone();
    let last_key = page.rows.last().map(|row| row.key.clone());
    let (hits, evidence) = page
        .rows
        .into_iter()
        .map(|row| (row.hit, row.citation))
        .unzip();
    let mut data = SearchData {
        page: Page::default(),
        query: query.clone(),
        tokens,
        kinds,
        area: selection.area,
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
    let mut coverage = match opened
        .reader
        .assess(
            &selection.evidence_kinds,
            None,
            format!("{} in snapshot {}", searched.join(", "), opened.snapshot_id),
        )
        .await
    {
        Ok(value) => value,
        Err(e) => return common::query_error(&e),
    };
    coverage.limitations.push("Lexical matching within the pinned evidence scope; an empty result does not establish that a capability is absent.".into());
    let partial =
        match enrichment_store::coverage::complete(opened.reader.runtime(), &coverage).await {
            Ok(complete) => !complete,
            Err(error) => return common::operation_error(&error, "search_coverage"),
        };
    let research = Research {
        summary: format!(
            "{} matches for the requested lexical query in {}; {} results already returned.",
            page.total,
            searched.join(", "),
            offset
        ),
        data: common::payload(&data),
        coverage,
        freshness: Freshness {
            registry_checked_at: None,
            latest_verified: false,
            source_version_match: match enrichment_store::research_outcomes::source_version(
                &service.repository.runtime,
                opened.context.mode,
                manifest.crate_version.as_deref(),
                &opened.release.key.version,
            )
            .await
            {
                Ok(value) => value,
                Err(error) => return common::operation_error(&error, "source_version_scope"),
            },
        },
        context_id: Some(opened.context.context_id.clone()),
        snapshot_id: Some(opened.snapshot_id.clone()),
        evidence,
        artifacts: vec![],
    };
    let mut result = research.ok_with_page(Page::new(0, Some(page.total), false, None));
    let returned = page.boundary.returned;
    let more = page.boundary.has_more;
    let next_cursor = if more && let Some(key) = last_key {
        match SearchCursor::new(
            scope.clone(),
            digest.clone(),
            page.boundary.next_offset,
            key,
        )
        .encode()
        {
            Ok(value) => Some(value),
            Err(e) => return common::operation_error(&e, "search_projection"),
        }
    } else {
        None
    };
    data.page = Page::new(returned, page.boundary.total, more, next_cursor);
    result.data = common::payload(&data);
    if partial {
        result = result.into_partial();
    }

    // An oversized first hit is complete in an immutable overflow artifact, including its
    // continuation. It is never silently skipped and cannot trap the caller on an empty page.
    common::enforce_budget(service, result, request.max_bytes).await
}
