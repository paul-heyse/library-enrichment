# `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.opaque.ArrowOpaquePredicate.json).

<a id="op-2e2808aa43878d63267c766f"></a>
## ArrowOpaquePredicate

`trait` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ArrowOpaquePredicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L106).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:106`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extension trait for safely turning [`ArrowOpaquePredicateOp`](../operations/buoyant_kernel.engine.arrow_expression.opaque.ArrowOpaquePredicateOp.md#op-cc579bb2abb02a2a2faf79a6) into an opaque [`Predicate`](../operations/buoyant_kernel.expressions.Predicate.md#op-0c0ee21b4ebfcd851e64ceb4).

<a id="op-2904c4f777d6369bc6490ceb"></a>
## arrow_opaque

`function` · `buoyant_kernel::engine::arrow_expression::opaque::ArrowOpaquePredicate::arrow_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_opaque<T: ArrowOpaquePredicateOp>(op: T, exprs: impl IntoIterator<Item = Expression>) -> Predicate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/opaque.rs#L108).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/opaque.rs:108`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new opaque predicate. See also [`Predicate::opaque`](../operations/buoyant_kernel.expressions.Predicate.md#op-386d6810ec10b93244da1c09).
