# `datafusion_expr::logical_plan::dml::CopyTo`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.dml.CopyTo.json).

<a id="op-07f8bb3eb141720e9af3dcd6"></a>
## CopyTo

`struct` · `datafusion_expr::logical_plan::dml::CopyTo` · datafusion-expr 55.1.0

```rust
struct CopyTo
```

Source: `src/logical_plan/dml.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Operator that copies the contents of a database to file(s)

<a id="op-2cbe4552f17fa2a8cde3a439"></a>
## clone

`function` · `datafusion_expr::logical_plan::dml::CopyTo::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CopyTo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::CopyTo", "path": "CopyTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/dml.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca7514f6cd1ce7bfafe32bba"></a>
## eq

`function` · `datafusion_expr::logical_plan::dml::CopyTo::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::CopyTo", "path": "CopyTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [65, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/dml.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fea7a6850870ce5c41eaa569"></a>
## file_type

`struct_field` · `datafusion_expr::logical_plan::dml::CopyTo::file_type` · datafusion-expr 55.1.0

```rust
file_type: std::sync::Arc<dyn FileType>
```

Source: `src/logical_plan/dml.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

File type trait

<a id="op-0d45f2a269c05288005a4c8f"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::CopyTo::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::CopyTo", "path": "CopyTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [58, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/dml.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2355d3b9c2e4adc83dd62206"></a>
## hash

`function` · `datafusion_expr::logical_plan::dml::CopyTo::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::CopyTo", "path": "CopyTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [95, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/dml.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-042cdedfca9791eade66a789"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::dml::CopyTo::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/dml.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The relation that determines the tuples to write to the output file(s)

<a id="op-b48b812eb79b94154e5d7df4"></a>
## new

`function` · `datafusion_expr::logical_plan::dml::CopyTo::new` · datafusion-expr 55.1.0

```rust
fn new(input: Arc<LogicalPlan>, output_url: String, partition_by: Vec<String>, file_type: Arc<dyn FileType>, options: HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::CopyTo", "path": "CopyTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [115, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a296afebe8675be308f0624"></a>
## options

`struct_field` · `datafusion_expr::logical_plan::dml::CopyTo::options` · datafusion-expr 55.1.0

```rust
options: std::collections::HashMap<String, String>
```

Source: `src/logical_plan/dml.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

SQL Options that can affect the formats

<a id="op-c4f60d5f765d7950b79f1681"></a>
## output_schema

`struct_field` · `datafusion_expr::logical_plan::dml::CopyTo::output_schema` · datafusion-expr 55.1.0

```rust
output_schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/dml.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema of the output (a single column "count")

<a id="op-f1c82208857463ae4b4a7bf6"></a>
## output_url

`struct_field` · `datafusion_expr::logical_plan::dml::CopyTo::output_url` · datafusion-expr 55.1.0

```rust
output_url: String
```

Source: `src/logical_plan/dml.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The location to write the file(s)

<a id="op-f575a55fcec417c695251642"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::dml::CopyTo::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::CopyTo", "path": "CopyTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [87, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/dml.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-756bfb2199808dc5249c7a33"></a>
## partition_by

`struct_field` · `datafusion_expr::logical_plan::dml::CopyTo::partition_by` · datafusion-expr 55.1.0

```rust
partition_by: Vec<String>
```

Source: `src/logical_plan/dml.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determines which, if any, columns should be used for hive-style partitioned writes
