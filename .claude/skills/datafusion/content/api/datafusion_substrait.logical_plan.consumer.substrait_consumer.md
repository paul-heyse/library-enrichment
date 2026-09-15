# `datafusion_substrait::logical_plan::consumer::substrait_consumer`

Crate `datafusion-substrait` · 3 public items · structured records in [`model/datafusion_substrait.logical_plan.consumer.substrait_consumer.json`](../model/datafusion_substrait.logical_plan.consumer.substrait_consumer.json)

## DefaultSubstraitConsumer

`struct` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer`

```rust
struct DefaultSubstraitConsumer<'a>
```

**Implements**: `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer`

**Methods** (1)

```rust
fn new(extensions: &'a Extensions, state: &'a SessionState) -> Self
```

**via `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer`**

```rust
async fn consume_extension_leaf(&self, rel: &ExtensionLeafRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_extension_multi(&self, rel: &ExtensionMultiRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_extension_single(&self, rel: &ExtensionSingleRel) -> datafusion::common::Result<LogicalPlan>
fn get_extensions(&self) -> &Extensions
fn get_function_registry(&self) -> &impl FunctionRegistry
fn get_outer_schema(&self, steps_out: usize) -> Option<Arc<DFSchema>>
fn lambda_variable(&self, steps_out: usize, field_idx: usize) -> datafusion::common::Result<Expr>
fn pop_lambda_parameters(&self)
fn pop_outer_schema(&self)
fn push_lambda_parameters(&self, lambda_parameters: &[Type], input_schema: &DFSchema) -> datafusion::common::Result<Vec<String>>
fn push_outer_schema(&self, schema: Arc<DFSchema>)
async fn resolve_table_ref(&self, table_ref: &TableReference) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
```

Default SubstraitConsumer for converting standard Substrait without user-defined extensions.

Used as the consumer in [crate::logical_plan::consumer::from_substrait_plan]

---

## DefaultSubstraitLambdaConsumer

`struct` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitLambdaConsumer`

```rust
struct DefaultSubstraitLambdaConsumer
```

**Derives**: Default

**Methods** (4)

```rust
fn lambda_variable(&self, steps_out: usize, field_idx: usize) -> datafusion::common::Result<Expr>
fn new() -> Self
fn pop_lambda_parameters(&self)
fn push_lambda_parameters(&self, consumer: &impl SubstraitConsumer, lambda_parameters: &[Type], input_schema: &DFSchema) -> datafusion::common::Result<Vec<String>>
```

Default implementation of lambda related methods of the [SubstraitConsumer] trait

Can be embedded into a custom [SubstraitConsumer] to implement them

---

## SubstraitConsumer

`trait` · `datafusion_substrait::logical_plan::consumer::substrait_consumer::SubstraitConsumer`

```rust
trait SubstraitConsumer: Send + Sync + Sized
```

**Implementors** (1)

- `datafusion_substrait::logical_plan::consumer::substrait_consumer::DefaultSubstraitConsumer`

**Methods** (41)

```rust
async fn consume_aggregate(&self, rel: &AggregateRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_cast(&self, expr: &substrait_expression::Cast, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_consistent_partition_window(&self, _rel: &ConsistentPartitionWindowRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_cross(&self, rel: &CrossRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_dynamic_parameter(&self, expr: &DynamicParameter, _input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_enum(&self, _expr: &Enum, _input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_exchange(&self, rel: &ExchangeRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_expression(&self, expr: &Expression, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_extension_leaf(&self, rel: &ExtensionLeafRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_extension_multi(&self, rel: &ExtensionMultiRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_extension_single(&self, rel: &ExtensionSingleRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_fetch(&self, rel: &FetchRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_field_reference(&self, expr: &FieldReference, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_filter(&self, rel: &FilterRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_if_then(&self, expr: &IfThen, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_join(&self, rel: &JoinRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_lambda(&self, expr: &proto::expression::Lambda, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_literal(&self, expr: &Literal) -> datafusion::common::Result<Expr>
async fn consume_multi_or_list(&self, _expr: &MultiOrList, _input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_nested(&self, expr: &Nested, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_project(&self, rel: &ProjectRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_read(&self, rel: &ReadRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_rel(&self, rel: &Rel) -> datafusion::common::Result<LogicalPlan>
async fn consume_scalar_function(&self, expr: &ScalarFunction, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_set(&self, rel: &SetRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_singular_or_list(&self, expr: &SingularOrList, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_sort(&self, rel: &SortRel) -> datafusion::common::Result<LogicalPlan>
async fn consume_subquery(&self, expr: &substrait_expression::Subquery, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
async fn consume_switch(&self, _expr: &SwitchExpression, _input_schema: &DFSchema) -> datafusion::common::Result<Expr>
fn consume_user_defined_literal(&self, user_defined_literal: &proto::expression::literal::UserDefined) -> datafusion::common::Result<ScalarValue>
fn consume_user_defined_type(&self, user_defined_type: &type::UserDefined) -> datafusion::common::Result<DataType>
async fn consume_window_function(&self, expr: &WindowFunction, input_schema: &DFSchema) -> datafusion::common::Result<Expr>
fn get_extensions(&self) -> &Extensions
fn get_function_registry(&self) -> &impl FunctionRegistry
fn get_outer_schema(&self, _steps_out: usize) -> Option<Arc<DFSchema>>
fn lambda_variable(&self, _steps_out: usize, _field_idx: usize) -> datafusion::common::Result<Expr>
fn pop_lambda_parameters(&self)
fn pop_outer_schema(&self)
fn push_lambda_parameters(&self, _lambda_parameters: &[Type], _input_schema: &DFSchema) -> datafusion::common::Result<Vec<String>>
fn push_outer_schema(&self, _schema: Arc<DFSchema>)
async fn resolve_table_ref(&self, table_ref: &TableReference) -> datafusion::common::Result<Option<Arc<dyn TableProvider>>>
```

This trait is used to consume Substrait plans, converting them into DataFusion Logical Plans.
It can be implemented by users to allow for custom handling of relations, expressions, etc.

Combined with the [crate::logical_plan::producer::SubstraitProducer] this allows for fully
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

---
