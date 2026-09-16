# `pyrefly_python::symbol_kind`

Crate `pyrefly_python` · 1 public items · structured records in [`model/pyrefly_python.symbol_kind.json`](../model/pyrefly_python.symbol_kind.json)

## SymbolKind

`enum` · `pyrefly_python::symbol_kind::SymbolKind`

```rust
enum SymbolKind
```

**Variants**: `Module`, `Attribute`, `Variable`, `Constant`, `Parameter`, `TypeParameter`, `TypeAlias`, `Function`, `Method`, `Class`

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn display_for_hover(self) -> String
fn to_lsp_completion_item_kind(self) -> CompletionItemKind
fn to_lsp_semantic_token_type_with_modifiers(self) -> (SemanticTokenType, Vec<SemanticTokenModifier>)
fn to_lsp_symbol_kind(self) -> lsp_types::SymbolKind
```

The kind of symbol of a binding.
It will be displayed in IDEs with different icons.
https://adamcoster.com/blog/vscode-workspace-symbol-provider-purpose might give you an idea of
how it will look in VSCode.

---
