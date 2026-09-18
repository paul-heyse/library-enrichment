//! Pinned comparisons read immutable evidence; version convenience explicitly resolves first.
use enrichment_core::compare::page::{AlternativeCursor, ComparisonCursor};
use enrichment_core::request::CompareRequest;
use enrichment_core::wire::data::{CompareData, ComparisonSide};
use enrichment_core::wire::{Coverage, Envelope, ErrorCode, Page};

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
        context_id: opened.context.context_id.clone(),
        snapshot_id: opened.snapshot_id.clone(),
        release: opened.release.clone(),
        environment: opened.environment.clone(),
    }
}

/// Compare two exact snapshots without changing either input.
pub async fn compare(service: &Service, request: CompareRequest) -> Envelope {
    if request.before_context_id.is_none() {
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
    id: &enrichment_core::identity::JobId,
    digest: &str,
) -> Envelope {
    read_inner(service, request, Some((id, digest))).await
}

async fn read_inner(
    service: &Service,
    request: CompareRequest,
    job: Option<(&enrichment_core::identity::JobId, &str)>,
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
        request.before_snapshot_id.as_ref(),
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
        request.after_snapshot_id.as_ref(),
    )
    .await
    {
        Ok(s) => s,
        Err(e) => return *e,
    };
    let selection = match enrichment_store::comparison_policy::select(
        &service.repository.runtime,
        request.scopes.clone(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "comparison_selection"),
    };
    let scopes = selection.scopes;
    let a = before.reader.manifest();
    let b = after.reader.manifest();
    let (configuration_differences, confounders) =
        match enrichment_store::comparison_context::assess(
            &service.repository.runtime,
            &before.environment,
            &after.environment,
            &a.observed_configuration,
            &b.observed_configuration,
        )
        .await
        {
            Ok(value) => value,
            Err(error) => return common::operation_error(&error, "comparison_configuration"),
        };
    let before_coverage = match before
        .reader
        .assess(
            &selection.kinds,
            None,
            "before requested comparison scopes".into(),
        )
        .await
    {
        Ok(value) => value,
        Err(error) => return common::query_error(&error),
    };
    let after_coverage = match after
        .reader
        .assess(
            &selection.kinds,
            None,
            "after requested comparison scopes".into(),
        )
        .await
    {
        Ok(value) => value,
        Err(error) => return common::query_error(&error),
    };
    let assessment = match enrichment_store::comparison_policy::assess(
        &service.repository.runtime,
        enrichment_store::comparison_policy::Inputs {
            before: before.release.key.clone(),
            after: after.release.key.clone(),
            before_normalizer: a.normalizer_version.clone(),
            after_normalizer: b.normalizer_version.clone(),
            scopes: scopes.clone(),
            confounders,
        },
        &before_coverage,
        &after_coverage,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return common::operation_error(&error, "comparison_coverage"),
    };
    let scope = enrichment_core::compare::page::SnapshotPair {
        before: before.snapshot_id.clone(),
        after: after.snapshot_id.clone(),
    };
    let budget = match common::byte_budget(service, request.max_bytes).await {
        Ok(value) => value,
        Err(error) => return common::query_error(&error),
    };
    let digest = enrichment_core::operation::selections::ComparisonSelection {
        witness: service.selection_witness(),
        scopes: scopes.clone(),
        max_items: request.max_items,
        max_bytes: budget,
    }
    .identity();
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
            incomplete_scopes: &assessment.incomplete_scopes,
            after_key: cursor.as_ref().map(|c| &c.after),
            limit: request.max_items.map_or(limit, |n| n.clamp(1, limit)),
            detail: detail.as_ref(),
            digest: &digest,
            offset,
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
    let last_key = page.changes.last().map(|(key, _)| key.clone());
    let selected: Vec<_> = page.changes.into_iter().map(|(_, change)| change).collect();
    let returned = page.boundary.returned;
    let mut data = CompareData {
        page: Page::default(),
        before: side(&before, before_coverage.clone()),
        after: side(&after, after_coverage.clone()),
        comparable: assessment.comparable,
        same_release: assessment.same_release,
        configuration_differences,
        confounders: assessment.confounders,
        scopes,
        api_complete: assessment.api_complete,
        total_changes: total,
        changes: selected,
        offset,
    };
    let coverage = Coverage {
        details: None,
        assessments: before_coverage
            .assessments
            .into_iter()
            .chain(after_coverage.assessments)
            .collect(),
        scope: "Normalized differences between the two pinned snapshots".into(),
        indexed: assessment.indexed,
        missing: assessment.missing,
        limitations: assessment.limitations,
    };
    let result = Research {
        summary: format!("{total} evidence change(s); returning {returned} from offset {offset}"),
        data: common::payload(&data),
        coverage,
        freshness: envelope::unverified_freshness(),
        context_id: Some(after.context.context_id.clone()),
        snapshot_id: Some(after.snapshot_id.clone()),
        evidence: Vec::new(),
        artifacts: Vec::new(),
    };
    let mut result = result.ok_with_page(Page::new(returned, Some(total), false, None));
    let returned = page.boundary.returned;
    let more = page.boundary.has_more;
    let next_cursor = if more && let Some(key) = last_key {
        match ComparisonCursor::new(
            scope.clone(),
            digest.clone(),
            page.boundary.next_offset,
            key,
        )
        .encode()
        {
            Ok(value) => Some(value),
            Err(e) => return common::operation_error(&e, "comparison_projection"),
        }
    } else {
        None
    };
    result.summary =
        format!("{total} evidence change(s); returning {returned}, with {offset} already returned");
    data.page = Page::new(returned, page.boundary.total, more, next_cursor);
    result.data = common::payload(&data);
    result.artifacts =
        match enrichment_store::comparison::artifacts(&service.repository.runtime, &data.changes)
            .await
        {
            Ok(artifacts) => artifacts,
            Err(error) => return common::operation_error(&error, "comparison_artifacts"),
        };

    if assessment.partial {
        result = result.into_partial();
    }
    if let Some((job_id, request_digest)) = job {
        let state = if result.status() == enrichment_core::wire::Status::Ok {
            enrichment_core::wire::JobState::Succeeded
        } else {
            enrichment_core::wire::JobState::Partial
        };
        let blobs = service.blobs.clone();
        let prepared =
            crate::delivery::prepare_comparison(&blobs, &service.repository.runtime, result).await;
        let (delivery, bounded) = match prepared {
            Ok(value) => value,
            Err(error) => return common::operation_error(&error, "comparison_delivery"),
        };
        let publication = enrichment_core::evidence::catalog::ComparisonPublication {
            job_id: *job_id,
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
