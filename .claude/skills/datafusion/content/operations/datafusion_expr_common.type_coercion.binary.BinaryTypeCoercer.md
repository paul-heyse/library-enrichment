# `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.BinaryTypeCoercer.json).

<a id="op-9d4ecae6c1446253b55ee223"></a>
## BinaryTypeCoercer

`struct` · `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer` · datafusion-expr-common 55.1.0

```rust
struct BinaryTypeCoercer<'a>
```

Source: `src/type_coercion/binary.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Provides type information about a binary expression, coercing different
input types into a sensible output type.

<a id="op-e67b06832b025bd95dbc39e8"></a>
## get_input_types

`function` · `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer::get_input_types` · datafusion-expr-common 55.1.0

```rust
fn get_input_types(&'a self) -> Result<(DataType, DataType)>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer", "path": "BinaryTypeCoercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [369, 2], "filename": "src/type_coercion/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/type_coercion/binary.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns the coerced input types for a binary expression evaluating the `op` with the left and right hand types

<a id="op-1b4e51d0cfa5ce951eaf3f66"></a>
## get_result_type

`function` · `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer::get_result_type` · datafusion-expr-common 55.1.0

```rust
fn get_result_type(&'a self) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer", "path": "BinaryTypeCoercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [369, 2], "filename": "src/type_coercion/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/type_coercion/binary.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns the resulting type of a binary expression evaluating the `op` with the left and right hand types

<a id="op-b35236896d14923a4de8bf75"></a>
## new

`function` · `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer::new` · datafusion-expr-common 55.1.0

```rust
fn new(lhs: &'a DataType, op: &'a Operator, rhs: &'a DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer", "path": "BinaryTypeCoercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [369, 2], "filename": "src/type_coercion/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/type_coercion/binary.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Creates a new [`BinaryTypeCoercer`](../operations/datafusion_expr_common.type_coercion.binary.BinaryTypeCoercer.md#op-9d4ecae6c1446253b55ee223), for reasoning about the input
and output types of a binary expression.

<a id="op-84d3c4537f167af146a9c89d"></a>
## set_lhs_spans

`function` · `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer::set_lhs_spans` · datafusion-expr-common 55.1.0

```rust
fn set_lhs_spans(&mut self, spans: Spans)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer", "path": "BinaryTypeCoercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [369, 2], "filename": "src/type_coercion/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/type_coercion/binary.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Sets the spans information for the left side of the binary expression,
so better diagnostics can be provided in case of errors.

<a id="op-463770c87ad24174438a49c9"></a>
## set_op_spans

`function` · `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer::set_op_spans` · datafusion-expr-common 55.1.0

```rust
fn set_op_spans(&mut self, spans: Spans)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer", "path": "BinaryTypeCoercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [369, 2], "filename": "src/type_coercion/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/type_coercion/binary.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Sets the spans information for the operator of the binary expression, so
better diagnostics can be provided in case of errors.

<a id="op-68bdec206b454db8cbeba065"></a>
## set_rhs_spans

`function` · `datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer::set_rhs_spans` · datafusion-expr-common 55.1.0

```rust
fn set_rhs_spans(&mut self, spans: Spans)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr_common::type_coercion::binary::BinaryTypeCoercer", "path": "BinaryTypeCoercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [369, 2], "filename": "src/type_coercion/binary.rs"}, "trait": null, "trait_path": null}`

Source: `src/type_coercion/binary.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Sets the spans information for the right side of the binary expression,
so better diagnostics can be provided in case of errors.
