//! Fragments read from a crate's own source archive (blueprint §4.2, "Targeted
//! documentation/source reader"; §4.5, discovery beyond API additions).
//!
//! Feature definitions, README sections, changelog sections and examples are `declared`
//! evidence: the author wrote them, and nothing here executes or interprets them. Source
//! excerpts for a symbol are cut on demand from the same tree.

use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::docsrs::ManifestFacts;
use crate::evidence::{EvidenceFragment, FragmentKind};
use crate::wire::EvidenceClass;

/// Producer name.
pub const PRODUCER: &str = "crate-source";
/// Producer version.
pub const VERSION: &str = "1";

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
#[must_use]
pub fn text_files(crate_root: &Path) -> Vec<(String, FragmentKind)> {
    let mut out: Vec<(String, FragmentKind)> = DOCUMENT_FILES
        .iter()
        .filter(|(file, _)| crate_root.join(file).is_file())
        .map(|(file, kind)| ((*file).to_owned(), *kind))
        .collect();
    if let Ok(entries) = std::fs::read_dir(crate_root.join("examples")) {
        let mut files: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "rs"))
            .collect();
        files.sort();
        for file in files {
            if let Some(name) = file.file_name().and_then(|f| f.to_str()) {
                out.push((format!("examples/{name}"), FragmentKind::Example));
            }
        }
    }
    out
}

/// Read the declared fragments out of an extracted crate directory.
///
/// `crate_root` is the directory holding `Cargo.toml`. `artifact_for` maps a relative file
/// path to the artifact it was stored as; a file with no artifact of its own is attributed
/// to `tarball_artifact_id`. Missing files are simply absent; nothing here fails.
#[must_use]
pub fn source_fragments(
    crate_root: &Path,
    facts: &ManifestFacts,
    manifest_artifact_id: &str,
    tarball_artifact_id: &str,
    artifact_for: &dyn Fn(&str) -> Option<String>,
) -> Vec<EvidenceFragment> {
    let mut out = Vec::new();

    for (feature, enables) in &facts.features {
        let text = if enables.is_empty() {
            format!("feature `{feature}` enables nothing further")
        } else {
            format!("feature `{feature}` enables: {}", enables.join(", "))
        };
        out.push(EvidenceFragment::new(
            FragmentKind::FeatureDefinition,
            feature,
            manifest_artifact_id,
            serde_json::json!({ "path": "Cargo.toml", "table": "features", "key": feature }),
            text,
            EvidenceClass::Declared,
            PRODUCER,
            VERSION,
        ));
    }

    for (file, kind) in text_files(crate_root) {
        let Ok(text) = std::fs::read_to_string(crate_root.join(&file)) else {
            continue;
        };
        let artifact_id = artifact_for(&file).unwrap_or_else(|| tarball_artifact_id.to_owned());
        if kind == FragmentKind::Example {
            let name = Path::new(&file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("example")
                .to_owned();
            out.push(EvidenceFragment::new(
                kind,
                &name,
                &artifact_id,
                serde_json::json!({ "path": file, "line": 1 }),
                bounded(&text),
                EvidenceClass::Declared,
                PRODUCER,
                VERSION,
            ));
            continue;
        }
        for section in split_markdown_sections(&text) {
            out.push(EvidenceFragment::new(
                kind,
                &section.heading,
                &artifact_id,
                serde_json::json!({
                    "path": file, "heading": section.heading, "line": section.line
                }),
                bounded(&section.body),
                EvidenceClass::Declared,
                PRODUCER,
                VERSION,
            ));
        }
    }

    out
}

/// One markdown section: the heading text and the body under it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkdownSection {
    /// Heading text without the `#` markers; `preamble` for text before any heading.
    pub heading: String,
    /// 1-based line of the heading.
    pub line: usize,
    /// Body text, trimmed.
    pub body: String,
}

/// Split markdown at ATX headings. Text before the first heading becomes a `preamble` section.
#[must_use]
pub fn split_markdown_sections(text: &str) -> Vec<MarkdownSection> {
    let mut sections = Vec::new();
    let mut current = MarkdownSection {
        heading: "preamble".to_owned(),
        line: 1,
        body: String::new(),
    };
    let mut in_fence = false;
    for (i, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
        }
        let hashes = line.trim_start().chars().take_while(|c| *c == '#').count();
        let heading = (!in_fence && (1..=6).contains(&hashes))
            .then(|| line.trim_start().trim_start_matches('#').trim())
            .filter(|h| !h.is_empty());
        if let Some(heading) = heading {
            let body = current.body.trim().to_owned();
            if !body.is_empty() || current.heading != "preamble" {
                sections.push(MarkdownSection {
                    body,
                    ..current.clone()
                });
            }
            current = MarkdownSection {
                heading: heading.to_owned(),
                line: i + 1,
                body: String::new(),
            };
        } else {
            current.body.push_str(line);
            current.body.push('\n');
        }
    }
    let body = current.body.trim().to_owned();
    if !body.is_empty() || current.heading != "preamble" {
        sections.push(MarkdownSection { body, ..current });
    }
    sections
}

/// A bounded source excerpt around a line, for `inspect_symbol` at `source` depth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SourceExcerpt {
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

/// Cut up to `max_lines` lines starting at `line` from `file` under `crate_root`.
///
/// `file` must be a relative path with no parent components; anything else is refused, so a
/// recorded span can never read outside the extracted crate.
#[must_use]
pub fn source_excerpt(
    crate_root: &Path,
    file: &str,
    line: usize,
    max_lines: usize,
) -> Option<SourceExcerpt> {
    let rel = Path::new(file);
    if rel.is_absolute()
        || rel
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)))
    {
        return None;
    }
    let text = std::fs::read_to_string(crate_root.join(rel)).ok()?;
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return None;
    }
    let start = line.max(1).min(lines.len());
    let end = (start + max_lines.max(1) - 1).min(lines.len());
    Some(SourceExcerpt {
        path: file.to_owned(),
        start_line: start,
        end_line: end,
        text: lines[start - 1..end].join("\n"),
        truncated: end < lines.len(),
    })
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
        let ex = source_excerpt(dir.path(), "src/lib.rs", 2, 3).expect("excerpt");
        assert_eq!((ex.start_line, ex.end_line), (2, 4));
        assert_eq!(ex.text, "l2\nl3\nl4");
        assert!(ex.truncated);
        assert!(source_excerpt(dir.path(), "../etc/passwd", 1, 3).is_none());
        assert!(source_excerpt(dir.path(), "/etc/passwd", 1, 3).is_none());
        assert!(source_excerpt(dir.path(), "src/missing.rs", 1, 3).is_none());
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
        let files = text_files(&root);
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
