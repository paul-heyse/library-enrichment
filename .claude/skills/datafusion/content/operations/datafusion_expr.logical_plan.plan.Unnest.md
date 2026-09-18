# `datafusion_expr::logical_plan::plan::Unnest`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Unnest.json).

<a id="op-d7c974ee90026eb633e603dd"></a>
## Unnest

`struct` · `datafusion_expr::logical_plan::plan::Unnest` · datafusion-expr 55.1.0

```rust
struct Unnest
```

Source: `src/logical_plan/plan.rs:4625`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Unnest a column that contains a nested list type. See
[`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a) for more details.

<a id="op-e6e251455026e55e3a778d69"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Unnest::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Unnest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4624, 17], "end": [4624, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:4624`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aefca1c119e09311c182ef28"></a>
## dependency_indices

`struct_field` · `datafusion_expr::logical_plan::plan::Unnest::dependency_indices` · datafusion-expr 55.1.0

```rust
dependency_indices: Vec<usize>
```

Source: `src/logical_plan/plan.rs:4638`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Having items aligned with the output columns
representing which column in the input schema each output column depends on

<a id="op-db9cc4797a83b1a7a85dde2e"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Unnest::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Unnest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4624, 24], "end": [4624, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:4624`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9e733d51c282b9cdacde11f"></a>
## exec_columns

`struct_field` · `datafusion_expr::logical_plan::plan::Unnest::exec_columns` · datafusion-expr 55.1.0

```rust
exec_columns: Vec<datafusion_common::Column>
```

Source: `src/logical_plan/plan.rs:4629`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Columns to run unnest on, can be a list of (List/Struct) columns

<a id="op-6f1da73666abef551403ffa7"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Unnest::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4624, 10], "end": [4624, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:4624`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-318a4b86222aca7e5e5b4cb3"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Unnest::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4624, 39], "end": [4624, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:4624`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de5716d1ef7ef256ad5725f5"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Unnest::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:4627`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The incoming logical plan

<a id="op-4222867b7741bd1ead783914"></a>
## list_type_columns

`struct_field` · `datafusion_expr::logical_plan::plan::Unnest::list_type_columns` · datafusion-expr 55.1.0

```rust
list_type_columns: Vec<(usize, ColumnUnnestList)>
```

Source: `src/logical_plan/plan.rs:4632`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

refer to the indices(in the input schema) of columns
that have type list to run unnest on

<a id="op-cd65f4d7dc05b5df3e142247"></a>
## options

`struct_field` · `datafusion_expr::logical_plan::plan::Unnest::options` · datafusion-expr 55.1.0

```rust
options: datafusion_common::UnnestOptions
```

Source: `src/logical_plan/plan.rs:4642`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Options

<a id="op-11c3b50a698ca01fbe346d8f"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Unnest::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4646, 1], "end": [4687, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:4647`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f128ad13a76d7963f6eb841"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Unnest::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:4640`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The output schema, containing the unnested field column.

<a id="op-aa8803bd1920803e68c1f7a4"></a>
## struct_type_columns

`struct_field` · `datafusion_expr::logical_plan::plan::Unnest::struct_type_columns` · datafusion-expr 55.1.0

```rust
struct_type_columns: Vec<usize>
```

Source: `src/logical_plan/plan.rs:4635`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

refer to the indices (in the input schema) of columns
that have type struct to run unnest on

<a id="op-b07c5f7cb53e0ca23dc27fa0"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::Unnest::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(input: Arc<LogicalPlan>, exec_columns: Vec<Column>, options: UnnestOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4689, 1], "end": [4826, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4690`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
