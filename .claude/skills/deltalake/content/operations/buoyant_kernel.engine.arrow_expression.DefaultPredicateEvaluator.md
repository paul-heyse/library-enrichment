# `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.DefaultPredicateEvaluator.json).

<a id="op-6f43808aac70ce64710fb8ce"></a>
## DefaultPredicateEvaluator

`struct` · `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DefaultPredicateEvaluator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L394).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:394`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3d9ae375d63fe13fbcb56e5"></a>
## evaluate

`function` · `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator::evaluate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L400).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator", "path": "DefaultPredicateEvaluator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [399, 1], "end": [420, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::PredicateEvaluator", "path": "PredicateEvaluator"}, "trait_path": "buoyant_kernel::PredicateEvaluator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:400`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e66b3df652b79e0ce46e87aa"></a>
## fmt

`function` · `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L393).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator", "path": "DefaultPredicateEvaluator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 10], "end": [393, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:393`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cc93877ec1150283303318f"></a>
## _input_schema

`struct_field` · `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator::_input_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
_input_schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L395).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:395`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af8295d122a5af92197c7231"></a>
## predicate

`struct_field` · `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator::predicate` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: expressions::PredicateRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L396).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:396`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
