//! Cursor selection identities are native records shared by handlers and query plans.
use crate::{
    compare::Scope,
    evidence::path::PublicPath,
    native_key::Key,
    native_union::Rule,
    request::InspectionOptions,
    search::spec::SearchSpec,
    wire::research::{DiscoveryKind, InspectionAspect},
};

crate::native_struct! {
    pub struct SearchSelection {
        spec: SearchSpec => Rule::Text,
        kinds: Vec<String> => Rule::Set,
        area: Option<PublicPath> => Rule::Text,
        max_items: Option<usize> => Rule::Text,
        max_bytes: usize => Rule::Text,
    }
}
crate::native_struct! {
    pub struct InspectionSelection {
        symbol_id: String => Rule::Reference(crate::native_union::Domain::Symbol),
        aspect: InspectionAspect => Rule::Text,
        max_items: usize => Rule::Text,
        max_characters: Option<usize> => Rule::Text,
        max_bytes: usize => Rule::Text,
        execution: Option<InspectionOptions> => Rule::Text,
    }
}
crate::native_struct! {
    pub struct DiscoverySelection {
        kind: DiscoveryKind => Rule::Text,
        max_items: usize => Rule::Text,
        max_characters: Option<usize> => Rule::Text,
        max_bytes: usize => Rule::Text,
    }
}
crate::native_struct! {
    pub struct ComparisonSelection {
        scopes: Vec<Scope> => Rule::Set,
        max_items: Option<usize> => Rule::Text,
        max_bytes: usize => Rule::Text,
    }
}
macro_rules! selection_key {
    ($($record:ident),* $(,)?) => {$(
        impl $record {
            pub fn identity(&self) -> String {
                Key::$record.record(self).expect("declared native selection identity")
            }
        }
    )*};
}
selection_key!(
    SearchSelection,
    InspectionSelection,
    DiscoverySelection,
    ComparisonSelection
);

crate::native_struct! {
    pub struct ArtifactSelection {
        section: Option<String> => Rule::Text,
        start: usize => Rule::Text,
        end: usize => Rule::Text,
        max_bytes: usize => Rule::Text,
    }
}
selection_key!(ArtifactSelection);

crate::native_struct! {
    pub struct ArtifactWindow {
        start: usize => Rule::Text,
        end: usize => Rule::Text,
        section: Option<String> => Rule::Text,
    }
}
crate::native_vocabulary! {
    pub enum ArtifactReadAction {
        Bytes = "bytes", Markdown = "markdown", Missing = "missing",
        Unsupported = "unsupported", Corrupt = "corrupt",
    }
}
crate::native_struct! {
    pub struct ArtifactReadPlan {
        action: ArtifactReadAction => Rule::Text,
        window: ArtifactWindow => Rule::Text,
        heading: Option<String> => Rule::Text,
        is_text: bool => Rule::Text,
    }
}
crate::native_struct! {
    pub struct ArtifactSlicePlan {
        start: usize => Rule::Text,
        end: usize => Rule::Text,
        read_bytes: usize => Rule::Text,
        invalid_offset: bool => Rule::Text,
    }
}
