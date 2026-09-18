# `datafusion_ffi::expr::distribution::FFI_GenericDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.distribution.FFI_GenericDistribution.json).

<a id="op-53158e6f2a425fa3f560a0f9"></a>
## FFI_GenericDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_GenericDistribution` · datafusion-ffi 55.1.0

```rust
struct FFI_GenericDistribution
```

Source: `src/expr/distribution.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34719b2c8b09b6b80d8a7850"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::distribution::FFI_GenericDistribution::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_GenericDistribution", "path": "FFI_GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [167, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c187e238f0a380501517aa57"></a>
## fmt

`function` · `datafusion_ffi::expr::distribution::FFI_GenericDistribution::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_GenericDistribution", "path": "FFI_GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 10], "end": [103, 15], "filename": "src/expr/distribution.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/distribution.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a78cb4780172a7976361f94"></a>
## try_from

`function` · `datafusion_ffi::expr::distribution::FFI_GenericDistribution::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: &GenericDistribution) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_GenericDistribution", "path": "FFI_GenericDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [167, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::GenericDistribution", "path": "GenericDistribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
