# `datafusion_expr::logical_plan::plan::RecursiveQuery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.RecursiveQuery.json).

<a id="op-225c82d05b1d36d76861ebaf"></a>
## RecursiveQuery

`struct` · `datafusion_expr::logical_plan::plan::RecursiveQuery` · datafusion-expr 55.1.0

```rust
struct RecursiveQuery
```

Source: `src/logical_plan/plan.rs:2324`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A variadic query operation, Recursive CTE.

# Recursive Query Evaluation

From the [Postgres Docs]:

1. Evaluate the non-recursive term. For `UNION` (but not `UNION ALL`),
   discard duplicate rows. Include all remaining rows in the result of the
   recursive query, and also place them in a temporary working table.

2. So long as the working table is not empty, repeat these steps:

* Evaluate the recursive term, substituting the current contents of the
  working table for the recursive self-reference. For `UNION` (but not `UNION
  ALL`), discard duplicate rows and rows that duplicate any previous result
  row. Include all remaining rows in the result of the recursive query, and
  also place them in a temporary intermediate table.

* Replace the contents of the working table with the contents of the
  intermediate table, then empty the intermediate table.

[Postgres Docs]: https://www.postgresql.org/docs/current/queries-with.html#QUERIES-WITH-RECURSIVE

<a id="op-f4adbfa9006b93230f516eaa"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::RecursiveQuery::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> RecursiveQuery
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RecursiveQuery", "path": "RecursiveQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2323, 17], "end": [2323, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:2323`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcf2aa5cca809a20f6696025"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::RecursiveQuery::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &RecursiveQuery) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RecursiveQuery", "path": "RecursiveQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2323, 24], "end": [2323, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:2323`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a96c7714bf6369fc9c5228b6"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::RecursiveQuery::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RecursiveQuery", "path": "RecursiveQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2323, 10], "end": [2323, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:2323`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-519e5f8872b32828f2dd18ab"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::RecursiveQuery::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RecursiveQuery", "path": "RecursiveQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2323, 39], "end": [2323, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:2323`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65ea832562c6081650ddbf03"></a>
## is_distinct

`struct_field` · `datafusion_expr::logical_plan::plan::RecursiveQuery::is_distinct` · datafusion-expr 55.1.0

```rust
is_distinct: bool
```

Source: `src/logical_plan/plan.rs:2334`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Should the output of the recursive term be deduplicated (`UNION`) or
not (`UNION ALL`).

<a id="op-7cdb2ebd5eed07426a2288d5"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::plan::RecursiveQuery::name` · datafusion-expr 55.1.0

```rust
name: String
```

Source: `src/logical_plan/plan.rs:2326`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Name of the query

<a id="op-65848b0f239a284bcdcf8cb9"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::RecursiveQuery::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RecursiveQuery", "path": "RecursiveQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2339, 1], "end": [2362, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:2340`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d5cbef5f41a7e5903b31025"></a>
## recursive_term

`struct_field` · `datafusion_expr::logical_plan::plan::RecursiveQuery::recursive_term` · datafusion-expr 55.1.0

```rust
recursive_term: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:2331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The recursive term (evaluated on the contents of the working table until
it returns an empty set)

<a id="op-154260386bec6a256804fe0e"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::RecursiveQuery::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:2336`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Schema exposed to parent plans after reconciling the static and recursive terms.

<a id="op-7e498619a5d5e55f09901910"></a>
## static_term

`struct_field` · `datafusion_expr::logical_plan::plan::RecursiveQuery::static_term` · datafusion-expr 55.1.0

```rust
static_term: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:2328`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The static term (initial contents of the working table)

<a id="op-e206316effe37f65433f45bf"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::RecursiveQuery::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(name: String, static_term: Arc<LogicalPlan>, recursive_term: Arc<LogicalPlan>, is_distinct: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::RecursiveQuery", "path": "RecursiveQuery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2364, 1], "end": [2381, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2365`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
