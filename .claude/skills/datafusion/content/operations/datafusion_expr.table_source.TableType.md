# `datafusion_expr::table_source::TableType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.table_source.TableType.json).

<a id="op-2967293de6082b164adec79c"></a>
## TableType

`enum` · `datafusion_expr::table_source::TableType` · datafusion-expr 55.1.0

```rust
enum TableType
```

Source: `src/table_source.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates the type of this table for metadata/catalog purposes.

<a id="op-32728e0f74015fb0283b7fe4"></a>
## Base

`variant` · `datafusion_expr::table_source::TableType::Base` · datafusion-expr 55.1.0

```rust
Base
```

Source: `src/table_source.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

An ordinary physical table.

<a id="op-1b228b6bf1acc0534f3a0755"></a>
## Temporary

`variant` · `datafusion_expr::table_source::TableType::Temporary` · datafusion-expr 55.1.0

```rust
Temporary
```

Source: `src/table_source.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A transient table.

<a id="op-4309e132f9efe6bc0cba3bf5"></a>
## View

`variant` · `datafusion_expr::table_source::TableType::View` · datafusion-expr 55.1.0

```rust
View
```

Source: `src/table_source.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A non-materialized table that itself uses a query internally to provide data.

<a id="op-d272d8ba7fc1ecfa75cb1fc8"></a>
## clone

`function` · `datafusion_expr::table_source::TableType::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::table_source::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 22], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_source.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ac352c7309433b64a3846b4"></a>
## eq

`function` · `datafusion_expr::table_source::TableType::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &TableType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::table_source::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 30], "end": [54, 39], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/table_source.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8de9d6a84865e9488de173ad"></a>
## fmt

`function` · `datafusion_expr::table_source::TableType::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::table_source::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_source.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa9cc1b0085f9517e8639f00"></a>
## fmt

`function` · `datafusion_expr::table_source::TableType::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::table_source::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [72, 2], "filename": "src/table_source.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/table_source.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
