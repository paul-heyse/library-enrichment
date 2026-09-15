//! Fragments read from a crate's own source archive (blueprint §4.2, "Targeted
//! documentation/source reader"; §4.5, discovery beyond API additions).
//!
//! Feature definitions, README sections, changelog sections and examples are `declared`
//! evidence: the author wrote them, and nothing here executes or interprets them. Source
//! excerpts for a symbol are cut on demand from the same tree.

use std::path::Path;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::docsrs::ManifestFacts;
use crate::evidence::{EvidenceFragment, FragmentKind};
use crate::wire::EvidenceClass;

/// Producer name.
pub const PRODUCER: &str = "crate-source";
/// Producer version.
pub const VERSION: &str = "2";

/// Largest text kept for one section or example.
const MAX_FRAGMENT_CHARS: usize = 8_000;

/// The text files a crate ships that become fragments, relative to the crate root.
pub const DOCUMENT_FILES: &[(&str, FragmentKind)] = &[
    ("README.md", FragmentKind::ReadmeSection),
    ("CHANGELOG.md", FragmentKind::ChangelogSection),
    ("CHANGES.md", FragmentKind::ChangelogSection),
    ("HISTORY.md", FragmentKind::ChangelogSection),
];

/// Files under an extracted crate that are worth storing as their own text artifacts: the
/// documents above plus every `examples/*.rs`.
/// # Errors
/// Unsafe entries, I/O and excessive document inventories remain explicit.
pub fn text_files(crate_root: &Path) -> std::io::Result<Vec<(String, FragmentKind)>> {
    let mut out = Vec::new();
    for (file, kind) in DOCUMENT_FILES {
        match std::fs::symlink_metadata(crate_root.join(file)) {
            Ok(metadata) if metadata.is_file() => out.push(((*file).into(), *kind)),
            Ok(_) => {
                return Err(std::io::Error::other(
                    "source document is not a regular file",
                ));
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    let directory = crate_root.join("examples");
    match std::fs::symlink_metadata(&directory) {
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => {
            return Err(std::io::Error::other(
                "examples is not a physical directory",
            ));
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
        Err(e) => return Err(e),
    }
    let mut count = 0usize;
    let mut bytes = 0usize;
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        count += 1;
        if count > 20_000 {
            return Err(std::io::Error::other(
                "source inventory exceeds 20000 entries",
            ));
        }
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            if !entry.file_type()?.is_file() {
                return Err(std::io::Error::other(
                    "source example is not a regular file",
                ));
            }
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| std::io::Error::other("source example name is not UTF-8"))?;
            bytes += name.len() + 9;
            if bytes > 1024 * 1024 || out.len() >= 8192 {
                return Err(std::io::Error::other(
                    "source descriptor inventory exceeds budget",
                ));
            }
            out.push((format!("examples/{name}"), FragmentKind::Example));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Emit declared feature records without retaining a feature-document collection.
pub fn visit_features(
    facts: &ManifestFacts,
    manifest_artifact_id: &str,
    emit: &mut dyn FnMut(EvidenceFragment) -> Result<(), String>,
) -> Result<(), String> {
    for (feature, enables) in &facts.features {
        crate::canonical::serialized_size(&(feature, enables), 256 * 1024)
            .map_err(|e| e.to_string())?;
        let text = if enables.is_empty() {
            format!("feature `{feature}` enables nothing further")
        } else {
            format!("feature `{feature}` enables: {}", enables.join(", "))
        };
        emit(EvidenceFragment::new(
            FragmentKind::FeatureDefinition,
            feature,
            manifest_artifact_id,
            serde_json::json!({ "path": "Cargo.toml", "table": "features", "key": feature }),
            text,
            EvidenceClass::Declared,
            PRODUCER,
            VERSION,
        )?)?;
    }
    Ok(())
}

/// Emit bounded text from one verified document. Rust markdown is sectioned; examples and
/// Python document excerpts retain their declared file label. No complete section list exists.
pub fn visit_document(
    text: &str,
    file: &str,
    artifact: &str,
    kind: FragmentKind,
    rust_sections: bool,
    emit: &mut dyn FnMut(EvidenceFragment) -> Result<(), String>,
) -> Result<(), String> {
    if text.len() > 64 * 1024 * 1024 {
        return Err("source document exceeds 64 MiB".into());
    }
    if !rust_sections || kind == FragmentKind::Example {
        let subject = if rust_sections {
            Path::new(file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("example")
        } else {
            file
        };
        emit(EvidenceFragment::new(
            kind,
            subject,
            artifact,
            serde_json::json!({"path":file,"line":1}),
            bounded(text),
            EvidenceClass::Declared,
            PRODUCER,
            VERSION,
        )?)?;
    } else {
        visit_markdown_sections(text, &mut |heading, line, body| {
            emit(EvidenceFragment::new(
                kind,
                heading,
                artifact,
                serde_json::json!({"path":file,"heading":heading,"line":line}),
                bounded(body),
                EvidenceClass::Declared,
                PRODUCER,
                VERSION,
            )?)
        })?;
    }
    Ok(())
}

/// Visit borrowed markdown section slices. Original line delimiters are retained in bodies.
/// The only live state is one heading, line number and byte range, even for very long sections.
pub fn visit_markdown_sections(
    text: &str,
    emit: &mut dyn FnMut(&str, usize, &str) -> Result<(), String>,
) -> Result<(), String> {
    let mut heading = "preamble";
    let mut heading_line = 1;
    let mut start = 0;
    let mut in_fence = false;
    for (i, range) in crate::evidence::text::lines(text).enumerate() {
        let line = &text[range.start..range.content_end];
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
        }
        let hashes = line.trim_start().chars().take_while(|c| *c == '#').count();
        let next = (!in_fence && (1..=6).contains(&hashes))
            .then(|| line.trim_start().trim_start_matches('#').trim())
            .filter(|h| !h.is_empty());
        if let Some(next) = next {
            if next.len() > 16 * 1024 {
                return Err("markdown heading exceeds 16 KiB".into());
            }
            let body = text[start..range.start].trim();
            if !body.is_empty() || heading != "preamble" {
                emit(heading, heading_line, body)?;
            }
            heading = next;
            heading_line = i + 1;
            start = range.end;
        }
    }
    let body = text[start..].trim();
    if !body.is_empty() || heading != "preamble" {
        emit(heading, heading_line, body)?;
    }
    Ok(())
}

/// Read an already validated extracted member with a byte bound before allocation growth.
pub fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    read_file_limited(path, 64 * 1024 * 1024)
}

/// Read a regular source member with an operation-specific byte bound.
/// # Errors
/// Non-regular or oversized bytes fail before the output grows beyond the bound.
pub fn read_file_limited(path: &Path, limit: u64) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    if limit == 0 || limit > 64 * 1024 * 1024 {
        return Err(std::io::Error::other("invalid source byte bound"));
    }
    let mut options = std::fs::File::options();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x20000 | 0x800);
    }
    let file = options.open(path)?;
    if !file.metadata()?.is_file() || file.metadata()?.len() > limit {
        return Err(std::io::Error::other(
            "source member is not a bounded regular file",
        ));
    }
    let mut bytes = Vec::new();
    file.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(std::io::Error::other("source member grew beyond bound"));
    }
    Ok(bytes)
}

