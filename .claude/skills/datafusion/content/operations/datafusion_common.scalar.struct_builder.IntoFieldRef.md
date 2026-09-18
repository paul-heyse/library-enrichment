# `datafusion_common::scalar::struct_builder::IntoFieldRef`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.struct_builder.IntoFieldRef.json).

<a id="op-882f4d1d1ad5b882511ed825"></a>
## IntoFieldRef

`trait` · `datafusion_common::scalar::struct_builder::IntoFieldRef` · datafusion-common 55.1.0

```rust
trait IntoFieldRef
```

Source: `src/scalar/struct_builder.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Trait for converting a type into a [`FieldRef`]

Used to avoid having to call `clone()` on a `FieldRef` when adding a field to
a `ScalarStructBuilder`.

TODO potentially upstream this to arrow-rs so that we can
use impl `Into<FieldRef>` instead

<a id="op-2f1f08ededdc7bba79029dbb"></a>
## into_field_ref

`function` · `datafusion_common::scalar::struct_builder::IntoFieldRef::into_field_ref` · datafusion-common 55.1.0

```rust
fn into_field_ref(self) -> FieldRef
```

Source: `src/scalar/struct_builder.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
