//! `symbol.inspect`: characterize a known candidate and its deployment requirements
//! (blueprint §7, `inspect_symbol`; §7.1 inspection requirements; §4.3 availability).
//!
//! Defaults to the public API plus a small documentation section. Availability is reported as
//! separate states -- documented in the observed build versus verified for the project -- and
//! no feature predicate is invented (gates R05, R06). Source depth reads a bounded excerpt from
//! the extracted crate archive, confined to it. LSP depth is a later phase and is named as
//! such rather than silently degraded.

use enrichment_core::evidence::EvidenceKind;
use enrichment_core::producer::source;
use enrichment_core::request::InspectRequest;
use enrichment_core::search::row_page::RowCursor;
use enrichment_core::wire::data::InspectData;
use enrichment_core::wire::{Envelope, ErrorCode, Freshness, InspectionAspect, Page};
use enrichment_store::research_inspection::{Failure, FailureStage, PageObservation};

use super::common;
use crate::envelope::{self, Research};
use crate::service::Service;

/// Aspects this build can return.
pub const ASPECTS: &[&str] = InspectionAspect::VALUES;

/// Select retained results first. Only explicit execution intent can schedule a producer.
pub async fn inspect(service: &Service, request: InspectRequest) -> Envelope {
    use enrichment_core::request::InspectionIntent;
    if request
        .execution
        .as_ref()
        .is_some_and(|options| options.intent != InspectionIntent::Retained)
    {
        return super::inspect_execution::submit(service, request).await;
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
    let selection = match enrichment_store::research_selection::inspection(
        &service.repository.runtime,
        &request.selection,
    )
    .await
    {
        Ok(selection) => selection,
        Err(error) => return common::operation_error(&error, "research_selection"),
    };
    let (routes, requirements) = match enrichment_store::research_inspection::routes(
        &service.repository.runtime,
        &selection,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "inspection_routes"),
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
    let manifest = opened.reader.manifest().clone();

    let bindings = match opened
        .reader
        .inspection_bindings(wanted, request.definition_id.as_deref())
        .await
    {
        Ok(value) => value,
        Err(error) => return common::query_error(&error),
    };

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
    let freshness = Freshness {
        registry_checked_at: None,
        source_version_match,
        latest_verified: false,
    };
    let context_id = Some(opened.context.context_id.clone());
    let snapshot_id = Some(opened.snapshot_id.clone());

    if bindings.definition_count == 0 {
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
    if bindings.definition_count > 1 {
        let candidates = bindings.candidates;
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
                bindings.definition_count
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

    let Some(selected) = bindings.selected else {
        return common::operation_error(
            &std::io::Error::other("native inspection selection omitted the unique binding"),
            "inspection_selection",
        );
    };
    let also_at = match opened
        .reader
        .aliases_for(&selected.definition_id, &selected.path)
        .await
    {
        Ok(rows) => rows,
        Err(err) => return common::query_error(&err),
    };
    let excerpt_chars = service.config.limits.excerpt_characters;
    let budget = match common::byte_budget(service, request.max_bytes).await {
        Ok(value) => value,
        Err(error) => return common::query_error(&error),
    };
    let mut selected_pages = std::collections::BTreeMap::new();
    for aspect in &selection {
        let digest = selection_digest(
            &service.selection_witness(),
            &selected.symbol_id,
            aspect,
            budget,
            &request.execution,
        );
        let after = match aspect
            .cursor
            .as_deref()
            .map(|text| RowCursor::decode(text, &manifest.snapshot_id, &digest))
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
    let mut pages = Vec::new();
    let mut failures = Vec::new();
    let mut observations = Vec::new();
    let mut fragments = Vec::new();
    let mut relationships = Vec::new();
    let mut children = Vec::new();
    let mut members = Vec::new();
    let mut execution_observations = Vec::new();
    let mut docs_truncated = false;
    for route in &routes {
        let aspect = &route.selection;
        let (limit, digest, after) = &selected_pages[&aspect.aspect];
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
                        let result = aspect_page(&manifest.snapshot_id, digest, &page);
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
                    let result = aspect_page(&manifest.snapshot_id, digest, &page);
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
                    let result = aspect_page(&manifest.snapshot_id, digest, &page);
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
                        let result = aspect_page(&manifest.snapshot_id, digest, &page);
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
                match opened
                    .reader
                    .fragment_page(
                        &selected.symbol_id,
                        &route.fragment_kinds,
                        *limit,
                        after.as_deref(),
                        aspect.max_characters,
                    )
                    .await
                {
                    Ok(page) => {
                        let result = aspect_page(&manifest.snapshot_id, digest, &page);
                        let recovery = match enrichment_store::research_inspection::text_recovery(
                            &service.repository.runtime,
                            enrichment_store::research_inspection::TextRecovery {
                                request: request.clone(),
                                symbol: selected.clone(),
                                snapshot: manifest.snapshot_id.clone(),
                                selection: aspect.clone(),
                                witness: service.selection_witness(),
                                max_bytes: budget,
                                after: after.clone(),
                            },
                        )
                        .await
                        {
                            Ok(recovery) => recovery,
                            Err(error) => {
                                return common::operation_error(&error, "inspection_recovery");
                            }
                        };
                        match enrichment_store::research_fragments::deliver(
                            &service.repository.runtime,
                            page.items,
                            recovery,
                            page.has_more,
                        )
                        .await
                        {
                            Ok(delivery) => {
                                if aspect.aspect == InspectionAspect::Documentation {
                                    docs_truncated = delivery.truncated;
                                }
                                fragments.extend(delivery.items);
                                Some(result)
                            }
                            Err(error) => Some(Err(error.into())),
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            }
            InspectionAspect::Source | InspectionAspect::Availability => None,
        };
        if let Some(result) = page_result {
            match result {
                Ok(page) => {
                    pages.push(PageObservation {
                        aspect: aspect.aspect,
                        page,
                    });
                }
                Err(e) => {
                    capture_failure(&mut failures, aspect.aspect, FailureStage::Page, &e);
                }
            }
        }
    }
    let ancillary = if requirements.ancillary {
        match opened.reader.ancillary_facts(&selected.symbol_id).await {
            Ok(facts) => Some(facts),
            Err(error) => {
                if requirements.source {
                    capture_failure(
                        &mut failures,
                        InspectionAspect::Source,
                        FailureStage::Ancillary,
                        &error,
                    );
                }
                if requirements.configuration {
                    capture_failure(
                        &mut failures,
                        InspectionAspect::Availability,
                        FailureStage::Ancillary,
                        &error,
                    );
                }
                None
            }
        }
    } else {
        None
    };
    let symbol = selected.clone();
    let mut availability = None;
    if requirements.configuration {
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
            Err(error) => capture_failure(
                &mut failures,
                InspectionAspect::Availability,
                FailureStage::Availability,
                &error.into(),
            ),
        }
    }

    let mut source_excerpt = None;
    if requirements.source {
        match source_for(
            service,
            &opened,
            ancillary.as_ref().and_then(|facts| facts.locator.as_ref()),
        )
        .await
        {
            Ok(Some(excerpt)) => source_excerpt = Some(excerpt),
            Err(error) => capture_failure(
                &mut failures,
                InspectionAspect::Source,
                FailureStage::Source,
                &error,
            ),
            Ok(None) => {}
        }
    }
    let kinds = match enrichment_store::research_outcomes::inspection_kinds(
        &service.repository.runtime,
        opened.release.key.ecosystem,
        &selection,
    )
    .await
    {
        Ok(kinds) => kinds,
        Err(error) => return common::operation_error(&error, "inspection_coverage_scope"),
    };
    let other_kinds = match enrichment_store::research_outcomes::non_execution_kinds(
        &service.repository.runtime,
        &kinds,
        &execution_observations,
    )
    .await
    {
        Ok(kinds) => kinds,
        Err(error) => return common::operation_error(&error, "inspection_execution_scope"),
    };
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
        if let Err(error) =
            enrichment_store::coverage::refresh(opened.reader.runtime(), &mut coverage).await
        {
            return common::operation_error(&error, "inspection_coverage");
        }
    }
    coverage = match enrichment_store::research_inspection::coverage(
        &service.repository.runtime,
        coverage,
        ancillary,
        requirements.source,
        source_excerpt.is_some(),
        &execution_observations,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "inspection_presentation"),
    };
    let outcomes = match enrichment_store::research_inspection::outcomes(
        &service.repository.runtime,
        &selection,
        &pages,
        &failures,
    )
    .await
    {
        Ok(outcomes) => outcomes,
        Err(error) => return common::operation_error(&error, "inspection_observations"),
    };
    let disposition = match enrichment_store::research_outcomes::inspection(
        &service.repository.runtime,
        opened.release.key.ecosystem,
        outcomes,
        &coverage,
        source_excerpt.is_some(),
        availability.is_some(),
    )
    .await
    {
        Ok(disposition) => disposition,
        Err(error) => return common::operation_error(&error, "inspection_outcomes"),
    };
    let partial = disposition.partial;
    let outcomes = disposition.outcomes;
    let returned_aspects = disposition.returned;
    let evidence = match enrichment_store::research_citations::fragments(
        &service.repository.runtime,
        fragments.iter().map(|item| &item.fragment).collect(),
        fragments.len(),
        excerpt_chars,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "citation_projection"),
    };
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
    witness: &enrichment_core::operation::selections::SelectionWitness,
    symbol: &str,
    aspect: &enrichment_core::wire::research::AspectSelection,
    budget: usize,
    execution: &Option<enrichment_core::request::InspectionOptions>,
) -> String {
    enrichment_core::operation::selections::InspectionSelection {
        witness: witness.clone(),
        symbol_id: symbol.into(),
        aspect: aspect.aspect,
        max_items: aspect.max_items,
        max_characters: aspect.max_characters,
        max_bytes: budget,
        execution: execution.clone(),
    }
    .identity()
}

fn capture_failure(
    failures: &mut Vec<Failure>,
    aspect: InspectionAspect,
    stage: FailureStage,
    error: &enrichment_store::QueryError,
) {
    failures.push(Failure {
        aspect,
        stage,
        reason: error.to_string(),
        diagnostic: error.diagnostic(),
    });
}

fn aspect_page<T>(
    snapshot: &enrichment_core::identity::SnapshotId,
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
    let Some(read) = opened
        .reader
        .source_read(
            opened.release.key.ecosystem,
            locator,
            service.config.limits.source_lines,
        )
        .await?
    else {
        return Ok(None);
    };
    let runtime = &service.repository.runtime;
    let artifact = read.artifact;
    let location = read.location;
    let blobs = service.blobs.clone();
    let protection = opened.reader.pinned().read_protection();
    if !location.archive {
        return runtime
            .blocking(move || {
                let _protection = protection;
                let capture = blobs.capture_input(&artifact, 64 * 1024 * 1024)?;
                source::source_excerpt_reader(
                    std::io::BufReader::new(capture),
                    &location.path,
                    location.start_line as usize,
                    location.max_lines,
                )
            })
            .await?
            .map_err(Into::into);
    }
    let candidates = enrichment_store::research_source::candidates(runtime, &location).await?;
    let directory = service.repository.source_directory().await?;
    let captured_protection = protection.clone();
    let (tree, observations) = runtime
        .blocking(move || -> std::io::Result<_> {
            let _protection = captured_protection;
            let tree = super::source_tree::open(
                directory,
                &artifact.sha256,
                blobs.capture_input(&artifact, 256 * 1024 * 1024)?,
            )?;
            let observations = candidates
                .into_iter()
                .map(|candidate| {
                    let present = match std::fs::symlink_metadata(tree.join(&candidate.path)) {
                        Ok(metadata) => metadata.is_file(),
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
                        Err(error) => return Err(error),
                    };
                    Ok(enrichment_store::research_source::Observation { candidate, present })
                })
                .collect::<std::io::Result<Vec<_>>>()?;
            Ok((tree, observations))
        })
        .await??;
    let Some(member) = enrichment_store::research_source::member(runtime, &observations).await?
    else {
        return Ok(None);
    };
    runtime
        .blocking(move || {
            let _protection = protection;
            source::source_excerpt_checked(
                &tree,
                &member.path,
                location.start_line as usize,
                location.max_lines,
            )
        })
        .await?
        .map_err(Into::into)
}
