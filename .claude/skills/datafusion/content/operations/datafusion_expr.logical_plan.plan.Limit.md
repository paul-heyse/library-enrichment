# `datafusion_expr::logical_plan::plan::Limit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Limit.json).

<a id="op-67e94f8f23b1855beec8a417"></a>
## Limit

`struct` · `datafusion_expr::logical_plan::plan::Limit` · datafusion-expr 55.1.0

```rust
struct Limit
```

Source: `src/logical_plan/plan.rs:3671`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Produces the first `n` tuples from its input and discards the rest.

<a id="op-027bb8991c532da2c96976f1"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Limit::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Limit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Limit", "path": "Limit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3670, 17], "end": [3670, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3670`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97544b3d8611d47f408ab32e"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Limit::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Limit) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Limit", "path": "Limit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3670, 24], "end": [3670, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3670`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a30a407603932ff98d4dd53"></a>
## fetch

`struct_field` · `datafusion_expr::logical_plan::plan::Limit::fetch` · datafusion-expr 55.1.0

```rust
fetch: Option<Box<Expr>>
```

Source: `src/logical_plan/plan.rs:3676`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Maximum number of rows to fetch,
None means fetching all rows

<a id="op-f1226b02be26135c8b642d19"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Limit::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Limit", "path": "Limit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3670, 10], "end": [3670, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3670`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-905d45521a1a85743eb3bca5"></a>
## get_fetch_type

`function` · `datafusion_expr::logical_plan::plan::Limit::get_fetch_type` · datafusion-expr 55.1.0

```rust
fn get_fetch_type(&self) -> Result<FetchType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Limit", "path": "Limit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3698, 1], "end": [3738, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3720`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get the fetch type from the limit plan.

<a id="op-4f1db01367fcae2e2f30d57f"></a>
## get_skip_type

`function` · `datafusion_expr::logical_plan::plan::Limit::get_skip_type` · datafusion-expr 55.1.0

```rust
fn get_skip_type(&self) -> Result<SkipType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Limit", "path": "Limit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3698, 1], "end": [3738, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3700`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get the skip type from the limit plan.

<a id="op-594fe5a20a892aeeb0b33ff7"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Limit::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Limit", "path": "Limit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3670, 51], "end": [3670, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3670`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d34a19859e603b192ce6060"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Limit::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:3678`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The logical plan

<a id="op-70195c3a002a8f58a7209f16"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Limit::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Limit) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Limit", "path": "Limit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3670, 39], "end": [3670, 49], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3670`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59bdb66bdbe97730860c7265"></a>
## skip

`struct_field` · `datafusion_expr::logical_plan::plan::Limit::skip` · datafusion-expr 55.1.0

```rust
skip: Option<Box<Expr>>
```

Source: `src/logical_plan/plan.rs:3673`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Number of rows to skip before fetch
