# `buoyant_kernel::transforms::expression::ExpressionDepthChecker`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transforms.expression.ExpressionDepthChecker.json).

<a id="op-66f1827732efc19ebf4497b4"></a>
## ExpressionDepthChecker

`struct` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ExpressionDepthChecker
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L427).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:427`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An expression "transform" that doesn't actually change the expression at all. Instead, it
measures the maximum depth of a expression, with a depth limit to prevent stack overflow. Useful
for verifying that a expression has reasonable depth before attempting to work with it.

<a id="op-89ce3602c3cd6272a451a1fc"></a>
## Output

`assoc_type` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(), Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L496).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:496`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b67df50fa0cac13fae01995a"></a>
## Residual

`assoc_type` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::Residual` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Residual = <<ExpressionDepthChecker as ExpressionTransform>::Output<()> as Carrier>::Residual
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L496).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:496`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b60f7291781f0f80d11d94af"></a>
## check_expr

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::check_expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn check_expr(expr: &Expression, depth_limit: usize) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L438).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 1], "end": [493, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:438`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Depth-checks the given expression against a given depth limit. The return value is the
largest depth seen, which is capped at one more than the depth limit (indicating the
recursion was terminated).

<a id="op-9f483b8fcad81b9be10574ec"></a>
## check_pred

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::check_pred` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn check_pred(pred: &Predicate, depth_limit: usize) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L445).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 1], "end": [493, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:445`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Depth-checks the given predicate against a given depth limit. The return value is the
largest depth seen, which is capped at one more than the depth limit (indicating the
recursion was terminated).

<a id="op-3437a5b05416b65bc81277c0"></a>
## transform_expr_binary

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_expr_binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_binary(&mut self, expr: &'a BinaryExpression) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L514).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:514`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5322485e7005f9f48af5eb48"></a>
## transform_expr_map_to_struct

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_expr_map_to_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_map_to_struct(&mut self, expr: &'a MapToStructExpression) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L534).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:534`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c878b430b0e8440f94ad632e"></a>
## transform_expr_opaque

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_expr_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_opaque(&mut self, expr: &'a OpaqueExpression) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L530).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:530`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b855bf55aa806676900ae25c"></a>
## transform_expr_pred

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_expr_pred` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_pred(&mut self, pred: &'a Predicate) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L502).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:502`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10479af1080452fdeeb0673e"></a>
## transform_expr_struct

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_expr_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_struct(&mut self, fields: &'a [ExpressionRef]) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L498).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:498`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14644426ef29bb6f07874097"></a>
## transform_pred_binary

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_pred_binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_binary(&mut self, pred: &'a BinaryPredicate) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L518).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:518`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18baac6c59759869f927a84f"></a>
## transform_pred_junction

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_pred_junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_junction(&mut self, pred: &'a JunctionPredicate) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L522).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:522`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7d10228ab39e82be2f281e8"></a>
## transform_pred_not

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_pred_not` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_not(&mut self, pred: &'a Predicate) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L506).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:506`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bc92f542bf83ba7ac2499e3"></a>
## transform_pred_opaque

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_pred_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_opaque(&mut self, pred: &'a OpaquePredicate) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L526).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:526`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b018d65795fb7261a2a35675"></a>
## transform_pred_unary

`function` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::transform_pred_unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_unary(&mut self, pred: &'a UnaryPredicate) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L510).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::expression::ExpressionDepthChecker", "path": "ExpressionDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 1], "end": [537, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::expression::ExpressionTransform", "path": "ExpressionTransform"}, "trait_path": "buoyant_kernel::transforms::expression::ExpressionTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:510`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7007af12d1a01de006afb512"></a>
## call_count

`struct_field` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::call_count` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
call_count: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L431).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:431`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b381200d9dd7f5c9764c131"></a>
## current_depth

`struct_field` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::current_depth` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
current_depth: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L430).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:430`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69a8e9f8ed161626d0c7398d"></a>
## depth_limit

`struct_field` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::depth_limit` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
depth_limit: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L428).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:428`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b1c376a8bbfb0d3424b3088"></a>
## max_depth_seen

`struct_field` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker::max_depth_seen` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_depth_seen: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L429).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:429`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
