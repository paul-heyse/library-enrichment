# `datafusion_physical_expr::expressions::cast::CastExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.cast.CastExpr.json).

<a id="op-b2ec9a951f65fbe4161b17ed"></a>
## CastExpr

`struct` · `datafusion_physical_expr::expressions::cast::CastExpr` · datafusion-physical-expr 55.1.0

```rust
struct CastExpr
```

Source: `src/expressions/cast.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

CAST expression casts an expression to a specific data type and returns a runtime error on invalid cast

<a id="op-a2aadb12aa9b76326719b241"></a>
## cast_options

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::cast_options` · datafusion-physical-expr 55.1.0

```rust
fn cast_options(&self) -> &CastOptions<'static>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The cast options

<a id="op-604f06040fb2fcb71449d9ef"></a>
## cast_type

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::cast_type` · datafusion-physical-expr 55.1.0

```rust
fn cast_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The data type to cast to

<a id="op-9f15307e6e48c5281ab5b3ba"></a>
## check_bigger_cast

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::check_bigger_cast` · datafusion-physical-expr 55.1.0

```rust
fn check_bigger_cast(cast_type: &DataType, src: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Check if casting from the specified source type to the target type is a
widening cast (e.g. from `Int8` to `Int16`).

<a id="op-3fc66dd949ab8e32fe899ead"></a>
## children

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c25ac879c8de22145a9c4b7"></a>
## clone

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> CastExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 17], "end": [60, 22], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expressions/cast.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9a072e3855bc3c60bd8703f"></a>
## data_type

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3953340eeab792f7d780f2ed"></a>
## eq

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [91, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expressions/cast.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec50d7b5e80fba89e79c0664"></a>
## evaluate

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dc5743faea6933422d9ceba"></a>
## evaluate_bounds

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::evaluate_bounds` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4274d84f851ea4f53315b71e"></a>
## expr

`struct_field` · `datafusion_physical_expr::expressions::cast::CastExpr::expr` · datafusion-physical-expr 55.1.0

```rust
expr: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/expressions/cast.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The expression to cast

<a id="op-73df0514b2b595bb083d379b"></a>
## expr

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::expr` · datafusion-physical-expr 55.1.0

```rust
fn expr(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The expression to cast

<a id="op-dc8ae7083cbbf0f5f1a95021"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 15], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/cast.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee6fe3f803e214e4f347bdf8"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [313, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expressions/cast.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4568728e14bfc6a960c6217"></a>
## fmt_sql

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90f94152549632565822f90f"></a>
## get_properties

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::get_properties` · datafusion-physical-expr 55.1.0

```rust
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A [`CastExpr`](../operations/datafusion_physical_expr.expressions.cast.CastExpr.md#op-b2ec9a951f65fbe4161b17ed) preserves the ordering of its child if the cast is done
under the same datatype family.

<a id="op-5e7bd4ec5e369572a72d71d0"></a>
## has_explicit_metadata

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::has_explicit_metadata` · datafusion-physical-expr 55.1.0

```rust
fn has_explicit_metadata(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Whether this cast has explicit metadata (vs pass-through from source).

<a id="op-8dc7b8752c2a7f24d5d12524"></a>
## has_explicit_nullability

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::has_explicit_nullability` · datafusion-physical-expr 55.1.0

```rust
fn has_explicit_nullability(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Whether this cast has explicit nullability (vs pass-through from source).

<a id="op-ac0a706cb7c5061433c8aedd"></a>
## hash

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [109, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expressions/cast.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72bde8b4680382d8afa3c908"></a>
## is_bigger_cast

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::is_bigger_cast` · datafusion-physical-expr 55.1.0

