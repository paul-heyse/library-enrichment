//! `artifact.read`: retrieve large result sections without flooding context (blueprint §7,
//! `read_artifact`; §7.1: only service-issued handles, never a filesystem path).
//!
//! Slices are byte ranges of a content-addressed blob, cut on a UTF-8 boundary for text and
//! base64-encoded otherwise, each with its own digest. Paging uses the same checksummed cursor
//! as search, bound to the artifact. A markdown `section` selects one heading's body.

use enrichment_core::canonical;
use enrichment_core::evidence::{Artifact, ArtifactKind, is_artifact_id};
use enrichment_core::producer::source::split_markdown_sections;
use enrichment_core::request::ReadArtifactRequest;
use enrichment_core::search::{Cursor, CursorError};
use enrichment_core::wire::data::{ArtifactSliceData, SliceEncoding};
use enrichment_core::wire::{
    Coverage, Envelope, ErrorCode, Freshness, Pagination, SourceVersionMatch,
};

use super::common;
use crate::envelope::{self, Research};
use crate::service::Service;

const SORT: &str = "bytes";

/// Read a slice of an artifact.
pub fn read(service: &Service, request: ReadArtifactRequest) -> Envelope {
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
    let artifact: Artifact = match service.blobs.find(id) {
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
        Err(err) => return common::store_error(&err),
    };
    let bytes = match service.blobs.read(&artifact.sha256) {
        Ok(b) => b,
        Err(err) => return common::store_error(&err),
    };
    let total = bytes.len() as u64;
    let is_text = artifact.media_type.starts_with("text/")
        || artifact.media_type.contains("json")
        || artifact.media_type.contains("toml")
        || artifact.media_type.contains("markdown");

    // A section narrows the byte window before paging.
    let mut window_start = 0usize;
    let mut window_end = bytes.len();
    let mut section_name = None;
    if let Some(section) = request
        .section
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if !is_text {
            return envelope::error(
                ErrorCode::UnsupportedFormat,
                format!(
                    "artifact {id} is {} and has no sections",
                    artifact.media_type
                ),
                "Omit `section` for binary artifacts.",
                false,
            );
        }
        let text = String::from_utf8_lossy(&bytes);
        let Some(found) = split_markdown_sections(&text)
            .into_iter()
            .find(|s| s.heading.eq_ignore_ascii_case(section))
        else {
            return envelope::error(
                ErrorCode::ArtifactUnavailable,
                format!("artifact {id} has no section named `{section}`"),
                "Read the artifact without `section` to see its headings.",
                false,
            );
        };
        // Locate the body bytes: from the heading line's end to the section's end.
        let heading_offset = text
            .lines()
            .take(found.line.saturating_sub(1))
            .map(|l| l.len() + 1)
            .sum::<usize>();
        let body_start = text[heading_offset..]
            .find('\n')
            .map_or(text.len(), |i| heading_offset + i + 1);
        let body_end = text[body_start..]
            .find(&found.body)
            .map_or(text.len(), |i| body_start + i + found.body.len());
        window_start = body_start.min(text.len());
        window_end = body_end.min(text.len());
        section_name = Some(found.heading);
    }

    let digest = canonical::short_id(
        "q",
        &serde_json::json!({ "section": section_name, "window": [window_start, window_end] }),
    );
    let offset = match &request.cursor {
        None => 0usize,
        Some(cursor) => match Cursor::decode(cursor, id, &digest, SORT) {
            Ok(c) => c.offset as usize,
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
    let start = (window_start + offset).min(window_end);
    let mut end = (start + raw_budget).min(window_end);
    if is_text {
        // Back off to a UTF-8 boundary: a continuation byte is `10xxxxxx`.
        while end > start && end < window_end && (bytes[end] & 0xC0) == 0x80 {
            end -= 1;
        }
    }
    let slice = &bytes[start..end];
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
        .then(|| Cursor::new(id, &digest, SORT, (end - window_start) as u64).encode());

    let data = ArtifactSliceData {
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
    let description = format!("{:?} from {}", artifact.kind, artifact.source_uri);
    let handle = common::handle_for(&artifact, description);
    let kind_note = match artifact.kind {
        ArtifactKind::CrateTarball => Some(
            "This is the compressed source archive; its README, changelog and examples are \
             stored as their own text artifacts and its source is reachable through \
             `inspect_symbol` at depth=source."
                .to_owned(),
        ),
        _ => None,
    };
    Research {
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
            scope: format!("bytes {start}..{end} of artifact {id}"),
            indexed: ["artifact_bytes".to_owned()].into_iter().collect(),
            missing: std::collections::BTreeSet::new(),
            limitations: kind_note.into_iter().collect(),
        },
        freshness: Freshness {
            registry_checked_at: None,
            source_version_match: SourceVersionMatch::Exact,
            latest_verified: false,
        },
        context_id: None,
        snapshot_id: None,
        evidence: Vec::new(),
        artifacts: handle.into_iter().collect(),
    }
    .ok_with_pagination(Pagination {
        returned: 1,
        total_matches: None,
        truncated: remaining > 0,
        next_cursor,
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

    #[test]
    fn base64_matches_the_standard_vectors() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }
}
