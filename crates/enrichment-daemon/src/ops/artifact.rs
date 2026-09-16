//! `artifact.read`: retrieve large result sections without flooding context (blueprint §7,
//! `read_artifact`; §7.1: only service-issued handles, never a filesystem path).
//!
//! Slices are byte ranges of a content-addressed blob, cut on a UTF-8 boundary for text and
//! base64-encoded otherwise, each with its own digest. Paging uses the same checksummed cursor
//! as search, bound to the artifact. A markdown `section` selects one heading's body.

use enrichment_core::canonical;
use enrichment_core::evidence::{Artifact, ArtifactKind, is_artifact_id};
use enrichment_core::request::ReadArtifactRequest;
use enrichment_core::search::{Cursor, CursorError};
use enrichment_core::wire::data::{ArtifactSliceData, SliceEncoding};
use enrichment_core::wire::research::ArtifactSection;
use enrichment_core::wire::{Coverage, Envelope, ErrorCode, Freshness, Page, SourceVersionMatch};

use super::common;
use crate::envelope::{self, Research};
use crate::service::Service;

const SORT: &str = "bytes";

/// Read a slice of an artifact.
pub async fn read(service: &Service, request: ReadArtifactRequest) -> Envelope {
    let id = request.artifact_id.trim();
    if !is_artifact_id(id) {
        return envelope::error(
            ErrorCode::ArtifactUnavailable,
            format!("`{id}` is not a service-issued artifact handle"),
            "Pass an `artifact_id` from a result's `artifacts`; filesystem paths are never \
             accepted.",
            false,
        );
    }
    let pin = match service.repository.catalog.pin().await {
        Ok(pin) => pin,
        Err(error) => return common::operation_error(&error, "artifact_catalog"),
    };
    let artifact = match pin.artifact(&service.repository.runtime, id).await {
        Ok(Some(a)) => a,
        Ok(None) => {
            return envelope::error(
                ErrorCode::ArtifactUnavailable,
                format!("artifact {id} is not stored on this service"),
                "Artifacts are minted by `resolve_library`; resolve the release again if the \
                 cache was cleared.",
                false,
            );
        }
        Err(err) => return common::operation_error(&err, "artifact_read"),
    };
    let selected_window = if let Some(ArtifactSection::Result { name }) = &request.section {
        match pin
            .result_section(&service.repository.runtime, id, name.as_str())
            .await
        {
            Ok(Some(window)) => Some(window),
            Ok(None) => {
                return envelope::error(
                    ErrorCode::ArtifactUnavailable,
                    "The captured native result has no requested section",
                    "Choose a section listed in delivery.sections.",
                    false,
                );
            }
            Err(error) => return common::operation_error(&error, "result_section"),
        }
    } else {
        None
    };
    let service = service.clone();
    let runtime = service.repository.runtime.clone();
    runtime
        .blocking(move || {
            // Keep the captured catalog lease until the byte driver has finished reading.
            let _pin = pin;
            read_blocking(&service, request, artifact, selected_window)
        })
        .await
        .unwrap_or_else(|error| common::operation_error(&error, "artifact_read"))
}

