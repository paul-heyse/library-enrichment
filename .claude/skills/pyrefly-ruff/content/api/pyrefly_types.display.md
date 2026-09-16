# `pyrefly_types::display`

Crate `pyrefly_types` · 3 public items · structured records in [`model/pyrefly_types.display.json`](../model/pyrefly_types.display.json)

## LspDisplayMode

`enum` · `pyrefly_types::display::LspDisplayMode`

```rust
enum LspDisplayMode
```

**Variants**: `Standard`, `Hover`, `SignatureHelp`, `Query`, `ProvideType`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Display mode for type formatting for certain LSP requests.

---

## ClassDisplayContext

`struct` · `pyrefly_types::display::ClassDisplayContext`

```rust
struct ClassDisplayContext<'a>
```

**Methods** (2)

```rust
fn display(&'a self, cls: &'a Class) -> impl Display + 'a
fn new(classes: &[&'a Class]) -> Self
```

---

## TypeDisplayContext

`struct` · `pyrefly_types::display::TypeDisplayContext`

```rust
struct TypeDisplayContext<'a>
```

**Derives**: Debug, Default

**Methods** (14)

```rust
fn add(&mut self, t: &'a Type)
fn always_display_expanded_unions(&mut self)
fn always_display_module_name(&mut self)
fn always_display_module_name_except_builtins(&mut self)
fn display(&'a self, t: &'a Type) -> impl Display + 'a
fn display_internal(&'a self, t: &'a Type) -> impl Display + 'a
fn display_quantified(&'a self, quantified: &'a Quantified) -> impl Display + 'a
fn fmt_helper_generic(&self, t: &Type, is_toplevel: bool, output: &mut impl TypeOutput) -> fmt::Result
fn get_types_with_location<'b>(&self, t: &'b Type, is_toplevel: bool) -> OutputWithLocations<'_>
fn new(xs: &[&'a Type]) -> Self
fn render_self_type_as_self(&mut self)
fn set_lsp_display_mode(&mut self, display_mode: LspDisplayMode)
fn set_stdlib(&mut self, stdlib: &'a Stdlib)
fn strip_library_schemas(&mut self)
```

---
