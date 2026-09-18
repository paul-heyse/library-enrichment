//! Typed syntax facts from PyPI JSON. Unmodeled source fields remain in the raw artifact.
use super::DistributionFile;
use crate::native_union::Rule;
use std::collections::BTreeMap;

crate::native_struct! { @source
    pub struct Versions {
        versions: Vec<String> => Rule::Sequence,
    }
}
crate::native_struct! { @source
    pub struct ReleaseInfo {
        #[serde(default)]
        license: Option<String> => Rule::Text,
        #[serde(default)]
        home_page: Option<String> => Rule::Text,
        #[serde(default)]
        project_urls: Option<BTreeMap<String, Option<String>>> => Rule::Map,
    }
}
crate::native_struct! { @source
    pub struct ReleaseMetadata {
        #[serde(default)]
        info: Option<ReleaseInfo> => Rule::Text,
        urls: Vec<DistributionFile> => Rule::Sequence,
    }
}
