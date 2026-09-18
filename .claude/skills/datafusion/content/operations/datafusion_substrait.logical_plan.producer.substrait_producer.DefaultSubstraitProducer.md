# `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.substrait_producer.DefaultSubstraitProducer.json).

<a id="op-3baee8fd14c3820e5dc5c7f4"></a>
## DefaultSubstraitProducer

`struct` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer` · datafusion-substrait 55.1.0

```rust
struct DefaultSubstraitProducer<'a>
```

Source: `src/logical_plan/producer/substrait_producer.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65a49859c4f872a63459b13c"></a>
## get_extensions

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::get_extensions` · datafusion-substrait 55.1.0

```rust
fn get_extensions(self) -> Extensions
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [591, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer", "path": "SubstraitProducer"}, "trait_path": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer"}`

Source: `src/logical_plan/producer/substrait_producer.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26aa0aee1e14ad9dcc09bd6a"></a>
## handle_extension

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::handle_extension` · datafusion-substrait 55.1.0

```rust
fn handle_extension(&mut self, plan: &Extension) -> datafusion::common::Result<Box<Rel>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [591, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer", "path": "SubstraitProducer"}, "trait_path": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer"}`

Source: `src/logical_plan/producer/substrait_producer.rs:527`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71c90be7977e1ee8003614ad"></a>
## lambda_parameter_type

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::lambda_parameter_type` · datafusion-substrait 55.1.0

```rust
fn lambda_parameter_type(&self, name: &str) -> datafusion::common::Result<substrait::proto::Type>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [591, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer", "path": "SubstraitProducer"}, "trait_path": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer"}`

Source: `src/logical_plan/producer/substrait_producer.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-734d7bbce9cd167271dbfb46"></a>
## lambda_variable

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::lambda_variable` · datafusion-substrait 55.1.0

```rust
fn lambda_variable(&self, name: &str) -> datafusion::common::Result<(u32, i32)>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [591, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer", "path": "SubstraitProducer"}, "trait_path": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer"}`

Source: `src/logical_plan/producer/substrait_producer.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c9dbda8660e69f5a8e13531"></a>
## new

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::new` · datafusion-substrait 55.1.0

```rust
fn new(state: &'a SessionState) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 1], "end": [512, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/producer/substrait_producer.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-872f1e2cfc9b4563533453c2"></a>
## pop_lambda_parameters

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::pop_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn pop_lambda_parameters(&mut self) -> datafusion::common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [591, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer", "path": "SubstraitProducer"}, "trait_path": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer"}`

Source: `src/logical_plan/producer/substrait_producer.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72359098054204a2548b7871"></a>
## push_lambda_parameters

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::push_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn push_lambda_parameters(&mut self, lambda_parameters: Vec<FieldRef>) -> datafusion::common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [591, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer", "path": "SubstraitProducer"}, "trait_path": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer"}`

Source: `src/logical_plan/producer/substrait_producer.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47a32ae85bd437aa14ffd782"></a>
## register_function

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::register_function` · datafusion-substrait 55.1.0

```rust
fn register_function(&mut self, fn_name: String) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [591, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer", "path": "SubstraitProducer"}, "trait_path": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer"}`

Source: `src/logical_plan/producer/substrait_producer.rs:515`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7fece56b810299aeca9d957"></a>
## register_type

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer::register_type` · datafusion-substrait 55.1.0

```rust
fn register_type(&mut self, type_name: String) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer", "path": "DefaultSubstraitProducer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [514, 1], "end": [591, 2], "filename": "src/logical_plan/producer/substrait_producer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer", "path": "SubstraitProducer"}, "trait_path": "datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer"}`

Source: `src/logical_plan/producer/substrait_producer.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
