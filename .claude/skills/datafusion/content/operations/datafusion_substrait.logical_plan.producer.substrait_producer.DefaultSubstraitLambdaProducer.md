# `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.substrait_producer.DefaultSubstraitLambdaProducer.json).

<a id="op-a60a5717c174750ac3417e6a"></a>
## DefaultSubstraitLambdaProducer

`struct` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer` · datafusion-substrait 55.1.0

```rust
struct DefaultSubstraitLambdaProducer
```

Source: `src/logical_plan/producer/substrait_producer.rs:596`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Default implementation of lambda related methods of the [SubstraitProducer](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.SubstraitProducer.md#op-aa195d4679f5143379239014) trait

Can be embedded into a custom [SubstraitProducer](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.SubstraitProducer.md#op-aa195d4679f5143379239014) to implement them

<a id="op-5326c06088603114bc36ff50"></a>
## default

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer::default` · datafusion-substrait 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer", "path": "DefaultSubstraitLambdaProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [604, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/logical_plan/producer/substrait_producer.rs:601`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91efd66000afb655c8e55128"></a>
## lambda_parameter_type

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer::lambda_parameter_type` · datafusion-substrait 55.1.0

```rust
fn lambda_parameter_type(&self, name: &str) -> datafusion::common::Result<substrait::proto::Type>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer", "path": "DefaultSubstraitLambdaProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [606, 1], "end": [652, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/producer/substrait_producer.rs:640`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30cd370dcce07a8d4800045d"></a>
## lambda_variable

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer::lambda_variable` · datafusion-substrait 55.1.0

```rust
fn lambda_variable(&self, name: &str) -> datafusion::common::Result<(u32, i32)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer", "path": "DefaultSubstraitLambdaProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [606, 1], "end": [652, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/producer/substrait_producer.rs:628`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-999ba1c9d2287860f7b23aab"></a>
## new

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer::new` · datafusion-substrait 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer", "path": "DefaultSubstraitLambdaProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [606, 1], "end": [652, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/producer/substrait_producer.rs:607`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78282accfde4f4a89897637f"></a>
## pop_lambda_parameters

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer::pop_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn pop_lambda_parameters(&mut self) -> datafusion::common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer", "path": "DefaultSubstraitLambdaProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [606, 1], "end": [652, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/producer/substrait_producer.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76cc0fb6b436d62bccba37b6"></a>
## push_lambda_parameters

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer::push_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn push_lambda_parameters(&mut self, lambda_parameters: HashMap<String, (usize, substrait::proto::Type)>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer", "path": "DefaultSubstraitLambdaProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [606, 1], "end": [652, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/producer/substrait_producer.rs:614`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Note you can construct the `lambda_parameters` argument using [lambda_parameters_map](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.lambda_parameters_map.md#op-95c1e42c0bf1a83c4d4dad54)
