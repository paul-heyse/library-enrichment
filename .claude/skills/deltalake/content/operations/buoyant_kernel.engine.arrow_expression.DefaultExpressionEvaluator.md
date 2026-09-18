# `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.DefaultExpressionEvaluator.json).

<a id="op-ad5991447301e816ad3823e7"></a>
## DefaultExpressionEvaluator

`struct` · `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DefaultExpressionEvaluator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L347).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:347`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7366d6612ac0be8befe651e"></a>
## evaluate

`function` · `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator::evaluate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L354).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator", "path": "DefaultExpressionEvaluator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [391, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::ExpressionEvaluator", "path": "ExpressionEvaluator"}, "trait_path": "buoyant_kernel::ExpressionEvaluator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:354`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68646bdf69f3333dc3fd6532"></a>
## fmt

`function` · `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L346).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator", "path": "DefaultExpressionEvaluator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 10], "end": [346, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:346`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-587f1eec29604f999a14bad4"></a>
## _input_schema

`struct_field` · `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator::_input_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
_input_schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L348).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:348`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34f102cb6e2644ef46620929"></a>
## expression

`struct_field` · `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator::expression` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
expression: expressions::ExpressionRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L349).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:349`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdd2a30785e80a10fc78d476"></a>
## output_type

`struct_field` · `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator::output_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
output_type: schema::DataType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L350).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:350`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
