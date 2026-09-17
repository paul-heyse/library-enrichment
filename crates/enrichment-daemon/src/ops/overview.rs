//! `library.overview`: discover capabilities without knowing symbol names (blueprint §7,
//! `library_overview`; §11.2, the breadth pass).
//!
//! A namespace tree with per-kind counts and a bounded sample per namespace, the feature map,
//! documentation and release-note headings and example names -- never a flat dump of every
//! symbol. Definitions are counted once however many paths reach them (gate R07).

use enrichment_core::evidence::EvidenceKind;
use enrichment_core::request::OverviewRequest;
use enrichment_core::search::row_page::RowCursor;
use enrichment_core::wire::data::{OverviewData, SnapshotSummary};
use enrichment_core::wire::research::DiscoverySelection;
use enrichment_core::wire::{
    Envelope, ErrorCode, Freshness, Page, RecoveryAction, SourceVersionMatch,
};

use super::common::{self, evidence_from_fragment};
use crate::envelope::Research;
use crate::service::Service;

/// Build the overview.
pub async fn overview(service: &Service, request: OverviewRequest) -> Envelope {
    let selections = request
        .discovery
        .clone()
        .unwrap_or_else(DiscoverySelection::defaults);
    if let Err(error) = DiscoverySelection::validate(&selections) {
        return crate::envelope::error(
            ErrorCode::UnsupportedFormat,
            error,
            "Use bounded, unique discovery selections.",
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
    let per_namespace = request
        .max_items
        .map_or(service.config.limits.namespace_entries, |m| {
            m.clamp(1, service.config.limits.namespace_entries)
        });
    let area = request
        .area
        .as_deref()
        .map(str::trim)
        .filter(|a| !a.is_empty());
    let raw = match opened.reader.overview(area, per_namespace).await {
        Ok(raw) => raw,
        Err(err) => return common::query_error(&err),
    };
    let manifest = opened.reader.manifest().clone();

    let budget = common::byte_budget(service, request.max_bytes);
    let namespaces = raw.namespaces;
    let truncated_namespaces = raw.truncated_namespaces;
    let mut discovery = Vec::new();
    for selection in selections {
        let kind = selection.kind;
        discovery.push(
            match discovery_facet(&opened, &request, selection, budget).await {
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

    let source_version_match =
        if manifest.crate_version.as_deref() == Some(opened.release.key.version.as_str()) {
            SourceVersionMatch::Exact
        } else if manifest.crate_version.is_none() {
            SourceVersionMatch::Unknown
        } else {
            SourceVersionMatch::Mismatched
        };

    // Cite only the selected discovery fragments; namespace counts come from the admitted
    // snapshot. An ancillary module-doc query must not override independent facet outcomes.
    let excerpt_chars = service.config.limits.excerpt_characters;
    let mut evidence = Vec::new();
    for fragment in discovery
        .iter()
        .flat_map(|facet| &facet.items)
        .take(per_namespace)
    {
        match evidence_from_fragment(&fragment.fragment, excerpt_chars) {
            Ok(citation) => evidence.push(citation),
            Err(error) => return common::operation_error(&error, "citation_identity"),
        }
    }

    let mut data = OverviewData {
        crate_name: manifest.crate_name.clone(),
        crate_version: manifest.crate_version.clone(),
        snapshot: SnapshotSummary {
            snapshot_id: manifest.snapshot_id.to_string(),
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

    let mut limitations = vec![
        "Discovery facets describe library-level source documents; area narrows the namespace tree. Each facet has its own continuation.".into(),
        "Counts and samples describe the documented build (observed_configuration), not the \
         calling project's feature set or target."
            .to_owned(),
        "Namespace samples are bounded; `truncated_children` and `truncated_namespaces` say \
         how much was left out. Use `search_evidence` to reach the rest."
            .to_owned(),
    ];
    if data.unresolved_reexports > 0 {
        limitations.push(format!(
            "{} re-export(s) point outside this crate and are listed by source path only.",
            data.unresolved_reexports
        ));
    }
    let mut kinds = vec![EvidenceKind::PublicApi];
    for facet in &data.discovery {
        use enrichment_core::wire::research::DiscoveryKind;
        kinds.push(match facet.kind {
            DiscoveryKind::Features => EvidenceKind::RegistryMetadata,
            DiscoveryKind::Documentation => EvidenceKind::Documentation,
            DiscoveryKind::ReleaseNotes => EvidenceKind::ReleaseNotes,
            DiscoveryKind::Examples => EvidenceKind::Examples,
        });
    }
    let mut coverage = match opened
        .reader
        .assess(
            &kinds,
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
    for facet in &mut data.discovery {
        if facet.state == enrichment_core::wire::AspectState::Absent {
            let kind = match facet.kind {
                enrichment_core::wire::research::DiscoveryKind::Features => {
                    EvidenceKind::RegistryMetadata
                }
                enrichment_core::wire::research::DiscoveryKind::Documentation => {
                    EvidenceKind::Documentation
                }
                enrichment_core::wire::research::DiscoveryKind::ReleaseNotes => {
                    EvidenceKind::ReleaseNotes
                }
                enrichment_core::wire::research::DiscoveryKind::Examples => EvidenceKind::Examples,
            };
            if coverage.assessments.iter().any(|assessment| {
                assessment.kind == kind
                    && assessment.state != enrichment_core::wire::ScopeState::Indexed
            }) {
                facet.state = enrichment_core::wire::AspectState::Unavailable;
                facet.reason =
                    Some("No retained match; this facet lacks complete qualified coverage.".into());
            } else {
                facet.reason = Some("No retained match in this qualified discovery scope.".into());
            }
        }
    }
    let partial = !coverage.complete()
        || data
            .discovery
            .iter()
            .any(|facet| facet.state == enrichment_core::wire::AspectState::Failed);
    coverage.limitations.extend(limitations);
    let research = Research {
        summary: format!(
            "{} {}: {} definitions across {} namespace(s); {} retained discovery fragments in this page.",
            opened.release.key.package,
            opened.release.key.version,
            data.definitions_by_kind.values().sum::<u64>(),
            data.namespaces.len() as u64 + truncated_namespaces,
            data.discovery
                .iter()
                .map(|facet| facet.items.len())
                .sum::<usize>()
        ),
        data: common::payload(&data),
        coverage,
        freshness: Freshness {
            registry_checked_at: None,
            source_version_match,
            latest_verified: false,
        },
        context_id: Some(opened.context.context_id.to_string()),
        snapshot_id: Some(opened.snapshot_id.to_string()),
        evidence,
        artifacts: Vec::new(),
    };
    if partial {
        research.partial()
    } else {
        research.ok()
    }
}

fn discovery_digest(selection: &DiscoverySelection, budget: usize) -> String {
    enrichment_core::canonical::digest_hex(&serde_json::json!([
        "library-discovery/2",
        selection.kind,
        selection.max_items,
        selection.max_characters,
        budget,
    ]))
}

async fn discovery_facet(
    opened: &common::Opened,
    request: &OverviewRequest,
    selection: DiscoverySelection,
    budget: usize,
) -> Result<enrichment_core::wire::data::DiscoveryFacet, Box<Envelope>> {
    let manifest = opened.reader.manifest();
    let digest = discovery_digest(&selection, budget);
    let after = match selection
        .cursor
        .as_deref()
        .map(|cursor| RowCursor::decode(cursor, manifest.snapshot_id.as_str(), &digest))
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
        .map(|key| RowCursor::encode(manifest.snapshot_id.as_str(), &digest, key.clone()))
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
    let mut items = Vec::new();
    for (fragment, text_complete) in page.items {
        let complete = if text_complete {
            None
        } else {
            let mut full = selection.clone();
            full.max_characters = None;
            let full_digest = discovery_digest(&full, budget);
            full.cursor = after.as_ref().map(|key| {
                RowCursor::encode(manifest.snapshot_id.as_str(), &full_digest, key.clone())
                    .expect("bounded retained identity serializes")
            });
            Some(RecoveryAction::CallTool {
                request: Box::new(enrichment_core::request::ResearchRequest::Overview(
                    OverviewRequest {
                        context_id: request.context_id.clone(),
                        snapshot_id: Some(manifest.snapshot_id.to_string()),
                        area: request.area.clone(),
                        max_items: request.max_items,
                        max_bytes: request.max_bytes,
                        discovery: Some(vec![full]),
                    },
                )),
            })
        };
        items.push(enrichment_core::wire::data::FragmentProjection {
            fragment,
            text_complete,
            complete,
        });
    }
    Ok(enrichment_core::wire::data::DiscoveryFacet {
        kind: selection.kind,
        state: if items.is_empty() {
            enrichment_core::wire::AspectState::Absent
        } else {
            enrichment_core::wire::AspectState::Available
        },
        reason: None,
        diagnostic: None,
        items,
        page: Some(accounting),
    })
}
