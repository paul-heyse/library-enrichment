# `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.substrait_producer.SubstraitProducer.json).

<a id="op-aa195d4679f5143379239014"></a>
## SubstraitProducer

`trait` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer` · datafusion-substrait 55.1.0

```rust
trait SubstraitProducer: Send + Sync + Sized
```

Source: `src/logical_plan/producer/substrait_producer.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

This trait is used to produce Substrait plans, converting them from DataFusion Logical Plans.
It can be implemented by users to allow for custom handling of relations, expressions, etc.

Combined with the [crate::logical_plan::consumer::SubstraitConsumer](../operations/datafusion_substrait.logical_plan.consumer.substrait_consumer.SubstraitConsumer.md#op-017a43c1611ff850c42b3a6b) this allows for fully
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

<a id="op-c864172c207f72e92202fdfc"></a>
## get_extensions

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::get_extensions` · datafusion-substrait 55.1.0

```rust
fn get_extensions(self) -> Extensions
```

Source: `src/logical_plan/producer/substrait_producer.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Consume the producer to generate the [Extensions](../operations/datafusion_substrait.extensions.Extensions.md#op-721f87201c35d0521d8e800c) for the Substrait plan based on the
functions that have been registered

<a id="op-d3b5e8a15aae6d48fa542801"></a>
## handle_aggregate

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_aggregate` · datafusion-substrait 55.1.0

```rust
fn handle_aggregate(&mut self, plan: &Aggregate) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07cd622672bd988bbc90524b"></a>
## handle_aggregate_function

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_aggregate_function` · datafusion-substrait 55.1.0

```rust
fn handle_aggregate_function(&mut self, agg_fn: &expr::AggregateFunction, schema: &DFSchemaRef) -> datafusion::common::Result<Measure>
```

Source: `src/logical_plan/producer/substrait_producer.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1011a926d007a2c2d7c2415d"></a>
## handle_alias

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_alias` · datafusion-substrait 55.1.0

```rust
fn handle_alias(&mut self, alias: &Alias, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0984fa665316bc19d04e0b96"></a>
## handle_between

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_between` · datafusion-substrait 55.1.0

```rust
fn handle_between(&mut self, between: &Between, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01a4e8536846915d486623bb"></a>
## handle_binary_expr

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_binary_expr` · datafusion-substrait 55.1.0

```rust
fn handle_binary_expr(&mut self, expr: &BinaryExpr, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aba694fc4a822c38cde91384"></a>
## handle_case

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_case` · datafusion-substrait 55.1.0

```rust
fn handle_case(&mut self, case: &Case, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c73c709c747a67ea0a7ef96"></a>
## handle_cast

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_cast` · datafusion-substrait 55.1.0

```rust
fn handle_cast(&mut self, cast: &Cast, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42b8a642a18b9ba430838110"></a>
## handle_column

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_column` · datafusion-substrait 55.1.0

```rust
fn handle_column(&mut self, column: &Column, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddc4d11cd38b1867c1611adf"></a>
## handle_distinct

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_distinct` · datafusion-substrait 55.1.0

```rust
fn handle_distinct(&mut self, plan: &Distinct) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f7df699db0fd80687e89b9e"></a>
## handle_empty_relation

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_empty_relation` · datafusion-substrait 55.1.0

```rust
fn handle_empty_relation(&mut self, plan: &EmptyRelation) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcd21ffdb2f8acff9b74f316"></a>
## handle_exists

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_exists` · datafusion-substrait 55.1.0

```rust
fn handle_exists(&mut self, exists: &Exists, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7066ccb0d283b03f5f8c0c7d"></a>
## handle_expr

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_expr` · datafusion-substrait 55.1.0

```rust
fn handle_expr(&mut self, expr: &Expr, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7d5b1aac451b9bef8a7617b"></a>
## handle_extension

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_extension` · datafusion-substrait 55.1.0

```rust
fn handle_extension(&mut self, _plan: &Extension) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-077c2863a26eab3bc79289d6"></a>
## handle_filter

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_filter` · datafusion-substrait 55.1.0

```rust
fn handle_filter(&mut self, plan: &Filter) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14ce513f390a2c3a33c72351"></a>
## handle_higher_order_function

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_higher_order_function` · datafusion-substrait 55.1.0

```rust
fn handle_higher_order_function(&mut self, scalar_fn: &expr::HigherOrderFunction, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8381c551445a320e52f9383"></a>
## handle_in_list

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_in_list` · datafusion-substrait 55.1.0

```rust
fn handle_in_list(&mut self, in_list: &InList, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e4e5c99ccbcdf5a076089dc"></a>
## handle_in_subquery

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_in_subquery` · datafusion-substrait 55.1.0

```rust
fn handle_in_subquery(&mut self, in_subquery: &InSubquery, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:405`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ab79dede5b66a2917aebdc9"></a>
## handle_join

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_join` · datafusion-substrait 55.1.0

```rust
fn handle_join(&mut self, plan: &Join) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ada6020e09dc62da0688cef3"></a>
## handle_lambda

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_lambda` · datafusion-substrait 55.1.0

```rust
fn handle_lambda(&mut self, lambda: &Lambda, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:444`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41c90252ea80f337a6f89b5a"></a>
## handle_lambda_variable

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_lambda_variable` · datafusion-substrait 55.1.0

```rust
fn handle_lambda_variable(&mut self, lambda_variable: &LambdaVariable, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fad986ad2fcd4fde9370bb8"></a>
## handle_like

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_like` · datafusion-substrait 55.1.0

```rust
fn handle_like(&mut self, like: &Like, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98efb258d9f36208486bb623"></a>
## handle_limit

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_limit` · datafusion-substrait 55.1.0

```rust
fn handle_limit(&mut self, plan: &Limit) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5a2ba91721b0fb1b45a6238"></a>
## handle_literal

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_literal` · datafusion-substrait 55.1.0

```rust
fn handle_literal(&mut self, value: &ScalarValue) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7220fff4cd9ebe4f9105ff5f"></a>
## handle_placeholder

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_placeholder` · datafusion-substrait 55.1.0

```rust
fn handle_placeholder(&mut self, placeholder: &Placeholder, _schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c17f4263e0967a5ef4b7bc4"></a>
## handle_plan

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_plan` · datafusion-substrait 55.1.0

```rust
fn handle_plan(&mut self, plan: &LogicalPlan) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5327b3978ce843134bf0072"></a>
## handle_projection

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_projection` · datafusion-substrait 55.1.0

```rust
fn handle_projection(&mut self, plan: &Projection) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ab60acdc19f28a7d357dc1e"></a>
## handle_repartition

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_repartition` · datafusion-substrait 55.1.0

```rust
fn handle_repartition(&mut self, plan: &Repartition) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1c40e2a0f23086207d5a541"></a>
## handle_scalar_function

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_scalar_function` · datafusion-substrait 55.1.0

```rust
fn handle_scalar_function(&mut self, scalar_fn: &expr::ScalarFunction, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d036989be7a6239f340faedb"></a>
## handle_scalar_subquery

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_scalar_subquery` · datafusion-substrait 55.1.0

```rust
fn handle_scalar_subquery(&mut self, subquery: &Subquery, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8e83374f0193f9baa0033d1"></a>
## handle_set_comparison

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_set_comparison` · datafusion-substrait 55.1.0

```rust
fn handle_set_comparison(&mut self, set_comparison: &SetComparison, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-684dfd7c79ff116a7b7b9df7"></a>
## handle_sort

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_sort` · datafusion-substrait 55.1.0

```rust
fn handle_sort(&mut self, plan: &Sort) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ae0f289d8037fe0f891d23b"></a>
## handle_subquery_alias

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_subquery_alias` · datafusion-substrait 55.1.0

```rust
fn handle_subquery_alias(&mut self, plan: &SubqueryAlias) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d862baef725b2bf1eb7461b"></a>
## handle_table_scan

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_table_scan` · datafusion-substrait 55.1.0

```rust
fn handle_table_scan(&mut self, plan: &TableScan) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d12e6cd49e93712beea5ced"></a>
## handle_try_cast

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_try_cast` · datafusion-substrait 55.1.0

```rust
fn handle_try_cast(&mut self, cast: &TryCast, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01b7467419549e43796ab3bb"></a>
## handle_unary_expr

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_unary_expr` · datafusion-substrait 55.1.0

```rust
fn handle_unary_expr(&mut self, expr: &Expr, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

For handling Not, IsNotNull, IsNull, IsTrue, IsFalse, IsUnknown, IsNotTrue, IsNotFalse, IsNotUnknown, Negative

<a id="op-815fd99661a9d5a9cd71d888"></a>
## handle_union

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_union` · datafusion-substrait 55.1.0

```rust
fn handle_union(&mut self, plan: &Union) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c87bcf33bed545a4bfc44fa"></a>
## handle_values

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_values` · datafusion-substrait 55.1.0

```rust
fn handle_values(&mut self, plan: &Values) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31a0b00a54c7a07a1320196c"></a>
## handle_window

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_window` · datafusion-substrait 55.1.0

```rust
fn handle_window(&mut self, plan: &Window) -> datafusion::common::Result<Box<Rel>>
```

Source: `src/logical_plan/producer/substrait_producer.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7d7a1cc9a0093822273e6c5"></a>
## handle_window_function

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::handle_window_function` · datafusion-substrait 55.1.0

```rust
fn handle_window_function(&mut self, window_fn: &WindowFunction, schema: &DFSchemaRef) -> datafusion::common::Result<Expression>
```

Source: `src/logical_plan/producer/substrait_producer.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-930371af25cb3cd2cd6e9e8e"></a>
## lambda_parameter_type

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::lambda_parameter_type` · datafusion-substrait 55.1.0

```rust
fn lambda_parameter_type(&self, _name: &str) -> datafusion::common::Result<substrait::proto::Type>
```

Source: `src/logical_plan/producer/substrait_producer.rs:490`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Get the type of the lambda parameter with the given `name`

Note for custom implementations it's possible to embed a [DefaultSubstraitLambdaProducer](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.DefaultSubstraitLambdaProducer.md#op-a60a5717c174750ac3417e6a) and forward this method to it

<a id="op-ae876a7e8dd8d78bc948ade1"></a>
## lambda_variable

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::lambda_variable` · datafusion-substrait 55.1.0

```rust
fn lambda_variable(&self, _name: &str) -> datafusion::common::Result<(u32, i32)>
```

Source: `src/logical_plan/producer/substrait_producer.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Get the (`steps_out`, `field_idx`) of the lambda variable with the given `name`. `steps_out` refers to the number
of lambda boundaries to traverse (0 = current lambda), and `field_idx` refers to the index within the lambda parameters

Note for custom implementations it's possible to embed a [DefaultSubstraitLambdaProducer](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.DefaultSubstraitLambdaProducer.md#op-a60a5717c174750ac3417e6a) and forward this method to it

<a id="op-7d43f3039db1d5fc6928c2cb"></a>
## pop_lambda_parameters

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::pop_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn pop_lambda_parameters(&mut self) -> datafusion::common::Result<()>
```

Source: `src/logical_plan/producer/substrait_producer.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Pop the last pushed `lambda_parameters` so that it unshadow any previously shadowed lambda parameter

Note for custom implementations it's possible to embed a [DefaultSubstraitLambdaProducer](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.DefaultSubstraitLambdaProducer.md#op-a60a5717c174750ac3417e6a) and forward this method to it

<a id="op-1aac9dfb4e778acf0389b5f1"></a>
## push_lambda_parameters

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::push_lambda_parameters` · datafusion-substrait 55.1.0

```rust
fn push_lambda_parameters(&mut self, _lambda_parameters: Vec<FieldRef>) -> datafusion::common::Result<()>
```

Source: `src/logical_plan/producer/substrait_producer.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Push the given `lambda_parameters` into this producer so they can be referenced by lambda variables

Note for custom implementations it's possible to embed a [DefaultSubstraitLambdaProducer](../operations/datafusion_substrait.logical_plan.producer.substrait_producer.DefaultSubstraitLambdaProducer.md#op-a60a5717c174750ac3417e6a) and forward this method to it

<a id="op-653a48fd4904801952252ab4"></a>
## register_function

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::register_function` · datafusion-substrait 55.1.0

```rust
fn register_function(&mut self, signature: String) -> u32
```

Source: `src/logical_plan/producer/substrait_producer.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Within a Substrait plan, functions are referenced using function anchors that are stored at
the top level of the [Plan](substrait::proto::Plan) within
[ExtensionFunction](substrait::proto::extensions::simple_extension_declaration::ExtensionFunction)
messages.

When given a function signature, this method should return the existing anchor for it if
there is one. Otherwise, it should generate a new anchor.

Unresolved upstream links (retained, not inferred): `substrait::proto::extensions::simple_extension_declaration::ExtensionFunction`, `substrait::proto::Plan`.

<a id="op-903c49344e485a5fa9aacaa8"></a>
## register_type

`function` · `datafusion_substrait::logical_plan::producer::substrait_producer::SubstraitProducer::register_type` · datafusion-substrait 55.1.0

```rust
fn register_type(&mut self, name: String) -> u32
```

Source: `src/logical_plan/producer/substrait_producer.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Within a Substrait plan, user defined types are referenced using type anchors that are stored at
the top level of the [Plan](substrait::proto::Plan) within
[ExtensionType](substrait::proto::extensions::simple_extension_declaration::ExtensionType)
messages.

When given a type name, this method should return the existing anchor for it if
there is one. Otherwise, it should generate a new anchor.

Unresolved upstream links (retained, not inferred): `substrait::proto::extensions::simple_extension_declaration::ExtensionType`, `substrait::proto::Plan`.
