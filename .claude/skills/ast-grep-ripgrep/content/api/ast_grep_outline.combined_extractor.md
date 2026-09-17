# `ast_grep_outline::combined_extractor`

Crate `ast-grep-outline` · 1 public items · structured records in [`model/ast_grep_outline.combined_extractor.json`](../model/ast_grep_outline.combined_extractor.json)

## CombinedExtractors

`struct` · `ast_grep_outline::combined_extractor::CombinedExtractors`

```rust
struct CombinedExtractors<L: Language>
```

**Methods** (3)

```rust
fn extract<'a, 'tree>(&'a self, root: Node<'tree, StrDoc<L>>) -> impl Iterator<Item = OutlineItem<'tree>> + use<{'lifetime': "'a"}, {'lifetime': "'tree"}, {'param': 'L'}> where L: LanguageExt
fn try_from(extractors: Vec<SerializableOutlineRule<L>>, globals: &GlobalRules) -> Result<Self, OutlineRuleError>
fn try_from_rules(extractors: Vec<SerializableOutlineRule<L>>, options: OutlineExtractorOptions, globals: &GlobalRules) -> Result<Self, OutlineRuleError>
```

Runtime outline extractors organized for a shared item traversal.

---
