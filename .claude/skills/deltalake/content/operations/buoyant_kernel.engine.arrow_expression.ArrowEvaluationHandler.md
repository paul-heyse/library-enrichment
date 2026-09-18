# `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.ArrowEvaluationHandler.json).

<a id="op-2ecde432ad3d3a1a7a4a2a96"></a>
## ArrowEvaluationHandler

`struct` · `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ArrowEvaluationHandler
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L251).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:251`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6287834f1ebe1cb626878356"></a>
## create_many

`function` · `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler::create_many` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create_many(&self, schema: SchemaRef, rows: &[&[Scalar]]) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L291).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler", "path": "ArrowEvaluationHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 1], "end": [344, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::EvaluationHandler", "path": "EvaluationHandler"}, "trait_path": "buoyant_kernel::EvaluationHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:291`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-247d5d9f93551621afff137a"></a>
## fmt

`function` · `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L250).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler", "path": "ArrowEvaluationHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 10], "end": [250, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:250`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c91dbb6b1e0da418f67153e"></a>
## new_expression_evaluator

`function` · `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler::new_expression_evaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_expression_evaluator(&self, schema: SchemaRef, expression: ExpressionRef, output_type: DataType) -> DeltaResult<Arc<dyn ExpressionEvaluator>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L254).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler", "path": "ArrowEvaluationHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 1], "end": [344, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::EvaluationHandler", "path": "EvaluationHandler"}, "trait_path": "buoyant_kernel::EvaluationHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:254`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e808d8d4491fab9c7653b25b"></a>
## new_predicate_evaluator

`function` · `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler::new_predicate_evaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_predicate_evaluator(&self, schema: SchemaRef, predicate: PredicateRef) -> DeltaResult<Arc<dyn PredicateEvaluator>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L267).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler", "path": "ArrowEvaluationHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 1], "end": [344, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::EvaluationHandler", "path": "EvaluationHandler"}, "trait_path": "buoyant_kernel::EvaluationHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:267`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f01ae69e13d0552e2d8d17e"></a>
## null_row

`function` · `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler::null_row` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn null_row(&self, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L281).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler", "path": "ArrowEvaluationHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 1], "end": [344, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::EvaluationHandler", "path": "EvaluationHandler"}, "trait_path": "buoyant_kernel::EvaluationHandler"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:281`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a single-row array with all-null leaf values. Note that if a nested struct is
included in the `output_type`, the entire struct will be NULL (instead of a not-null struct
with NULL fields).