fn read_blocking(
    service: &Service,
    request: ReadArtifactRequest,
    artifact: Artifact,
    selected_window: Option<enrichment_store::result::Window>,
) -> Envelope {
    let id = request.artifact_id.trim();
    let mut file = match service.blobs.capture(&artifact, 256 * 1024 * 1024) {
        Ok(b) => b,
        Err(err) => return common::operation_error(&err, "artifact_read"),
    };
    let total = artifact.size_bytes;
    let is_text = artifact.media_type.starts_with("text/")
        || artifact.media_type.contains("json")
        || artifact.media_type.contains("toml")
        || artifact.media_type.contains("markdown");

    // A section narrows the byte window before paging.
    let mut window_start = 0usize;
    let mut window_end = match usize::try_from(total) {
        Ok(n) => n,
        Err(e) => return common::operation_error(&e, "artifact_read"),
    };
    let mut section_name = None;
    if let Some(section) = &request.section {
        match section {
            ArtifactSection::Result { name } => {
                let Some(window) = selected_window else {
                    return common::operation_error(
                        &std::io::Error::other("native result window absent"),
                        "result_section",
                    );
                };
                if window.start > window.end || window.end > total {
                    return common::operation_error(
                        &std::io::Error::other("native result window exceeds retained bytes"),
                        "result_section",
                    );
                }
                window_start = window.start as usize;
                window_end = window.end as usize;
                section_name = Some(name.as_str().to_owned());
            }
            ArtifactSection::Markdown { heading } => {
                if !is_text || heading.trim().is_empty() || heading.len() > 512 {
                    return envelope::error(
                        ErrorCode::UnsupportedFormat,
                        "Markdown selection requires text and a nonempty heading of at most 512 bytes",
                        "Use a valid Markdown heading or omit section for byte reading.",
                        false,
                    );
                }
                match super::artifact_window::section(&mut file, heading.trim()) {
                    Ok(Some((start, end, heading))) => {
                        window_start = start;
                        window_end = end;
                        section_name = Some(heading);
                    }
                    Ok(None) => {
                        return envelope::error(
                            ErrorCode::ArtifactUnavailable,
                            format!("artifact {id} has no heading {heading}"),
                            "Read the artifact without section to inspect its headings.",
                            false,
                        );
                    }
                    Err(error) => return common::operation_error(&error, "artifact_read"),
                }
            }
        }
    }

    let digest = canonical::short_id(
        "q",
        &serde_json::json!({ "section": section_name, "window": [window_start, window_end], "budget": common::byte_budget(service,request.max_bytes) }),
    );
    let offset = match &request.cursor {
        None => 0usize,
        Some(cursor) => match Cursor::decode(cursor, id, &digest, SORT) {
            Ok(c) => match usize::try_from(c.offset)
                .ok()
                .filter(|n| *n <= window_end - window_start)
            {
                Some(offset) => offset,
                None => {
                    return envelope::error(
                        ErrorCode::InvalidCursor,
                        "Artifact cursor exceeds its byte window",
                        "Restart without a cursor.",
                        false,
                    );
                }
            },
            Err(err) => {
                let next = match err {
                    CursorError::Malformed => "Start again without a cursor.",
                    CursorError::Mismatch { .. } => {
                        "A cursor is valid only for the same artifact and section. Start again \
                         without a cursor."
                    }
                };
                return envelope::error(ErrorCode::InvalidCursor, err.to_string(), next, false);
            }
        },
    };

    let budget = common::byte_budget(service, request.max_bytes);
    // Base64 inflates by 4/3; keep the encoded slice inside the budget.
    let raw_budget = if is_text { budget } else { budget * 3 / 4 };
    let start = window_start + offset;
    let mut end = start.saturating_add(raw_budget).min(window_end);
    let bytes = match super::artifact_window::range(
        &mut file,
        start,
        (end - start).saturating_add(1).min(window_end - start),
    ) {
        Ok(bytes) => bytes,
        Err(error) => return common::operation_error(&error, "artifact_read"),
    };
    if is_text {
        // Back off to a UTF-8 boundary: a continuation byte is `10xxxxxx`.
        while end > start && end < window_end && (bytes[end - start] & 0xC0) == 0x80 {
            end -= 1;
        }
    }
    let render = |end: usize| -> std::io::Result<Envelope> {
        let slice = &bytes[..end - start];
        let (content, encoding) = if is_text {
            match std::str::from_utf8(slice) {
                Ok(text) => (text.to_owned(), SliceEncoding::Utf8),
                Err(_) => (base64(slice), SliceEncoding::Base64),
            }
        } else {
            (base64(slice), SliceEncoding::Base64)
        };
        let remaining = (window_end - end) as u64;
        let next_cursor = (remaining > 0)
            .then(|| Cursor::new(id, &digest, SORT, (end - window_start) as u64).encode())
            .transpose()
            .map_err(std::io::Error::other)?;

        let data = ArtifactSliceData {
            page: Page::default(),
            artifact: artifact.clone(),
            encoding,
            start: start as u64,
            end: end as u64,
            total,
            content,
            content_digest: canonical::sha256_hex(slice),
            remaining,
            section: section_name.clone(),
        };
        let description = format!(
            "{:?}, first recorded at {}",
            artifact.kind, artifact.source_uri
        );
        let handle = common::handle_for(&artifact, description);
        let notes = match artifact.kind {
            ArtifactKind::CrateTarball => vec![
                "This is the compressed source archive; its README, changelog and examples are \
                 stored as their own text artifacts and its source is reachable through \
                 `inspect_symbol` with an explicit source aspect."
                    .to_owned(),
            ],
            _ => Vec::new(),
        };
        let mut result = Research {
            summary: format!(
                "{} bytes {start}..{end} of {total} from artifact {id}{}",
                match encoding {
                    SliceEncoding::Utf8 => "UTF-8",
                    SliceEncoding::Base64 => "base64",
                },
                section_name
                    .as_deref()
                    .map(|s| format!(" (section `{s}`)"))
                    .unwrap_or_default()
            ),
            data: common::to_object(&data),
            coverage: Coverage {
                details: None,
                assessments: Vec::new(),
                scope: format!("bytes {start}..{end} of artifact {id}"),
                indexed: ["artifact_bytes".to_owned()].into_iter().collect(),
                missing: std::collections::BTreeSet::new(),
                limitations: notes,
            },
            freshness: Freshness {
                registry_checked_at: None,
                // `Unknown`, not `Exact`. This field answers "how well does the evidence's source
                // version match the version that was asked about", and `read_artifact` is reached
                // by digest with no version asked about at all. Worse, the only provenance
                // available here is the stored record -- the FIRST retrieval's -- and a file
                // unchanged between two releases hashes identically, so `artifact.source_uri` can
                // name a different release of the same package. `Exact` beside that URI read as a
                // guarantee the service cannot make. The digest is the guarantee; the version
                // relationship is genuinely unknown. See register row R-17.
                source_version_match: SourceVersionMatch::Unknown,
                latest_verified: false,
            },
            context_id: None,
            snapshot_id: None,
            evidence: Vec::new(),
            artifacts: handle.into_iter().collect(),
        }
        .ok_with_page(Page::new(1, None, remaining > 0, next_cursor));
        result.delivery.set_limits(request.max_bytes, budget);
        Ok(result)
    };
    let full = match render(end) {
        Ok(result) => result,
        Err(error) => return common::operation_error(&error, "artifact_read"),
    };
    if common::json_size(&full) <= budget {
        return full;
    }
    // Search actual encoded sizes over valid byte boundaries; retain the largest fitting
    // prefix instead of repeatedly discarding half a page. The full/end-of-window candidate
    // is tested separately because dropping its cursor reduces framing size discontinuously.
    let boundaries: Vec<usize> = (0..=end - start)
        .filter(|offset| {
            !is_text
                || *offset == 0
                || start + offset == window_end
                || (bytes[*offset] & 0xC0) != 0x80
        })
        .collect();
    let mut low = 0usize;
    let mut high = boundaries.len();
    let mut best = None;
    while low < high {
        let mid = low + (high - low) / 2;
        let result = match render(start + boundaries[mid]) {
            Ok(result) => result,
            Err(error) => return common::operation_error(&error, "artifact_read"),
        };
        if common::json_size(&result) <= budget {
            if boundaries[mid] > 0 {
                best = Some(result);
            }
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    best.unwrap_or_else(|| {
        let minimum = boundaries
            .iter()
            .copied()
            .find(|n| *n > 0)
            .and_then(|n| render(start + n).ok())
            .map_or(budget.saturating_add(1), |result| {
                common::json_size(&result)
            });
        crate::delivery::budget_failure(request.max_bytes, budget, minimum)
    })
}

/// Standard base64 without a dependency: artifacts are the only binary the service returns.
fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            TABLE[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn indexed_unicode_sections_page_exactly_with_high_encoded_utilization() {
        use enrichment_core::wire::research::ResultSectionName;
        let dir = tempfile::tempdir().unwrap();
        let service = Service::open(
            enrichment_core::config::Config::default(),
            enrichment_store::StatePaths::explicit(
                dir.path().join("cache"),
                dir.path().join("data"),
            ),
        )
        .unwrap();
        let changes = serde_json::json!([{"text": "é😀\n\"\\".repeat(3000)}]);
        let answer = envelope::ok(
            "indexed Unicode result",
            common::to_object(&serde_json::json!({"changes": changes})),
            Coverage {
                details: None,
                assessments: Vec::new(),
                scope: "section fixture".into(),
                indexed: Default::default(),
                missing: Default::default(),
                limitations: Vec::new(),
            },
        );
        let (artifact, _) =
            enrichment_store::result::store(&service.blobs, &answer, "service:bounded-result/3")
                .unwrap();
        service
            .repository
            .catalog
            .retain_result(&service.repository.runtime, &service.blobs, &artifact)
            .await
            .unwrap();
        let mut request = ReadArtifactRequest {
            artifact_id: artifact.artifact_id.clone(),
            section: Some(ArtifactSection::Result {
                name: ResultSectionName::Changes,
            }),
            max_bytes: Some(4096),
            ..Default::default()
        };
        let mut content = String::new();
        let mut last_end = None;
        let mut pages = 0;
        loop {
            let result = read(&service, request.clone()).await;
            assert_eq!(
                result.status(),
                enrichment_core::wire::Status::Ok,
                "{}",
                result.summary
            );
            let size = common::json_size(&result);
            assert!(size <= 4096);
            let data: ArtifactSliceData =
                serde_json::from_value(serde_json::Value::Object(result.data)).unwrap();
            assert_eq!(data.encoding, SliceEncoding::Utf8);
            if let Some(end) = last_end {
                assert_eq!(data.start, end);
            }
            assert!(data.end > data.start);
            last_end = Some(data.end);
            assert_eq!(
                data.content_digest,
                canonical::sha256_hex(data.content.as_bytes())
            );
            content.push_str(&data.content);
            pages += 1;
            assert!(pages < 100);
            if !data.page.has_more {
                assert!(data.page.next_cursor.is_none());
                break;
            }
            assert!(size >= 4096 - 16, "underfilled encoded page: {size}");
            request.cursor = data.page.next_cursor;
        }
        assert!(pages > 1);
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&content).unwrap(),
            changes
        );
    }

    #[test]
    fn base64_matches_the_standard_vectors() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }
}
