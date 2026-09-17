# `ast_grep_core::replacer::template`

Crate `ast-grep-core` · 3 public items · structured records in [`model/ast_grep_core.replacer.template.json`](../model/ast_grep_core.replacer.template.json)

## TemplateFix

`enum` · `ast_grep_core::replacer::template::TemplateFix`

Also reachable as `ast_grep_core::replacer::TemplateFix`

```rust
enum TemplateFix
```

**Variants**: `Textual`, `WithMetaVar`

**Implements**: `ast_grep_core::replacer::Replacer`

**Methods** (4)

```rust
fn exact_node_var(&self) -> Option<&str>
fn try_new<L: Language>(template: &str, lang: &L) -> Result<Self, TemplateFixError>
fn used_vars(&self) -> HashSet<&str>
fn with_transform<L: Language>(tpl: &str, lang: &L, trans: &[String]) -> Self
```

**via `ast_grep_core::replacer::Replacer`**

```rust
fn generate_replacement(&self, nm: &NodeMatch<'_, D>) -> Underlying<D>
```

---

## TemplateFixError

`enum` · `ast_grep_core::replacer::template::TemplateFixError`

Also reachable as `ast_grep_core::replacer::TemplateFixError`

```rust
enum TemplateFixError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## Template

`struct` · `ast_grep_core::replacer::template::Template`

```rust
struct Template
```

---
