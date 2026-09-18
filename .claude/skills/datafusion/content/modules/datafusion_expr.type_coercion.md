# `datafusion_expr::type_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.json).

<a id="op-e9f5403bd846ccf38493f864"></a>
## type_coercion

`module` · `datafusion_expr::type_coercion` · datafusion-expr 55.1.0

```rust
mod type_coercion
```

Source: `src/type_coercion/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Type coercion rules for DataFusion

Coercion is performed automatically by DataFusion when the types
of arguments passed to a function or needed by operators do not
exactly match the types required by that function / operator. In
this case, DataFusion will attempt to *coerce* the arguments to
types accepted by the function by inserting CAST operations.

CAST operations added by coercion are lossless and never discard
information.

For example coercion from i32 -> i64 might be
performed because all valid i32 values can be represented using an
i64. However, i64 -> i32 is never performed as there are i64
values which can not be represented by i32 values.
