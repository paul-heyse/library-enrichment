# `datafusion_expr::extension_types::array_formatter_factory`

Crate `datafusion-expr` · 1 public items · structured records in [`model/datafusion_expr.extension_types.array_formatter_factory.json`](../model/datafusion_expr.extension_types.array_formatter_factory.json)

## DFArrayFormatterFactory

`struct` · `datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory`

```rust
struct DFArrayFormatterFactory
```

**Implements**: `arrow_cast::display::ArrayFormatterFactory`

**Derives**: Debug

**Methods** (1)

```rust
fn new(registry: ExtensionTypeRegistryRef) -> Self
```

**via `arrow_cast::display::ArrayFormatterFactory`**

```rust
fn create_array_formatter<'formatter>(&self, array: &'formatter dyn Array, options: &FormatOptions<'formatter>, field: Option<&'formatter Field>) -> Result<Option<ArrayFormatter<'formatter>>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.extension_types.array_formatter_factory.DFArrayFormatterFactory.md).


A factory for creating [`ArrayFormatter`]s that checks whether a registered extension type can
format a given array based on its metadata.

---
