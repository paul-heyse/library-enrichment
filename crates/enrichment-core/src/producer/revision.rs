//! Immutable repository acquisition identity; no subprocess or network access.
use crate::{identity::Ecosystem, request::ResolveRequest};
mod inputs;

crate::native_struct! {
/// Declared input reachability is distinct from a proof of complete generated/build inputs.
pub struct RevisionExtraction {
    archive_sha256: String => crate::native_union::Rule::Sha256,
    policy: String => crate::native_union::Rule::NonEmpty,
    omissions: Vec<crate::archive::ArchiveOmission> => crate::native_union::Rule::Set,
    selected_package: String => crate::native_union::Rule::Text,
    declared_inputs: Vec<String> => crate::native_union::Rule::Set,
    /// Candidate source directories from the selected package and declared local dependencies.
    source_roots: Vec<String> => crate::native_union::Rule::Set,
    missing_inputs: Vec<String> => crate::native_union::Rule::Set,
    affected_omissions: Vec<String> => crate::native_union::Rule::Set,
    source_closure: SourceClosure => crate::native_union::Rule::Text,
}
}

crate::native_vocabulary! { pub enum SourceClosure { Unknown = "unknown", Incomplete = "incomplete" } }

crate::native_struct! {
/// Filesystem observations collected without deciding the selected package's coverage.
/// The store lowers these facts into its native requested-domain assessment before publication.
pub struct RevisionInputs {
    archive_sha256: String => crate::native_union::Rule::Sha256,
    policy: String => crate::native_union::Rule::NonEmpty,
    omissions: Vec<crate::archive::ArchiveOmission> => crate::native_union::Rule::Set,
    selected_package: String => crate::native_union::Rule::Text,
    declared_inputs: Vec<String> => crate::native_union::Rule::Set,
    /// Candidate source directories from the selected package and declared local dependencies.
    source_roots: Vec<String> => crate::native_union::Rule::Set,
    missing_inputs: Vec<String> => crate::native_union::Rule::Set,
}
}

impl RevisionInputs {
    /// Account for the selected package and ancestor Cargo configuration without following
    /// links. Static archive presence never establishes generated, submodule or LFS closure.
    pub fn collect(
        archive_sha256: String,
        extracted: &crate::archive::Extracted,
        package_subdir: &str,
        ecosystem: Ecosystem,
        manifest: &str,
    ) -> Result<Self, String> {
        inputs::collect(
            archive_sha256,
            extracted,
            package_subdir,
            ecosystem,
            manifest,
        )
    }
}

