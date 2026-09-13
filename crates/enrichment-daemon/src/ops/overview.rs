//! `library.overview`: discover capabilities without knowing symbol names (blueprint §7,
//! `library_overview`; §11.2, the breadth pass).
//!
//! A namespace tree with per-kind counts and a bounded sample per namespace, the feature map,
//! documentation and release-note headings and example names -- never a flat dump of every
//! symbol. Definitions are counted once however many paths reach them (gate R07).

use enrichment_core::evidence::{EvidenceKind, FragmentKind};
use enrichment_core::request::OverviewRequest;
use enrichment_core::wire::data::{NamespaceFacet, OverviewChild, OverviewData, SnapshotSummary};
use enrichment_core::wire::{Coverage, Envelope, Freshness, SourceVersionMatch};

use super::common::{self, evidence_from_fragment};
use crate::envelope::Research;
use crate::service::Service;

/// Build the overview.
pub async fn overview(service: &Service, request: OverviewRequest) -> Envelope {
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

    // Bound the namespace list by the byte budget: fewer complete namespaces, never a cut one.
    let budget = common::byte_budget(service, request.max_bytes);
    let mut namespaces: Vec<NamespaceFacet> = Vec::new();
    let mut truncated_namespaces = 0u64;
    let mut used = 2048usize; // headroom for the rest of the payload
    for ns in raw.namespaces {
        let facet = NamespaceFacet {
            path: ns.path,
            doc_summary: ns.doc_summary,
            counts_by_kind: ns.counts_by_kind,
            children: ns
                .children
                .into_iter()
                .map(|c| OverviewChild {
                    path: c.path,
                    kind: c.kind,
                    doc_summary: c.doc_summary,
                    is_reexport: c.is_reexport,
                    deprecated: c.deprecated,
                })
                .collect(),
            truncated_children: ns.truncated_children,
        };
        let size = common::json_size(&facet);
        if used + size > budget && !namespaces.is_empty() {
            truncated_namespaces += 1;
            continue;
        }
        used += size;
        namespaces.push(facet);
    }

    let source_version_match =
        if manifest.crate_version.as_deref() == Some(opened.release.key.version.as_str()) {
            SourceVersionMatch::Exact
        } else if manifest.crate_version.is_none() {
            SourceVersionMatch::Unknown
        } else {
            SourceVersionMatch::Mismatched
        };

    // Cite the module docs and the feature table as evidence, bounded.
    let excerpt_chars = service.config.limits.excerpt_characters;
    let mut evidence = Vec::new();
    if let Ok(fragments) = opened.reader.fragments_for(&manifest.crate_name).await {
        for fragment in fragments
            .iter()
            .filter(|f| f.kind == FragmentKind::DocText)
            .take(1)
        {
            evidence.push(evidence_from_fragment(
                service,
                fragment,
                source_version_match,
                excerpt_chars,
            ));
        }
    }
    if let Ok(fragments) = opened
        .reader
        .fragments_of_kind(FragmentKind::FeatureDefinition)
        .await
    {
        for fragment in fragments.iter().take(per_namespace) {
            evidence.push(evidence_from_fragment(
                service,
                fragment,
                source_version_match,
                excerpt_chars,
            ));
        }
    }

    let data = OverviewData {
        crate_name: manifest.crate_name.clone(),
        crate_version: manifest.crate_version.clone(),
        snapshot: SnapshotSummary {
            snapshot_id: manifest.snapshot_id.to_string(),
            normalizer_version: manifest.normalizer_version.clone(),
            counts: manifest.counts.clone(),
            published_at: manifest.published_at.clone(),
        },
        observed_configuration: manifest.observed_configuration.clone(),
        area: area.map(str::to_owned),
        definitions_by_kind: raw.definitions_by_kind,
        namespaces,
        truncated_namespaces,
        features: raw.features,
        documentation_headings: raw.documentation_headings,
        release_note_headings: raw.release_note_headings,
        examples: raw.examples,
        reexports: raw.reexports,
        unresolved_reexports: raw.unresolved_reexports,
    };

    let mut limitations = vec![
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
    let mut missing: std::collections::BTreeSet<String> = manifest
        .missing
        .iter()
        .map(|k| k.as_str().to_owned())
        .collect();
    if !manifest.indexed.contains(&EvidenceKind::ReleaseNotes) {
        missing.insert(EvidenceKind::ReleaseNotes.as_str().to_owned());
    }
    let partial = !missing.is_empty();
    let coverage = Coverage {
        scope: format!(
            "module tree, feature map and document headings of {} {} from snapshot {}",
            opened.release.key.package, opened.release.key.version, manifest.snapshot_id
        ),
        indexed: manifest
            .indexed
            .iter()
            .map(|k| k.as_str().to_owned())
            .collect(),
        missing,
        limitations,
    };
    let research = Research {
        summary: format!(
            "{} {}: {} definitions across {} namespace(s); {} feature(s), {} example(s).",
            opened.release.key.package,
            opened.release.key.version,
            data.definitions_by_kind.values().sum::<u64>(),
            data.namespaces.len() as u64 + truncated_namespaces,
            data.features.len(),
            data.examples.len()
        ),
        data: common::to_object(&data),
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
