//! `symbol.inspect`: characterize a known candidate and its deployment requirements
//! (blueprint §7, `inspect_symbol`; §7.1 inspection requirements; §4.3 availability).
//!
//! Defaults to the public API plus a small documentation section. Availability is reported as
//! separate states -- documented in the observed build versus verified for the project -- and
//! no feature predicate is invented (gates R05, R06). Source depth reads a bounded excerpt from
//! the extracted crate archive, confined to it. LSP depth is a later phase and is named as
//! such rather than silently degraded.

use enrichment_core::evidence::{
    ArtifactKind, Availability, EvidenceKind, FragmentKind, RequestedConfiguration, Symbol,
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

/// Select retained results first. Only explicit execution intent can schedule a producer.
pub async fn inspect(service: &Service, request: InspectRequest) -> Envelope {
    use enrichment_core::request::InspectionIntent;
    if let Some(options) = &request.execution {
        if let Err(error) = options.validate(service.config.limits.verification_input_bytes) {
            return envelope::error(
                ErrorCode::UnsupportedFormat,
                error,
                "Use bounded typed inspection options.",
                false,
            );
        }
        if options.intent != InspectionIntent::Retained
            && request
                .aspects
                .as_ref()
                .is_some_and(|a| a.iter().any(|a| a == "runtime"))
            && options.runtime.is_none()
        {
            return envelope::error(
                ErrorCode::UnsupportedFormat,
                "Runtime execution requires an explicit module and attribute selection",
                "Set execution.runtime to the exact selected public binding.",
                false,
            );
        }
        if options.intent != InspectionIntent::Retained {
            return super::inspect_execution::submit(service, request).await;
        }
    }
    read(service, request).await
}

/// Inspect one symbol.
pub async fn read(service: &Service, request: InspectRequest) -> Envelope {
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
        .find(|a| !ASPECTS.contains(&a.as_str()) && !matches!(a.as_str(), "semantics" | "runtime"))
    {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            format!("`{unknown}` is not an aspect"),
            format!("Use any of: {}, semantics, runtime.", ASPECTS.join(", ")),
            false,
        );
    }
    let want = |a: &str| aspects.iter().any(|x| x == a);
    let include_docs = want("documentation") && request.depth != InspectDepth::Signature;

    let opened =
        match common::open_context(service, &request.context_id, request.snapshot_id.as_deref())
            .await
        {
            Ok(opened) => opened,
            Err(envelope) => return *envelope,
        };
    let manifest = opened.reader.manifest().clone();

    // Exact path first; a bare name is accepted only when it names one definition.
    let mut matches = match opened
        .reader
        .symbols_at(wanted, request.definition_id.as_deref(), include_docs)
        .await
    {
        Ok(m) => m,
        Err(err) => return common::query_error(&err),
    };
    if matches.is_empty() && !wanted.contains("::") && !wanted.contains('.') {
        matches = match opened
            .reader
            .symbols_ending_with(wanted, request.definition_id.as_deref(), include_docs)
            .await
        {
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
            "Use `search_evidence` or `library_overview` to find the path and its definition_id; \
             an explicit definition_id must belong to that path in this snapshot. Absence from the \
             documented build is not proof the item does not exist under another feature set \
             or target.",
            false,
        );
    }
    if definitions.len() > 1 {
        let mut candidates: Vec<_> = matches
            .iter()
            .map(|s| enrichment_core::wire::data::InspectionCandidate {
                path: s.path.clone(),
                definition_id: s.definition_id.clone(),
                kind: s.kind,
                qualifier: s.qualifier.clone(),
            })
            .collect();
        candidates.sort_by(|a, b| {
            a.definition_id
                .cmp(&b.definition_id)
                .then_with(|| a.path.cmp(&b.path))
        });
        candidates.dedup();
        let data = InspectData {
            symbol: None,
            observations: Vec::new(),
            docs_truncated: false,
            also_at: Vec::new(),
            candidates: candidates.clone(),
            aspects: Vec::new(),
            availability: None,
            relationships: Vec::new(),
            fragments: Vec::new(),
            source: None,
            execution_observations: Vec::new(),
            producer_runs: Vec::new(),
        };
        return Research {
            summary: format!(
                "`{wanted}` is ambiguous: {} definitions match.",
                definitions.len()
            ),
            data: common::to_object(&data),
            coverage: Coverage {
                scope: format!("candidate definitions for `{wanted}`"),
                indexed: [EvidenceKind::PublicApi.as_str().to_owned()]
                    .into_iter()
                    .collect(),
                missing: std::collections::BTreeSet::new(),
                limitations: vec![
                    "The reference matched more than one definition; nothing was guessed. Pass \
                     a candidate's path as symbol_path and its definition_id to select exactly."
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
    let also_at = match opened
        .reader
        .aliases_for(&selected.definition_id, &selected.path)
        .await
    {
        Ok(rows) => rows,
        Err(err) => return common::query_error(&err),
    };
    let observations = match opened
        .reader
        .observations_for(&selected.symbol_id, include_docs)
        .await
    {
        Ok(rows) => rows,
        Err(err) => return common::query_error(&err),
    };

    let excerpt_chars = service.config.limits.excerpt_characters;
    let budget = common::byte_budget(service, request.max_bytes);
    let mut evidence = Vec::new();
    let mut fragments = Vec::new();
    let mut returned_aspects = Vec::new();

    // signature: always cheap, from the normalized evidence -- never LSP (gate C17 premise).
    if want("signature") {
        returned_aspects.push("signature".to_owned());
        let selected_fragments = match opened
            .reader
            .fragments(enrichment_store::query::FragmentSelection {
                path: None,
                symbol_id: Some(&selected.symbol_id),
                kinds: &[FragmentKind::ApiSignature],
                limit: 256,
                require_complete: true,
            })
            .await
        {
            Ok(rows) => rows,
            Err(err) => return common::query_error(&err),
        };
        for f in selected_fragments {
            evidence.push(evidence_from_fragment(
                service,
                &f,
                source_version_match,
                excerpt_chars,
            ));
            fragments.push(f);
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
                let docs = match opened
                    .reader
                    .fragments(enrichment_store::query::FragmentSelection {
                        path: None,
                        symbol_id: Some(&selected.symbol_id),
                        kinds: &[FragmentKind::DocText],
                        limit: 256,
                        require_complete: true,
                    })
                    .await
                {
                    Ok(rows) => rows,
                    Err(err) => return common::query_error(&err),
                };
                for f in &docs {
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
        relationships = match opened
            .reader
            .relationships_for(
                &selected.symbol_id,
                service.config.limits.namespace_entries.saturating_mul(2),
            )
            .await
        {
            Ok(rows) => rows,
            Err(err) => return common::query_error(&err),
        };
    }

    if want("examples") {
        returned_aspects.push("examples".to_owned());
        let name = selected.name.clone();
        let examples = match opened.reader.examples_for(&name).await {
            Ok(rows) => rows,
            Err(err) => return common::query_error(&err),
        };
        {
            for f in &examples {
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
    if selected.python.is_some() && manifest.missing.contains(&EvidenceKind::RuntimeApi) {
        missing.insert(EvidenceKind::RuntimeApi.as_str().to_owned());
        limitations.push("Only source/stub declarations are available: native implementation source and runtime signatures have not been observed.".into());
    }

    let mut availability = None;
    if want("availability") {
        returned_aspects.push("availability".to_owned());
        let requested = RequestedConfiguration {
            features: (opened.environment.features_known
                || !opened.environment.features.is_empty())
            .then(|| opened.environment.features.clone()),
            default_features: opened.environment.default_features,
            target: opened.environment.target.clone(),
        };
        availability = manifest
            .observed_configuration
            .clone()
            .map(|observed| Availability::assess(observed, requested, &selected.cfg_hints));
    }

    let mut source_excerpt = None;
    if request.depth == InspectDepth::Source && want("source") {
        returned_aspects.push("source".to_owned());
        match source_for(service, &opened, &selected).await {
            Ok(Some(excerpt)) => source_excerpt = Some(excerpt),
            Err(error) => return common::query_error(&error),
            Ok(None) => {
                missing.insert(EvidenceKind::SourceExcerpts.as_str().to_owned());
                limitations.push(
                    "The recorded span could not be located in the extracted crate archive; \
                     the definition may live in a generated or external file."
                        .to_owned(),
                );
            }
        }
    }
    let is_runtime = want("runtime")
        || request
            .execution
            .as_ref()
            .is_some_and(|o| o.runtime.is_some());
    let mut execution_observations = Vec::new();
    if want("semantics") || want("runtime") || request.execution.is_some() {
        execution_observations = match super::inspect_execution::retained(
            &opened.reader,
            &selected,
            request.execution.as_ref(),
            is_runtime,
        )
        .await
        {
            Ok(rows) => rows,
            Err(error) => return common::query_error(&error),
        };
        if execution_observations.is_empty() {
            missing.insert(
                if is_runtime {
                    "runtime_api"
                } else {
                    "semantic_queries"
                }
                .into(),
            );
            limitations.push("No retained execution observation matches this exact scope. Execution requires explicit execute_on_miss or rerun intent and an enabled profile.".into());
        } else {
            returned_aspects.push(if is_runtime { "runtime" } else { "semantics" }.into());
        }
    }
    for observation in &execution_observations {
        use enrichment_core::evidence::execution::{ExecutionOutcome, ExecutionPayload};
        let (outcome, notes) = match &observation.payload {
            ExecutionPayload::SemanticQuery(q) => (q.outcome, &q.limitations),
            ExecutionPayload::RuntimeObject(q) => (q.outcome, &q.limitations),
            ExecutionPayload::UsageProbe(_) => continue,
        };
        for note in notes {
            if !limitations.contains(note) {
                limitations.push(note.clone());
            }
        }
        if !matches!(outcome, ExecutionOutcome::Results | ExecutionOutcome::Empty) {
            missing.insert(
                if is_runtime {
                    "runtime_api"
                } else {
                    "semantic_queries"
                }
                .into(),
            );
        }
    }
    limitations.push(
        "Availability describes the documented build; whether the project has this item \
         depends on its features and target, which are reported separately."
            .to_owned(),
    );

    let data = InspectData {
        symbol: Some(symbol),
        observations,
        docs_truncated,
        also_at,
        candidates: Vec::new(),
        aspects: returned_aspects,
        availability,
        relationships,
        fragments,
        source: source_excerpt,
        execution_observations,
        producer_runs: Vec::new(),
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
async fn source_for(
    service: &Service,
    opened: &common::Opened,
    symbol: &Symbol,
) -> Result<Option<source::SourceExcerpt>, enrichment_store::QueryError> {
    let (Some(file), Some(line)) = (symbol.span_file.as_deref(), symbol.span_line) else {
        return Ok(None);
    };
    let line = line as usize;
    if symbol.python.is_some() {
        let inputs = opened
            .reader
            .inputs_for_role(&format!("python-source:{file}"))
            .await?;
        let Some(input) = inputs.first() else {
            return Ok(None);
        };
        if inputs.iter().any(|i| i.sha256 != input.sha256) {
            return Err(std::io::Error::other(
                "conflicting source inputs require qualified inspection",
            )
            .into());
        }
        let artifact = input.clone();
        let blobs = service.blobs.clone();
        let file = file.to_owned();
        let max_lines = service.config.limits.source_lines;
        return tokio::task::spawn_blocking(move || {
            let capture = blobs.capture_input(&artifact, 64 * 1024 * 1024)?;
            source::source_excerpt_reader(std::io::BufReader::new(capture), &file, line, max_lines)
        })
        .await
        .map_err(std::io::Error::other)?
        .map_err(Into::into);
    }
    let inputs = opened
        .reader
        .inputs_of_kind(ArtifactKind::CrateTarball)
        .await?;
    let Some(input) = inputs.first() else {
        return Ok(None);
    };
    if inputs.iter().any(|i| i.sha256 != input.sha256) {
        return Err(std::io::Error::other(
            "conflicting crate source artifacts require qualified inspection",
        )
        .into());
    }
    let artifact = input.clone();
    let blobs = service.blobs.clone();
    let root = service.paths.unpacked();
    let digest = input.sha256.clone();
    let file = file.to_owned();
    let max_lines = service.config.limits.source_lines;
    tokio::task::spawn_blocking(move || {
        let crate_root = super::source_tree::open(
            &root,
            &digest,
            blobs.capture_input(&artifact, 256 * 1024 * 1024)?,
        )?;
        let candidates = std::iter::once(file.clone()).chain(
            file.match_indices('/')
                .map(|(i, _)| file[i + 1..].to_owned()),
        );
        for candidate in candidates {
            let relative = std::path::Path::new(&candidate);
            if relative
                .components()
                .any(|c| !matches!(c, std::path::Component::Normal(_)))
            {
                continue;
            }
            if crate_root.join(relative).is_file() {
                return source::source_excerpt_checked(&crate_root, &candidate, line, max_lines);
            }
        }
        Ok(None)
    })
    .await
    .map_err(std::io::Error::other)?
    .map_err(Into::into)
}
