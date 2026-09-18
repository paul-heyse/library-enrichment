# `datafusion_physical_plan::filter_pushdown::PushedDown`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter_pushdown.PushedDown.json).

<a id="op-4168cba55c6e8d62d2eccce0"></a>
## PushedDown

`enum` · `datafusion_physical_plan::filter_pushdown::PushedDown` · datafusion-physical-plan 55.1.0

```rust
enum PushedDown
```

Source: `src/filter_pushdown.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Discriminant for the result of pushing down a filter into a child node.

<a id="op-4975c9a7712b152e8ac21295"></a>
## No

`variant` · `datafusion_physical_plan::filter_pushdown::PushedDown::No` · datafusion-physical-plan 55.1.0

```rust
No
```

Source: `src/filter_pushdown.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The predicate could not be pushed down into the child node.

<a id="op-2df4d22e99c3ae0d8e851bc7"></a>
## Yes

`variant` · `datafusion_physical_plan::filter_pushdown::PushedDown::Yes` · datafusion-physical-plan 55.1.0

```rust
Yes
```

Source: `src/filter_pushdown.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The predicate was successfully pushed down into the child node.

<a id="op-b72da7b707c13b636cebe762"></a>
## and

`function` · `datafusion_physical_plan::filter_pushdown::PushedDown::and` · datafusion-physical-plan 55.1.0

```rust
fn and(self, other: PushedDown) -> PushedDown
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDown", "path": "PushedDown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [157, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Logical AND operation: returns `Yes` only if both operands are `Yes`.

<a id="op-2d928751509d3f258e27a6ac"></a>
## clone

`function` · `datafusion_physical_plan::filter_pushdown::PushedDown::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PushedDown
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDown", "path": "PushedDown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 17], "end": [125, 22], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/filter_pushdown.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bc79c44e42fff20c15556eb"></a>
## fmt

`function` · `datafusion_physical_plan::filter_pushdown::PushedDown::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDown", "path": "PushedDown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 10], "end": [125, 15], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter_pushdown.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61d61f9aa6ec08752b362ebf"></a>
## or

`function` · `datafusion_physical_plan::filter_pushdown::PushedDown::or` · datafusion-physical-plan 55.1.0

```rust
fn or(self, other: PushedDown) -> PushedDown
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDown", "path": "PushedDown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [157, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Logical OR operation: returns `Yes` if either operand is `Yes`.

<a id="op-11e9da01d136dd3dcd0b567c"></a>
## wrap_expression

`function` · `datafusion_physical_plan::filter_pushdown::PushedDown::wrap_expression` · datafusion-physical-plan 55.1.0

```rust
fn wrap_expression(self, expr: Arc<dyn PhysicalExpr>) -> PushedDownPredicate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDown", "path": "PushedDown"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [157, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Wrap a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) with this pushdown result.
