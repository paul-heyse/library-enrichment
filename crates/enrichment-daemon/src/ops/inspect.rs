//! `symbol.inspect`: characterize a known candidate and its deployment requirements
//! (blueprint §7, `inspect_symbol`; §7.1 inspection requirements; §4.3 availability).
//!
//! Defaults to the public API plus a small documentation section. Availability is reported as
//! separate states -- documented in the observed build versus verified for the project -- and
//! no feature predicate is invented (gates R05, R06). Source depth reads a bounded excerpt from
//! the extracted crate archive, confined to it. LSP depth is a later phase and is named as
//! such rather than silently degraded.

use enrichment_core::evidence::{
    ArtifactKind, Availability, EvidenceKind, FragmentKind, ObservedConfiguration,
    RequestedConfiguration, Symbol,
};
use enrichment_core::producer::source;
use enrichment_core::request::{InspectDepth, InspectRequest};
use enrichment_core::wire::data::InspectData;
use enrichment_core::wire::{Coverage, Envelope, ErrorCode, Freshness, SourceVersionMatch};

use super::common::{self, evidence_from_fragment};
use crate::envelope::{self, Research};
use crate::service::Service;

/// Aspects this build can return.
pub const ASPECTS: &[&str] = &[
    "signature",
    "availability",
    "relationships",
    "documentation",
    "examples",
    "source",
];

