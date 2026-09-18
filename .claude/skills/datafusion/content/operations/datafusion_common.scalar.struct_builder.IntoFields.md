# `datafusion_common::scalar::struct_builder::IntoFields`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.struct_builder.IntoFields.json).

<a id="op-a0fbd4f6f1a7d37281315f9a"></a>
## IntoFields

`trait` · `datafusion_common::scalar::struct_builder::IntoFields` · datafusion-common 55.1.0

```rust
trait IntoFields
```

Source: `src/scalar/struct_builder.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Trait for converting a type into a [`Fields`]

This avoids to avoid having to call clone() on an Arc'd `Fields` when adding
a field to a `ScalarStructBuilder`

TODO potentially upstream this to arrow-rs so that we can
use impl `Into<Fields>` instead

<a id="op-87dcefb5713e14719769994f"></a>
## into

`function` · `datafusion_common::scalar::struct_builder::IntoFields::into` · datafusion-common 55.1.0

```rust
fn into(self) -> Fields
```

Source: `src/scalar/struct_builder.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
