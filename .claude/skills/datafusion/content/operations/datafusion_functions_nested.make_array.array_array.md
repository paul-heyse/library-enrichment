# `datafusion_functions_nested::make_array::array_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.make_array.array_array.json).

<a id="op-e64a5653bcd7a153cec263d7"></a>
## array_array

`function` · `datafusion_functions_nested::make_array::array_array` · datafusion-functions-nested 55.1.0

```rust
fn array_array<O: OffsetSizeTrait>(args: &[arrow::array::ArrayRef], data_type: arrow::datatypes::DataType, field_name: &str) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/make_array.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

Convert one or more [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) of the same type into a
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
