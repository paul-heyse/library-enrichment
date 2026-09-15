# `datafusion_functions_window_common::field`

Crate `datafusion-functions-window-common` · 1 public items · structured records in [`model/datafusion_functions_window_common.field.json`](../model/datafusion_functions_window_common.field.json)

## WindowUDFFieldArgs

`struct` · `datafusion_functions_window_common::field::WindowUDFFieldArgs`

Also reachable as `datafusion_expr::function::WindowUDFFieldArgs`

```rust
struct WindowUDFFieldArgs<'a>
```

**Methods** (4)

```rust
fn get_input_field(&self, index: usize) -> Option<FieldRef>
fn input_fields(&self) -> &[FieldRef]
fn name(&self) -> &str
fn new(input_fields: &'a [FieldRef], display_name: &'a str) -> Self
```

Metadata for defining the result field from evaluating a
user-defined window function.

---
