//! Deterministic differences between immutable observations, not a compatibility proof.

pub mod page;

pub const MAX_FIELD_PATH_STEPS: usize = 64;
pub const MAX_COMPARISON_FIELDS: usize = 65_536;

crate::native_struct! {
    pub struct ScopeDefinition {
        scope: Scope => crate::native_union::Rule::Text,
        evidence_kind: crate::evidence::EvidenceKind => crate::native_union::Rule::Text,
    }
}
macro_rules! comparison_scopes {
    ($($variant:ident = $name:literal, $kind:ident;)*) => {
        crate::native_vocabulary! {
            /// Independent axes of a release comparison, with one native evidence policy.
            #[derive(PartialOrd, Ord)]
            pub enum Scope { $($variant = $name),* }
        }
        impl Scope {
            pub fn definitions() -> Vec<ScopeDefinition> {
                vec![$(ScopeDefinition { scope: Self::$variant, evidence_kind: crate::evidence::EvidenceKind::$kind }),*]
            }
        }
    };
}
comparison_scopes! {
    Api = "api", PublicApi;
    Docs = "docs", Documentation;
    Configuration = "configuration", RegistryMetadata;
    ReleaseNotes = "release_notes", ReleaseNotes;
    Examples = "examples", Examples;
    Relationships = "relationships", PublicApi;
}

crate::native_vocabulary! {
/// A set or field difference, not a compatibility verdict.
pub enum ChangeKind {
    Added = "added",
    Removed = "removed",
    Changed = "changed",
}
}

crate::native_struct! {
/// Each value retains its own qualified source, including a legitimate absent observation.
pub struct Alternative {
    value: AlternativeValue => crate::native_union::Rule::Text,
    source: Option<crate::evidence::relational::FactSource> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
    /// Documentation fields in this payload are null in the API axis; documentation has its
    /// own comparison scope. Callable/parameter/qualifier declarations retain their native types.
    pub struct ApiComparisonObservation {
        origin: crate::evidence::relational::ApiOrigin => crate::native_union::Rule::Text,
        payload: crate::evidence::relational::ApiPayload => crate::native_union::Rule::Text,
    }
}
crate::native_union! { @tag "axis";
    /// Closed native values shared by set equality, before/after delivery and value artifacts.
    pub enum ComparisonValue {
        Api = "api" {
            kind: crate::evidence::SymbolKind => crate::native_union::Rule::Text,
            qualifier: Option<String> => crate::native_union::Rule::Text,
            definition_path: String => crate::native_union::Rule::Text,
            defined_in_package: String => crate::native_union::Rule::Text,
            is_reexport: bool => crate::native_union::Rule::Text,
            observation: Option<Box<ApiComparisonObservation>> => crate::native_union::Rule::Text,
        },
        Fragment = "fragment" {
            kind: crate::evidence::FragmentKind => crate::native_union::Rule::Text,
            text: String => crate::native_union::Rule::Text,
            evidence_class: crate::wire::EvidenceClass => crate::native_union::Rule::Text,
        },
        RustDocumentation = "rust_documentation" {
            configuration: crate::producer::docsrs::DocsRsMetadata => crate::native_union::Rule::Text,
        },
        PythonHeader = "python_header" {
            values: Vec<String> => crate::native_union::Rule::Sequence,
        },
        Relationship = "relationship" {
            relation: crate::evidence::RelationKind => crate::native_union::Rule::Text,
            qualifier: Option<String> => crate::native_union::Rule::Text,
            target_kind: String => crate::native_union::Rule::Text,
            target_symbol_id: Option<String> => crate::native_union::Rule::Text,
            target_definition_id: Option<String> => crate::native_union::Rule::Text,
            target_package: Option<String> => crate::native_union::Rule::Text,
            target_path: Option<String> => crate::native_union::Rule::Text,
        },
    }
}

crate::native_union! { @tag "mode";
/// A final encoded comparison cell or its exact immutable artifact.
pub enum AlternativeValue {
    Inline = "inline" { value: ComparisonValue => crate::native_union::Rule::Text },
    Artifact = "artifact" {
        artifact: crate::wire::ArtifactHandle => crate::native_union::Rule::Text,
        size_bytes: u64 => crate::native_union::Rule::Text,
        sha256: String => crate::native_union::Rule::NonEmpty,
    },
}
}

crate::native_struct! {
    /// A schema field or an explicitly positioned sequence item, never a parsed dotted path.
    pub struct ComparisonFieldPath {
        steps: Vec<ComparisonPathStep> => crate::native_union::Rule::SequenceBounds { min: 0, max: MAX_FIELD_PATH_STEPS as u64 },
    }
}
crate::native_union! {
    pub enum ComparisonPathStep {
        Field = "field" { name: String => crate::native_union::Rule::NonEmpty },
        Item = "item" { ordinal: u32 => crate::native_union::Rule::Coordinate(crate::native_union::Unit::Ordinal) },
    }
}

crate::native_struct! {
/// A changed key with independently paged observational alternatives.
pub struct Change {
    change_id: String => crate::native_union::Rule::Text,
    scope: Scope => crate::native_union::Rule::Text,
    kind: ChangeKind => crate::native_union::Rule::Text,
    subject: String => crate::native_union::Rule::Text,
    before: Option<Vec<Alternative>> => crate::native_union::Rule::Sequence,
    after: Option<Vec<Alternative>> => crate::native_union::Rule::Sequence,
    /// Native value-set differences at schema fields and declared sequence ordinals. Parent
    /// paths also capture changes in alternative correlation and whole-sequence ordering.
    fields: Vec<ComparisonFieldPath> => crate::native_union::Rule::SequenceBounds { min: 0, max: MAX_COMPARISON_FIELDS as u64 },
    interpretation: String => crate::native_union::Rule::Text,
    /// Values and their source references share this bounded alternative order.
    before_page: crate::wire::Page => crate::native_union::Rule::Text,
    after_page: crate::wire::Page => crate::native_union::Rule::Text,
}
}
