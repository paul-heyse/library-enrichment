# `ruff_python_ast::script`

Crate `ruff_python_ast` · 2 public items · structured records in [`model/ruff_python_ast.script.json`](../model/ruff_python_ast.script.json)

## ScriptSourceMap

`struct` · `ruff_python_ast::script::ScriptSourceMap`

```rust
struct ScriptSourceMap
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn map_range(&self, range: TextRange) -> TextRange
```

Maps offsets in extracted script metadata to offsets in the original Python source.

---

## ScriptTag

`struct` · `ruff_python_ast::script::ScriptTag`

```rust
struct ScriptTag
```

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn metadata(&self) -> &str
fn parse(contents: &[u8]) -> Option<Self>
fn source_map(&self) -> &ScriptSourceMap
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

PEP 723 metadata as parsed from a `script` comment block.

See: <https://peps.python.org/pep-0723/>

Vendored from: <https://github.com/astral-sh/uv/blob/debe67ffdb0cd7835734100e909b2d8f79613743/crates/uv-scripts/src/lib.rs#L283>

---