/// A bounded source excerpt starting at a recorded line; it does not identify the end of an item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SourceExcerpt {
    pub window_kind: SourceWindowKind,
    /// The file, relative to the crate root.
    pub path: String,
    /// First line included, 1-based.
    pub start_line: usize,
    /// Last line included, 1-based.
    pub end_line: usize,
    /// The text.
    pub text: String,
    /// Whether the file had more lines after `end_line`.
    pub truncated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceWindowKind {
    RecordedLineWindow,
}

/// Cut up to `max_lines` lines starting at `line` from `file` under `crate_root`.
///
/// `file` must be a relative path with no parent components; anything else is refused, so a
/// recorded span can never read outside the extracted crate.
pub fn source_excerpt_checked(
    crate_root: &Path,
    file: &str,
    line: usize,
    max_lines: usize,
) -> std::io::Result<Option<SourceExcerpt>> {
    let rel = Path::new(file);
    if rel.is_absolute()
        || rel
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)))
    {
        return Err(std::io::Error::other(
            "source path must be a relative archive member",
        ));
    }
    let mut path = crate_root.to_owned();
    for component in rel.components() {
        path.push(component);
        if std::fs::symlink_metadata(&path)?.file_type().is_symlink() {
            return Err(std::io::Error::other("source member contains a link"));
        }
    }
    let mut options = std::fs::File::options();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x20000 | 0x800);
    }
    let input = options.open(path)?;
    if !input.metadata()?.is_file() || input.metadata()?.len() > 64 * 1024 * 1024 {
        return Err(std::io::Error::other(
            "source member is not a bounded regular file",
        ));
    }
    source_excerpt_reader(std::io::BufReader::new(input), file, line, max_lines)
}

