# `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.PhysicalSortExpr.json).

<a id="op-80fd045cf8466b7fe98e5ea8"></a>
## PhysicalSortExpr

`struct` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr` · datafusion-physical-expr-common 55.1.0

```rust
struct PhysicalSortExpr
```

Source: `src/sort_expr.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Represents Sort operation for a column in a RecordBatch

Example:
```
# use std::any::Any;
# use std::collections::HashMap;
# use std::fmt::{Display, Formatter};
# use std::hash::Hasher;
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use datafusion_common::Result;
# use arrow::compute::SortOptions;
# use arrow::datatypes::{DataType, Field, FieldRef, Schema};
# use datafusion_expr_common::columnar_value::ColumnarValue;
# use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
# use datafusion_physical_expr_common::sort_expr::PhysicalSortExpr;
# // this crate doesn't have a physical expression implementation
# // so make a really simple one
# #[derive(Clone, Debug, PartialEq, Eq, Hash)]
# struct MyPhysicalExpr;
# impl PhysicalExpr for MyPhysicalExpr {
#  fn data_type(&self, input_schema: &Schema) -> Result<DataType> {todo!()}
#  fn nullable(&self, input_schema: &Schema) -> Result<bool> {todo!() }
#  fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue> {todo!() }
#  fn return_field(&self, input_schema: &Schema) -> Result<FieldRef> { unimplemented!() }
#  fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>> {todo!()}
#  fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>> {todo!()}
# fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
# }
# impl Display for MyPhysicalExpr {
#    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "a") }
# }
# fn col(name: &str) -> Arc<dyn PhysicalExpr> { Arc::new(MyPhysicalExpr) }
// Sort by a ASC
let options = SortOptions::default();
let sort_expr = PhysicalSortExpr::new(col("a"), options);
assert_eq!(sort_expr.to_string(), "a ASC");

// Sort by a DESC NULLS LAST
let sort_expr = PhysicalSortExpr::new_default(col("a"))
  .desc()
  .nulls_last();
assert_eq!(sort_expr.to_string(), "a DESC NULLS LAST");
```

