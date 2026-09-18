# `datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.LogicalPlanBuilderOptions.json).

<a id="op-31e1018d882771257f67ed38"></a>
## LogicalPlanBuilderOptions

`struct` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions` · datafusion-expr 55.1.0

```rust
struct LogicalPlanBuilderOptions
```

Source: `src/logical_plan/builder.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Options for [`LogicalPlanBuilder`](../operations/datafusion_expr.logical_plan.builder.LogicalPlanBuilder.md#op-e319e4d14d504d0b1af00d77)

<a id="op-d5ee10dea11e6e236222b709"></a>
## clone

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> LogicalPlanBuilderOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions", "path": "LogicalPlanBuilderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 26], "end": [70, 31], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/builder.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26a91cee4bc5523a16659831"></a>
## default

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions::default` · datafusion-expr 55.1.0

```rust
fn default() -> LogicalPlanBuilderOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions", "path": "LogicalPlanBuilderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 10], "end": [70, 17], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/logical_plan/builder.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ead70ee6db4b982b701552de"></a>
## fmt

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions", "path": "LogicalPlanBuilderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 19], "end": [70, 24], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/builder.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6deb2eaa11ceb6e69beb90d1"></a>
## new

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions::new` · datafusion-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions", "path": "LogicalPlanBuilderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [87, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37efdd6669b9de1b439f54c9"></a>
## with_add_implicit_group_by_exprs

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions::with_add_implicit_group_by_exprs` · datafusion-expr 55.1.0

```rust
fn with_add_implicit_group_by_exprs(self, add: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions", "path": "LogicalPlanBuilderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [87, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Should the builder add functionally dependent expressions as additional aggregation groupings.
