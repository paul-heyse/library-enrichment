# `ast_grep_core::replacer`

Crate `ast-grep-core` · 1 public items · structured records in [`model/ast_grep_core.replacer.json`](../model/ast_grep_core.replacer.json)

## Replacer

`trait` · `ast_grep_core::replacer::Replacer`

```rust
trait Replacer<D: Doc>
```

**Implementors** (3)

- `ast_grep_config::fixer::Fixer`
- `ast_grep_core::node::Node`
- `ast_grep_core::replacer::template::TemplateFix`

**Methods** (2)

```rust
fn generate_replacement(&self, nm: &NodeMatch<'_, D>) -> Underlying<D>
fn get_replaced_range(&self, nm: &NodeMatch<'_, D>, matcher: impl Matcher) -> Range<usize>
```

Replace meta variable in the replacer string

---
