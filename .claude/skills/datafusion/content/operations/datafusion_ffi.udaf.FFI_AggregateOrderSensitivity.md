# `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udaf.FFI_AggregateOrderSensitivity.json).

<a id="op-afb8aa0e4e235d61e03e85fb"></a>
## FFI_AggregateOrderSensitivity

`enum` · `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity` · datafusion-ffi 55.1.0

```rust
enum FFI_AggregateOrderSensitivity
```

Source: `src/udaf/mod.rs:629`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca2c25db1a2b9995b22cdfc5"></a>
## Beneficial

`variant` · `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity::Beneficial` · datafusion-ffi 55.1.0

```rust
Beneficial
```

Source: `src/udaf/mod.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9ebcb4da79d6e3165e507bd"></a>
## HardRequirement

`variant` · `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity::HardRequirement` · datafusion-ffi 55.1.0

```rust
HardRequirement
```

Source: `src/udaf/mod.rs:631`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52a32cf0c45404316be787c9"></a>
## Insensitive

`variant` · `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity::Insensitive` · datafusion-ffi 55.1.0

```rust
Insensitive
```

Source: `src/udaf/mod.rs:630`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69c3095df87d97f068620932"></a>
## SoftRequirement

`variant` · `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity::SoftRequirement` · datafusion-ffi 55.1.0

```rust
SoftRequirement
```

Source: `src/udaf/mod.rs:632`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fad92717a644a096d444773"></a>
## fmt

`function` · `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::FFI_AggregateOrderSensitivity", "path": "FFI_AggregateOrderSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [628, 10], "end": [628, 15], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udaf/mod.rs:628`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84a8fabf54dc61c03c1ecf1a"></a>
## from

`function` · `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity::from` · datafusion-ffi 55.1.0

```rust
fn from(value: AggregateOrderSensitivity) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::FFI_AggregateOrderSensitivity", "path": "FFI_AggregateOrderSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [647, 1], "end": [656, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::order::AggregateOrderSensitivity", "path": "AggregateOrderSensitivity"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/udaf/mod.rs:648`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
