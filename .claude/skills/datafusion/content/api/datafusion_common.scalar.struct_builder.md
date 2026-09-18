# `datafusion_common::scalar::struct_builder`

Crate `datafusion-common` · 3 public items · structured records in [`model/datafusion_common.scalar.struct_builder.json`](../model/datafusion_common.scalar.struct_builder.json)

## ScalarStructBuilder

`struct` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder`

Also reachable as `datafusion_common::scalar::ScalarStructBuilder`

```rust
struct ScalarStructBuilder
```

**Derives**: Debug, Default

**Methods** (6)

```rust
fn build(self) -> Result<ScalarValue>
fn new() -> Self
fn new_null(fields: impl IntoFields) -> ScalarValue
fn with_array(self, field: impl IntoFieldRef, value: ArrayRef) -> Self
fn with_name_and_scalar(self, name: &str, value: ScalarValue) -> Self
fn with_scalar(self, field: impl IntoFieldRef, value: ScalarValue) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.scalar.struct_builder.ScalarStructBuilder.md).


Builder for [`ScalarValue::Struct`].

See examples on [`ScalarValue`]

---

## IntoFieldRef

`trait` · `datafusion_common::scalar::struct_builder::IntoFieldRef`

```rust
trait IntoFieldRef
```

**Methods** (1)

```rust
fn into_field_ref(self) -> FieldRef
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.scalar.struct_builder.IntoFieldRef.md).


Trait for converting a type into a [`FieldRef`]

Used to avoid having to call `clone()` on a `FieldRef` when adding a field to
a `ScalarStructBuilder`.

TODO potentially upstream this to arrow-rs so that we can
use impl `Into<FieldRef>` instead

---

## IntoFields

`trait` · `datafusion_common::scalar::struct_builder::IntoFields`

```rust
trait IntoFields
```

**Methods** (1)

```rust
fn into(self) -> Fields
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.scalar.struct_builder.IntoFields.md).


Trait for converting a type into a [`Fields`]

This avoids to avoid having to call clone() on an Arc'd `Fields` when adding
a field to a `ScalarStructBuilder`

TODO potentially upstream this to arrow-rs so that we can
use impl `Into<Fields>` instead

---
