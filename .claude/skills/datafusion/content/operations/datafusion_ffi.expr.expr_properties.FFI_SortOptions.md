# `datafusion_ffi::expr::expr_properties::FFI_SortOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.expr_properties.FFI_SortOptions.json).

<a id="op-dc48860c417ad5e67285fe66"></a>
## FFI_SortOptions

`struct` · `datafusion_ffi::expr::expr_properties::FFI_SortOptions` · datafusion-ffi 55.1.0

```rust
struct FFI_SortOptions
```

Source: `src/expr/expr_properties.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e071183d868b6378a5b312f"></a>
## descending

`struct_field` · `datafusion_ffi::expr::expr_properties::FFI_SortOptions::descending` · datafusion-ffi 55.1.0

```rust
descending: bool
```

Source: `src/expr/expr_properties.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1afa8ee6cf11dc1362aacaee"></a>
## fmt

`function` · `datafusion_ffi::expr::expr_properties::FFI_SortOptions::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::expr_properties::FFI_SortOptions", "path": "FFI_SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 10], "end": [93, 15], "filename": "src/expr/expr_properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/expr_properties.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4217d38566ad75f1eff7642"></a>
## from

`function` · `datafusion_ffi::expr::expr_properties::FFI_SortOptions::from` · datafusion-ffi 55.1.0

```rust
fn from(value: &SortOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::expr_properties::FFI_SortOptions", "path": "FFI_SortOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [106, 2], "filename": "src/expr/expr_properties.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::SortOptions", "path": "SortOptions"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr/expr_properties.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bf26480933e61857a125ec4"></a>
## nulls_first

`struct_field` · `datafusion_ffi::expr::expr_properties::FFI_SortOptions::nulls_first` · datafusion-ffi 55.1.0

```rust
nulls_first: bool
```

Source: `src/expr/expr_properties.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
