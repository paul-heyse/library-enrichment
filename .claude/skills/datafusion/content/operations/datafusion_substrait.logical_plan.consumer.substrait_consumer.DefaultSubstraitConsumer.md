# `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.substrait_consumer.DefaultSubstraitConsumer.json).

<a id="op-80858c3dfe09cecf3e5d3dc0"></a>
## DefaultSubstraitConsumer

`struct` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer` · datafusion-substrait 55.1.0

```rust
struct DefaultSubstraitConsumer<'a>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Default SubstraitConsumer for converting standard Substrait without user-defined extensions.

Used as the consumer in [crate::logical_plan::consumer::from_substrait_plan](../operations/datafusion_substrait.logical_plan.consumer.plan.from_substrait_plan.md#op-4093698229f9655b9e4034f1)

<a id="op-85538df914557da9e2e9e179"></a>
## consume_extension_leaf

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::consume_extension_leaf` · datafusion-substrait 55.1.0

```rust
async fn consume_extension_leaf(&self, rel: &ExtensionLeafRel) -> datafusion::common::Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:614`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d21e43e2b23eaaf53f57f53"></a>
## consume_extension_multi

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::consume_extension_multi` · datafusion-substrait 55.1.0

```rust
async fn consume_extension_multi(&self, rel: &ExtensionMultiRel) -> datafusion::common::Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6396a3314046fc58e7447f9c"></a>
## consume_extension_single

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::consume_extension_single` · datafusion-substrait 55.1.0

```rust
async fn consume_extension_single(&self, rel: &ExtensionSingleRel) -> datafusion::common::Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:628`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96554e487587572ed100a180"></a>
## get_extensions

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::get_extensions` · datafusion-substrait 55.1.0

```rust
fn get_extensions(&self) -> &Extensions
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c4ea9636e1a096e02928717"></a>
## get_function_registry

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::get_function_registry` · datafusion-substrait 55.1.0

```rust
fn get_function_registry(&self) -> &impl FunctionRegistry
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-248b57017533a50227445835"></a>
## get_outer_schema

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::get_outer_schema` · datafusion-substrait 55.1.0

```rust
fn get_outer_schema(&self, steps_out: usize) -> Option<Arc<DFSchema>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:604`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd68dbbb423770770fc984b8"></a>
## lambda_variable

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::lambda_variable` · datafusion-substrait 55.1.0

```rust
fn lambda_variable(&self, steps_out: usize, field_idx: usize) -> datafusion::common::Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:682`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-513b88738ac6ab77c1a818c4"></a>
## new

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::new` · datafusion-substrait 55.1.0

```rust
fn new(extensions: &'a Extensions, state: &'a SessionState) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [574, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:566`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd1324e4b0c36367683278cf"></a>
## pop_lambda_parameters

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::pop_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn pop_lambda_parameters(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:678`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffa25fe43869e14881ff6dd3"></a>
## pop_outer_schema

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::pop_outer_schema` · datafusion-substrait 55.1.0

```rust
fn pop_outer_schema(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-283c67b05a9933b253162642"></a>
## push_lambda_parameters

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::push_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn push_lambda_parameters(&self, lambda_parameters: &[Type], input_schema: &DFSchema) -> datafusion::common::Result<Vec<String>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:669`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5985eaf005f163512b5e8de"></a>
## push_outer_schema

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::push_outer_schema` · datafusion-substrait 55.1.0

```rust
fn push_outer_schema(&self, schema: Arc<DFSchema>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:596`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa47a5aafedd1dfb834ed8f2"></a>
## resolve_table_ref

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer::resolve_table_ref` · datafusion-substrait 55.1.0

```rust
async fn resolve_table_ref(&self, table_ref: &TableReference) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer", "path": "DefaultSubstraitConsumer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [577, 1], "end": [689, 2], "filename": "src/logical_plan/consumer/substrait_consumer.rs"}, "trait": {"args": null, "id": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer", "path": "SubstraitConsumer"}, "trait_path": "datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer"}`

Source: `src/logical_plan/consumer/substrait_consumer.rs:578`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
