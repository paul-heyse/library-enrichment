# `datafusion_functions_nested::make_array`

Crate `datafusion-functions-nested` · 5 public items · structured records in [`model/datafusion_functions_nested.make_array.json`](../model/datafusion_functions_nested.make_array.json)

## array_array

`function` · `datafusion_functions_nested::make_array::array_array`

```rust
fn array_array<O: OffsetSizeTrait>(args: &[arrow::array::ArrayRef], data_type: arrow::datatypes::DataType, field_name: &str) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Convert one or more [`ArrayRef`] of the same type into a
`ListArray` or 'LargeListArray' depending on the offset size.

# Example (non nested)

Calling `array(col1, col2)` where col1 and col2 are non nested
would return a single new `ListArray`, where each row was a list
of 2 elements:

```text
┌─────────┐   ┌─────────┐           ┌──────────────┐
│ ┌─────┐ │   │ ┌─────┐ │           │ ┌──────────┐ │
│ │  A  │ │   │ │  X  │ │           │ │  [A, X]  │ │
│ ├─────┤ │   │ ├─────┤ │           │ ├──────────┤ │
│ │NULL │ │   │ │  Y  │ │──────────▶│ │[NULL, Y] │ │
│ ├─────┤ │   │ ├─────┤ │           │ ├──────────┤ │
│ │  C  │ │   │ │  Z  │ │           │ │  [C, Z]  │ │
│ └─────┘ │   │ └─────┘ │           │ └──────────┘ │
└─────────┘   └─────────┘           └──────────────┘
  col1           col2                    output
```

# Example (nested)

Calling `array(col1, col2)` where col1 and col2 are lists
would return a single new `ListArray`, where each row was a list
of the corresponding elements of col1 and col2.

``` text
┌──────────────┐   ┌──────────────┐        ┌─────────────────────────────┐
│ ┌──────────┐ │   │ ┌──────────┐ │        │ ┌────────────────────────┐  │
│ │  [A, X]  │ │   │ │    []    │ │        │ │    [[A, X], []]        │  │
│ ├──────────┤ │   │ ├──────────┤ │        │ ├────────────────────────┤  │
│ │[NULL, Y] │ │   │ │[Q, R, S] │ │───────▶│ │ [[NULL, Y], [Q, R, S]] │  │
│ ├──────────┤ │   │ ├──────────┤ │        │ ├────────────────────────│  │
│ │  [C, Z]  │ │   │ │   NULL   │ │        │ │    [[C, Z], NULL]      │  │
│ └──────────┘ │   │ └──────────┘ │        │ └────────────────────────┘  │
└──────────────┘   └──────────────┘        └─────────────────────────────┘
     col1               col2                         output
```

---

## coerce_types_inner

`function` · `datafusion_functions_nested::make_array::coerce_types_inner`

```rust
fn coerce_types_inner(arg_types: &[arrow::datatypes::DataType], name: &str) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>>
```

---

## make_array

`function` · `datafusion_functions_nested::make_array::make_array`

Also reachable as `datafusion::prelude::make_array`, `datafusion_functions_nested::expr_fn::make_array`

```rust
fn make_array(arg: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Returns an Arrow array using the specified input expressions.

---

## make_array_udf

`function` · `datafusion_functions_nested::make_array::make_array_udf`

```rust
fn make_array_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MakeArray

---

## MakeArray

`struct` · `datafusion_functions_nested::make_array::MakeArray`

```rust
struct MakeArray
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
