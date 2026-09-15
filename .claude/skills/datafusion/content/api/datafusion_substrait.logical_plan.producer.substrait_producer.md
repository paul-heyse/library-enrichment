# `datafusion_substrait::logical_plan::producer::substrait_producer`

Crate `datafusion-substrait` · 4 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.substrait_producer.json`](../model/datafusion_substrait.logical_plan.producer.substrait_producer.json)

## lambda_parameters_map

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::lambda_parameters_map`

```rust
fn lambda_parameters_map(producer: &mut impl SubstraitProducer, lambda_parameters: Vec<datafusion::arrow::datatypes::FieldRef>) -> datafusion::common::Result<datafusion::common::HashMap<String, (usize, substrait::proto::Type)>>
```

Produces a map of lambda parameters as expected by [DefaultSubstraitLambdaProducer::push_lambda_parameters]

---

## DefaultSubstraitLambdaProducer

`struct` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitLambdaProducer`

```rust
struct DefaultSubstraitLambdaProducer
```

**Derives**: Default

**Methods** (5)

```rust
fn lambda_parameter_type(&self, name: &str) -> datafusion::common::Result<substrait::proto::Type>
fn lambda_variable(&self, name: &str) -> datafusion::common::Result<(u32, i32)>
fn new() -> Self
fn pop_lambda_parameters(&mut self) -> datafusion::common::Result<()>
fn push_lambda_parameters(&mut self, lambda_parameters: HashMap<String, (usize, substrait::proto::Type)>)
```

Default implementation of lambda related methods of the [SubstraitProducer] trait

Can be embedded into a custom [SubstraitProducer] to implement them

---

## DefaultSubstraitProducer

`struct` · `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer`

```rust
struct DefaultSubstraitProducer<'a>
```

**Implements**: `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer`

**Methods** (1)

```rust
fn new(state: &'a SessionState) -> Self
```

**via `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer`**

```rust
fn get_extensions(self) -> Extensions
fn handle_extension(&mut self, plan: &Extension) -> datafusion::common::Result<Box<Rel>>
fn lambda_parameter_type(&self, name: &str) -> datafusion::common::Result<substrait::proto::Type>
fn lambda_variable(&self, name: &str) -> datafusion::common::Result<(u32, i32)>
fn pop_lambda_parameters(&mut self) -> datafusion::common::Result<()>
fn push_lambda_parameters(&mut self, lambda_parameters: Vec<FieldRef>) -> datafusion::common::Result<()>
fn register_function(&mut self, fn_name: String) -> u32
fn register_type(&mut self, type_name: String) -> u32
```

---

## SubstraitProducer

`trait` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer`

```rust
trait SubstraitProducer: Send + Sync + Sized
```

**Implementors** (1)

- `datafusion_substrait::logical_plan::producer::substrait_producer::DefaultSubstraitProducer`

**Methods** (46)

```rust
fn get_extensions(self) -> Extensions
fn handle_aggregate(&mut self, plan: &Aggregate) -> datafusion::common::Result<Box<Rel>>
fn handle_aggregate_function(&mut self, agg_fn: &expr::AggregateFunction, schema: &DFSchemaRef) -> datafusion::common::Result<Measure>
fn handle_alias(&mut self, alias: &Alias, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_between(&mut self, between: &Between, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_binary_expr(&mut self, expr: &BinaryExpr, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_case(&mut self, case: &Case, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_cast(&mut self, cast: &Cast, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_column(&mut self, column: &Column, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_distinct(&mut self, plan: &Distinct) -> datafusion::common::Result<Box<Rel>>
fn handle_empty_relation(&mut self, plan: &EmptyRelation) -> datafusion::common::Result<Box<Rel>>
fn handle_exists(&mut self, exists: &Exists, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_expr(&mut self, expr: &Expr, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_extension(&mut self, _plan: &Extension) -> datafusion::common::Result<Box<Rel>>
fn handle_filter(&mut self, plan: &Filter) -> datafusion::common::Result<Box<Rel>>
fn handle_higher_order_function(&mut self, scalar_fn: &expr::HigherOrderFunction, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_in_list(&mut self, in_list: &InList, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_in_subquery(&mut self, in_subquery: &InSubquery, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_join(&mut self, plan: &Join) -> datafusion::common::Result<Box<Rel>>
fn handle_lambda(&mut self, lambda: &Lambda, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_lambda_variable(&mut self, lambda_variable: &LambdaVariable, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_like(&mut self, like: &Like, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_limit(&mut self, plan: &Limit) -> datafusion::common::Result<Box<Rel>>
fn handle_literal(&mut self, value: &ScalarValue) -> datafusion::common::Result<Expression>
fn handle_placeholder(&mut self, placeholder: &Placeholder, _schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_plan(&mut self, plan: &LogicalPlan) -> datafusion::common::Result<Box<Rel>>
fn handle_projection(&mut self, plan: &Projection) -> datafusion::common::Result<Box<Rel>>
fn handle_repartition(&mut self, plan: &Repartition) -> datafusion::common::Result<Box<Rel>>
fn handle_scalar_function(&mut self, scalar_fn: &expr::ScalarFunction, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_scalar_subquery(&mut self, subquery: &Subquery, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_set_comparison(&mut self, set_comparison: &SetComparison, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_sort(&mut self, plan: &Sort) -> datafusion::common::Result<Box<Rel>>
fn handle_subquery_alias(&mut self, plan: &SubqueryAlias) -> datafusion::common::Result<Box<Rel>>
fn handle_table_scan(&mut self, plan: &TableScan) -> datafusion::common::Result<Box<Rel>>
fn handle_try_cast(&mut self, cast: &TryCast, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_unary_expr(&mut self, expr: &Expr, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn handle_union(&mut self, plan: &Union) -> datafusion::common::Result<Box<Rel>>
fn handle_values(&mut self, plan: &Values) -> datafusion::common::Result<Box<Rel>>
fn handle_window(&mut self, plan: &Window) -> datafusion::common::Result<Box<Rel>>
fn handle_window_function(&mut self, window_fn: &WindowFunction, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
fn lambda_parameter_type(&self, _name: &str) -> datafusion::common::Result<substrait::proto::Type>
fn lambda_variable(&self, _name: &str) -> datafusion::common::Result<(u32, i32)>
fn pop_lambda_parameters(&mut self) -> datafusion::common::Result<()>
fn push_lambda_parameters(&mut self, _lambda_parameters: Vec<FieldRef>) -> datafusion::common::Result<()>
fn register_function(&mut self, signature: String) -> u32
fn register_type(&mut self, name: String) -> u32
```

This trait is used to produce Substrait plans, converting them from DataFusion Logical Plans.
It can be implemented by users to allow for custom handling of relations, expressions, etc.

Combined with the [crate::logical_plan::consumer::SubstraitConsumer] this allows for fully
customizable Substrait serde.

# Example Usage

```
# use std::sync::Arc;
# use substrait::proto::{Expression, Rel};
# use substrait::proto::rel::RelType;
# use datafusion::arrow::datatypes::FieldRef;
# use datafusion::common::DFSchemaRef;
# use datafusion::error::Result;
# use datafusion::execution::SessionState;
# use datafusion::logical_expr::{Between, Extension, Projection};
# use datafusion_substrait::extensions::Extensions;
# use datafusion_substrait::logical_plan::producer::{from_projection, SubstraitProducer, DefaultSubstraitLambdaProducer, lambda_parameters_map};

struct CustomSubstraitProducer {
    extensions: Extensions,
    state: Arc<SessionState>,
    // You can reuse existing producer code related to lambdas
    lambda_producer: DefaultSubstraitLambdaProducer,
}

impl SubstraitProducer for CustomSubstraitProducer {

    fn register_function(&mut self, signature: String) -> u32 {
       self.extensions.register_function(&signature)
    }

    fn register_type(&mut self, type_name: String) -> u32 {
        self.extensions.register_type(&type_name)
    }

    fn get_extensions(self) -> Extensions {
        self.extensions
    }

   fn push_lambda_parameters(
       &mut self,
       lambda_parameters: Vec<FieldRef>,
   ) -> datafusion::common::Result<()> {
       let lambda_parameters_map = lambda_parameters_map(self, lambda_parameters)?;

       self.lambda_producer
           .push_lambda_parameters(lambda_parameters_map);

       Ok(())
   }

   fn pop_lambda_parameters(&mut self) -> datafusion::common::Result<()> {
       self.lambda_producer.pop_lambda_parameters()
   }

   fn lambda_variable(&self, name: &str) -> datafusion::common::Result<(u32, i32)> {
       self.lambda_producer.lambda_variable(name)
   }

   fn lambda_parameter_type(
       &self,
       name: &str,
   ) -> datafusion::common::Result<substrait::proto::Type> {
       self.lambda_producer.lambda_parameter_type(name)
   }

    // You can set additional metadata on the Rels you produce
    fn handle_projection(&mut self, plan: &Projection) -> Result<Box<Rel>> {
        let mut rel = from_projection(self, plan)?;
        match rel.rel_type {
            Some(RelType::Project(mut project)) => {
                let mut project = project.clone();
                // set common metadata or advanced extension
                project.common = None;
                project.advanced_extension = None;
                Ok(Box::new(Rel {
                    rel_type: Some(RelType::Project(project)),
                }))
            }
            rel_type => Ok(Box::new(Rel { rel_type })),
       }
    }

    // You can tweak how you convert expressions for your target system
    fn handle_between(&mut self, between: &Between, schema: &DFSchemaRef) -> Result<Expression> {
       // add your own encoding for Between
       todo!()
   }

    // You can fully control how you convert UserDefinedLogicalNodes into Substrait
    fn handle_extension(&mut self, _plan: &Extension) -> Result<Box<Rel>> {
        // implement your own serializer into Substrait
       todo!()
   }
}
```

---