crate::native_struct! {
/// The validated initial Git provider and package selection.
pub struct Revision {
    /// Canonical HTTPS repository URL.
    repository: String => crate::native_union::Rule::NonEmpty,
    /// Validated owner/repository path for API construction.
    repository_path: String => crate::native_union::Rule::NonEmpty,
    /// Full lowercase commit identity.
    commit: String => crate::native_union::Rule::NonEmpty,
    /// Empty means the repository root.
    package_subdir: String => crate::native_union::Rule::Text,
}
}
impl Revision {
    /// Validate identity without accepting mutable refs or path selectors.
    ///
    /// # Errors
    /// Missing, unsupported or ambiguous identities are rejected.
    pub fn from_request(request: &ResolveRequest) -> Result<Self, String> {
        let repository = request
            .repository
            .as_deref()
            .ok_or("revision requires repository")?;
        let path = repository.strip_prefix("https://github.com/").ok_or(
            "Only canonical public https://github.com/owner/repository URLs are supported",
        )?;
        let parts: Vec<_> = path.split('/').collect();
        if parts.len() != 2
            || parts.iter().any(|p| {
                p.is_empty()
                    || *p == "."
                    || *p == ".."
                    || !p
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
            })
            || path.ends_with(".git")
        {
            return Err("Repository requires exactly owner/name, without URL selectors, credentials, .git suffix or encoded paths".into());
        }
        let commit = request
            .revision
            .as_deref()
            .ok_or("revision requires a full commit SHA")?;
        if commit.len() != 40 || !commit.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err("revision must be a full 40-hex commit SHA, never a branch or tag".into());
        }
        let subdir = request.package_subdir.as_deref().unwrap_or("");
        if !subdir.is_empty()
            && subdir.split('/').any(|p| {
                p.is_empty()
                    || p == "."
                    || p == ".."
                    || !p
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
            })
        {
            return Err(
                "package_subdir must be an explicit relative path of ordinary components".into(),
            );
        }
        Ok(Self {
            repository: repository.to_ascii_lowercase(),
            repository_path: path.to_ascii_lowercase(),
            commit: commit.to_ascii_lowercase(),
            package_subdir: subdir.into(),
        })
    }
    /// Registry identity includes the package root so monorepo packages cannot alias.
    #[must_use]
    pub fn registry(&self) -> String {
        format!("git:{}#path={}", self.repository, self.package_subdir)
    }

    /// A usable immutable citation, separate from the package's registry identity.
    /// Encode source path segments so spaces, Unicode and fragment characters remain data.
    pub fn source_uri(&self, path: &str) -> Result<String, String> {
        if path
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
            || path.contains('\\')
            || path.chars().any(char::is_control)
        {
            return Err("revision source requires a safe relative file path".into());
        }
        let mut uri = url::Url::parse(&self.repository).map_err(|error| error.to_string())?;
        uri.path_segments_mut()
            .map_err(|()| "revision repository cannot contain paths")?
            .push("blob")
            .push(&self.commit)
            .extend(
                self.package_subdir
                    .split('/')
                    .filter(|part| !part.is_empty()),
            )
            .extend(path.split('/'));
        Ok(uri.into())
    }
}

/// Whether `field` is inherited from the workspace root rather than stated in this manifest.
///
/// Cargo spells this `field.workspace = true`, and it is what most multi-crate repositories
/// write. Revision mode reads the repository's own manifest, not the normalized one
/// `cargo package` publishes, so this is the ordinary case rather than an exotic one.
fn inherited(project: &toml::Value, field: &str) -> bool {
    project
        .get(field)
        .and_then(|v| v.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false)
}