/// Inspect one symbol.
pub async fn inspect(service: &Service, request: InspectRequest) -> Envelope {
    let wanted = request.symbol_path.trim();
    if wanted.is_empty() {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            "`symbol_path` must not be empty",
            "Pass a qualified path such as `serde::de::Deserialize`, or a bare name.",
            false,
        );
    }
    let aspects: Vec<String> = request
        .aspects
        .clone()
        .unwrap_or_else(|| ASPECTS.iter().map(|a| (*a).to_owned()).collect());
    if let Some(unknown) = aspects
        .iter()
        .find(|a| !ASPECTS.contains(&a.as_str()) && a.as_str() != "semantics")
    {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            format!("`{unknown}` is not an aspect"),
            format!("Use any of: {}, semantics.", ASPECTS.join(", ")),
            false,
        );
    }
    let want = |a: &str| aspects.iter().any(|x| x == a);

    let opened =
        match common::open_context(service, &request.context_id, request.snapshot_id.as_deref())
            .await
        {
            Ok(opened) => opened,
            Err(envelope) => return *envelope,
        };
    let manifest = opened.reader.manifest().clone();

    // Exact path first; a bare name is accepted only when it names one definition.
    let mut matches = match opened.reader.symbols_at(wanted).await {
        Ok(m) => m,
        Err(err) => return common::query_error(&err),
    };
    if matches.is_empty() && !wanted.contains("::") {
        matches = match opened.reader.symbols_ending_with(wanted).await {
            Ok(m) => m,
            Err(err) => return common::query_error(&err),
        };
    }
    let mut definitions: Vec<String> = matches.iter().map(|s| s.definition_id.clone()).collect();
    definitions.sort();
    definitions.dedup();

    let source_version_match =
        if manifest.crate_version.as_deref() == Some(opened.release.key.version.as_str()) {
            SourceVersionMatch::Exact
        } else {
            SourceVersionMatch::Unknown
        };
    let freshness = Freshness {
        registry_checked_at: None,
        source_version_match,
        latest_verified: false,
    };
    let context_id = Some(opened.context.context_id.to_string());
    let snapshot_id = Some(opened.snapshot_id.to_string());

    if matches.is_empty() {
        return envelope::error(
            ErrorCode::ArtifactUnavailable,
            format!(
                "`{wanted}` is not a public path in {} {} as documented",
                opened.release.key.package, opened.release.key.version
            ),
            "Use `search_evidence` or `library_overview` to find the path; absence from the \
             documented build is not proof the item does not exist under another feature set \
             or target.",
            false,
        );
    }
    if definitions.len() > 1 {
        let mut candidates: Vec<String> = matches.iter().map(|s| s.path.clone()).collect();
        candidates.sort();
        candidates.dedup();
        let data = InspectData {
            symbol: None,
            docs_truncated: false,
            also_at: Vec::new(),
            candidates: candidates.clone(),
            aspects: Vec::new(),
            availability: None,
            relationships: Vec::new(),
            fragments: Vec::new(),
            source: None,
        };
        return Research {
            summary: format!(
                "`{wanted}` is ambiguous: {} definitions match.",
                definitions.len()
            ),
            data: common::to_object(&data),
            coverage: Coverage {
                scope: format!("candidate paths for `{wanted}`"),
                indexed: [EvidenceKind::PublicApi.as_str().to_owned()]
                    .into_iter()
                    .collect(),
                missing: std::collections::BTreeSet::new(),
                limitations: vec![
                    "The reference matched more than one definition; nothing was guessed. Pass \
                     one of `candidates` as a qualified path."
                        .to_owned(),
                ],
            },
            freshness,
            context_id,
            snapshot_id,
            evidence: Vec::new(),
            artifacts: Vec::new(),
        }
        .partial();
    }

    // One definition: prefer the definition's own path, list the others.
    matches.sort_by(|a, b| {
        a.is_reexport
            .cmp(&b.is_reexport)
            .then_with(|| a.path.cmp(&b.path))
    });
    let selected: Symbol = matches[0].clone();
    let also_at: Vec<String> = matches
        .iter()
        .skip(1)
        .map(|s| s.path.clone())
        .chain(
            opened
                .reader
                .all_symbols()
                .await
                .unwrap_or_default()
                .into_iter()
                .filter(|s| s.definition_id == selected.definition_id && s.path != selected.path)
                .map(|s| s.path),
        )
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    let excerpt_chars = service.config.limits.excerpt_characters;
    let budget = common::byte_budget(service, request.max_bytes);
    let mut evidence = Vec::new();
    let mut fragments = Vec::new();
    let mut returned_aspects = Vec::new();

    // signature: always cheap, from the normalized evidence -- never LSP (gate C17 premise).
    let symbol_fragments = opened
        .reader
        .fragments_for(&selected.path)
        .await
        .unwrap_or_default();
    if want("signature") {
        returned_aspects.push("signature".to_owned());
        for f in symbol_fragments
            .iter()
            .filter(|f| f.kind == FragmentKind::ApiSignature)
        {
            evidence.push(evidence_from_fragment(
                service,
                f,
                source_version_match,
                excerpt_chars,
            ));
            fragments.push(f.clone());
        }
    }

    let mut symbol = selected.clone();
    let mut docs_truncated = false;
    match request.depth {
        InspectDepth::Signature => {
            symbol.docs = None;
        }
        InspectDepth::Documentation | InspectDepth::Source => {
            if want("documentation") {
                returned_aspects.push("documentation".to_owned());
                let cap = budget / 3;
                if let Some(docs) = &symbol.docs
                    && docs.len() > cap
                {
                    symbol.docs = Some(common::truncate(docs, cap));
                    docs_truncated = true;
                }
                for f in symbol_fragments
                    .iter()
                    .filter(|f| f.kind == FragmentKind::DocText)
                {
                    evidence.push(evidence_from_fragment(
                        service,
                        f,
                        source_version_match,
                        excerpt_chars,
                    ));
                }
            } else {
                symbol.docs = None;
            }
        }
    }

    let mut relationships = Vec::new();
    if want("relationships") {
        returned_aspects.push("relationships".to_owned());
        relationships = opened
            .reader
            .relationships_for(&selected.symbol_id)
            .await
            .unwrap_or_default();
        relationships.truncate(service.config.limits.namespace_entries * 2);
    }

    if want("examples") {
        returned_aspects.push("examples".to_owned());
        let name = selected.name.clone();
        if let Ok(examples) = opened
            .reader
            .fragments_matching_any(std::slice::from_ref(&name), Some(&[FragmentKind::Example]))
            .await
        {
            for f in examples.iter().take(3) {
                evidence.push(evidence_from_fragment(
                    service,
                    f,
                    source_version_match,
                    excerpt_chars,
                ));
                fragments.push(f.clone());
            }
        }
    }

    let mut missing = std::collections::BTreeSet::new();
    let mut limitations = Vec::new();
    let mut availability = None;
    if want("availability") {
        returned_aspects.push("availability".to_owned());
        let observed: ObservedConfiguration = manifest.observed_configuration.clone();
        let requested = RequestedConfiguration {
            features: (!opened.environment.features.is_empty())
                .then(|| opened.environment.features.clone()),
            default_features: opened.environment.default_features,
            target: opened.environment.target.clone(),
        };
        availability = Some(Availability::assess(
            observed,
            requested,
            &selected.cfg_hints,
        ));
    }

    let mut source_excerpt = None;
    if request.depth == InspectDepth::Source && want("source") {
        returned_aspects.push("source".to_owned());
        match source_for(service, &opened, &selected) {
            Some(excerpt) => source_excerpt = Some(excerpt),
            None => {
                missing.insert(EvidenceKind::SourceExcerpts.as_str().to_owned());
                limitations.push(
                    "The recorded span could not be located in the extracted crate archive; \
                     the definition may live in a generated or external file."
                        .to_owned(),
                );
            }
        }
    }
    if want("semantics") {
        missing.insert("semantics".to_owned());
        limitations.push(
            "Semantic observations (rust-analyzer) are not implemented in this build; they \
             land in phase 4. The signature above comes from normalized evidence."
                .to_owned(),
        );
    }
    limitations.push(
        "Availability describes the documented build; whether the project has this item \
         depends on its features and target, which are reported separately."
            .to_owned(),
    );

    let data = InspectData {
        symbol: Some(symbol),
        docs_truncated,
        also_at,
        candidates: Vec::new(),
        aspects: returned_aspects,
        availability,
        relationships,
        fragments,
        source: source_excerpt,
    };
    let partial = !missing.is_empty();
    let research = Research {
        summary: format!(
            "{} `{}`{}{}",
            selected.kind.as_str(),
            selected.path,
            selected
                .deprecated
                .as_ref()
                .map(|_| " (deprecated)")
                .unwrap_or(""),
            selected
                .doc_summary
                .as_deref()
                .map(|d| format!(": {d}"))
                .unwrap_or_default()
        ),
        data: common::to_object(&data),
        coverage: Coverage {
            scope: format!(
                "{} in snapshot {} of {} {}",
                selected.path,
                manifest.snapshot_id,
                opened.release.key.package,
                opened.release.key.version
            ),
            indexed: manifest
                .indexed
                .iter()
                .map(|k| k.as_str().to_owned())
                .collect(),
            missing,
            limitations,
        },
        freshness,
        context_id,
        snapshot_id,
        evidence,
        artifacts: Vec::new(),
    };
    if partial {
        research.partial()
    } else {
        research.ok()
    }
}

