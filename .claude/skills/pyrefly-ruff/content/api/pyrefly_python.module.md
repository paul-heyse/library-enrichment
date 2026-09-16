# `pyrefly_python::module`

Crate `pyrefly_python` · 3 public items · structured records in [`model/pyrefly_python.module.json`](../model/pyrefly_python.module.json)

## GENERATED_TOKEN

`static` · `pyrefly_python::module::GENERATED_TOKEN`

```rust
static GENERATED_TOKEN: &str
```

---

## Module

`struct` · `pyrefly_python::module::Module`

```rust
struct Module
```

**Implements**: `dupe::Dupe`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (23)

```rust
fn allows_top_level_await(&self) -> bool
fn code_at(&self, range: TextRange) -> &str
fn contents(&self) -> &Arc<String>
fn display<'a>(&'a self, x: &'a impl DisplayWith<Module>) -> impl Display + 'a
fn display_pos(&self, offset: TextSize) -> DisplayPos
fn display_range(&self, range: TextRange) -> DisplayRange
fn from_lsp_position(&self, position: lsp_types::Position, notebook_cell: Option<usize>) -> TextSize
fn from_lsp_range(&self, position: lsp_types::Range, notebook_cell: Option<usize>) -> TextRange
fn ignore(&self) -> &Ignore
fn is_generated(&self) -> bool
fn is_notebook(&self) -> bool
fn line_count(&self) -> usize
fn lined_buffer(&self) -> &LinedBuffer
fn name(&self) -> ModuleName
fn new(name: ModuleName, path: ModulePath, contents: Arc<String>) -> Self
fn new_notebook(name: ModuleName, path: ModulePath, notebook: Arc<Notebook>) -> Self
fn notebook(&self) -> Option<&Notebook>
fn path(&self) -> &ModulePath
fn source_type(&self) -> PySourceType
fn suppression_effect(&self, source_range: &DisplayRange, error_kind: &str, enabled_ignores: &SmallSet<Tool>, type_ignore_unknown_tag_behavior: TypeIgnoreUnknownTagBehavior) -> SuppressionEffect
fn to_cell_for_lsp(&self, x: TextSize) -> Option<usize>
fn to_lsp_position(&self, x: TextSize) -> lsp_types::Position
fn to_lsp_range(&self, x: TextRange) -> lsp_types::Range
```

Information about a module, notably its name, path, and contents.

---

## TextRangeWithModule

`struct` · `pyrefly_python::module::TextRangeWithModule`

```rust
struct TextRangeWithModule
```

**Fields**: `module`, `range`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(module: Module, range: TextRange) -> Self
```

---
