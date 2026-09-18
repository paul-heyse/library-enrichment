//! `library.overview`: discover capabilities without knowing symbol names (blueprint §7,
//! `library_overview`; §11.2, the breadth pass).
//!
//! A namespace tree with per-kind counts and a bounded sample per namespace, the feature map,
//! documentation and release-note headings and example names -- never a flat dump of every
//! symbol. Definitions are counted once however many paths reach them (gate R07).

use enrichment_core::request::OverviewRequest;
use enrichment_core::search::row_page::RowCursor;
use enrichment_core::wire::data::{OverviewData, SnapshotSummary};
use enrichment_core::wire::research::DiscoverySelection;
use enrichment_core::wire::{Envelope, ErrorCode, Freshness, Page, RecoveryAction};

use super::common;
use crate::envelope::Research;
use crate::service::Service;

/// Build the overview.
pub async fn overview(service: &Service, request: OverviewRequest) -> Envelope {
    let selections = match enrichment_store::research_selection::discovery(
        &service.repository.runtime,
        &request.discovery,
    )
    .await
    {
        Ok(selections) => selections,
        Err(error) => return common::operation_error(&error, "research_selection"),
    };

    let opened = match common::open_context(
        service,
        &request.context_id,
        request.snapshot_id.as_ref(),
    )
    .await
    {
        Ok(opened) => opened,
        Err(envelope) => return *envelope,
    };
    let overview_selection = match enrichment_store::research_overview::select(
        &service.repository.runtime,
        request.area.as_deref(),
        request.max_items,
        service.config.limits.namespace_entries,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "overview_selection"),
    };
    let per_namespace = overview_selection.per_namespace;
    let area = overview_selection.area.as_deref();
    let raw = match opened.reader.overview(area, per_namespace).await {
        Ok(raw) => raw,
        Err(err) => return common::query_error(&err),
    };
    let manifest = opened.reader.manifest().clone();

    let budget = match common::byte_budget(service, request.max_bytes).await {
        Ok(value) => value,
        Err(error) => return common::query_error(&error),
    };
    let namespaces = raw.namespaces;
    let truncated_namespaces = raw.truncated_namespaces;
    let mut discovery = Vec::new();
    for selection in selections {
        let kind = selection.kind;
        discovery.push(
            match discovery_facet(
                &opened,
                &request,
                selection,
                budget,
                &service.selection_witness(),
            )
            .await
            {
                Ok(facet) => facet,
                Err(error) => enrichment_core::wire::data::DiscoveryFacet {
                    kind,
                    state: enrichment_core::wire::AspectState::Failed,
                    reason: Some(error.summary.clone()),
                    diagnostic: error.error().map(|detail| detail.diagnostic.clone()),
                    items: Vec::new(),
                    page: None,
                },
            },
        );
    }

    let source_version_match = match enrichment_store::research_outcomes::source_version(
        &service.repository.runtime,
        opened.context.mode,
        manifest.crate_version.as_deref(),
        &opened.release.key.version,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "source_version_scope"),
    };

    // Cite only the selected discovery fragments; namespace counts come from the admitted
    // snapshot. An ancillary module-doc query must not override independent facet outcomes.
    let excerpt_chars = service.config.limits.excerpt_characters;
    let evidence = match enrichment_store::research_citations::fragments(
        &service.repository.runtime,
        discovery
            .iter()
            .flat_map(|facet| facet.items.iter().map(|item| &item.fragment))
            .collect(),
        per_namespace,
        excerpt_chars,
    )
    .await
    {
        Ok(evidence) => evidence,
        Err(error) => return common::operation_error(&error, "citation_projection"),
    };

    let mut data = OverviewData {
        crate_name: manifest.crate_name.clone(),
        crate_version: manifest.crate_version.clone(),
        snapshot: SnapshotSummary {
            snapshot_id: manifest.snapshot_id.clone(),
            normalizer_version: manifest.normalizer_version.clone(),
            counts: manifest.counts.clone(),
            published_at: manifest.published_at,
        },
        observed_configuration: manifest.observed_configuration.clone(),
        area: area.map(str::to_owned),
        definitions_by_kind: raw.definitions_by_kind,
        namespaces,
        truncated_namespaces,
        discovery,
        reexports: raw.reexports,
        unresolved_reexports: raw.unresolved_reexports,
    };

    let presentation = match enrichment_store::research_overview::present(
        &service.repository.runtime,
        &data,
        &opened.release.key.package,
        &opened.release.key.version,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "overview_presentation"),
    };
    let mut coverage = match opened
        .reader
        .assess(
            &presentation.kinds,
            None,
            format!(
                "overview of {} {} from snapshot {}",
                opened.release.key.package, opened.release.key.version, manifest.snapshot_id
            ),
        )
        .await
    {
        Ok(value) => value,
        Err(e) => return common::query_error(&e),
    };
    let disposition = match enrichment_store::research_outcomes::discovery(
        &service.repository.runtime,
        data.discovery,
        &coverage,
    )
    .await
    {
        Ok(disposition) => disposition,
        Err(error) => return common::operation_error(&error, "discovery_outcomes"),
    };
    data.discovery = disposition.facets;
    let partial = disposition.partial;
    coverage.limitations.extend(presentation.limitations);
    let research = Research {
        summary: presentation.summary,
        data: common::payload(&data),
        coverage,
        freshness: Freshness {
            registry_checked_at: None,
            source_version_match,
            latest_verified: false,
        },
        context_id: Some(opened.context.context_id.clone()),
        snapshot_id: Some(opened.snapshot_id.clone()),
        evidence,
        artifacts: Vec::new(),
    };
    if partial {
        research.partial()
    } else {
        research.ok()
    }
}

