# `datafusion_expr::logical_plan::plan::DescribeTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.DescribeTable.json).

<a id="op-3d024ace09d78391eafeaa5c"></a>
## DescribeTable

`struct` · `datafusion_expr::logical_plan::plan::DescribeTable` · datafusion-expr 55.1.0

```rust
struct DescribeTable
```

Source: `src/logical_plan/plan.rs:3451`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Describe the schema of table

# Example output:

```sql
> describe traces;
+--------------------+-----------------------------+-------------+
| column_name        | data_type                   | is_nullable |
+--------------------+-----------------------------+-------------+
| attributes         | Utf8                        | YES         |
| duration_nano      | Int64                       | YES         |
| end_time_unix_nano | Int64                       | YES         |
| service.name       | Dictionary(Int32, Utf8)     | YES         |
| span.kind          | Utf8                        | YES         |
| span.name          | Utf8                        | YES         |
| span_id            | Dictionary(Int32, Utf8)     | YES         |
| time               | Timestamp(Nanosecond, None) | NO          |
| trace_id           | Dictionary(Int32, Utf8)     | YES         |
| otel.status_code   | Utf8                        | YES         |
| parent_span_id     | Utf8                        | YES         |
+--------------------+-----------------------------+-------------+
```

<a id="op-da926e975fad74395f2928c7"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::DescribeTable::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> DescribeTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DescribeTable", "path": "DescribeTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3450, 17], "end": [3450, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3450`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89f27d8f1c6cba9b55ebe494"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::DescribeTable::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &DescribeTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DescribeTable", "path": "DescribeTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3450, 24], "end": [3450, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3450`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e80a0ffda879eec4ddd400fc"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::DescribeTable::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DescribeTable", "path": "DescribeTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3450, 10], "end": [3450, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3450`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e85929a1966c1bd155fedd4a"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::DescribeTable::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DescribeTable", "path": "DescribeTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3450, 39], "end": [3450, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3450`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-815a65d815003e586a443cfc"></a>
## output_schema

`struct_field` · `datafusion_expr::logical_plan::plan::DescribeTable::output_schema` · datafusion-expr 55.1.0

```rust
output_schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:3455`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

schema of describe table output

<a id="op-0c41aefca2a933c395bb91b3"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::DescribeTable::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, _other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DescribeTable", "path": "DescribeTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3460, 1], "end": [3465, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3461`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddf679ea2ef0164f15f14e32"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::DescribeTable::schema` · datafusion-expr 55.1.0

```rust
schema: std::sync::Arc<arrow::datatypes::Schema>
```

Source: `src/logical_plan/plan.rs:3453`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Table schema
