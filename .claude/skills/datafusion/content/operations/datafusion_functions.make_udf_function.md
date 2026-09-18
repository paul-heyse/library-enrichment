# `datafusion_functions::make_udf_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.make_udf_function.json).

<a id="op-632a4f7d179e4a131a22e73e"></a>
## make_udf_function

`macro` · `datafusion_functions::make_udf_function` · datafusion-functions 55.1.0

```rust
macro_rules! make_udf_function
```

Source: `src/macros.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Creates a singleton `ScalarUDF` of the `$UDF` function and a function
named `$NAME` which returns that singleton. Optionally use a custom constructor
`$CTOR` which defaults to `$UDF::new()` if not specified.

This is used to ensure creating the list of `ScalarUDF` only happens once.
