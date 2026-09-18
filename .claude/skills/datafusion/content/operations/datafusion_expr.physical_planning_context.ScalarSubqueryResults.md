# `datafusion_expr::physical_planning_context::ScalarSubqueryResults`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.physical_planning_context.ScalarSubqueryResults.json).

<a id="op-6b0683e0d87a9cc813cb37e5"></a>
## ScalarSubqueryResults

`struct` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults` · datafusion-expr 55.1.0

```rust
struct ScalarSubqueryResults
```

Source: `src/physical_planning_context.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Shared results container for uncorrelated scalar subqueries.

Each entry corresponds to one scalar subquery, identified by its index.
Each slot is populated at execution time by `ScalarSubqueryExec`, read by
`ScalarSubqueryExpr` instances that share this container, and cleared when
the plan is reset for re-execution.

<a id="op-2402a9cc8e863215a5c7a7e7"></a>
## clear

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::clear` · datafusion-expr 55.1.0

```rust
fn clear(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [183, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Clears all populated results so the container can be reused.

<a id="op-42ccacc7d2f891b1ca9e4815"></a>
## clone

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ScalarSubqueryResults
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 10], "end": [132, 15], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/physical_planning_context.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7714fc71680d9e3c2eb8a1f9"></a>
## default

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::default` · datafusion-expr 55.1.0

```rust
fn default() -> ScalarSubqueryResults
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 17], "end": [132, 24], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/physical_planning_context.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51d185e2848902d7cf6ac23f"></a>
## eq

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 1], "end": [197, 2], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/physical_planning_context.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed3c14ea46773705e8f8a5eb"></a>
## fmt

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [191, 2], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_planning_context.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f2b7c7b7c31426afd603c97"></a>
## get

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::get` · datafusion-expr 55.1.0

```rust
fn get(&self, index: SubqueryIndex) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [183, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the scalar value stored at `index`, if it has been populated.

<a id="op-d512d21abbadbbb15382ed3c"></a>
## hash

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [205, 2], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/physical_planning_context.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-287e5477f521b1c5a30dea7c"></a>
## new

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::new` · datafusion-expr 55.1.0

```rust
fn new(n: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [183, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new shared results container with `n` empty slots.

<a id="op-9153df898d7e4a7aa05df1e5"></a>
## ptr_eq

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::ptr_eq` · datafusion-expr 55.1.0

```rust
fn ptr_eq(this: &Self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [183, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if `this` and `other` point to the same shared container.

<a id="op-47172394a7ce9a628923b34c"></a>
## set

`function` · `datafusion_expr::physical_planning_context::ScalarSubqueryResults::set` · datafusion-expr 55.1.0

```rust
fn set(&self, index: SubqueryIndex, value: ScalarValue) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::ScalarSubqueryResults", "path": "ScalarSubqueryResults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [183, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Stores `value` in the slot at `index`.
