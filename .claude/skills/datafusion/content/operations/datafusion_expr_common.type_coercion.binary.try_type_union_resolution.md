# `datafusion_expr_common::type_coercion::binary::try_type_union_resolution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.try_type_union_resolution.json).

<a id="op-e278248c2bb217d5c8bd5dc0"></a>
## try_type_union_resolution

`function` · `datafusion_expr_common::type_coercion::binary::try_type_union_resolution` · datafusion-expr-common 55.1.0

```rust
fn try_type_union_resolution(data_types: &[arrow::datatypes::DataType]) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>>
```

Source: `src/type_coercion/binary.rs:815`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Handle type union resolution including struct type and others.
