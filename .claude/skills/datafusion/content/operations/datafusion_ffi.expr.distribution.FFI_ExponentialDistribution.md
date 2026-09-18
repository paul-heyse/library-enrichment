# `datafusion_ffi::expr::distribution::FFI_ExponentialDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.distribution.FFI_ExponentialDistribution.json).

<a id="op-43e463bbfa96036b5bbbe118"></a>
## FFI_ExponentialDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_ExponentialDistribution` · datafusion-ffi 55.1.0

```rust
struct FFI_ExponentialDistribution
```

Source: `src/expr/distribution.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7bf1af7758b84cddc0f98b3"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::distribution::FFI_ExponentialDistribution::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_ExponentialDistribution", "path": "FFI_ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [132, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49bfca3e674b96758e2a9e96"></a>
## fmt

`function` · `datafusion_ffi::expr::distribution::FFI_ExponentialDistribution::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_ExponentialDistribution", "path": "FFI_ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 10], "end": [82, 15], "filename": "src/expr/distribution.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/distribution.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b41b84a0568dbaba5241aa45"></a>
## try_from

`function` · `datafusion_ffi::expr::distribution::FFI_ExponentialDistribution::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: &ExponentialDistribution) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_ExponentialDistribution", "path": "FFI_ExponentialDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [132, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::ExponentialDistribution", "path": "ExponentialDistribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
