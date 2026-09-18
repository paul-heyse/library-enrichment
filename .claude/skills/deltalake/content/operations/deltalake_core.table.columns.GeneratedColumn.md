# `deltalake_core::table::columns::GeneratedColumn`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.columns.GeneratedColumn.json).

<a id="op-323e94ffbe100a218af8d85a"></a>
## GeneratedColumn

`struct` · `deltalake_core::table::columns::GeneratedColumn` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct GeneratedColumn
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L43).

Source: `crates/core/src/table/columns.rs:43`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A generated column

<a id="op-ec380214c6856380b259142c"></a>
## as_any

`function` · `deltalake_core::table::columns::GeneratedColumn::as_any` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_any(&self) -> &dyn Any
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L80).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::GeneratedColumn", "path": "GeneratedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [83, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/table/columns.rs:80`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fb3c4f7adf384628b2fcaf1"></a>
## clone

`function` · `deltalake_core::table::columns::GeneratedColumn::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> GeneratedColumn
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L42).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::GeneratedColumn", "path": "GeneratedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 32], "end": [42, 37], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/table/columns.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f7d0a8d1c6b1f8519d31576"></a>
## data_type

`struct_field` · `deltalake_core::table::columns::GeneratedColumn::data_type` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data_type: kernel::DataType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L51).

Source: `crates/core/src/table/columns.rs:51`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Data Type

<a id="op-4cf99207134ae7711b7315f2"></a>
## eq

`function` · `deltalake_core::table::columns::GeneratedColumn::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &GeneratedColumn) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L42).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::GeneratedColumn", "path": "GeneratedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 14], "end": [42, 23], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/table/columns.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebe23bfe5d51f15990c36ae1"></a>
## fmt

`function` · `deltalake_core::table::columns::GeneratedColumn::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L42).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::GeneratedColumn", "path": "GeneratedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 25], "end": [42, 30], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/table/columns.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-470db9450743779b4a862f4d"></a>
## generation_expr

`struct_field` · `deltalake_core::table::columns::GeneratedColumn::generation_expr` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
generation_expr: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L47).

Source: `crates/core/src/table/columns.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The SQL string that generate the column value.

<a id="op-eca7c9cbb24ba2f7475a3c67"></a>
## get_expression

`function` · `deltalake_core::table::columns::GeneratedColumn::get_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_expression(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::GeneratedColumn", "path": "GeneratedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [83, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/table/columns.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b647d1e7cae05cc275289efe"></a>
## get_generation_expression

`function` · `deltalake_core::table::columns::GeneratedColumn::get_generation_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_generation_expression(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::GeneratedColumn", "path": "GeneratedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [69, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/columns.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the SQL expression used to generate this column's values.

<a id="op-a36e7a127539bba232cacc9a"></a>
## get_name

`function` · `deltalake_core::table::columns::GeneratedColumn::get_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_name(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::GeneratedColumn", "path": "GeneratedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [83, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/table/columns.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab5359a08acde9cd20314cd0"></a>
## name

`struct_field` · `deltalake_core::table::columns::GeneratedColumn::name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L45).

Source: `crates/core/src/table/columns.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The full path to the field.

<a id="op-d66544950849de9fc11be921"></a>
## new

`function` · `deltalake_core::table::columns::GeneratedColumn::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(field_name: &str, sql_generation: &str, data_type: &DataType) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L56).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::columns::GeneratedColumn", "path": "GeneratedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [69, 2], "filename": "crates/core/src/table/columns.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/columns.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new invariant

<a id="op-17874f21f8b61fec08ca1533"></a>
## validation_expr

`struct_field` · `deltalake_core::table::columns::GeneratedColumn::validation_expr` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
validation_expr: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/columns.rs#L49).

Source: `crates/core/src/table/columns.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The SQL string that must always evaluate to true.
