# `datafusion_ffi::expr::distribution::FFI_UniformDistribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.distribution.FFI_UniformDistribution.json).

<a id="op-235159f8025994855846652b"></a>
## FFI_UniformDistribution

`struct` · `datafusion_ffi::expr::distribution::FFI_UniformDistribution` · datafusion-ffi 55.1.0

```rust
struct FFI_UniformDistribution
```

Source: `src/expr/distribution.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-199e47ffecc68994f7b4d6f2"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::distribution::FFI_UniformDistribution::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_UniformDistribution", "path": "FFI_UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [118, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9702061742ee3acf1d657be8"></a>
## fmt

`function` · `datafusion_ffi::expr::distribution::FFI_UniformDistribution::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_UniformDistribution", "path": "FFI_UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 15], "filename": "src/expr/distribution.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/distribution.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c77ba0d094998b413fa9a9f"></a>
## try_from

`function` · `datafusion_ffi::expr::distribution::FFI_UniformDistribution::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: &UniformDistribution) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_UniformDistribution", "path": "FFI_UniformDistribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [118, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::UniformDistribution", "path": "UniformDistribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
