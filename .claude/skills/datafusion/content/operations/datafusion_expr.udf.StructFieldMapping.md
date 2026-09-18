# `datafusion_expr::udf::StructFieldMapping`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udf.StructFieldMapping.json).

<a id="op-6d089568e5d0f65c97aebb3e"></a>
## StructFieldMapping

`struct` · `datafusion_expr::udf::StructFieldMapping` · datafusion-expr 55.1.0

```rust
struct StructFieldMapping
```

Source: `src/udf.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Describes how a struct-producing UDF's output fields correspond to its
input arguments. This enables the optimizer to propagate orderings
through struct projections (e.g., so that sorting by a struct field
can be recognized as equivalent to sorting by the source column).

See [`ScalarUDFImpl::struct_field_mapping`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-046bec345ba5869e7f1b1c29) for details.

<a id="op-e9872710f16405a5dc3369dd"></a>
## field_accessor

`struct_field` · `datafusion_expr::udf::StructFieldMapping::field_accessor` · datafusion-expr 55.1.0

```rust
field_accessor: std::sync::Arc<ScalarUDF>
```

Source: `src/udf.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The UDF used to construct field access expressions on the output.
For example, the `get_field` UDF for accessing struct fields.

<a id="op-c01bc49b321360f3e57a0e15"></a>
## fields

`struct_field` · `datafusion_expr::udf::StructFieldMapping::fields` · datafusion-expr 55.1.0

```rust
fields: Vec<(Vec<datafusion_common::ScalarValue>, usize)>
```

Source: `src/udf.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

For each output field: the literal arguments to pass to the
`field_accessor` UDF (after the base expression), and the index
of the corresponding input argument that produces the field's value.

For `named_struct('a', col1, 'b', col2)`, this would be:
`[(["a"], 1), (["b"], 3)]` — field `"a"` comes from arg index 1.
