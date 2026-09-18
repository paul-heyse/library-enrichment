# `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.substrait_consumer.DefaultSubstraitLambdaConsumer.json).

<a id="op-ebf3c1a81399bbb124a92130"></a>
## DefaultSubstraitLambdaConsumer

`struct` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer` · datafusion-substrait 55.1.0

```rust
struct DefaultSubstraitLambdaConsumer
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:694`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Default implementation of lambda related methods of the [SubstraitConsumer](../operations/datafusion_substrait.logical_plan.consumer.substrait_consumer.SubstraitConsumer.md#op-017a43c1611ff850c42b3a6b) trait

Can be embedded into a custom [SubstraitConsumer](../operations/datafusion_substrait.logical_plan.consumer.substrait_consumer.SubstraitConsumer.md#op-017a43c1611ff850c42b3a6b) to implement them

<a id="op-06d635476b3bf67fbf4d0623"></a>
## default

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer::default` · datafusion-substrait 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer", "path": "DefaultSubstraitLambdaConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [707, 1], "end": [711, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:708`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af176319ea7f5c2a9232fe28"></a>
## lambda_variable

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer::lambda_variable` · datafusion-substrait 55.1.0

```rust
fn lambda_variable(&self, steps_out: usize, field_idx: usize) -> datafusion::common::Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer", "path": "DefaultSubstraitLambdaConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [713, 1], "end": [781, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:755`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6428b1f8029833892036bee2"></a>
## new

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer::new` · datafusion-substrait 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer", "path": "DefaultSubstraitLambdaConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [713, 1], "end": [781, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:714`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3ae1ffd7fca89efb4fbfb28"></a>
## pop_lambda_parameters

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer::pop_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn pop_lambda_parameters(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer", "path": "DefaultSubstraitLambdaConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [713, 1], "end": [781, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:751`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd933b0d35f44e4e14a5eaf0"></a>
## push_lambda_parameters

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer::push_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn push_lambda_parameters(&self, consumer: &impl SubstraitConsumer, lambda_parameters: &[Type], input_schema: &DFSchema) -> datafusion::common::Result<Vec<String>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer", "path": "DefaultSubstraitLambdaConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [713, 1], "end": [781, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
