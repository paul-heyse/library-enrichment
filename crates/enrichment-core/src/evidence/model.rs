//! The normalized evidence records (blueprint §6.1): symbols, relationships, fragments,
//! availability and the snapshot manifest.
//!
//! Identities are content-derived and package-qualified (§3.1): a symbol is named by its
//! canonical public path, a definition by the path it is defined at. Rustdoc's item IDs are
//! kept in their qualified locators, never as cross-release identity.

crate::native_vocabulary! {
    /// Declaration kind, shared by native fields, admission and generated wire enums.
    #[derive(Hash, PartialOrd, Ord)]
    #[schemars(inline)]
    pub enum SymbolKind {
        Module = "module",
        Struct = "struct",
        Class = "class",
        Attribute = "attribute",
        Union = "union",
        Enum = "enum",
        Variant = "variant",
        StructField = "struct_field",
        Trait = "trait",
        TraitAlias = "trait_alias",
        TypeAlias = "type_alias",
        Function = "function",
        Method = "method",
        Constant = "constant",
        Static = "static",
        Macro = "macro",
        ProcMacro = "proc_macro",
        AssocType = "assoc_type",
        AssocConst = "assoc_const",
        Primitive = "primitive",
        ExternCrate = "extern_crate",
        ExternType = "extern_type",
        Import = "import",
    }
}
crate::native_struct! {
    /// A deprecation notice exactly as the producer recorded it.
    pub struct Deprecated {
        since: Option<String> => crate::native_union::Rule::Text,
        note: Option<String> => crate::native_union::Rule::Text,
    }
}

crate::native_struct! {
    /// Public binding identity. Observed signatures, documentation and source coordinates
    /// remain in independently qualified observations, never synthesized into this header.
    pub struct SymbolHeader {
        symbol_id: String => crate::native_union::Rule::Reference(crate::native_union::Domain::Symbol),
        definition_id: String => crate::native_union::Rule::Reference(crate::native_union::Domain::Definition),
        path: String => crate::native_union::Rule::NonEmpty,
        name: String => crate::native_union::Rule::NonEmpty,
        kind: SymbolKind => crate::native_union::Rule::Text,
        parent_path: Option<String> => crate::native_union::Rule::Text,
        is_reexport: bool => crate::native_union::Rule::Text,
        definition_path: String => crate::native_union::Rule::NonEmpty,
        defined_in_package: String => crate::native_union::Rule::NonEmpty,
        qualifier: Option<String> => crate::native_union::Rule::Text,
    }
}

crate::native_struct! {
    /// Complete native consensus over qualified API observations for ancillary projections.
    pub struct AncillaryFacts {
        cfg_hints: Option<Vec<String>> => crate::native_union::Rule::Sequence,
        locator: Option<crate::evidence::relational::Locator> => crate::native_union::Rule::Text,
        cfg_alternatives: u64 => crate::native_union::Rule::Text,
        locator_alternatives: u64 => crate::native_union::Rule::Text,
    }
}

crate::native_struct! {
    pub struct DefinitionIdentity {
        package: String => crate::native_union::Rule::NonEmpty,
        path: String => crate::native_union::Rule::NonEmpty,
        kind: SymbolKind => crate::native_union::Rule::Text,
        qualifier: Option<String> => crate::native_union::Rule::Text,
    }
}

impl SymbolHeader {
    /// Derive a definition identity.
    #[must_use]
    pub fn definition_id_for(
        crate_name: &str,
        definition_path: &str,
        kind: SymbolKind,
        qualifier: Option<&str>,
    ) -> String {
        crate::native_key::Key::Definition
            .record(&DefinitionIdentity {
                package: crate_name.into(),
                path: definition_path.into(),
                kind,
                qualifier: qualifier.map(str::to_owned),
            })
            .expect("declared native declaration identity")
    }
}

crate::native_vocabulary! {
/// Typed relations between symbols (§6.1).
///
/// The values are, in order: `reexports`, `implements`, `member_of`, `documents`, `returns`,
/// `accepts`.
#[derive(Hash)]
#[schemars(inline)]
pub enum RelationKind {
    Reexports = "reexports", Implements = "implements", Inherits = "inherits",
    MemberOf = "member_of", Documents = "documents", Returns = "returns", Accepts = "accepts",
}
}

crate::native_vocabulary! {
    /// What kind of text a fragment is.
    #[derive(Hash)]
    #[schemars(inline)]
    pub enum FragmentKind {
        ApiSignature = "api_signature", DocText = "doc_text",
        FeatureDefinition = "feature_definition", ReadmeSection = "readme_section",
        ChangelogSection = "changelog_section", Example = "example", SourceExcerpt = "source_excerpt"
    }
}

impl FragmentKind {
    /// The `search_evidence` kind family this fragment belongs to.
    #[must_use]
    pub fn family(self) -> &'static str {
        match self {
            Self::ApiSignature => "api",
            Self::DocText => "docs",
            Self::FeatureDefinition => "features",
            Self::ReadmeSection => "docs",
            Self::ChangelogSection => "release_notes",
            Self::Example => "examples",
            Self::SourceExcerpt => "source",
        }
    }
}

crate::native_struct! {
/// The configuration a documentation build was observed under (§4.3).
pub struct ObservedConfiguration {
    /// Features the docs build enabled, as the manifest declared them.
    features: Vec<String> => crate::native_union::Rule::Set,
    /// Whether all features were enabled.
    all_features: bool => crate::native_union::Rule::Text,
    /// Whether default features were disabled.
    no_default_features: bool => crate::native_union::Rule::Text,
    /// The target the JSON itself declares.
    target: String => crate::native_union::Rule::Text,
    /// The rustdoc JSON format version.
    format_version: u32 => crate::native_union::Rule::Text,
    /// Where the configuration was read from.
    source: String => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// What a caller declared about its own environment, for comparison.
#[derive(Default)]
pub struct RequestedConfiguration {
    features: Option<Vec<String>> => crate::native_union::Rule::Set,
    default_features: Option<bool> => crate::native_union::Rule::Text,
    target: Option<String> => crate::native_union::Rule::Text,
}
}

crate::native_vocabulary! {
/// Documentation presence and verified project availability are distinct claims.
#[schemars(inline)]
pub enum AvailabilityStatus {
    DocumentedAvailable = "documented_available",
    ProjectAvailabilityUnverified = "project_availability_unverified",
}
}

crate::native_struct! {
/// Native availability selection; Rust only decodes the admitted result.
pub struct Availability {
    status: AvailabilityStatus => crate::native_union::Rule::Text,
    observed_configuration: ObservedConfiguration => crate::native_union::Rule::Text,
    requested_configuration: RequestedConfiguration => crate::native_union::Rule::Text,
    notes: Vec<String> => crate::native_union::Rule::Sequence,
}
}

crate::native_struct! {
/// Counts an overview and a manifest report.
#[derive(Default)]
pub struct SnapshotCounts {
    /// Public paths.
    symbols: u64 => crate::native_union::Rule::Text,
    /// Distinct definitions.
    definitions: u64 => crate::native_union::Rule::Text,
    /// Paths that re-export a definition reachable elsewhere.
    reexports: u64 => crate::native_union::Rule::Text,
    /// Re-exports whose target is outside this crate.
    unresolved_reexports: u64 => crate::native_union::Rule::Text,
    /// Typed edges.
    relationships: u64 => crate::native_union::Rule::Text,
    /// Fragments.
    fragments: u64 => crate::native_union::Rule::Text,
    /// Items the producer saw in total, including those not surfaced as symbols.
    producer_items: u64 => crate::native_union::Rule::Text,
}
}
