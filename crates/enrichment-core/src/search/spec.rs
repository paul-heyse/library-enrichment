//! Bounded search command identity; lexical transformations belong to native DataFusion.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SearchSpec {
    pub version: String,
    pub query: String,
}
impl SearchSpec {
    pub const VERSION: &'static str = "lexical/3";
    #[must_use]
    pub fn new(query: &str) -> Self {
        Self {
            version: Self::VERSION.into(),
            query: query.into(),
        }
    }
}
