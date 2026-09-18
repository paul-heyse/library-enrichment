# `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.substrait_consumer.SubstraitConsumer.json).

<a id="op-017a43c1611ff850c42b3a6b"></a>
## SubstraitConsumer

`trait` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer` · datafusion-substrait 55.1.0

```rust
trait SubstraitConsumer: Send + Sync + Sized
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

This trait is used to consume Substrait plans, converting them into DataFusion Logical Plans.
It can be implemented by users to allow for custom handling of relations, expressions, etc.

Combined with the [crate::logical_plan::producer::SubstraitProducer](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.SubstraitProducer.md#op-aa195d4679f5143379239014) this allows for fully
customizable Substrait serde.

# Example Usage

```
# use async_trait::async_trait;
# use datafusion::catalog::TableProvider;
# use datafusion::common::{not_impl_err, substrait_err, DFSchema, ScalarValue, TableReference};
# use datafusion::error::Result;
# use datafusion::execution::{FunctionRegistry, SessionState};
# use datafusion::logical_expr::{Expr, LogicalPlan, LogicalPlanBuilder};
# use std::sync::Arc;
# use substrait::proto;
# use substrait::proto::{ExtensionLeafRel, FilterRel, ProjectRel, Type};
# use datafusion::arrow::datatypes::DataType;
# use datafusion::logical_expr::expr::ScalarFunction;
# use datafusion_substrait::extensions::Extensions;
# use datafusion_substrait::logical_plan::consumer::{
#     from_project_rel, from_substrait_rel, from_substrait_rex, SubstraitConsumer, DefaultSubstraitLambdaConsumer
# };

struct CustomSubstraitConsumer {
    extensions: Arc<Extensions>,
    state: Arc<SessionState>,
    // You can reuse existing consumer code related to lambdas
    lambda_consumer: DefaultSubstraitLambdaConsumer,
}

#[async_trait]
impl SubstraitConsumer for CustomSubstraitConsumer {
    async fn resolve_table_ref(
        &self,
        table_ref: &TableReference,
    ) -> Result<Option<Arc<dyn TableProvider>>> {
        let table = table_ref.table().to_string();
        let schema = self.state.schema_for_ref(table_ref.clone())?;
        let table_provider = schema.table(&table).await?;
        Ok(table_provider)
    }

    fn get_extensions(&self) -> &Extensions {
        self.extensions.as_ref()
    }

    fn get_function_registry(&self) -> &impl FunctionRegistry {
        self.state.as_ref()
    }

    fn push_lambda_parameters(
       &self,
       lambda_parameters: &[Type],
       input_schema: &DFSchema,
   ) -> datafusion::common::Result<Vec<String>> {
       self.lambda_consumer.push_lambda_parameters(
           self,
           lambda_parameters,
           input_schema,
       )
   }

    fn pop_lambda_parameters(&self) {
       self.lambda_consumer.pop_lambda_parameters();
   }

   fn lambda_variable(
       &self,
       steps_out: usize,
       field_idx: usize,
   ) -> datafusion::common::Result<Expr> {
       self.lambda_consumer.lambda_variable(steps_out, field_idx)
   }

    // You can reuse existing consumer code to assist in handling advanced extensions
    async fn consume_project(&self, rel: &ProjectRel) -> Result<LogicalPlan> {
        let df_plan = from_project_rel(self, rel).await?;
        if let Some(advanced_extension) = rel.advanced_extension.as_ref() {
            not_impl_err!(
                "decode and handle an advanced extension: {:?}",
                advanced_extension
            )
        } else {
            Ok(df_plan)
        }
    }

    // You can implement a fully custom consumer method if you need special handling
    async fn consume_filter(&self, rel: &FilterRel) -> Result<LogicalPlan> {
        let input = self.consume_rel(rel.input.as_ref().unwrap()).await?;
        let expression =
            self.consume_expression(rel.condition.as_ref().unwrap(), input.schema())
                .await?;
        // though this one is quite boring
        LogicalPlanBuilder::from(input).filter(expression)?.build()
    }

    // You can add handlers for extension relations
    async fn consume_extension_leaf(
        &self,
        rel: &ExtensionLeafRel,
    ) -> Result<LogicalPlan> {
        not_impl_err!(
            "handle protobuf Any {} as you need",
            rel.detail.as_ref().unwrap().type_url
        )
    }

    // and handlers for user-define types
    fn consume_user_defined_type(&self, typ: &proto::r#type::UserDefined) -> Result<DataType> {
        let type_string = self.extensions.types.get(&typ.type_reference).unwrap();
        match type_string.as_str() {
            "u!foo" => not_impl_err!("handle foo conversion"),
            "u!bar" => not_impl_err!("handle bar conversion"),
            _ => substrait_err!("unexpected type")
        }
    }

    // and user-defined literals
    fn consume_user_defined_literal(&self, literal: &proto::expression::literal::UserDefined) -> Result<ScalarValue> {
        // extract type_reference from the new TypeAnchorType oneof
        let type_ref = match literal.type_anchor_type {
            Some(proto::expression::literal::user_defined::TypeAnchorType::TypeReference(r)) => r,
            Some(proto::expression::literal::user_defined::TypeAnchorType::TypeAliasReference(_)) => {
                return not_impl_err!("Type alias references are not yet supported")
            }
            None => 0,
        };
        let type_string = self.extensions.types.get(&type_ref).unwrap();
        match type_string.as_str() {
            "u!foo" => not_impl_err!("handle foo conversion"),
            "u!bar" => not_impl_err!("handle bar conversion"),
            _ => substrait_err!("unexpected type")
        }
    }
}
```

<a id="op-59f429433988aee851106d45"></a>
## consume_aggregate

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_aggregate` · datafusion-substrait 55.1.0

```rust
async fn consume_aggregate(&self, rel: &AggregateRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd4d0762160362d2055a3361"></a>
## consume_cast

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_cast` · datafusion-substrait 55.1.0

```rust
async fn consume_cast(&self, expr: &substrait_expression::Cast, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d440dec25fe8d4e3548a45d"></a>
## consume_consistent_partition_window

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_consistent_partition_window` · datafusion-substrait 55.1.0

```rust
async fn consume_consistent_partition_window(&self, _rel: &ConsistentPartitionWindowRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1af1b413d99e2e38468b8c3"></a>
## consume_cross

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_cross` · datafusion-substrait 55.1.0

```rust
async fn consume_cross(&self, rel: &CrossRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0426366a400b0cdf75ff76c"></a>
## consume_dynamic_parameter

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_dynamic_parameter` · datafusion-substrait 55.1.0

```rust
async fn consume_dynamic_parameter(&self, expr: &DynamicParameter, _input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:399`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6c721a08f35d895f187d6a9"></a>
## consume_enum

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_enum` · datafusion-substrait 55.1.0

```rust
async fn consume_enum(&self, _expr: &Enum, _input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19476882afb4b6a171f8660b"></a>
## consume_exchange

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_exchange` · datafusion-substrait 55.1.0

```rust
async fn consume_exchange(&self, rel: &ExchangeRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4a3ace6f3660dd7cdcd63e7"></a>
## consume_expression

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_expression` · datafusion-substrait 55.1.0

```rust
async fn consume_expression(&self, expr: &Expression, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

All [Expression]s to be converted pass through this method.
You can provide your own implementation if you wish to customize the conversion behaviour.

Unresolved upstream links (retained, not inferred): `Expression`.

<a id="op-1f4926be1f009076ff273aff"></a>
## consume_extension_leaf

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_extension_leaf` · datafusion-substrait 55.1.0

```rust
async fn consume_extension_leaf(&self, rel: &ExtensionLeafRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4679e084352b53850a982c97"></a>
## consume_extension_multi

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_extension_multi` · datafusion-substrait 55.1.0

```rust
async fn consume_extension_multi(&self, rel: &ExtensionMultiRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae402fd31f12af4ebdcafe4a"></a>
## consume_extension_single

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_extension_single` · datafusion-substrait 55.1.0

```rust
async fn consume_extension_single(&self, rel: &ExtensionSingleRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e709b03675b230bc1f1de28"></a>
## consume_fetch

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_fetch` · datafusion-substrait 55.1.0

```rust
async fn consume_fetch(&self, rel: &FetchRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c1cde6aa843ad9868cbd569"></a>
## consume_field_reference

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_field_reference` · datafusion-substrait 55.1.0

```rust
async fn consume_field_reference(&self, expr: &FieldReference, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e570ef5dd4a28ffbaf119933"></a>
## consume_filter

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_filter` · datafusion-substrait 55.1.0

```rust
async fn consume_filter(&self, rel: &FilterRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94b6d2c866ac0efddbc2b00d"></a>
## consume_if_then

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_if_then` · datafusion-substrait 55.1.0

```rust
async fn consume_if_then(&self, expr: &IfThen, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-547c6804ad5f0710b3f50c32"></a>
## consume_join

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_join` · datafusion-substrait 55.1.0

```rust
async fn consume_join(&self, rel: &JoinRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-957838d91356a28c9bb647e2"></a>
## consume_lambda

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_lambda` · datafusion-substrait 55.1.0

```rust
async fn consume_lambda(&self, expr: &proto::expression::Lambda, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4810a96f2be8fb37c40b93a1"></a>
## consume_literal

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_literal` · datafusion-substrait 55.1.0

```rust
async fn consume_literal(&self, expr: &Literal) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3e9364fd22db08b22d79330"></a>
## consume_multi_or_list

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_multi_or_list` · datafusion-substrait 55.1.0

```rust
async fn consume_multi_or_list(&self, _expr: &MultiOrList, _input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2db808a769cece014084d3f"></a>
## consume_nested

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_nested` · datafusion-substrait 55.1.0

```rust
async fn consume_nested(&self, expr: &Nested, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e4d4b8904fbbf31df63602d"></a>
## consume_project

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_project` · datafusion-substrait 55.1.0

```rust
async fn consume_project(&self, rel: &ProjectRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0d333cf40eb1e34f9c21ab3"></a>
## consume_read

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_read` · datafusion-substrait 55.1.0

```rust
async fn consume_read(&self, rel: &ReadRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5018fca8538a2c9d1ff1c29"></a>
## consume_rel

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_rel` · datafusion-substrait 55.1.0

```rust
async fn consume_rel(&self, rel: &Rel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

All [Rel]s to be converted pass through this method.
You can provide your own implementation if you wish to customize the conversion behaviour.

Unresolved upstream links (retained, not inferred): `Rel`.

<a id="op-b794d9fbd0e42b1a14da79d2"></a>
## consume_scalar_function

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_scalar_function` · datafusion-substrait 55.1.0

```rust
async fn consume_scalar_function(&self, expr: &ScalarFunction, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3113c5646a22e7b4da2c25a"></a>
## consume_set

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_set` · datafusion-substrait 55.1.0

```rust
async fn consume_set(&self, rel: &SetRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9568db4cf63837d42027d9b3"></a>
## consume_singular_or_list

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_singular_or_list` · datafusion-substrait 55.1.0

```rust
async fn consume_singular_or_list(&self, expr: &SingularOrList, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:351`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bf6e7faab0833f08e629611"></a>
## consume_sort

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_sort` · datafusion-substrait 55.1.0

```rust
async fn consume_sort(&self, rel: &SortRel) -> datafusion::common::Result<LogicalPlan>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ee532eac2cc5260f2dcd4ec"></a>
## consume_subquery

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_subquery` · datafusion-substrait 55.1.0

```rust
async fn consume_subquery(&self, expr: &substrait_expression::Subquery, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-679f55a9345623941ffd4bd8"></a>
## consume_switch

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_switch` · datafusion-substrait 55.1.0

```rust
async fn consume_switch(&self, _expr: &SwitchExpression, _input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb666cf89e4cbccb17576c1c"></a>
## consume_user_defined_literal

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_user_defined_literal` · datafusion-substrait 55.1.0

```rust
fn consume_user_defined_literal(&self, user_defined_literal: &proto::expression::literal::UserDefined) -> datafusion::common::Result<ScalarValue>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:503`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a62cf5b9fdc5dd6ffdbc09fa"></a>
## consume_user_defined_type

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_user_defined_type` · datafusion-substrait 55.1.0

```rust
fn consume_user_defined_type(&self, user_defined_type: &type::UserDefined) -> datafusion::common::Result<DataType>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78ae52dc34647acc19046fe2"></a>
## consume_window_function

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::consume_window_function` · datafusion-substrait 55.1.0

```rust
async fn consume_window_function(&self, expr: &WindowFunction, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7c2d9c836b0b483ec6e52e5"></a>
## get_extensions

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::get_extensions` · datafusion-substrait 55.1.0

```rust
fn get_extensions(&self) -> &Extensions
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0514ce0a1e808d0a82859b5"></a>
## get_function_registry

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::get_function_registry` · datafusion-substrait 55.1.0

```rust
fn get_function_registry(&self) -> &impl FunctionRegistry
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e20148c947325aebb1274322"></a>
## get_outer_schema

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::get_outer_schema` · datafusion-substrait 55.1.0

```rust
fn get_outer_schema(&self, _steps_out: usize) -> Option<Arc<DFSchema>>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Get the outer schema at the given nesting depth.
`steps_out = 1` is the immediately enclosing query, `steps_out = 2`
is two levels out, etc. Returns `None` if `steps_out` is 0 or
exceeds the current nesting depth (the caller should treat this as
an error in the Substrait plan).

<a id="op-34cefcd31af92d4501264886"></a>
## lambda_variable

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::lambda_variable` · datafusion-substrait 55.1.0

```rust
fn lambda_variable(&self, _steps_out: usize, _field_idx: usize) -> datafusion::common::Result<Expr>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:546`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Returns an expression corresponding to the lambda variable with the given field_idx within the lambda it originates from,
at the lambda `step_outs` of the current scope

Note for custom implementations it's possible to embed a [DefaultSubstraitLambdaConsumer](../operations/datafusion_substrait.logical_plan.consumer.substrait_consumer.DefaultSubstraitLambdaConsumer.md#op-ebf3c1a81399bbb124a92130) and forward this method to it

<a id="op-1f4a004947e1726f9ad19a58"></a>
## pop_lambda_parameters

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::pop_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn pop_lambda_parameters(&self)
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:540`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Pop lambda parameters from the stack when leaving a lambda.

<a id="op-5751890bb0506f2379a03bb3"></a>
## pop_outer_schema

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::pop_outer_schema` · datafusion-substrait 55.1.0

```rust
fn pop_outer_schema(&self)
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Pop an outer schema from the stack when leaving a subquery.

<a id="op-8efc2e91e8e443881f05140e"></a>
## push_lambda_parameters

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::push_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn push_lambda_parameters(&self, _lambda_parameters: &[Type], _input_schema: &DFSchema) -> datafusion::common::Result<Vec<String>>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Push the given lambda parameters onto the stack when entering a lambda and
returns the names they got assigned

Note for custom implementations it's possible to embed a [DefaultSubstraitLambdaConsumer](../operations/datafusion_substrait.logical_plan.consumer.substrait_consumer.DefaultSubstraitLambdaConsumer.md#op-ebf3c1a81399bbb124a92130) and forward this method to it

<a id="op-66514c241dc51e9614f2cc4e"></a>
## push_outer_schema

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::push_outer_schema` · datafusion-substrait 55.1.0

```rust
fn push_outer_schema(&self, _schema: Arc<DFSchema>)
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Push an outer schema onto the stack when entering a subquery.

<a id="op-139c421fc8411cb8eff10a5d"></a>
## resolve_table_ref

`function` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer::resolve_table_ref` · datafusion-substrait 55.1.0

```rust
async fn resolve_table_ref(&self, table_ref: &TableReference) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
```

Source: `src/logical_plan/consumer/substrait_consumer.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