fn discovery_digest(
    witness: &enrichment_core::operation::selections::SelectionWitness,
    selection: &DiscoverySelection,
    budget: usize,
) -> String {
    enrichment_core::operation::selections::DiscoverySelection {
        witness: witness.clone(),
        kind: selection.kind,
        max_items: selection.max_items,
        max_characters: selection.max_characters,
        max_bytes: budget,
    }
    .identity()
}

async fn discovery_facet(
    opened: &common::Opened,
    request: &OverviewRequest,
    selection: DiscoverySelection,
    budget: usize,
    witness: &enrichment_core::operation::selections::SelectionWitness,
) -> Result<enrichment_core::wire::data::DiscoveryFacet, Box<Envelope>> {
    let manifest = opened.reader.manifest();
    let digest = discovery_digest(witness, &selection, budget);
    let after = match selection
        .cursor
        .as_deref()
        .map(|cursor| RowCursor::decode(cursor, &manifest.snapshot_id, &digest))
        .transpose()
    {
        Ok(value) => value.map(|cursor| cursor.after),
        Err(error) => {
            return Err(Box::new(crate::envelope::error(
                ErrorCode::InvalidCursor,
                error.to_string(),
                "Restart this discovery facet without a cursor for this snapshot and selection.",
                false,
            )));
        }
    };
    let page = match opened
        .reader
        .discovery_page(
            selection.kind.fragment_kind(),
            selection.max_items,
            after.as_deref(),
            selection.max_characters,
        )
        .await
    {
        Ok(page) => page,
        Err(error) => return Err(Box::new(common::query_error(&error))),
    };
    let next_cursor = match page
        .next_key
        .as_ref()
        .map(|key| RowCursor::encode(&manifest.snapshot_id, &digest, key.clone()))
        .transpose()
    {
        Ok(cursor) => cursor,
        Err(error) => {
            return Err(Box::new(common::operation_error(
                &error,
                "discovery_projection",
            )));
        }
    };
    let accounting = Page::new(page.items.len() as u64, None, page.has_more, next_cursor);
    let mut full = selection.clone();
    full.max_characters = None;
    let full_digest = discovery_digest(witness, &full, budget);
    full.cursor = after.as_ref().map(|key| {
        RowCursor::encode(&manifest.snapshot_id, &full_digest, key.clone())
            .expect("bounded retained identity serializes")
    });
    let recovery = RecoveryAction::CallTool {
        request: Box::new(enrichment_core::request::ResearchRequest::Overview(
            OverviewRequest {
                context_id: request.context_id.clone(),
                snapshot_id: Some(manifest.snapshot_id.clone()),
                area: request.area.clone(),
                max_items: request.max_items,
                max_bytes: request.max_bytes,
                discovery: Some(vec![full]),
            },
        )),
    };
    let delivery = enrichment_store::research_fragments::deliver(
        opened.reader.runtime(),
        page.items,
        recovery,
        page.has_more,
    )
    .await
    .map_err(|error| Box::new(common::operation_error(&error, "fragment_delivery")))?;
    Ok(enrichment_core::wire::data::DiscoveryFacet {
        kind: selection.kind,
        state: enrichment_core::wire::AspectState::Available,
        reason: None,
        diagnostic: None,
        items: delivery.items,
        page: Some(accounting),
    })
}