<a id="op-a6d15b07d9bc0e938b8af9f5"></a>
## asc

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::asc` · datafusion-physical-expr-common 55.1.0

```rust
fn asc(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the sort sort options to ASC

<a id="op-3233b19c35fbe63a4af652d1"></a>
## clone

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> PhysicalSortExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 10], "end": [79, 15], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort_expr.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0471de6d221b77d18883084"></a>
## desc

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::desc` · datafusion-physical-expr-common 55.1.0

```rust
fn desc(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the sort sort options to DESC

<a id="op-19004f2d4848af8713a3c754"></a>
## eq

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 1], "end": [306, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sort_expr.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb49414cfda211d141f25857"></a>
## evaluate_to_sort_column

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::evaluate_to_sort_column` · datafusion-physical-expr-common 55.1.0

```rust
fn evaluate_to_sort_column(&self, batch: &RecordBatch) -> Result<SortColumn>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Evaluates the sort expression into a `SortColumn` that can be passed
into the arrow sort kernel.

<a id="op-f43fa57b709ba17bd1ab62ac"></a>
## expr

`struct_field` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::expr` · datafusion-physical-expr-common 55.1.0

```rust
expr: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/sort_expr.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Physical expression representing the column to sort

<a id="op-2f81633c3c1ac0ed3b85555e"></a>
## fmt

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [319, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/sort_expr.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb6da95627ed273561c925a2"></a>
## fmt

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 17], "end": [79, 22], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort_expr.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95cfce0ca062dfbfedc7cc79"></a>
## fmt_sql

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::fmt_sql` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt_sql(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Like [`PhysicalExpr::fmt_sql`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-7c15755c74044b7b06f8a4d8) prints a [`PhysicalSortExpr`](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortExpr.md#op-80fd045cf8466b7fe98e5ea8) in a SQL-like format.

<a id="op-eef7fc040f096affd53ec53c"></a>
## from

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: PhysicalSortRequirement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [444, 1], "end": [455, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement", "path": "PhysicalSortRequirement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/sort_expr.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The default sort options `ASC, NULLS LAST` when the requirement does
not specify sort options. This default is consistent with PostgreSQL.

Reference: <https://www.postgresql.org/docs/current/queries-order.html>

<a id="op-3fc133eee340cb51f1ea9b76"></a>
## hash

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::hash` · datafusion-physical-expr-common 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [313, 2], "filename": "src/sort_expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sort_expr.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-838b289d68529a65dcc0eceb"></a>
## new

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(expr: Arc<dyn PhysicalExpr>, options: SortOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new PhysicalSortExpr

<a id="op-ca485ac016459de44c6596a5"></a>
## new_default

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::new_default` · datafusion-physical-expr-common 55.1.0

```rust
fn new_default(expr: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new PhysicalSortExpr with default [`SortOptions`](../operations/arrow_schema.SortOptions.md#op-78d98c3e0c6da432658d0949)

<a id="op-67a2e59a8230678e39a7df7e"></a>
## nulls_first

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::nulls_first` · datafusion-physical-expr-common 55.1.0

```rust
fn nulls_first(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the sort sort options to NULLS FIRST

<a id="op-ca207d1504d32a939c4a9f27"></a>
## nulls_last

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::nulls_last` · datafusion-physical-expr-common 55.1.0

```rust
fn nulls_last(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the sort sort options to NULLS LAST

<a id="op-0d9395c9a0336136ac881538"></a>
## options

`struct_field` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::options` · datafusion-physical-expr-common 55.1.0

```rust
options: arrow::compute::kernels::sort::SortOptions
```

Source: `src/sort_expr.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Option to specify how the given column should be sorted

<a id="op-8173fec961a02df36e40b3e5"></a>
## reverse

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::reverse` · datafusion-physical-expr-common 55.1.0

```rust
fn reverse(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Reverses the sort expression. For instance, `[a ASC NULLS LAST]` turns
into `[a DESC NULLS FIRST]`. Such reversals are useful in planning, e.g.
when constructing equivalent window expressions.

<a id="op-464797b6b146dac95d953a44"></a>
## satisfy

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::satisfy` · datafusion-physical-expr-common 55.1.0

```rust
fn satisfy(&self, requirement: &PhysicalSortRequirement, schema: &Schema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Checks whether this sort expression satisfies the given `requirement`.
If sort options are unspecified in `requirement`, only expressions are
compared for inequality. See [`options_compatible`](../operations/datafusion_physical_expr_common.sort_expr.options_compatible.md#op-b72ca8ec43c3e9b99e47fdd0) for details on
how sort options compare with one another.

<a id="op-65b58857e3a5031f19081ac2"></a>
## satisfy_expr

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::satisfy_expr` · datafusion-physical-expr-common 55.1.0

```rust
fn satisfy_expr(&self, sort_expr: &Self, schema: &Schema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [184, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Checks whether this sort expression satisfies the given `sort_expr`.
See [`options_compatible`](../operations/datafusion_physical_expr_common.sort_expr.options_compatible.md#op-b72ca8ec43c3e9b99e47fdd0) for details on how sort options compare with
one another.

<a id="op-76066a2d9842578f9c8cc954"></a>
## try_from_proto

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::try_from_proto` · datafusion-physical-expr-common 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalSortExprNode, ctx: &physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [227, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Reconstruct a [`PhysicalSortExpr`](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortExpr.md#op-80fd045cf8466b7fe98e5ea8) from its protobuf representation.

<a id="op-b6f987685d8abe8077147f1e"></a>
## try_to_proto

`function` · `datafusion_physical_expr_common::sort_expr::PhysicalSortExpr::try_to_proto` · datafusion-physical-expr-common 55.1.0

```rust
fn try_to_proto(&self, ctx: &physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<datafusion_proto_models::protobuf::PhysicalSortExprNode>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::sort_expr::PhysicalSortExpr", "path": "PhysicalSortExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [227, 2], "filename": "src/sort_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_expr.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Serialize this sort expression, encoding its child expression through
`ctx`.