```rust
fn is_bigger_cast(&self, src: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Check if the cast is a widening cast (e.g. from `Int8` to `Int16`).

<a id="op-d8f0729e31480c91812d43d6"></a>
## new

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(expr: Arc<dyn PhysicalExpr>, cast_type: DataType, cast_options: Option<CastOptions<'static>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new `CastExpr` using only a `DataType`.

This constructor creates a type-only cast where metadata and nullability
are passed through from the source expression (with extension type keys
stripped from metadata). This is the most common use case when you only
need to change the data type.

For explicit control over the output field's metadata and nullability,
use [`CastExpr::new_with_target_field`](../operations/datafusion_physical_expr.expressions.cast.CastExpr.md#op-ee1086d0f2f59103f6cce510) or the individual builder methods.

<a id="op-ee1086d0f2f59103f6cce510"></a>
## new_with_target_field

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::new_with_target_field` · datafusion-physical-expr 55.1.0

```rust
fn new_with_target_field(expr: Arc<dyn PhysicalExpr>, target_field: FieldRef, cast_options: Option<CastOptions<'static>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new `CastExpr` with an explicit target `FieldRef`.

The provided `target_field` determines the output characteristics:
- The field's data type becomes the cast target type
- The field's metadata is used exactly as provided
- The field's nullability is preserved

This is the preferred constructor when the caller has explicit field
information that should be used exactly (for example, during schema
enforcement or adapter layers).

See [`CastExpr::new`](../operations/datafusion_physical_expr.expressions.cast.CastExpr.md#op-d8f0729e31480c91812d43d6) for type-only casts where source metadata should
pass through.

<a id="op-ed19a1d7f85cf2bc3792c9a8"></a>
## nullable

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fcd384ca0b56927ab356bfe"></a>
## propagate_constraints

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::propagate_constraints` · datafusion-physical-expr 55.1.0

```rust
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17e8dcf64b31bcd97b08d15d"></a>
## return_field

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-decf8b2f96dd843f7b8b015d"></a>
## target_field

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::target_field` · datafusion-physical-expr 55.1.0

```rust
fn target_field(&self) -> &FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The target field this cast was constructed with.

For a type-only cast this is a field synthesized from the target data
type alone; only its data type is meaningful. Note that the returned
field may not match what `return_field()` returns when evaluated against
a schema, since `return_field()` may incorporate source field information.

Prefer [`cast_type()`], [`target_metadata()`], and [`target_nullable()`]
for direct access to the individual components.

[`cast_type()`]: CastExpr::cast_type
[`target_metadata()`]: CastExpr::target_metadata
[`target_nullable()`]: CastExpr::target_nullable

<a id="op-9a97139c9877cc94cf2a9ffd"></a>
## target_metadata

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::target_metadata` · datafusion-physical-expr 55.1.0

```rust
fn target_metadata(&self) -> Option<&HashMap<String, String>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Explicit metadata for the output field, or `None` to pass through source metadata.

<a id="op-38437dcef8200f7532f3ebec"></a>
## target_nullable

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::target_nullable` · datafusion-physical-expr 55.1.0

```rust
fn target_nullable(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [276, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Explicit nullability for the output field, or `None` to pass through source nullability.

<a id="op-d03aaa25f523902de2c6ac3f"></a>
## try_from_proto

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::try_from_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalExprNode, ctx: &datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [442, 2], "filename": "src/expressions/cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/cast.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Reconstruct a [`CastExpr`](../operations/datafusion_physical_expr.expressions.cast.CastExpr.md#op-b2ec9a951f65fbe4161b17ed) from its protobuf representation.

Takes the whole [`PhysicalExprNode`] so the decode signature matches
other migrated expressions and can inspect outer-node metadata if
needed in the future.

[`PhysicalExprNode`]: datafusion_proto_models::protobuf::PhysicalExprNode

<a id="op-edd71b4679f5c6f34db05658"></a>
## try_to_proto

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::try_to_proto` · datafusion-physical-expr 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f558f826dae4a22806ba9f7"></a>
## with_new_children

`function` · `datafusion_physical_expr::expressions::cast::CastExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::cast::CastExpr", "path": "CastExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 1], "end": [405, 2], "filename": "src/expressions/cast.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/expressions/cast.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
