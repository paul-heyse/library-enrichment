# `datafusion_ffi::expr::distribution::FFI_GaussianDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.distribution.FFI_GaussianDistribution.json).

<a id="op-994d99b81ba3926254c8729c"></a>
## FFI_GaussianDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_GaussianDistribution` · datafusion-ffi 55.1.0

```rust
struct FFI_GaussianDistribution
```

Source: `src/expr/distribution.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1be4a59258d2c552122cedd"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::distribution::FFI_GaussianDistribution::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_GaussianDistribution", "path": "FFI_GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [142, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c13f88e7f530c90768f96f2a"></a>
## fmt

`function` · `datafusion_ffi::expr::distribution::FFI_GaussianDistribution::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_GaussianDistribution", "path": "FFI_GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 10], "end": [90, 15], "filename": "src/expr/distribution.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/distribution.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-508ab1f2f025e38fed5f98dd"></a>
## try_from

`function` · `datafusion_ffi::expr::distribution::FFI_GaussianDistribution::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: &GaussianDistribution) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_GaussianDistribution", "path": "FFI_GaussianDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [142, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GaussianDistribution", "path": "GaussianDistribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