/// Cut original source bytes from an already verified reader, sharing CRLF/CR/LF semantics.
/// # Errors
/// Invalid selected UTF-8, I/O, a long line or an excerpt above one MiB is refused.
pub fn source_excerpt_reader(
    source: impl std::io::BufRead,
    file: &str,
    line: usize,
    max_lines: usize,
) -> std::io::Result<Option<SourceExcerpt>> {
    let mut reader = crate::evidence::text::LineReader::new(source, 1024 * 1024);
    let start = line.max(1);
    let limit = max_lines.clamp(1, 1024);
    let mut selected = Vec::new();
    let mut count = 0;
    let mut end = 0;
    let mut truncated = false;
    for index in 1..=start.saturating_add(limit) {
        let Some(bytes) = reader.next_line()? else {
            break;
        };
        if index < start {
            continue;
        }
        if count == limit {
            truncated = true;
            break;
        }
        if bytes.len() > (1024 * 1024usize).saturating_sub(selected.len()) {
            return Err(std::io::Error::other("source excerpt exceeds one MiB"));
        }
        let text = std::str::from_utf8(&bytes).map_err(std::io::Error::other)?;
        let span = crate::evidence::text::lines(text)
            .next()
            .expect("one logical line");
        end = selected.len() + span.content_end;
        selected.extend_from_slice(&bytes);
        count += 1;
    }
    if count == 0 {
        return Ok(None);
    }
    selected.truncate(end);
    Ok(Some(SourceExcerpt {
        window_kind: SourceWindowKind::RecordedLineWindow,
        path: file.into(),
        start_line: start,
        end_line: start + count - 1,
        text: String::from_utf8(selected).map_err(std::io::Error::other)?,
        truncated,
    }))
}

