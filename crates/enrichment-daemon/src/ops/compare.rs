//! Pinned comparisons read immutable evidence; version convenience explicitly resolves first.
use std::collections::BTreeSet;

use enrichment_core::canonical;
use enrichment_core::compare::{Scope, page::ComparisonCursor};
use enrichment_core::evidence::{EvidenceKind, relational::CoverageOutcome};
use enrichment_core::request::CompareRequest;
use enrichment_core::wire::data::{CompareData, ComparisonSide, ConfigurationDifference};
use enrichment_core::wire::{Coverage, Envelope, ErrorCode, Pagination};
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
fn side(opened: &Opened) -> ComparisonSide {
    ComparisonSide {
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
    let (Some(before_id), Some(after_id)) = (&request.before_context_id, &request.after_context_id)
    else {
        return invalid("Both context IDs are required");
    };
    let catalog = match service.repository.catalog.pin().await {
        Ok(value) => value,
        Err(e) => return common::store_error(&e),
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
    let mut api_complete = a.normalizer_version == b.normalizer_version;
    for reader in [&before.reader, &after.reader] {
        let coverage = match reader.coverage().await {
            Ok(value) => value,
            Err(e) => return common::query_error(&e),
        };
        let api: Vec<_> = coverage
            .iter()
            .filter(|c| c.kind == EvidenceKind::PublicApi)
            .collect();
        api_complete &= api.iter().any(|c| {
            matches!(&c.subject,
            enrichment_core::evidence::relational::SubjectRef::Library { release_id }
            if release_id == reader.manifest().release_id.as_str())
        }) && api
            .iter()
            .all(|c| c.outcome == CoverageOutcome::Indexed && c.gaps.is_empty());
    }
    if want_api && !api_complete {
        confounders.push("API coverage or normalizer compatibility is incomplete; an empty API delta does not establish unchanged API".into());
    }
    let same_release = before.release.key.version == after.release.key.version;
    if same_release && before.release.key.artifact_digest != after.release.key.artifact_digest {
        confounders.push("Different artifact variants of the same version were selected".into());
    }
    let scope = format!("{}:{}", before.snapshot_id, after.snapshot_id);
    let budget = common::byte_budget(service, request.max_bytes);
    let digest = canonical::digest_hex(&json!([
        "typed-comparison/1",
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
    let offset = cursor.as_ref().map_or(0, |c| c.returned_before);
    let limit = service.config.limits.search_results.clamp(1, 1000);
    let page = match enrichment_store::comparison::page(
        &before.reader,
        &after.reader,
        &scopes,
        cursor.as_ref().map(|c| &c.after),
        request.max_items.map_or(limit, |n| n.clamp(1, limit)),
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
    let (keys, selected): (Vec<_>, Vec<_>) = page.changes.into_iter().unzip();
    let returned = selected.len() as u64;
    let mut data = CompareData {
        before: side(&before),
        after: side(&after),
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
        scope: "Normalized differences between the two pinned snapshots".into(),
        indexed: BTreeSet::from(["comparison".into()]),
        missing: BTreeSet::new(),
        limitations: confounders,
    };
    coverage.limitations.push("A clean API comparison does not imply unchanged behavior; dependency, runtime and project compatibility require exact-environment verification.".into());
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
    let mut result = result.ok_with_pagination(Pagination {
        returned,
        total_matches: Some(total),
        truncated: false,
        next_cursor: None,
    });
    loop {
        let returned = data.changes.len() as u64;
        let more = page.has_more || offset + returned < total;
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
                Err(e) => return common::store_error(&e),
            }
        } else {
            None
        };
        result.data = common::to_object(&data);
        result.summary = format!(
            "{total} evidence change(s); returning {returned}, with {offset} already returned"
        );
        result.pagination = Pagination {
            returned,
            total_matches: Some(total),
            truncated: more,
            next_cursor,
        };
        if common::json_size(&result) <= budget || data.changes.len() <= 1 {
            break;
        }
        data.changes.pop();
    }
    if !data.comparable || (want_api && !api_complete) {
        use crate::envelope::IntoPartial;
        result = result.into_partial();
    }
    common::enforce_budget(service, result, request.max_bytes)
}
