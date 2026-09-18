# `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.accumulator.StateFieldsArgs.json).

<a id="op-e614e74caed7b2cb33bc425c"></a>
## StateFieldsArgs

`struct` · `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs` · datafusion-functions-aggregate-common 55.1.0

```rust
struct StateFieldsArgs<'a>
```

Source: `src/accumulator.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

[`StateFieldsArgs`](../operations/datafusion_functions_aggregate_common.accumulator.StateFieldsArgs.md#op-e614e74caed7b2cb33bc425c) contains information about the fields that an
aggregate function's accumulator should have. Used for `AggregateUDFImpl::state_fields`.

<a id="op-d8ad21b769e2b378d5b9245e"></a>
## input_fields

`struct_field` · `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs::input_fields` · datafusion-functions-aggregate-common 55.1.0

```rust
input_fields: &'a [arrow::datatypes::FieldRef]
```

Source: `src/accumulator.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The input fields of the aggregate function.

<a id="op-d62d20124cb1d933aeb0843c"></a>
## is_distinct

`struct_field` · `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs::is_distinct` · datafusion-functions-aggregate-common 55.1.0

```rust
is_distinct: bool
```

Source: `src/accumulator.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Whether the aggregate function is distinct.

<a id="op-da464b712293c2d3362447ec"></a>
## name

`struct_field` · `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs::name` · datafusion-functions-aggregate-common 55.1.0

```rust
name: &'a str
```

Source: `src/accumulator.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The name of the aggregate function.

<a id="op-b5275ea9239884d8c28a96bc"></a>
## ordering_fields

`struct_field` · `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs::ordering_fields` · datafusion-functions-aggregate-common 55.1.0

```rust
ordering_fields: &'a [arrow::datatypes::FieldRef]
```

Source: `src/accumulator.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The ordering fields of the aggregate function.

<a id="op-842264518fc2796334e04b5e"></a>
## return_field

`struct_field` · `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs::return_field` · datafusion-functions-aggregate-common 55.1.0

```rust
return_field: arrow::datatypes::FieldRef
```

Source: `src/accumulator.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The return fields of the aggregate function.

<a id="op-4101155a5dd6be8e35df20e2"></a>
## return_type

`function` · `datafusion_functions_aggregate_common::accumulator::StateFieldsArgs::return_type` · datafusion-functions-aggregate-common 55.1.0

```rust
fn return_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::accumulator::StateFieldsArgs", "path": "StateFieldsArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [111, 2], "filename": "src/accumulator.rs"}, "trait": null, "trait_path": null}`

Source: `src/accumulator.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The return type of the aggregate function.