fn bounded(text: &str) -> String {
    if text.chars().count() <= MAX_FRAGMENT_CHARS {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(MAX_FRAGMENT_CHARS - 1).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MarkdownSection {
        heading: String,
        line: usize,
        body: String,
    }
    fn split_markdown_sections(text: &str) -> Vec<MarkdownSection> {
        let mut out = Vec::new();
        visit_markdown_sections(text, &mut |heading, line, body| {
            out.push(MarkdownSection {
                heading: heading.into(),
                line,
                body: body.into(),
            });
            Ok(())
        })
        .expect("sections");
        out
    }
    fn source_fragments(
        root: &Path,
        facts: &ManifestFacts,
        manifest: &str,
        tarball: &str,
        artifact_for: &dyn Fn(&str) -> Option<String>,
    ) -> Vec<EvidenceFragment> {
        let mut out = Vec::new();
        visit_features(facts, manifest, &mut |row| {
            out.push(row);
            Ok(())
        })
        .expect("features");
        for (file, kind) in text_files(root).expect("fixture inventory") {
            let text = std::fs::read_to_string(root.join(&file)).expect("text");
            visit_document(
                &text,
                &file,
                &artifact_for(&file).unwrap_or_else(|| tarball.into()),
                kind,
                true,
                &mut |row| {
                    out.push(row);
                    Ok(())
                },
            )
            .expect("document");
        }
        out
    }

    #[test]
    fn section_visits_borrow_original_crlf_body_and_propagate_sink_failure() {
        let text = format!(
            "# Large\r\n{}\r\n# Next\r\nlast",
            "x".repeat(2 * 1024 * 1024)
        );
        let mut visits = 0;
        visit_markdown_sections(&text, &mut |heading, line, body| {
            visits += 1;
            if visits == 1 {
                assert_eq!((heading, line), ("Large", 1));
                assert_eq!(body.len(), 2 * 1024 * 1024);
                assert_eq!(body.as_ptr(), text[9..].as_ptr());
            } else {
                assert_eq!((heading, line, body), ("Next", 3, "last"));
            }
            Ok(())
        })
        .unwrap();
        assert_eq!(visits, 2);
        let mut calls = 0;
        let error = visit_markdown_sections(&text, &mut |_, _, _| {
            calls += 1;
            Err("stop".into())
        })
        .unwrap_err();
        assert_eq!(error, "stop");
        assert_eq!(calls, 1);
    }

    #[test]
    fn markdown_splits_at_headings_and_ignores_fenced_hashes() {
        let text =
            "intro\n\n# Title\n\nbody one\n\n```\n# not a heading\n```\n\n## Sub\nbody two\n";
        let sections = split_markdown_sections(text);
        let headings: Vec<&str> = sections.iter().map(|s| s.heading.as_str()).collect();
        assert_eq!(headings, vec!["preamble", "Title", "Sub"]);
        assert!(sections[1].body.contains("# not a heading"));
        assert_eq!(sections[2].line, 11);
        assert_eq!(sections[2].body, "body two");
    }

    #[test]
    fn a_source_excerpt_is_bounded_and_confined() {
        let dir = tempfile::tempdir().expect("dir");
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).expect("mkdir");
        std::fs::write(src.join("lib.rs"), "l1\nl2\nl3\nl4\nl5\n").expect("write");
        let ex = source_excerpt_checked(dir.path(), "src/lib.rs", 2, 3)
            .expect("read")
            .expect("excerpt");
        assert_eq!((ex.start_line, ex.end_line), (2, 4));
        assert_eq!(ex.text, "l2\nl3\nl4");
        assert!(ex.truncated);
        assert!(source_excerpt_checked(dir.path(), "../etc/passwd", 1, 3).is_err());
        assert!(source_excerpt_checked(dir.path(), "/etc/passwd", 1, 3).is_err());
        assert!(source_excerpt_checked(dir.path(), "src/missing.rs", 1, 3).is_err());
    }

    #[test]
    fn source_excerpt_preserves_original_delimiters_and_final_empty_line() {
        let bytes = "a\r\r\n🌎é\r\n".as_bytes();
        let excerpt =
            source_excerpt_reader(std::io::BufReader::with_capacity(1, bytes), "src.py", 2, 2)
                .expect("read")
                .expect("excerpt");
        assert_eq!(excerpt.text, "\r\n🌎é");
        assert_eq!(
            (excerpt.start_line, excerpt.end_line, excerpt.truncated),
            (2, 3, true)
        );
        let end = source_excerpt_reader(bytes, "src.py", 4, 2)
            .expect("read")
            .expect("final empty line");
        assert_eq!(
            (
                end.text.as_str(),
                end.start_line,
                end.end_line,
                end.truncated
            ),
            ("", 4, 4, false)
        );
        assert!(source_excerpt_reader(&b"\xff\n"[..], "bad", 1, 2).is_err());
    }

    #[test]
    fn the_fixture_crate_yields_features_readme_changelog_and_example() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("repo root")
            .join("tests/fixtures/crates/enr-fixture-0.2.0");
        let manifest = std::fs::read_to_string(root.join("Cargo.toml")).expect("manifest");
        let facts = super::super::docsrs::manifest_facts(&manifest).expect("facts");
        let files = text_files(&root).expect("fixture inventory");
        assert!(files.iter().any(|(f, _)| f == "README.md"));
        assert!(
            files
                .iter()
                .any(|(f, k)| f == "examples/basic.rs" && *k == FragmentKind::Example)
        );
        let fragments = source_fragments(&root, &facts, "art_manifest", "art_tarball", &|file| {
            (file == "README.md").then(|| "art_readme".to_owned())
        });
        let kinds = |k: FragmentKind| fragments.iter().filter(|f| f.kind == k).count();
        assert_eq!(kinds(FragmentKind::FeatureDefinition), 3);
        assert!(kinds(FragmentKind::ReadmeSection) >= 2);
        assert!(fragments.iter().any(|f| {
            f.kind == FragmentKind::ChangelogSection && f.text.contains("behaviour-only")
        }));
        assert!(
            fragments
                .iter()
                .any(|f| f.kind == FragmentKind::Example && f.subject == "basic")
        );
        assert!(
            fragments
                .iter()
                .all(|f| f.evidence_class == EvidenceClass::Declared)
        );
        assert!(
            fragments
                .iter()
                .filter(|f| f.kind == FragmentKind::ReadmeSection)
                .all(|f| f.artifact_id == "art_readme")
        );
        assert!(
            fragments
                .iter()
                .filter(|f| f.kind == FragmentKind::ChangelogSection)
                .all(|f| f.artifact_id == "art_tarball")
        );
    }
}
