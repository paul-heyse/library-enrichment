# `datafusion_ffi::expr::distribution::FFI_Distribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.distribution.FFI_Distribution.json).

<a id="op-deb02accc2fbdacd462d0f11"></a>
## FFI_Distribution

`enum` · `datafusion_ffi::expr::distribution::FFI_Distribution` · datafusion-ffi 55.1.0

```rust
enum FFI_Distribution
```

Source: `src/expr/distribution.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`Distribution`](../operations/datafusion_expr_common.statistics.Distribution.md#op-01128278dc758957f40620ba) across FFI boundaries.
See ['Distribution'] for the meaning of each variant.

<a id="op-4d2872f396460e02f11dfa8f"></a>
## Bernoulli

`variant` · `datafusion_ffi::expr::distribution::FFI_Distribution::Bernoulli` · datafusion-ffi 55.1.0

```rust
Bernoulli
```

Source: `src/expr/distribution.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5665ef0c7329daa6bd76f654"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::distribution::FFI_Distribution::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_Distribution", "path": "FFI_Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [60, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efaf3b5369493902890f540f"></a>
## Exponential

`variant` · `datafusion_ffi::expr::distribution::FFI_Distribution::Exponential` · datafusion-ffi 55.1.0

```rust
Exponential
```

Source: `src/expr/distribution.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70a7b03564933070a234cada"></a>
## Gaussian

`variant` · `datafusion_ffi::expr::distribution::FFI_Distribution::Gaussian` · datafusion-ffi 55.1.0

```rust
Gaussian
```

Source: `src/expr/distribution.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01ac9f4abce3d036cad9439f"></a>
## Generic

`variant` · `datafusion_ffi::expr::distribution::FFI_Distribution::Generic` · datafusion-ffi 55.1.0

```rust
Generic
```

Source: `src/expr/distribution.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c21635f4a8803abba337d12"></a>
## Uniform

`variant` · `datafusion_ffi::expr::distribution::FFI_Distribution::Uniform` · datafusion-ffi 55.1.0

```rust
Uniform
```

Source: `src/expr/distribution.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a5084476853a68703b0e3b6"></a>
## fmt

`function` · `datafusion_ffi::expr::distribution::FFI_Distribution::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_Distribution", "path": "FFI_Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/expr/distribution.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/distribution.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81094af661ca2814b2ea926d"></a>
## try_from

`function` · `datafusion_ffi::expr::distribution::FFI_Distribution::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: &Distribution) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::distribution::FFI_Distribution", "path": "FFI_Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [60, 2], "filename": "src/expr/distribution.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/distribution.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
