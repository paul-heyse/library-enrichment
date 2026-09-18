# `deltalake_core::table::columns::Constraint`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.columns.Constraint.json).

<a id="op-19695308bb03eaf3852587df"></a>
## Constraint

`struct` · `deltalake_core::table::columns::Constraint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Constraint
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L10).

Source: `crates/core/src/table/columns.rs:10`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A constraint in a check constraint

<a id="op-b7c1ec342c7f7d00bbe71bae"></a>
## as_any

`function` · `deltalake_core::table::columns::Constraint::as_any` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_any(&self) -> &dyn Any
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L36).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [39, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/table/columns.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2a50d11a432e4c4e8ed8ce1"></a>
## clone

`function` · `deltalake_core::table::columns::Constraint::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Constraint
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 41], "end": [9, 46], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/table/columns.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4567bca8cecca4ac95180755"></a>
## default

`function` · `deltalake_core::table::columns::Constraint::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Constraint
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 32], "end": [9, 39], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/table/columns.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef4113a12d23496b84332d20"></a>
## deserialize

`function` · `deltalake_core::table::columns::Constraint::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 59], "end": [9, 70], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/table/columns.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-065f22d8b2de1aef1366336c"></a>
## eq

`function` · `deltalake_core::table::columns::Constraint::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Constraint) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 14], "end": [9, 23], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/table/columns.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e8692779b450cec207e60c1"></a>
## expr

`struct_field` · `deltalake_core::table::columns::Constraint::expr` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
expr: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L14).

Source: `crates/core/src/table/columns.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The SQL string that must always evaluate to true.

<a id="op-42b8ca021785a468b061b79b"></a>
## fmt

`function` · `deltalake_core::table::columns::Constraint::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 25], "end": [9, 30], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/table/columns.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bde5d5d329acc29f9fe33ea3"></a>
## get_expression

`function` · `deltalake_core::table::columns::Constraint::get_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_expression(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [39, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/table/columns.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b0dd28f311846afffc5dc91"></a>
## get_name

`function` · `deltalake_core::table::columns::Constraint::get_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_name(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L28).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [39, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/table/columns.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9277d70bcf7be341b54c6a33"></a>
## name

`struct_field` · `deltalake_core::table::columns::Constraint::name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L12).

Source: `crates/core/src/table/columns.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The full path to the field.

<a id="op-2352e7679ef6d8835c351267"></a>
## new

`function` · `deltalake_core::table::columns::Constraint::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(field_name: &str, invariant_sql: &str) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [17, 1], "end": [25, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/columns.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new invariant

<a id="op-836f64f5827e6a3df2d16a1d"></a>
## serialize

`function` · `deltalake_core::table::columns::Constraint::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 48], "end": [9, 57], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/table/columns.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
