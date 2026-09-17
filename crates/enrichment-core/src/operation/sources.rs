//! Retained source observations, separate from release selection and normalized evidence.
use crate::native_union::Rule;
crate::native_union! {
    pub enum RegistryFacts {
        RustIndex = "rust_index" { entries: Vec<crate::registry::facts::Fact> => Rule::Set },
        PythonFiles = "python_files" { entries: Vec<crate::producer::python::facts::Fact> => Rule::Set },
        PythonVersions = "python_versions" { entries: Vec<String> => Rule::Set },
    }
}
crate::native_struct! {
    pub struct RevisionCapture {
        identity: crate::producer::revision::Revision => Rule::Text,
        tree: String => Rule::NonEmpty,
        declared_project_version: Option<String> => Rule::Text,
        archive: crate::evidence::Artifact => Rule::Text,
        commit: crate::evidence::Artifact => Rule::Text,
        archive_response: crate::http::Fetched => Rule::Text,
        commit_response: crate::http::Fetched => Rule::Text,
        inputs: crate::producer::revision::RevisionInputs => Rule::Text,
        decoder: String => Rule::Sha256,
    }
}
crate::native_struct! {
    pub struct RevisionReceipt {
        capture_id: String => Rule::NonEmpty,
        binding: super::DefinitionBinding => Rule::Text,
        source: RevisionCapture => Rule::Text,
        extraction: crate::producer::revision::RevisionExtraction => Rule::Text,
    }
}
crate::native_struct! {
    pub struct RegistryCapture {
        artifact: crate::evidence::Artifact => Rule::Text,
        accept: Option<String> => Rule::Text,
        decoder: String => Rule::NonEmpty,
        facts: RegistryFacts => Rule::Text,
    }
}
