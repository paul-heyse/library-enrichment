# `datafusion_substrait::extensions`

Crate `datafusion-substrait` · 1 public items · structured records in [`model/datafusion_substrait.extensions.json`](../model/datafusion_substrait.extensions.json)

## Extensions

`struct` · `datafusion_substrait::extensions::Extensions`

```rust
struct Extensions
```

**Fields**: `functions`, `types`, `type_variations`

**Implements**: `core::convert::TryFrom`

**Derives**: Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn register_function(&mut self, function_name: &str) -> u32
fn register_type(&mut self, type_name: &str) -> u32
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &Vec<SimpleExtensionDeclaration>) -> datafusion::common::Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.extensions.Extensions.md).


Substrait uses [SimpleExtensions](https://substrait.io/extensions/#simple-extensions) to define
behavior of plans in addition to what's supported directly by the protobuf definitions.
That includes functions, but also provides support for custom types and variations for existing
types. This structs facilitates the use of these extensions in DataFusion.
TODO: DF doesn't yet use extensions for type variations <https://github.com/apache/datafusion/issues/11544>
TODO: DF doesn't yet provide valid extensionUris <https://github.com/apache/datafusion/issues/11545>

---
