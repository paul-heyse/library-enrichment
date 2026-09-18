# `datafusion_expr::logical_plan::plan::Values`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Values.json).

<a id="op-010f7ca4902e5270a581b33e"></a>
## Values

`struct` · `datafusion_expr::logical_plan::plan::Values` · datafusion-expr 55.1.0

```rust
struct Values
```

Source: `src/logical_plan/plan.rs:2425`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Values expression. See
[Postgres VALUES](https://www.postgresql.org/docs/current/queries-values.html)
documentation for more details.

<a id="op-ddb858ce333ed4e48a9a051f"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Values::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Values
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2424, 17], "end": [2424, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:2424`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b8161ff7e8fe308c8ae4680"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Values::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Values) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2424, 24], "end": [2424, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:2424`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f35604e2022ce3dc696e37cb"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Values::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2424, 10], "end": [2424, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:2424`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65b15cb757e89565bec30919"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Values::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2424, 39], "end": [2424, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:2424`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52e74471fd962818c3027b46"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Values::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Values", "path": "Values"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2433, 1], "end": [2440, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:2434`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d6458c65a3bbafd0a87f48b"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Values::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:2427`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The table schema

<a id="op-880897d8a2b01ca04c1f2b31"></a>
## values

`struct_field` · `datafusion_expr::logical_plan::plan::Values::values` · datafusion-expr 55.1.0

```rust
values: Vec<Vec<Expr>>
```

Source: `src/logical_plan/plan.rs:2429`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Values
