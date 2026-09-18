# `datafusion_physical_expr::higher_order_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.higher_order_function.json).

<a id="op-00f7b55c5c0b35ec1adacbee"></a>
## higher_order_function

`module` · `datafusion_physical_expr::higher_order_function` · datafusion-physical-expr 55.1.0

```rust
mod higher_order_function
```

Source: `src/higher_order_function.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Declaration of built-in (higher order) functions.
This module contains built-in functions' enumeration and metadata.

Generally, a function has:
* a signature
* a return type, that is a function of the incoming argument's types
* the computation, that must accept each valid signature

* Signature: see `Signature`
* Return type: a function `(arg_types) -> return_type`. E.g. for array_transform, ([[f32]], v -> v*2) -> [f32], ([[f32]], v -> v > 3.0) -> [bool].

This module also has a set of coercion rules to improve user experience: if an argument i32 is passed
to a function that supports f64, it is coerced to f64.

Unresolved upstream links (retained, not inferred): `f32`, `bool`.
