# `datafusion_functions_aggregate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.json).

<a id="op-ce94f648cdfcacc6630ed836"></a>
## datafusion_functions_aggregate

`module` · `datafusion_functions_aggregate` · datafusion-functions-aggregate 55.1.0

```rust
mod datafusion_functions_aggregate
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

 Aggregate Function packages for [DataFusion].

 This crate contains a collection of various aggregate function packages for DataFusion,
 implemented using the extension API. Users may wish to control which functions
 are available to control the binary size of their application as well as
 use dialect specific implementations of functions (e.g. Spark vs Postgres)

 Each package is implemented as a separate
 module, activated by a feature flag.

 [DataFusion]: https://crates.io/crates/datafusion

 # Available Packages
 See the list of [modules](#modules) in this crate for available packages.

 # Using A Package
 You can register all functions in all packages using the [`register_all`](../operations/datafusion_functions_aggregate.register_all.md#op-44c22c7212d5afb67186d254) function.

 Each package also exports an `expr_fn` submodule to help create [`Expr`]s that invoke
 functions using a fluent style. For example:

[`Expr`]: datafusion_expr::Expr

 # Implementing A New Package

 To add a new package to this crate, you should follow the model of existing
 packages. The high level steps are:

 1. Create a new module with the appropriate [AggregateUDF](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379) implementations.

 2. Use the macros in [`macros`](../modules/datafusion_functions_aggregate.macros.md#op-32d59fc286bd418b44d2afd5) to create standard entry points.

 3. Add a new feature to `Cargo.toml`, with any optional dependencies

 4. Use the `make_package!` macro to expose the module when the
    feature is enabled.
