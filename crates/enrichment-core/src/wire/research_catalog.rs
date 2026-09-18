//! Finite research policy. Vocabulary, defaults and eligibility share this declaration.
use crate::{
    evidence::{EvidenceKind, FragmentKind},
    native_union::Rule,
};

crate::native_vocabulary! {
    pub enum RequiredObservation { None = "none", Source = "source", Configuration = "configuration", Execution = "execution" }
}

crate::native_struct! {
    pub struct AspectDefinition {
        aspect: InspectionAspect => Rule::Text,
        ordinal: u32 => Rule::Text,
        default_max_items: Option<usize> => Rule::Text,
        default_max_characters: Option<usize> => Rule::Text,
        preview: bool => Rule::Text,
        rust_evidence: EvidenceKind => Rule::Text,
        python_evidence: EvidenceKind => Rule::Text,
        observation: RequiredObservation => Rule::Text,
        fragment_kinds: Vec<FragmentKind> => Rule::SequenceBounds { min: 0, max: 2 },
    }
}
macro_rules! inspection_aspects {
    ($($variant:ident = $name:literal, $ordinal:literal, $items:expr, $characters:expr, $preview:literal, $rust:ident, $python:ident, $observation:ident, [$($fragment:ident),*];)*) => {
        crate::native_vocabulary! {
            /// Independently selectable inspection aspects.
            #[derive(PartialOrd, Ord)]
            pub enum InspectionAspect { $($variant = $name),* }
        }
        impl InspectionAspect {
            pub fn definitions() -> Vec<AspectDefinition> {
                vec![$(AspectDefinition {
                    aspect: Self::$variant, ordinal: $ordinal,
                    default_max_items: $items, default_max_characters: $characters, preview: $preview,
                    rust_evidence: EvidenceKind::$rust, python_evidence: EvidenceKind::$python, observation: RequiredObservation::$observation,
                    fragment_kinds: vec![$(FragmentKind::$fragment),*],
                }),*]
            }
        }
    };
}
inspection_aspects! {
    Signature = "signature", 0, Some(32), None, false, PublicApi, PublicApi, None, [];
    Availability = "availability", 1, Some(32), None, false, PublicApi, PublicApi, Configuration, [];
    Relationships = "relationships", 2, None, None, false, PublicApi, PublicApi, None, [];
    Documentation = "documentation", 3, Some(3), Some(1200), true, Documentation, Documentation, None, [DocText, ReadmeSection];
    Examples = "examples", 4, None, None, true, Examples, Examples, None, [Example];
    Source = "source", 5, None, None, false, CrateSource, DistributionSource, Source, [];
    Semantics = "semantics", 6, None, None, false, SemanticQueries, SemanticQueries, Execution, [];
    Runtime = "runtime", 7, None, None, false, RuntimeApi, RuntimeApi, Execution, [];
    Children = "children", 8, None, None, false, PublicApi, PublicApi, None, [];
    Members = "members", 9, None, None, false, PublicApi, PublicApi, None, [];
}

crate::native_struct! {
    pub struct DiscoveryDefinition {
        kind: DiscoveryKind => Rule::Text,
        ordinal: u32 => Rule::Text,
        fragment_kind: FragmentKind => Rule::Text,
        evidence_kind: EvidenceKind => Rule::Text,
        default_max_items: usize => Rule::Text,
        default_max_characters: Option<usize> => Rule::Text,
    }
}
macro_rules! discovery_kinds {
    ($($variant:ident = $name:literal, $ordinal:literal, $fragment:ident, $evidence:ident, $items:literal, $characters:expr;)*) => {
        crate::native_vocabulary! {
            #[derive(PartialOrd, Ord)]
            pub enum DiscoveryKind { $($variant = $name),* }
        }
        impl DiscoveryKind {
            pub const fn fragment_kind(self) -> FragmentKind {
                match self { $(Self::$variant => FragmentKind::$fragment),* }
            }
            pub const fn evidence_kind(self) -> EvidenceKind {
                match self { $(Self::$variant => EvidenceKind::$evidence),* }
            }
            pub fn definitions() -> Vec<DiscoveryDefinition> {
                vec![$(DiscoveryDefinition {
                    kind: Self::$variant, ordinal: $ordinal,
                    fragment_kind: FragmentKind::$fragment, evidence_kind: EvidenceKind::$evidence,
                    default_max_items: $items, default_max_characters: $characters,
                }),*]
            }
        }
    };
}
discovery_kinds! {
    Features = "features", 0, FeatureDefinition, RegistryMetadata, 8, Some(256);
    Documentation = "documentation", 1, ReadmeSection, Documentation, 8, Some(256);
    ReleaseNotes = "release_notes", 2, ChangelogSection, ReleaseNotes, 8, Some(256);
    Examples = "examples", 3, Example, Examples, 8, Some(256);
}
