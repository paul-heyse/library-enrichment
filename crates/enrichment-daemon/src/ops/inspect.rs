//! `symbol.inspect`: characterize a known candidate and its deployment requirements
//! (blueprint §7, `inspect_symbol`; §7.1 inspection requirements; §4.3 availability).
//!
//! Defaults to the public API plus a small documentation section. Availability is reported as
//! separate states -- documented in the observed build versus verified for the project -- and
//! no feature predicate is invented (gates R05, R06). Source depth reads a bounded excerpt from
//! the extracted crate archive, confined to it. LSP depth is a later phase and is named as
//! such rather than silently degraded.

use enrichment_core::evidence::{ArtifactKind, EvidenceKind, FragmentKind, SymbolHeader};
use enrichment_core::producer::source;
use enrichment_core::request::InspectRequest;
use enrichment_core::search::row_page::RowCursor;
use enrichment_core::wire::data::InspectData;
use enrichment_core::wire::{
    AspectOutcome, AspectState, Envelope, ErrorCode, Freshness, InspectionAspect, Page,
    SourceVersionMatch,
};

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
    "semantics",
    "runtime",
    "children",
    "members",
];

/// Select retained results first. Only explicit execution intent can schedule a producer.
pub async fn inspect(service: &Service, request: InspectRequest) -> Envelope {
    use enrichment_core::request::InspectionIntent;
    if let Err(error) = request.selection.validate() {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            error,
            "Use a unique, bounded aspect selection.",
            false,
        );
    }
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
                .selection
                .aspects()
                .iter()
                .any(|a| a.aspect == enrichment_core::wire::InspectionAspect::Runtime)
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
            let selected = request.selection.aspects();
            let target = if options.runtime.is_some() {
                InspectionAspect::Runtime
            } else {
                InspectionAspect::Semantics
            };
            if !selected.iter().any(|selection| selection.aspect == target) {
                return envelope::error(
                    ErrorCode::UnsupportedFormat,
                    "Execution intent must name its selected inspection aspect",
                    "Include runtime for execution.runtime, or semantics for semantic execution, in selection.aspects.",
                    false,
                );
            }
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
    if let Err(error) = request.selection.validate() {
        return envelope::error(
            ErrorCode::UnsupportedFormat,
            error,
            "Use a unique, bounded aspect selection.",
            false,
        );
    }
    let selection = request.selection.aspects();
    let want = |name: &str| selection.iter().any(|s| s.aspect.as_str() == name);

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
        .symbols_at(wanted, request.definition_id.as_deref())
        .await
    {
        Ok(m) => m,
        Err(err) => return common::query_error(&err),
    };
    if matches.is_empty() && !wanted.contains("::") && !wanted.contains('.') {
        matches = match opened
            .reader
            .symbols_ending_with(wanted, request.definition_id.as_deref())
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
            children: Vec::new(),
            members: Vec::new(),
            aspect_outcomes: Vec::new(),
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
            data: common::payload(&data),
            coverage: match opened.reader.assess(&[EvidenceKind::PublicApi], None,
                format!("candidate definitions for `{wanted}`")).await {
                Ok(mut coverage) => {
                    coverage.limitations.push("The reference matched multiple definitions. Select a candidate path and definition_id; no identity was guessed.".into());
                    coverage
                }
                Err(error) => return common::query_error(&error),
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
    let selected: SymbolHeader = matches[0].clone();
    let also_at = match opened
        .reader
        .aliases_for(&selected.definition_id, &selected.path)
        .await
    {
        Ok(rows) => rows,
        Err(err) => return common::query_error(&err),
    };
    let excerpt_chars = service.config.limits.excerpt_characters;
    let budget = common::byte_budget(service, request.max_bytes);
    let mut selected_pages = std::collections::BTreeMap::new();
    for aspect in &selection {
        let digest = selection_digest(&selected.symbol_id, aspect, budget, &request.execution);
        let after = match aspect
            .cursor
            .as_deref()
            .map(|text| RowCursor::decode(text, manifest.snapshot_id.as_str(), &digest))
            .transpose()
        {
            Ok(value) => value.map(|c| c.after),
            Err(e) => {
                return envelope::error(
                    ErrorCode::InvalidCursor,
                    e.to_string(),
                    "Restart this aspect without a cursor for the selected snapshot and limits.",
                    false,
                );
            }
        };
        selected_pages.insert(aspect.aspect, (aspect.max_items, digest, after));
    }
    let mut outcomes = Vec::new();
    let mut observations = Vec::new();
    let mut evidence = Vec::new();
    let mut fragments = Vec::new();
    let mut returned_aspects = Vec::new();
    let mut relationships = Vec::new();
    let mut children = Vec::new();
    let mut members = Vec::new();
    let mut execution_observations = Vec::new();
    let mut missing = std::collections::BTreeSet::new();
    let mut limitations = Vec::new();
    let mut docs_truncated = false;
    for aspect in &selection {
        let (limit, digest, after) = &selected_pages[&aspect.aspect];
        let mut outcome = AspectOutcome {
            aspect: aspect.aspect,
            state: AspectState::Available,
            reason: None,
            page: None,
            diagnostic: None,
        };
        let page_result = match aspect.aspect {
            InspectionAspect::Semantics | InspectionAspect::Runtime => {
                match super::inspect_execution::retained_page(
                    &opened.reader,
                    &selected,
                    request.execution.as_ref(),
                    aspect.aspect == InspectionAspect::Runtime,
                    *limit,
                    after.as_deref(),
                )
                .await
                {
                    Ok(page) => {
                        let result = aspect_page(manifest.snapshot_id.as_str(), digest, &page);
                        execution_observations.extend(page.items);
                        Some(result)
                    }
                    Err(error) => Some(Err(error)),
                }
            }
            InspectionAspect::Signature => match opened
                .reader
                .observation_page(&selected.symbol_id, false, *limit, after.as_deref())
                .await
            {
                Ok(page) => {
                    let result = aspect_page(&manifest.snapshot_id.to_string(), digest, &page);
                    observations = page.items;
                    Some(result)
                }
                Err(e) => Some(Err(e)),
            },
            InspectionAspect::Relationships => match opened
                .reader
                .relationship_page(&selected.symbol_id, *limit, after.as_deref())
                .await
            {
                Ok(page) => {
                    let result = aspect_page(&manifest.snapshot_id.to_string(), digest, &page);
                    relationships = page.items;
                    Some(result)
                }
                Err(e) => Some(Err(e)),
            },
            InspectionAspect::Children | InspectionAspect::Members => {
                match opened
                    .reader
                    .navigation_page(
                        &selected.symbol_id,
                        aspect.aspect == InspectionAspect::Members,
                        *limit,
                        after.as_deref(),
                    )
                    .await
                {
                    Ok(page) => {
                        let result = aspect_page(manifest.snapshot_id.as_str(), digest, &page);
                        let entries = page
                            .items
                            .into_iter()
                            .map(|symbol| enrichment_core::wire::data::InspectionCandidate {
                                path: symbol.path,
                                definition_id: symbol.definition_id,
                                kind: symbol.kind,
                                qualifier: symbol.qualifier,
                            })
                            .collect();
                        if aspect.aspect == InspectionAspect::Members {
                            members = entries;
                        } else {
                            children = entries;
                        }
                        Some(result)
                    }
                    Err(error) => Some(Err(error)),
                }
            }
            InspectionAspect::Documentation | InspectionAspect::Examples => {
                let kinds = if aspect.aspect == InspectionAspect::Documentation {
                    vec![FragmentKind::DocText, FragmentKind::ReadmeSection]
                } else {
                    vec![FragmentKind::Example]
                };
                match opened
                    .reader
                    .fragment_page(
                        &selected.symbol_id,
                        &kinds,
                        *limit,
                        after.as_deref(),
                        aspect.max_characters,
                    )
                    .await
                {
                    Ok(page) => {
                        let result = aspect_page(&manifest.snapshot_id.to_string(), digest, &page);
                        if aspect.aspect == InspectionAspect::Documentation {
                            docs_truncated = page.has_more;
                        }
                        for (fragment, text_complete) in page.items {
                            match evidence_from_fragment(&fragment, excerpt_chars) {
                                Ok(citation) => evidence.push(citation),
                                Err(error) => {
                                    return common::operation_error(&error, "citation_identity");
                                }
                            }
                            let complete = (!text_complete).then(|| {
                                let mut full_aspect = aspect.clone();
                                full_aspect.max_characters = None;
                                let mut execution = request.execution.clone();
                                if let Some(options) = &mut execution {
                                    options.intent = enrichment_core::request::InspectionIntent::Retained;
                                }
                                let full_digest = selection_digest(&selected.symbol_id, &full_aspect, budget, &execution);
                                full_aspect.cursor = after.as_ref().map(|key| RowCursor::encode(
                                    manifest.snapshot_id.as_str(), &full_digest, key.clone(),
                                ).expect("bounded admitted fragment identity serializes"));
                                enrichment_core::wire::RecoveryAction::CallTool {
                                    request: Box::new(enrichment_core::request::ResearchRequest::Inspect(InspectRequest {
                                        context_id: opened.context.context_id.to_string(), snapshot_id: Some(manifest.snapshot_id.to_string()),
                                        symbol_path: selected.path.clone(), definition_id: Some(selected.definition_id.clone()),
                                        selection: enrichment_core::wire::ResearchSelection::Explicit { aspects: vec![full_aspect] },
                                        max_bytes: request.max_bytes, execution,
                                    })),
                                }
                            });
                            docs_truncated |=
                                aspect.aspect == InspectionAspect::Documentation && !text_complete;
                            fragments.push(enrichment_core::wire::data::FragmentProjection {
                                fragment,
                                text_complete,
                                complete,
                            });
                        }
                        Some(result)
                    }
                    Err(e) => Some(Err(e)),
                }
            }
            _ => {
                if after.is_some() {
                    return envelope::error(
                        ErrorCode::InvalidCursor,
                        "This aspect has no collection continuation",
                        "Remove its cursor.",
                        false,
                    );
                }
                None
            }
        };
        if let Some(result) = page_result {
            match result {
                Ok(page) => {
                    if page.returned == 0 {
                        outcome.state = AspectState::Absent;
                        outcome.reason = Some("No retained rows match this selection; see coverage before inferring absence".into());
                    }
                    outcome.page = Some(page);
                }
                Err(e) => {
                    let error = common::query_error(&e);
                    outcome.state = AspectState::Failed;
                    outcome.reason = Some(error.summary.clone());
                    outcome.diagnostic = error.error().map(|e| e.diagnostic.clone());
                }
            }
        }
        if outcome.state != AspectState::Failed {
            returned_aspects.push(aspect.aspect.as_str().into());
        }
        outcomes.push(outcome);
    }
    let ancillary = if want("source") || want("availability") {
        match opened.reader.ancillary_facts(&selected.symbol_id).await {
            Ok(facts) => Some(facts),
            Err(error) => {
                for aspect in [InspectionAspect::Source, InspectionAspect::Availability] {
                    if want(aspect.as_str()) {
                        fail_aspect(&mut outcomes, aspect, &error);
                    }
                }
                None
            }
        }
    } else {
        None
    };
    if ancillary
        .as_ref()
        .is_some_and(|facts| facts.cfg_alternatives > 1)
    {
        limitations.push("Qualified observations disagree on cfg hints; read the independent signature observations.".into());
    }
    if ancillary
        .as_ref()
        .is_some_and(|facts| facts.locator_alternatives > 1)
    {
        limitations.push("Qualified observations have different source coordinates; no single source window was selected.".into());
    }
    let symbol = selected.clone();
    let mut availability = None;
    if want("availability") {
        match enrichment_store::availability::select(
            opened.reader.runtime(),
            manifest.observed_configuration.as_ref(),
            &opened.environment,
            ancillary
                .as_ref()
                .and_then(|facts| facts.cfg_hints.as_deref()),
        )
        .await
        {
            Ok(selected) => availability = selected,
            Err(error) => fail_aspect(&mut outcomes, InspectionAspect::Availability, &error.into()),
        }
    }

    let mut source_excerpt = None;
    if want("source") {
        match source_for(
            service,
            &opened,
            ancillary.as_ref().and_then(|facts| facts.locator.as_ref()),
        )
        .await
        {
            Ok(Some(excerpt)) => source_excerpt = Some(excerpt),
            Err(error) => fail_aspect(&mut outcomes, InspectionAspect::Source, &error),
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
    for observation in &execution_observations {
        use enrichment_core::evidence::execution::{ExecutionOutcome, ExecutionPayload};
        let (outcome, notes, kind) = match &observation.payload {
            ExecutionPayload::SemanticQuery(q) => {
                (q.outcome, &q.limitations, EvidenceKind::SemanticQueries)
            }
            ExecutionPayload::RuntimeObject(q) => {
                (q.outcome, &q.limitations, EvidenceKind::RuntimeApi)
            }
            ExecutionPayload::UsageProbe(_) => continue,
        };
        for note in notes {
            if !limitations.contains(note) {
                limitations.push(note.clone());
            }
        }
        if !matches!(outcome, ExecutionOutcome::Results | ExecutionOutcome::Empty) {
            missing.insert(kind.as_str().into());
        }
    }
    limitations.push(
        "Availability describes the documented build; whether the project has this item \
         depends on its features and target, which are reported separately."
            .to_owned(),
    );

    let kinds: Vec<_> = selection
        .iter()
        .map(|aspect| match aspect.aspect {
            InspectionAspect::Signature
            | InspectionAspect::Availability
            | InspectionAspect::Relationships
            | InspectionAspect::Children
            | InspectionAspect::Members => EvidenceKind::PublicApi,
            InspectionAspect::Documentation => EvidenceKind::Documentation,
            InspectionAspect::Examples => EvidenceKind::Examples,
            InspectionAspect::Source => {
                if opened.release.key.ecosystem == enrichment_core::identity::Ecosystem::Python {
                    EvidenceKind::DistributionSource
                } else {
                    EvidenceKind::CrateSource
                }
            }
            InspectionAspect::Runtime => EvidenceKind::RuntimeApi,
            InspectionAspect::Semantics => EvidenceKind::SemanticQueries,
        })
        .collect();
    let execution_kinds: std::collections::BTreeSet<_> = execution_observations
        .iter()
        .map(|fact| match fact.payload {
            enrichment_core::evidence::execution::ExecutionPayload::SemanticQuery(_) => {
                EvidenceKind::SemanticQueries
            }
            enrichment_core::evidence::execution::ExecutionPayload::RuntimeObject(_) => {
                EvidenceKind::RuntimeApi
            }
            enrichment_core::evidence::execution::ExecutionPayload::UsageProbe(_) => {
                EvidenceKind::UsageProbes
            }
        })
        .collect();
    let other_kinds: Vec<_> = kinds
        .iter()
        .filter(|kind| !execution_kinds.contains(kind))
        .copied()
        .collect();
    let scope = format!(
        "requested aspects of {} in {}",
        selected.path, manifest.snapshot_id
    );
    let mut coverage = if other_kinds.is_empty() {
        enrichment_core::wire::Coverage::unassessed(scope)
    } else {
        match opened
            .reader
            .assess(&other_kinds, Some(&selected.symbol_id), scope)
            .await
        {
            Ok(value) => value,
            Err(error) => return common::query_error(&error),
        }
    };
    if !execution_observations.is_empty() {
        let ids = execution_observations
            .iter()
            .map(|fact| fact.source.artifact_id.clone())
            .collect::<Vec<_>>();
        let assessed = match opened.reader.assess_execution(&ids).await {
            Ok(value) => value,
            Err(error) => return common::query_error(&error),
        };
        coverage.assessments.extend(assessed.assessments);
        coverage.limitations.extend(assessed.limitations);
        coverage.refresh_kinds();
    }
    for outcome in &mut outcomes {
        if outcome.state == AspectState::Failed {
            continue;
        }
        if outcome.state == AspectState::Absent
            && coverage.assessments.iter().any(|item| {
                item.kind
                    == kinds[selection
                        .iter()
                        .position(|a| a.aspect == outcome.aspect)
                        .expect("selected aspect")]
                    && item.state != enrichment_core::wire::ScopeState::Indexed
            })
        {
            outcome.state = AspectState::Unavailable;
        }
        if outcome.aspect == InspectionAspect::Source && source_excerpt.is_none() {
            outcome.state = AspectState::Unavailable;
            outcome.reason = Some("No trustworthy recorded source window is available".into());
        }
        if outcome.aspect == InspectionAspect::Availability && availability.is_none() {
            outcome.state = AspectState::Unavailable;
            outcome.reason = Some("No observed build configuration is retained; availability has not been established for the requested environment".into());
        }
        if matches!(
            outcome.aspect,
            InspectionAspect::Semantics | InspectionAspect::Runtime
        ) && outcome.page.as_ref().is_some_and(|page| page.returned == 0)
        {
            outcome.state = AspectState::Unavailable;
            outcome.reason = Some("No retained execution result for this scope; explicit execution intent and an enabled profile are required".into());
        }
    }
    returned_aspects.retain(|name| {
        outcomes
            .iter()
            .any(|o| o.aspect.as_str() == name && o.state == AspectState::Available)
    });
    coverage.missing.extend(missing);
    coverage.limitations.extend(limitations);
    let partial = !coverage.complete()
        || !coverage.missing.is_empty()
        || outcomes
            .iter()
            .any(|o| matches!(o.state, AspectState::Unavailable | AspectState::Failed));
    let data = InspectData {
        children,
        members,
        aspect_outcomes: outcomes,
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
    let research = Research {
        summary: format!("{} `{}`", selected.kind.as_str(), selected.path),
        data: common::payload(&data),
        coverage,
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

fn selection_digest(
    symbol: &str,
    aspect: &enrichment_core::wire::research::AspectSelection,
    budget: usize,
    execution: &Option<enrichment_core::request::InspectionOptions>,
) -> String {
    enrichment_core::canonical::digest_hex(&serde_json::json!([
        symbol,
        aspect.aspect,
        aspect.max_items,
        aspect.max_characters,
        budget,
        execution,
    ]))
}

fn fail_aspect(
    outcomes: &mut [AspectOutcome],
    aspect: InspectionAspect,
    error: &enrichment_store::QueryError,
) {
    if let Some(outcome) = outcomes.iter_mut().find(|o| o.aspect == aspect) {
        outcome.state = AspectState::Failed;
        outcome.reason = Some(error.to_string());
        outcome.diagnostic = Some(error.diagnostic());
    }
}

fn aspect_page<T>(
    snapshot: &str,
    digest: &str,
    page: &enrichment_store::query::NativePage<T>,
) -> Result<Page, enrichment_store::QueryError> {
    let cursor = page
        .next_key
        .as_ref()
        .map(|key| RowCursor::encode(snapshot, digest, key.clone()))
        .transpose()
        .map_err(std::io::Error::other)?;
    Ok(Page::new(
        page.items.len() as u64,
        None,
        page.has_more,
        cursor,
    ))
}

/// Cut a bounded source window; the recorded start is not a trustworthy item-end span.
async fn source_for(
    service: &Service,
    opened: &common::Opened,
    locator: Option<&enrichment_core::evidence::relational::Locator>,
) -> Result<Option<source::SourceExcerpt>, enrichment_store::QueryError> {
    use enrichment_core::evidence::relational::Locator;
    let Some((file, line)) = locator.and_then(|locator| match locator {
        Locator::RustdocItem {
            reported_file: Some(file),
            reported_line: Some(line),
            ..
        }
        | Locator::PythonDeclaration {
            file,
            line: Some(line),
            ..
        } => Some((file.as_str(), *line)),
        _ => None,
    }) else {
        return Ok(None);
    };
    let line = line as usize;
    if opened.release.key.ecosystem == enrichment_core::identity::Ecosystem::Python {
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
