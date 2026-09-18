# `buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.kernel_predicates.KernelPredicateEvaluatorDefaults.json).

<a id="op-d38a5feeac98a5d4b3d862f4"></a>
## KernelPredicateEvaluatorDefaults

`struct` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct KernelPredicateEvaluatorDefaults
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L506).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:506`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A collection of provided methods from the [`KernelPredicateEvaluator`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-1dc6ecdedb87f64e3b8474da) trait, factored out to
allow reuse by multiple bool-output predicate evaluator implementations.

<a id="op-9ed5ffb3e359a219a0a8c459"></a>
## eval_pred_binary_scalars

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults::eval_pred_binary_scalars` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_binary_scalars(op: BinaryPredicateOp, left: &Scalar, right: &Scalar, inverted: bool) -> Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L542).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults", "path": "KernelPredicateEvaluatorDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [507, 1], "end": [587, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:542`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Directly evaluates a boolean comparison. See
[`KernelPredicateEvaluator::eval_pred_binary_scalars`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-1da9dd1156dc513bb7d27b4a).

<a id="op-0233b8212d3226e537bc693f"></a>
## eval_pred_scalar

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults::eval_pred_scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_scalar(val: &Scalar, inverted: bool) -> Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L509).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults", "path": "KernelPredicateEvaluatorDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [507, 1], "end": [587, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:509`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Directly evaluates a boolean scalar. See [`KernelPredicateEvaluator::eval_pred_scalar`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-43819d3f9c8df9f0fc23a842).

<a id="op-4c9aeb6f68a239708304c811"></a>
## eval_pred_scalar_is_null

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults::eval_pred_scalar_is_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_scalar_is_null(val: &Scalar, inverted: bool) -> Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L517).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults", "path": "KernelPredicateEvaluatorDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [507, 1], "end": [587, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:517`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Directly null-tests a scalar. See [`KernelPredicateEvaluator::eval_pred_scalar_is_null`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-279e486aa8a990294c509fe9).

<a id="op-66c27eae46abf9e8f0b025b4"></a>
## finish_eval_pred_junction

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults::finish_eval_pred_junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn finish_eval_pred_junction(op: JunctionPredicateOp, preds: &mut dyn Iterator<Item = Option<bool>>, inverted: bool) -> Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L568).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults", "path": "KernelPredicateEvaluatorDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [507, 1], "end": [587, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:568`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Finishes evaluating a (possibly inverted) junction operation. See
[`KernelPredicateEvaluator::finish_eval_pred_junction`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-0ee991f832c056b15d36ce28).

The inputs were already inverted by the caller, if needed.

With AND (OR), any FALSE (TRUE) input dominates, forcing a FALSE (TRUE) output.  If there
was no dominating input, then any NULL input forces NULL output.  Otherwise, return the
non-dominant value. Inverting the operation also inverts the dominant value.

<a id="op-35b76f0c472c3a4ca420793d"></a>
## partial_cmp_scalars

`function` · `buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults::partial_cmp_scalars` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partial_cmp_scalars(ord: Ordering, a: &Scalar, b: &Scalar, inverted: bool) -> Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L529).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::kernel_predicates::KernelPredicateEvaluatorDefaults", "path": "KernelPredicateEvaluatorDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [507, 1], "end": [587, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:529`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly inverted) partial comparison of two scalars using SQL/logical semantics.

Returns `None` if the scalars are incomparable (different types, NULL values, or
unsupported types like Struct/Array/Map).

NOTE: This implements SQL NULL semantics where NULL is incomparable to everything,
including itself. For physical/structural comparison of query plans, use `==` on
`Scalar` directly (which provides physical equality).