/// Validate a static Cargo/PEP 621 project name without executing a build backend.
///
/// # Errors
/// Dynamic or missing project identity and a mismatched package are rejected.
pub fn project_identity(
    text: &str,
    ecosystem: Ecosystem,
    name: &str,
) -> Result<Option<String>, String> {
    let manifest: toml::Value = toml::from_str(text).map_err(|e| e.to_string())?;
    let project = manifest
        .get(match ecosystem {
            Ecosystem::Rust => "package",
            Ecosystem::Python => "project",
        })
        .ok_or(
            "Selected root lacks a static package/project table; supply the exact package_subdir",
        )?;
    // `name.workspace = true` is a different failure from a missing name, and pointing a caller
    // at `package_subdir` for it sends them looking for a package root that was already correct.
    // The name is stated in the workspace root, which is not this manifest, and this producer
    // reads one manifest.
    if inherited(project, "name") {
        return Err(format!(
            "Selected root inherits its name from the workspace root (`name.workspace = true`), \
             so this manifest does not state which package it is. Request {name} at a package \
             root that declares its own name."
        ));
    }
    let declared = project.get("name").and_then(toml::Value::as_str).ok_or(
        "Selected root lacks a static package/project name; supply the exact package_subdir",
    )?;
    let normalized = |s: &str| match ecosystem {
        Ecosystem::Rust => s.to_owned(),
        Ecosystem::Python => super::python::normalize_name(s),
    };
    if normalized(declared) != normalized(name) {
        return Err(format!(
            "Selected root declares {declared}, not requested {name}"
        ));
    }
    // An inherited version reports absent, exactly like a manifest that states none. That is
    // true rather than convenient: this manifest does not state a version, and the revision path
    // already tells the caller that a manifest version establishes no published release
    // association. What must never happen is guessing one.
    Ok(project
        .get("version")
        .and_then(toml::Value::as_str)
        .map(str::to_owned))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn revision_identity_rejects_mutable_and_unsafe_selectors() {
        let request = ResolveRequest {
            repository: Some("https://github.com/org/repo".into()),
            revision: Some("0123456789abcdef0123456789abcdef01234567".into()),
            ..Default::default()
        };
        let revision = Revision::from_request(&request).expect("identity");
        for repository in [
            "http://github.com/org/repo",
            "https://github.com/org/repo?ref=main",
            "https://github.com/org/../repo",
            "https://github.com/org/%2frepo",
            "https://user@github.com/org/repo",
            "https://github.com/org/repo/tree/main",
        ] {
            assert!(
                Revision::from_request(&ResolveRequest {
                    repository: Some(repository.into()),
                    ..request.clone()
                })
                .is_err()
            );
        }
        for commit in [
            "main",
            "v1.0",
            "abcdef",
            "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
        ] {
            assert!(
                Revision::from_request(&ResolveRequest {
                    revision: Some(commit.into()),
                    ..request.clone()
                })
                .is_err()
            );
        }
        assert!(
            Revision::from_request(&ResolveRequest {
                package_subdir: Some("../x".into()),
                ..request.clone()
            })
            .is_err()
        );
        let other = Revision::from_request(&ResolveRequest {
            package_subdir: Some("lib/x".into()),
            ..request
        })
        .expect("subdir");
        assert_ne!(revision.registry(), other.registry());
        assert!(project_identity("[workspace]\nmembers=[]", Ecosystem::Rust, "x").is_err());
        assert_eq!(
            project_identity(
                "[project]\nname='X.Y'\nversion='2'",
                Ecosystem::Python,
                "x-y"
            )
            .expect("identity"),
            Some("2".into())
        );
    }

    #[test]
    fn a_workspace_inherited_field_is_reported_as_what_it_is() {
        // Revision mode reads the repository's own manifest, not the normalized one
        // `cargo package` publishes, so `field.workspace = true` is the ordinary case in a
        // multi-crate repository rather than an exotic one.

        // An inherited *version* is absent. Guessing the workspace root's version would
        // attribute it to a crate that did not claim it.
        assert_eq!(
            project_identity(
                "[package]\nname = \"demo\"\nversion.workspace = true\n",
                Ecosystem::Rust,
                "demo",
            )
            .expect("an inherited version is not a failure"),
            None,
        );

        // An inherited *name* is still an error -- this manifest cannot say which package it is
        // -- but the message must not send the caller looking for a package root that was
        // already correct. That misdirection is the defect this guards.
        let error = project_identity(
            "[package]\nname.workspace = true\nversion = \"1.0.0\"\n",
            Ecosystem::Rust,
            "demo",
        )
        .expect_err("an inherited name cannot be checked against the request");
        assert!(error.contains("workspace"), "{error}");
        assert!(
            !error.contains("package_subdir"),
            "the package root is not the problem: {error}"
        );
    }

    #[test]
    fn source_citations_keep_commit_package_and_encoded_file_path() {
        let revision = Revision {
            repository: "https://github.com/org/repo".into(),
            repository_path: "org/repo".into(),
            commit: "a".repeat(40),
            package_subdir: "crates/demo".into(),
        };
        let citation = revision.source_uri("docs/é #notes.md").unwrap();
        assert_eq!(
            citation,
            format!(
                "https://github.com/org/repo/blob/{}/crates/demo/docs/%C3%A9%20%23notes.md",
                "a".repeat(40)
            )
        );
        let parsed = url::Url::parse(&citation).unwrap();
        assert!(parsed.fragment().is_none() && parsed.query().is_none());
        for path in ["", "/outside", "../outside", "a/../outside", "a\\b"] {
            assert!(revision.source_uri(path).is_err());
        }
    }
}
