# `datafusion_functions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.json).

<a id="op-0819c5e775f1c71025669776"></a>
## datafusion_functions

`module` · `datafusion_functions` · datafusion-functions 55.1.0

```rust
mod datafusion_functions
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

 Function packages for [DataFusion].

 This crate contains a collection of various function packages for DataFusion,
 implemented using the extension API. Users may wish to control which functions
 are available to control the binary size of their application as well as
 use dialect specific implementations of functions (e.g. Spark vs Postgres)

 Each package is implemented as a separate
 module, activated by a feature flag.

 [DataFusion]: https://crates.io/crates/datafusion

 # Available Packages
 See the list of [modules](#modules) in this crate for available packages.

 # Using A Package
 You can register all functions in all packages using the [`register_all`](../operations/datafusion_functions.register_all.md#op-f9df861972813bbf8aa26db5) function.

 To access and use only the functions in a certain package, use the
 `functions()` method in each module.

 ```
 # fn main() -> datafusion_common::Result<()> {
 # let mut registry = datafusion_execution::registry::MemoryFunctionRegistry::new();
 # use datafusion_execution::FunctionRegistry;
 // get the encoding functions
 use datafusion_functions::encoding;
 for udf in encoding::functions() {
   registry.register_udf(udf)?;
 }
 # Ok(())
 # }
 ```

 Each package also exports an `expr_fn` submodule to help create [`Expr`]s that invoke
 functions using a fluent style. For example:

 ```
 // create an Expr that will invoke the encode function
 use datafusion_expr::{col, lit};
 use datafusion_functions::expr_fn;
 // Equivalent to "encode(my_data, 'hex')" in SQL:
 let expr = expr_fn::encode(col("my_data"), lit("hex"));
 ```

[`Expr`]: datafusion_expr::Expr

 # Implementing A New Package

 To add a new package to this crate, you should follow the model of existing
 packages. The high level steps are:

 1. Create a new module with the appropriate [`ScalarUDF`] implementations.

 2. Use the macros in [`macros`](../modules/datafusion_functions.macros.md#op-983dc0abfecc70fd00ea1581) to create standard entry points.

 3. Add a new feature to `Cargo.toml`, with any optional dependencies

 4. Use the `make_package!` macro to expose the module when the
    feature is enabled.

 [`ScalarUDF`]: datafusion_expr::ScalarUDF
