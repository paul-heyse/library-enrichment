//! Bounded search command identity; lexical transformations belong to native DataFusion.
crate::native_struct! {
#[derive(Hash)]
pub struct SearchSpec {
    version: String => crate::native_union::Rule::NonEmpty,
    query: String => crate::native_union::Rule::NonEmpty,
}
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
