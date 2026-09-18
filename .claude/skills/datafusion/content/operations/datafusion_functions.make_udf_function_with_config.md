# `datafusion_functions::make_udf_function_with_config`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.make_udf_function_with_config.json).

<a id="op-231ad436fb1b187f1db3cff3"></a>
## make_udf_function_with_config

`macro` · `datafusion_functions::make_udf_function_with_config` · datafusion-functions 55.1.0

```rust
macro_rules! make_udf_function_with_config
```

Source: `src/macros.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Creates a singleton `ScalarUDF` of the `$UDF` function and a function
named `$NAME` which returns that singleton. The function takes a
configuration argument of type `$CONFIG_TYPE` to create the UDF.
