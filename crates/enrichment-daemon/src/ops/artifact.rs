//! `artifact.read`: retrieve large result sections without flooding context (blueprint §7,
//! `read_artifact`; §7.1: only service-issued handles, never a filesystem path).
//!
//! Slices are byte ranges of a content-addressed blob, cut on a UTF-8 boundary for text and
//! base64-encoded otherwise, each with its own digest. Paging uses the same checksummed cursor
//! as search, bound to the artifact. A markdown `section` selects one heading's body.

use enrichment_core::canonical;
use enrichment_core::evidence::{Artifact, ArtifactKind, is_artifact_id};
use enrichment_core::operation::selections::{
    ArtifactReadAction, ArtifactSlicePlan, ArtifactWindow,
};
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
            Ok(window) => match window
                .map(|window| -> Result<_, std::num::TryFromIntError> {
                    Ok(ArtifactWindow {
                        start: usize::try_from(window.start)?,
                        end: usize::try_from(window.end)?,
                        section: Some(name.as_str().into()),
                    })
                })
                .transpose()
            {
                Ok(window) => window,
                Err(error) => return common::operation_error(&error, "result_section"),
            },
            Err(error) => return common::operation_error(&error, "result_section"),
        }
    } else {
        None
    };
    let runtime = service.repository.runtime.clone();
    let plan = match enrichment_store::artifact_selection::plan(
        &runtime,
        &artifact,
        &request,
        selected_window,
    )
    .await
    {
        Ok(plan) => plan,
        Err(error) => return common::operation_error(&error, "artifact_selection"),
    };
    match plan.action {
        ArtifactReadAction::Missing => {
            return envelope::error(
                ErrorCode::ArtifactUnavailable,
                "The retained result has no requested section",
                "Choose a section listed in delivery.sections.",
                false,
            );
        }
        ArtifactReadAction::Unsupported => {
            return envelope::error(
                ErrorCode::UnsupportedFormat,
                "Markdown selection requires text and a nonempty heading of at most 512 bytes",
                "Use a valid Markdown heading or omit section for byte reading.",
                false,
            );
        }
        ArtifactReadAction::Corrupt => {
            return common::operation_error(
                &std::io::Error::other("native result window exceeds retained bytes"),
                "result_section",
            );
        }
        ArtifactReadAction::Bytes | ArtifactReadAction::Markdown => {}
    }
    let scan_markdown = plan.action == ArtifactReadAction::Markdown;
    let protection = match pin.protect_artifact(&artifact).await {
        Ok(protection) => protection,
        Err(error) => return common::operation_error(&error, "artifact_retention"),
    };
    let capture_scope = (pin.clone(), protection.clone());
    let blobs = service.blobs.clone();
    let captured = artifact.clone();
    let prepared = runtime
        .blocking(move || -> std::io::Result<_> {
            // A cancelled waiter cannot release durable ownership while blocking I/O runs.
            let _scope = capture_scope;
            let mut file = blobs.capture(&captured, 256 * 1024 * 1024)?;
            let windows = if scan_markdown {
                super::artifact_window::sections(&mut file)?
            } else {
                Vec::new()
            };
            Ok((file, windows))
        })
        .await;
    let (file, windows) = match prepared {
        Ok(Ok(prepared)) => prepared,
        Ok(Err(error)) => return common::operation_error(&error, "artifact_read"),
        Err(error) => return common::operation_error(&error, "artifact_read"),
    };
    let window = if scan_markdown {
        let Some(heading) = plan.heading.as_deref() else {
            return common::operation_error(
                &std::io::Error::other("native markdown instruction has no heading"),
                "artifact_selection",
            );
        };
        match enrichment_store::artifact_selection::markdown(&runtime, windows, heading).await {
            Ok(Some(window)) => window,
            Ok(None) => {
                return envelope::error(
                    ErrorCode::ArtifactUnavailable,
                    format!("artifact {id} has no heading {heading}"),
                    "Read the artifact without section to inspect its headings.",
                    false,
                );
            }
            Err(error) => return common::operation_error(&error, "artifact_selection"),
        }
    } else {
        plan.window
    };
    let budget = common::byte_budget(service, request.max_bytes);
    let digest = enrichment_core::operation::selections::ArtifactSelection {
        section: window.section.clone(),
        start: window.start,
        end: window.end,
        max_bytes: budget,
    }
    .identity();
    let offset = match &request.cursor {
        None => 0,
        Some(cursor) => match Cursor::decode(cursor, id, &digest, SORT) {
            Ok(cursor) => match usize::try_from(cursor.offset) {
                Ok(offset) => offset,
                Err(error) => {
                    return envelope::error(
                        ErrorCode::InvalidCursor,
                        error.to_string(),
                        "Restart without a cursor.",
                        false,
                    );
                }
            },
            Err(error) => {
                let next = match error {
                    CursorError::Malformed => "Start again without a cursor.",
                    CursorError::Mismatch { .. } => {
                        "A cursor is valid only for the same artifact and section. Start again without a cursor."
                    }
                };
                return envelope::error(ErrorCode::InvalidCursor, error.to_string(), next, false);
            }
        },
    };
    let slice = match enrichment_store::artifact_selection::slice(
        &runtime,
        &window,
        offset,
        budget,
        plan.is_text,
    )
    .await
    {
        Ok(slice) => slice,
        Err(error) => return common::operation_error(&error, "artifact_slice"),
    };
    if slice.invalid_offset {
        return envelope::error(
            ErrorCode::InvalidCursor,
            "Artifact cursor exceeds its byte window",
            "Restart without a cursor.",
            false,
        );
    }
    runtime
        .blocking(move || {
            // The captured provider lease survives selection, scanning, byte copying and encoding.
            let _pin = (pin, protection);
            read_blocking(
                request,
                ReadBytes {
                    artifact,
                    file,
                    window,
                    slice,
                    digest,
                    is_text: plan.is_text,
                    budget,
                },
            )
        })
        .await
        .unwrap_or_else(|error| common::operation_error(&error, "artifact_read"))
}

struct ReadBytes {
    artifact: Artifact,
    file: std::fs::File,
    window: ArtifactWindow,
    slice: ArtifactSlicePlan,
    digest: String,
    is_text: bool,
    budget: usize,
}

fn read_blocking(request: ReadArtifactRequest, prepared: ReadBytes) -> Envelope {
    let ReadBytes {
        artifact,
        mut file,
        window,
        slice,
        digest,
        is_text,
        budget,
    } = prepared;
    let id = request.artifact_id.trim();
    let total = artifact.size_bytes;
    let (window_start, window_end, section_name) = (window.start, window.end, window.section);
    let start = slice.start;
    let mut end = slice.end;
    let bytes = match super::artifact_window::range(&mut file, start, slice.read_bytes) {
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
            data: common::payload(&data),
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
        let changes = envelope::fixture_payload(&"é😀\n\"\\".repeat(3000));
        let answer = envelope::ok(
            "indexed Unicode result",
            changes.clone(),
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
            enrichment_store::result::store(&service.blobs, &answer, "service:bounded-result/4")
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
                name: ResultSectionName::Data,
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
            let enrichment_core::wire::data::ToolData::ReadArtifact(data) = result.data else {
                panic!("native artifact payload")
            };
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
            serde_json::to_value(changes).unwrap()
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
