# `buoyant_kernel::EvaluationHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.EvaluationHandler.json).

<a id="op-048603b8f393366d7f668758"></a>
## EvaluationHandler

`trait` · `buoyant_kernel::EvaluationHandler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait EvaluationHandler: AsAny
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L448).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:448`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Provides expression evaluation capability to Delta Kernel.

Delta Kernel can use this handler to evaluate a predicate on partition filters,
fill up partition column values, and any computation on data using Expressions.

<a id="op-670b511a89ac8af172812461"></a>
## create_many

`function` · `buoyant_kernel::EvaluationHandler::create_many` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create_many(&self, schema: SchemaRef, rows: &[&[Scalar]]) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L521).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:521`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a multi-row [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) by applying the given schema to multiple rows of values.

Each element in `rows` represents one row of data, where each row is a slice of structured
scalar values (one scalar per top-level field in the schema).

# Parameters

- `schema`: Schema describing the structure of each row.
- `rows`: Slice of rows, where each row contains one structured scalar per top-level schema
  field.

# Returns

A multi-row `EngineData` containing all rows.

# Errors

Returns an error if any row has a number of scalars that does not match the number of
top-level fields in `schema`, or if any scalar value cannot be appended to its corresponding
field's builder (e.g. due to a type mismatch).

# Example

For a schema with fields `[add: Struct, remove: Struct]`, each row should contain exactly 2
scalars: one for the `add` field and one for the `remove` field.

<a id="op-e8e8e0d5a3b7d6da4cccf072"></a>
## new_expression_evaluator

`function` · `buoyant_kernel::EvaluationHandler::new_expression_evaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_expression_evaluator(&self, input_schema: SchemaRef, expression: ExpressionRef, output_type: DataType) -> DeltaResult<Arc<dyn ExpressionEvaluator>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L466).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:466`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create an [`ExpressionEvaluator`](../operations/buoyant_kernel.ExpressionEvaluator.md#op-02eea996b6bdb19503c4315a) that can evaluate the given [`Expression`](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51)
on columnar batches with the given [`Schema`] to produce data of [`DataType`].

If the provided output type is a struct, its fields describe the columns of output produced
by the evaluator. Otherwise, the output schema is a single column named "output" of the
specified `output_type`. In all cases, the output schema is only used for its names (all
field names will be updated to match) and nullability (non-nullable columns can be converted
to nullable). Any mismatch in types (including number of columns) will produce an error.

# Parameters

- `input_schema`: Schema of the input data.
- `expression`: Expression to evaluate.
- `output_type`: Expected result data type.

[`Schema`]: crate::schema::StructType
[`DataType`]: crate::schema::DataType

<a id="op-056c41990aed483ac2c71ce9"></a>
## new_predicate_evaluator

`function` · `buoyant_kernel::EvaluationHandler::new_predicate_evaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_predicate_evaluator(&self, input_schema: SchemaRef, predicate: PredicateRef) -> DeltaResult<Arc<dyn PredicateEvaluator>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L484).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:484`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a [`PredicateEvaluator`](../operations/buoyant_kernel.PredicateEvaluator.md#op-a92b57d01e446154b97fd576) that can evaluate the given [`Predicate`](../operations/buoyant_kernel.expressions.Predicate.md#op-0c0ee21b4ebfcd851e64ceb4) on columnar
batches with the given [`Schema`] to produce a column of boolean results.

The output schema is a single nullable boolean column named "output".

# Parameters

- `input_schema`: Schema of the input data.
- `predicate`: Predicate to evaluate.

[`Schema`]: crate::schema::StructType

<a id="op-384633742b3f82e46ae42e0d"></a>
## null_row

`function` · `buoyant_kernel::EvaluationHandler::null_row` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn null_row(&self, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L494).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:494`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a single-row all-null-value [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) with the schema specified by
`output_schema`.
