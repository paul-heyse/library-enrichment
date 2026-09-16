//! Pinned comparisons read immutable evidence; version convenience explicitly resolves first.
use std::collections::BTreeSet;

use enrichment_core::canonical;
use enrichment_core::compare::{
    Scope,
    page::{AlternativeCursor, ComparisonCursor},
};
use enrichment_core::evidence::EvidenceKind;
use enrichment_core::request::CompareRequest;
use enrichment_core::wire::data::{CompareData, ComparisonSide, ConfigurationDifference};
use enrichment_core::wire::{Coverage, Envelope, ErrorCode, Page};
use serde_json::json;

use super::common::{self, Opened};
use crate::envelope::{self, Research};
use crate::service::Service;

fn invalid(message: &str) -> Envelope {
    envelope::error(
        ErrorCode::UnsupportedFormat,
        message,
        "Pass either before/after context IDs (optional snapshot IDs), or ecosystem/name/from_version/to_version.",
        false,
    )
}
fn side(opened: &Opened, coverage: Coverage) -> ComparisonSide {
    ComparisonSide {
        coverage,
        context_id: opened.context.context_id.to_string(),
        snapshot_id: opened.snapshot_id.to_string(),
        release: opened.release.clone(),
        environment: opened.environment.clone(),
    }
}

/// Compare two exact snapshots without changing either input.
pub async fn compare(service: &Service, request: CompareRequest) -> Envelope {
    let pinned = request.before_context_id.is_some() || request.after_context_id.is_some();
    let versions = request.ecosystem.is_some()
        || request.name.is_some()
        || request.from_version.is_some()
        || request.to_version.is_some();
    if pinned == versions {
        return invalid("Exactly one comparison input form is required");
    }
    if !pinned {
        if let Err(error) = request.resolutions() {
            return invalid(&error);
        }
        return super::compare_job::submit(service, request).await;
    }
    read(service, request).await
}

pub(super) async fn read(service: &Service, request: CompareRequest) -> Envelope {
    read_inner(service, request, None).await
}

pub(super) async fn read_job(
    service: &Service,
    request: CompareRequest,
    id: &str,
    digest: &str,
) -> Envelope {
    read_inner(service, request, Some((id, digest))).await
}

