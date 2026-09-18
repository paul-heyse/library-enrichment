# `datafusion_common::hash_utils::AsDynArray`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.AsDynArray.json).

<a id="op-5b5c4ff0f3b7e72e82770e77"></a>
## AsDynArray

`trait` · `datafusion_common::hash_utils::AsDynArray` · datafusion-common 55.1.0

```rust
trait AsDynArray
```

Source: `src/hash_utils.rs:1207`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Something that can be returned as a `&dyn Array`.

We want `create_hashes` to accept either `&dyn Array` or `ArrayRef`,
and this seems the best way to do so.

We tried having it accept `AsRef<dyn Array>`
but that is not implemented for and cannot be implemented for
`&dyn Array` so callers that have the latter would not be able
to call `create_hashes` directly. This shim trait makes it possible.

<a id="op-a9472149e1a8ca9e3e4afa54"></a>
## as_dyn_array

`function` · `datafusion_common::hash_utils::AsDynArray::as_dyn_array` · datafusion-common 55.1.0

```rust
fn as_dyn_array(&self) -> &dyn Array
```

Source: `src/hash_utils.rs:1208`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
