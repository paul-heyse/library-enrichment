# `arrow_array::types::bytes::ByteArrayNativeType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.bytes.ByteArrayNativeType.json).

<a id="op-1025dca4ebb64a9a4b3b77a8"></a>
## ByteArrayNativeType

`trait` · `arrow_array::types::bytes::ByteArrayNativeType` · arrow-array 59.3.0

```rust
trait ByteArrayNativeType: std::fmt::Debug + Send + Sync
```

Source: `src/types.rs:1599`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb140371302ca37b33a4fa75"></a>
## from_bytes_checked

`function` · `arrow_array::types::bytes::ByteArrayNativeType::from_bytes_checked` · arrow-array 59.3.0

```rust
fn from_bytes_checked(b: &[u8]) -> Option<&Self>
```

Source: `src/types.rs:1600`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-876295e9a1312c87ea3d07df"></a>
## from_bytes_unchecked

`function` · `arrow_array::types::bytes::ByteArrayNativeType::from_bytes_unchecked` · arrow-array 59.3.0

```rust
unsafe fn from_bytes_unchecked(b: &[u8]) -> &Self
```

Source: `src/types.rs:1605`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

# Safety

`b` must be a valid byte sequence for `Self`
