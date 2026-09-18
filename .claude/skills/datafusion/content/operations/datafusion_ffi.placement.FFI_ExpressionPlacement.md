# `datafusion_ffi::placement::FFI_ExpressionPlacement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.placement.FFI_ExpressionPlacement.json).

<a id="op-b742f0fe8fd8c421bd6d32a7"></a>
## FFI_ExpressionPlacement

`enum` · `datafusion_ffi::placement::FFI_ExpressionPlacement` · datafusion-ffi 55.1.0

```rust
enum FFI_ExpressionPlacement
```

Source: `src/placement.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbfae2fd20df3916fc260f4a"></a>
## Column

`variant` · `datafusion_ffi::placement::FFI_ExpressionPlacement::Column` · datafusion-ffi 55.1.0

```rust
Column
```

Source: `src/placement.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec85949341ea02eaf2c9ccfc"></a>
## KeepInPlace

`variant` · `datafusion_ffi::placement::FFI_ExpressionPlacement::KeepInPlace` · datafusion-ffi 55.1.0

```rust
KeepInPlace
```

Source: `src/placement.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6274371795a5c2e614e98bf5"></a>
## Literal

`variant` · `datafusion_ffi::placement::FFI_ExpressionPlacement::Literal` · datafusion-ffi 55.1.0

```rust
Literal
```

Source: `src/placement.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a026757af1a7bb131817a63"></a>
## MoveTowardsLeafNodes

`variant` · `datafusion_ffi::placement::FFI_ExpressionPlacement::MoveTowardsLeafNodes` · datafusion-ffi 55.1.0

```rust
MoveTowardsLeafNodes
```

Source: `src/placement.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1237610c7df6bb6c61d01177"></a>
## clone

`function` · `datafusion_ffi::placement::FFI_ExpressionPlacement::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> FFI_ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::placement::FFI_ExpressionPlacement", "path": "FFI_ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 17], "end": [22, 22], "filename": "src/placement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/placement.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b01e61f4222dd8c16e07dec"></a>
## eq

`function` · `datafusion_ffi::placement::FFI_ExpressionPlacement::eq` · datafusion-ffi 55.1.0

```rust
fn eq(&self, other: &FFI_ExpressionPlacement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::placement::FFI_ExpressionPlacement", "path": "FFI_ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 30], "end": [22, 39], "filename": "src/placement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/placement.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8b02172df560308738e03a6"></a>
## fmt

`function` · `datafusion_ffi::placement::FFI_ExpressionPlacement::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::placement::FFI_ExpressionPlacement", "path": "FFI_ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 15], "filename": "src/placement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/placement.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e99f1618eb1b39d9a14839d"></a>
## from

`function` · `datafusion_ffi::placement::FFI_ExpressionPlacement::from` · datafusion-ffi 55.1.0

```rust
fn from(value: ExpressionPlacement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::placement::FFI_ExpressionPlacement", "path": "FFI_ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [39, 2], "filename": "src/placement.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::placement::ExpressionPlacement", "path": "ExpressionPlacement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/placement.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
