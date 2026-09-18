# `datafusion_ffi::table_source::FFI_TableType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.table_source.FFI_TableType.json).

<a id="op-5645d015e02993fbe878954c"></a>
## FFI_TableType

`enum` · `datafusion_ffi::table_source::FFI_TableType` · datafusion-ffi 55.1.0

```rust
enum FFI_TableType
```

Source: `src/table_source.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI safe version of [`TableType`](../operations/datafusion_expr.table_source.TableType.md#op-2967293de6082b164adec79c).

<a id="op-e1e4f345efdb865ac0a343f3"></a>
## Base

`variant` · `datafusion_ffi::table_source::FFI_TableType::Base` · datafusion-ffi 55.1.0

```rust
Base
```

Source: `src/table_source.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cd479030d116f8eb30fc8ec"></a>
## Temporary

`variant` · `datafusion_ffi::table_source::FFI_TableType::Temporary` · datafusion-ffi 55.1.0

```rust
Temporary
```

Source: `src/table_source.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-903be0804eba3dfbabe7505f"></a>
## View

`variant` · `datafusion_ffi::table_source::FFI_TableType::View` · datafusion-ffi 55.1.0

```rust
View
```

Source: `src/table_source.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a26d50b82d8877eaf5f27b03"></a>
## clone

`function` · `datafusion_ffi::table_source::FFI_TableType::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> FFI_TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_source::FFI_TableType", "path": "FFI_TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 17], "end": [59, 22], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_source.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4443b9e68d44435b42206890"></a>
## eq

`function` · `datafusion_ffi::table_source::FFI_TableType::eq` · datafusion-ffi 55.1.0

```rust
fn eq(&self, other: &FFI_TableType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_source::FFI_TableType", "path": "FFI_TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 30], "end": [59, 39], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/table_source.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f076bd60bdce39989f2dd658"></a>
## fmt

`function` · `datafusion_ffi::table_source::FFI_TableType::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_source::FFI_TableType", "path": "FFI_TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 10], "end": [59, 15], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_source.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e767431e3546d57ddf07868"></a>
## from

`function` · `datafusion_ffi::table_source::FFI_TableType::from` · datafusion-ffi 55.1.0

```rust
fn from(value: TableType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_source::FFI_TableType", "path": "FFI_TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [84, 2], "filename": "src/table_source.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::table_source::TableType", "path": "TableType"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_source.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