async fn read_inner(
    service: &Service,
    request: CompareRequest,
    job: Option<(&str, &str)>,
) -> Envelope {
    let (Some(before_id), Some(after_id)) = (&request.before_context_id, &request.after_context_id)
    else {
        return invalid("Both context IDs are required");
    };
    let catalog = match service.repository.catalog.pin().await {
        Ok(value) => value,
        Err(e) => return common::operation_error(&e, "comparison_projection"),
    };
    let before = match common::open_context_at(
        service,
        std::sync::Arc::clone(&catalog),
        before_id,
        request.before_snapshot_id.as_deref(),
    )
    .await
    {
        Ok(s) => s,
        Err(e) => return *e,
    };
    let after = match common::open_context_at(
        service,
        catalog,
        after_id,
        request.after_snapshot_id.as_deref(),
    )
    .await
    {
        Ok(s) => s,
        Err(e) => return *e,
    };
    if before.release.key.ecosystem != after.release.key.ecosystem
        || before.release.key.package != after.release.key.package
        || before.release.key.registry != after.release.key.registry
    {
        return invalid("Comparison requires the same package, ecosystem and registry");
    }
    let mut scopes = request.scopes.clone().unwrap_or_else(|| {
        vec![
            Scope::Api,
            Scope::Docs,
            Scope::Configuration,
            Scope::ReleaseNotes,
            Scope::Examples,
            Scope::Relationships,
        ]
    });
    scopes.sort();
    scopes.dedup();
    if scopes.is_empty() {
        return invalid("Select at least one comparison scope");
    }
    let want_api = scopes.contains(&Scope::Api);
    let a = before.reader.manifest();
    let b = after.reader.manifest();
    let mut confounders = Vec::new();
    let mut configuration_differences = Vec::new();
    let environments = [json!(before.environment), json!(after.environment)];
    for field in [
        "toolchain",
        "target",
        "features",
        "features_known",
        "default_features",
        "lock_digest",
        "resolution",
    ] {
        if environments[0][field] != environments[1][field] {
            configuration_differences.push(ConfigurationDifference {
                field: field.into(),
                before: environments[0][field].clone(),
                after: environments[1][field].clone(),
            });
        }
    }
    if a.observed_configuration != b.observed_configuration {
        configuration_differences.push(ConfigurationDifference {
            field: "observed_configuration".into(),
            before: json!(a.observed_configuration),
            after: json!(b.observed_configuration),
        });
    }
    for (label, env) in [
        ("before", &before.environment),
        ("after", &after.environment),
    ] {
        for (field, unknown) in [
            ("toolchain", env.toolchain.is_none()),
            ("target", env.target.is_none()),
            ("features/extras", !env.features_known),
            ("dependency resolution", env.lock_digest.is_none()),
        ] {
            if unknown {
                confounders.push(format!("{label} {field} is unknown"));
            }
        }
    }
    if !configuration_differences.is_empty() {
        confounders.push("Environment or observed configuration differs; do not attribute every difference to the release".into());
    }
    let kinds: BTreeSet<_> = scopes
        .iter()
        .map(|scope| match scope {
            Scope::Api | Scope::Relationships => EvidenceKind::PublicApi,
            Scope::Docs => EvidenceKind::Documentation,
            Scope::Configuration => EvidenceKind::RegistryMetadata,
            Scope::ReleaseNotes => EvidenceKind::ReleaseNotes,
            Scope::Examples => EvidenceKind::Examples,
        })
        .collect();
    let mut assessments = Vec::new();
    for (label, reader) in [("before", &before.reader), ("after", &after.reader)] {
        match reader
            .assess(
                &kinds.iter().copied().collect::<Vec<_>>(),
                None,
                format!("{label} requested comparison scopes"),
            )
            .await
        {
            Ok(coverage) => assessments.push(coverage),
            Err(e) => return common::query_error(&e),
        }
    }
    let after_coverage = assessments.pop().expect("two assessed sides");
    let before_coverage = assessments.pop().expect("two assessed sides");
    let scope_complete = before_coverage.complete() && after_coverage.complete();
    let api_complete = kinds.contains(&EvidenceKind::PublicApi)
        && a.normalizer_version == b.normalizer_version
        && [&before_coverage, &after_coverage]
            .iter()
            .all(|coverage| coverage.indexed.contains(EvidenceKind::PublicApi.as_str()));
    if !scope_complete {
        confounders.push("Requested evidence scopes are incomplete; an empty delta does not establish unchanged evidence in those scopes".into());
    }
    if a.normalizer_version != b.normalizer_version {
        confounders
            .push("Normalizer versions differ; normalization may explain differences".into());
    }
    let same_release = before.release.key.version == after.release.key.version;
    if same_release && before.release.key.artifact_digest != after.release.key.artifact_digest {
        confounders.push("Different artifact variants of the same version were selected".into());
    }
    let scope = format!("{}:{}", before.snapshot_id, after.snapshot_id);
    let budget = common::byte_budget(service, request.max_bytes);
    let digest = canonical::digest_hex(&json!([
        "typed-comparison/2",
        scopes,
        budget,
        request.max_items
    ]));
    let cursor = match request
        .cursor
        .as_deref()
        .map(|c| ComparisonCursor::decode(c, &scope, &digest))
        .transpose()
    {
        Ok(value) => value,
        Err(e) => {
            return envelope::error(
                ErrorCode::InvalidCursor,
                e.to_string(),
                "Restart without a cursor for this snapshot pair and scope.",
                false,
            );
        }
    };
    if cursor.is_some() && request.alternative_cursor.is_some() {
        return invalid("Changed-key and alternative cursors are independent; pass one at a time");
    }
    let detail = match request
        .alternative_cursor
        .as_deref()
        .map(|text| AlternativeCursor::decode(text, &scope, &digest))
        .transpose()
    {
        Ok(value) => value,
        Err(e) => {
            return envelope::error(
                ErrorCode::InvalidCursor,
                e.to_string(),
                "Restart the comparison for this snapshot pair and selection.",
                false,
            );
        }
    };
    let offset = cursor.as_ref().map_or(0, |c| c.returned_before);
    let limit = service.config.limits.search_results.clamp(1, 1000);
    let page = match enrichment_store::comparison::page(
        &before.reader,
        &after.reader,
        &service.blobs,
        enrichment_store::comparison::Selection {
            scopes: &scopes,
            after_key: cursor.as_ref().map(|c| &c.after),
            limit: request.max_items.map_or(limit, |n| n.clamp(1, limit)),
            detail: detail.as_ref(),
            digest: &digest,
        },
    )
    .await
    {
        Ok(value) => value,
        Err(e) => return common::query_error(&e.into()),
    };
    let total = page.total;
    if offset > total {
        return invalid("Comparison cursor exceeds the pinned result count");
    }
    let (keys, mut selected): (Vec<_>, Vec<_>) = page.changes.into_iter().unzip();
    for change in &mut selected {
        let kind = match change.scope {
            Scope::Api | Scope::Relationships => EvidenceKind::PublicApi,
            Scope::Docs => EvidenceKind::Documentation,
            Scope::Configuration => EvidenceKind::RegistryMetadata,
            Scope::ReleaseNotes => EvidenceKind::ReleaseNotes,
            Scope::Examples => EvidenceKind::Examples,
        };
        if ![&before_coverage, &after_coverage]
            .iter()
            .all(|coverage| coverage.indexed.contains(kind.as_str()))
        {
            change.interpretation.push_str(" Coverage is incomplete on at least one side for this scope; an unobserved alternative does not establish absence.");
        }
    }
    let returned = selected.len() as u64;
    let mut data = CompareData {
        page: Page::default(),
        before: side(&before, before_coverage.clone()),
        after: side(&after, after_coverage.clone()),
        comparable: confounders.is_empty(),
        same_release,
        configuration_differences,
        confounders: confounders.clone(),
        scopes,
        api_complete,
        total_changes: total,
        changes: selected,
        offset,
    };
    let mut coverage = Coverage {
        details: None,
        assessments: before_coverage
            .assessments
            .into_iter()
            .chain(after_coverage.assessments)
            .collect(),
        scope: "Normalized differences between the two pinned snapshots".into(),
        indexed: before_coverage
            .indexed
            .intersection(&after_coverage.indexed)
            .cloned()
            .collect(),
        missing: before_coverage
            .missing
            .union(&after_coverage.missing)
            .cloned()
            .collect(),
        limitations: confounders,
    };
    coverage.limitations.push("A clean API comparison does not imply unchanged behavior; dependency, runtime and project compatibility require exact-environment verification.".into());
    if want_api {
        coverage.limitations.push("Signature deltas compare retained producer representations. Compiler rendering can differ, including Infallible and never-type (!) representations; a rendered difference alone does not establish a breaking source-level change.".into());
    }
    if want_api && !api_complete {
        coverage.missing.insert("complete_api_comparison".into());
    }
    let result = Research {
        summary: format!("{total} evidence change(s); returning {returned} from offset {offset}"),
        data: common::to_object(&data),
        coverage,
        freshness: envelope::unverified_freshness(),
        context_id: Some(after.context.context_id.to_string()),
        snapshot_id: Some(after.snapshot_id.to_string()),
        evidence: Vec::new(),
        artifacts: Vec::new(),
    };
    let mut result = result.ok_with_page(Page::new(returned, Some(total), false, None));
    loop {
        let returned = data.changes.len() as u64;
        let more = detail.is_none() && (page.has_more || offset + returned < total);
        let next_cursor = if more && let Some(key) = keys.get(data.changes.len().saturating_sub(1))
        {
            match ComparisonCursor::new(
                scope.clone(),
                digest.clone(),
                offset + returned,
                key.clone(),
            )
            .encode()
            {
                Ok(value) => Some(value),
                Err(e) => return common::operation_error(&e, "comparison_projection"),
            }
        } else {
            None
        };
        result.summary = format!(
            "{total} evidence change(s); returning {returned}, with {offset} already returned"
        );
        data.page = Page::new(
            returned,
            Some(if detail.is_some() { returned } else { total }),
            more,
            next_cursor,
        );
        result.data = common::to_object(&data);
        result.artifacts = value_artifacts(&data.changes);
        if common::json_size(&result) <= budget || data.changes.len() <= 1 {
            break;
        }
        data.changes.pop();
    }
    if !data.comparable || !scope_complete || (want_api && !api_complete) {
        result = result.into_partial();
    }
    if let Some((job_id, request_digest)) = job {
        let state = if result.status() == enrichment_core::wire::Status::Ok {
            enrichment_core::wire::JobState::Succeeded
        } else {
            enrichment_core::wire::JobState::Partial
        };
        let blobs = service.blobs.clone();
        let prepared = service
            .repository
            .runtime
            .blocking(move || crate::delivery::prepare_comparison(&blobs, result))
            .await;
        let (delivery, bounded) = match prepared {
            Ok(Ok(value)) => value,
            Ok(Err(error)) => return common::operation_error(&error, "comparison_delivery"),
            Err(error) => return common::operation_error(&error, "comparison_delivery"),
        };
        let publication = enrichment_core::evidence::catalog::ComparisonPublication {
            job_id: job_id.into(),
            request_digest: request_digest.into(),
            before_context_id: before.context.context_id.clone(),
            before_snapshot_id: before.snapshot_id.clone(),
            after_context_id: after.context.context_id.clone(),
            after_snapshot_id: after.snapshot_id.clone(),
            state,
            delivery,
        };
        let fence = match service.jobs.publication_fence(job_id) {
            Ok(fence) => fence,
            Err(error) => return common::operation_error(&error, "comparison_claim"),
        };
        if let Err(error) = service
            .repository
            .publish_comparison(publication, fence)
            .await
        {
            return common::operation_error(&error, "comparison_publication");
        }
        return bounded;
    }
    common::enforce_budget(service, result, request.max_bytes).await
}

/// Include exactly the value artifacts reachable from the final fitted page.
fn value_artifacts(
    changes: &[enrichment_core::compare::Change],
) -> Vec<enrichment_core::wire::ArtifactHandle> {
    let mut artifacts = std::collections::BTreeMap::new();
    for alternative in changes
        .iter()
        .flat_map(|change| change.before.iter().chain(&change.after).flatten())
    {
        if let enrichment_core::compare::AlternativeValue::Artifact { artifact, .. } =
            &alternative.value
        {
            artifacts.insert(artifact.receipt.artifact_id.clone(), artifact.clone());
        }
    }
    artifacts.into_values().collect()
}
