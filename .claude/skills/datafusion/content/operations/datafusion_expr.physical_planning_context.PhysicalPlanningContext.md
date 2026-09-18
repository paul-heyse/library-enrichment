# `datafusion_expr::physical_planning_context::PhysicalPlanningContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.physical_planning_context.PhysicalPlanningContext.json).

<a id="op-6c42198b3422be2c46b9cf8b"></a>
## PhysicalPlanningContext

`struct` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext` · datafusion-expr 55.1.0

```rust
struct PhysicalPlanningContext
```

Source: `src/physical_planning_context.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Context used while converting a logical plan subtree into a physical plan.

Unlike [`ExecutionProps`](crate::execution_props::ExecutionProps), which
applies to the overall planning and execution of a query, this context can
differ between recursively planned subtrees. It currently carries:

* the state needed to create physical expressions for
  [`Expr::ScalarSubquery`] nodes that read from a shared
  [`ScalarSubqueryResults`](../operations/datafusion_expr.physical_planning_context.ScalarSubqueryResults.md#op-6b0683e0d87a9cc813cb37e5) container, and
* the qualifiers assigned to the [`Expr::LambdaVariable`]s that are in scope.

The physical planner builds this context from the set of uncorrelated scalar
subqueries it has scheduled for a subtree. It is then passed explicitly
through `create_physical_expr` so that function can find the slot index for
each [`Subquery`]. While planning the body of a lambda,
`create_physical_expr` extends the context with the lambda's parameters via
[`Self::with_qualified_lambda_variables`](../operations/datafusion_expr.physical_planning_context.PhysicalPlanningContext.md#op-076df30d35680178ef2bdde3).

An empty [`PhysicalPlanningContext`](../operations/datafusion_expr.physical_planning_context.PhysicalPlanningContext.md#op-6c42198b3422be2c46b9cf8b) (the [`Default`]) is what every
non-physical-planner caller passes; if such a caller encounters a scalar
subquery, `create_physical_expr` returns a `not_impl_err`.

[`Expr::ScalarSubquery`]: crate::Expr::ScalarSubquery
[`Expr::LambdaVariable`]: crate::Expr::LambdaVariable
[`Subquery`]: crate::logical_plan::Subquery

Unresolved upstream links (retained, not inferred): ``Default``.

<a id="op-794068f39581125589c4a998"></a>
## clone

`function` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> PhysicalPlanningContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::PhysicalPlanningContext", "path": "PhysicalPlanningContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 15], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/physical_planning_context.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f72267d685ac744af0b121e3"></a>
## default

`function` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext::default` · datafusion-expr 55.1.0

```rust
fn default() -> PhysicalPlanningContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::PhysicalPlanningContext", "path": "PhysicalPlanningContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 24], "end": [49, 31], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/physical_planning_context.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68982347f8a87fba061913e5"></a>
## fmt

`function` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::PhysicalPlanningContext", "path": "PhysicalPlanningContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 17], "end": [49, 22], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_planning_context.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70ee9ae1256d3087e6a16c2f"></a>
## index_of

`function` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext::index_of` · datafusion-expr 55.1.0

```rust
fn index_of(&self, subquery: &logical_plan::Subquery) -> Option<SubqueryIndex>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::PhysicalPlanningContext", "path": "PhysicalPlanningContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [108, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the slot index assigned to `subquery`, if any.

<a id="op-68e0cba00a39c8c6672ef574"></a>
## lambda_variable_qualifier

`function` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext::lambda_variable_qualifier` · datafusion-expr 55.1.0

```rust
fn lambda_variable_qualifier(&self, name: &str) -> Option<&TableReference>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::PhysicalPlanningContext", "path": "PhysicalPlanningContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [108, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the qualifier of the lambda variable `name`, if it is in scope.

<a id="op-ee24dc3ecd8e3b7681fa7278"></a>
## new

`function` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext::new` · datafusion-expr 55.1.0

```rust
fn new(indexes: HashMap<logical_plan::Subquery, SubqueryIndex>, results: ScalarSubqueryResults) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::PhysicalPlanningContext", "path": "PhysicalPlanningContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [108, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a [`PhysicalPlanningContext`](../operations/datafusion_expr.physical_planning_context.PhysicalPlanningContext.md#op-6c42198b3422be2c46b9cf8b) from an index map and a shared
results container. The index map must use the same indices as slots in
`results`.

<a id="op-38214bef2b192de00869c00e"></a>
## results

`function` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext::results` · datafusion-expr 55.1.0

```rust
fn results(&self) -> &ScalarSubqueryResults
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::PhysicalPlanningContext", "path": "PhysicalPlanningContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [108, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the shared results container.

<a id="op-076df30d35680178ef2bdde3"></a>
## with_qualified_lambda_variables

`function` · `datafusion_expr::physical_planning_context::PhysicalPlanningContext::with_qualified_lambda_variables` · datafusion-expr 55.1.0

```rust
fn with_qualified_lambda_variables(self, qualifier: &TableReference, variables: &[String]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::PhysicalPlanningContext", "path": "PhysicalPlanningContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [108, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Adds a mapping for each variable to the given qualifier. Existing
variables with conflicting names are shadowed.
