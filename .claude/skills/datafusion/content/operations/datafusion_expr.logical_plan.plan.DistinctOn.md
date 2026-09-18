# `datafusion_expr::logical_plan::plan::DistinctOn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.DistinctOn.json).

<a id="op-2d615a993213f8f8243f3531"></a>
## DistinctOn

`struct` · `datafusion_expr::logical_plan::plan::DistinctOn` · datafusion-expr 55.1.0

```rust
struct DistinctOn
```

Source: `src/logical_plan/plan.rs:3761`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Removes duplicate rows from the input

<a id="op-f255e3db130a7a2dc37f2a2c"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::DistinctOn::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> DistinctOn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DistinctOn", "path": "DistinctOn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3760, 17], "end": [3760, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3760`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eee4bddb63ee7ebe15d15114"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::DistinctOn::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &DistinctOn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DistinctOn", "path": "DistinctOn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3760, 24], "end": [3760, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3760`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5accb03e5ff2391b85f113ef"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::DistinctOn::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DistinctOn", "path": "DistinctOn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3760, 10], "end": [3760, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3760`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed589ff35b34a9c3a6e5affb"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::DistinctOn::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DistinctOn", "path": "DistinctOn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3760, 39], "end": [3760, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3760`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93e01cb5b28b482a406d3334"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::DistinctOn::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:3771`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The logical plan that is being DISTINCT'd

<a id="op-15b3f582d0809eb1c3774e95"></a>
## on_expr

`struct_field` · `datafusion_expr::logical_plan::plan::DistinctOn::on_expr` · datafusion-expr 55.1.0

```rust
on_expr: Vec<Expr>
```

Source: `src/logical_plan/plan.rs:3763`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The `DISTINCT ON` clause expression list

<a id="op-82a1a85c29c960e52abebc5e"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::DistinctOn::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DistinctOn", "path": "DistinctOn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3840, 1], "end": [3872, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3841`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ad28a82a114d86575d9e9d2"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::DistinctOn::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:3773`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema description of the DISTINCT ON output

<a id="op-02ccaee27efecd450e586dc6"></a>
## select_expr

`struct_field` · `datafusion_expr::logical_plan::plan::DistinctOn::select_expr` · datafusion-expr 55.1.0

```rust
select_expr: Vec<Expr>
```

Source: `src/logical_plan/plan.rs:3765`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The selected projection expression list

<a id="op-a4a5ab097c608488ee4ff3a3"></a>
## sort_expr

`struct_field` · `datafusion_expr::logical_plan::plan::DistinctOn::sort_expr` · datafusion-expr 55.1.0

```rust
sort_expr: Option<Vec<expr::Sort>>
```

Source: `src/logical_plan/plan.rs:3769`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The `ORDER BY` clause, whose initial expressions must match those of the `ON` clause when
present. Note that those matching expressions actually wrap the `ON` expressions with
additional info pertaining to the sorting procedure (i.e. ASC/DESC, and NULLS FIRST/LAST).

<a id="op-69b88228518e3db00801882e"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::DistinctOn::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(on_expr: Vec<Expr>, select_expr: Vec<Expr>, sort_expr: Option<Vec<SortExpr>>, input: Arc<LogicalPlan>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DistinctOn", "path": "DistinctOn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3776, 1], "end": [3837, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3778`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `DistinctOn` struct.

<a id="op-61cb7cde76abfe2f665231cd"></a>
## with_sort_expr

`function` · `datafusion_expr::logical_plan::plan::DistinctOn::with_sort_expr` · datafusion-expr 55.1.0

```rust
fn with_sort_expr(self, sort_expr: Vec<SortExpr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::DistinctOn", "path": "DistinctOn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3776, 1], "end": [3837, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3816`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Try to update `self` with a new sort expressions.

Validates that the sort expressions are a super-set of the `ON` expressions.