/// Cut a bounded source excerpt for a symbol from the extracted crate tarball.
fn source_for(
    service: &Service,
    opened: &common::Opened,
    symbol: &Symbol,
) -> Option<source::SourceExcerpt> {
    let file = symbol.span_file.as_deref()?;
    let line = symbol.span_line? as usize;
    // The tarball artifact is recorded with the release's resolution; its digest names the
    // extraction directory.
    let stored: serde_json::Value = service
        .catalog
        .document(&opened.context.context_id, "resolution")
        .ok()
        .flatten()?;
    let tarball_kind = serde_json::to_value(ArtifactKind::CrateTarball).ok()?;
    let tarball_sha = stored["data"]["artifacts"]
        .as_array()?
        .iter()
        .find(|a| a["kind"] == tarball_kind)
        .and_then(|a| a["sha256"].as_str())?
        .to_owned();
    let unpacked = service.paths.unpacked().join(tarball_sha);
    let crate_root = std::fs::read_dir(&unpacked)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.is_dir())?;
    // rustdoc records spans relative to the crate root for hosted builds; strip any prefix up
    // to a component that exists under the extracted root.
    let candidates = std::iter::once(file.to_owned()).chain(
        file.match_indices('/')
            .map(|(i, _)| file[i + 1..].to_owned()),
    );
    for candidate in candidates {
        if crate_root.join(&candidate).is_file() {
            return source::source_excerpt(
                &crate_root,
                &candidate,
                line,
                service.config.limits.source_lines,
            );
        }
    }
    None
}
