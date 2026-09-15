# `datafusion_functions`

Crate `datafusion-functions` · 10 public items · structured records in [`model/datafusion_functions.json`](../model/datafusion_functions.json)

## all_default_functions

`function` · `datafusion_functions::all_default_functions`

Also reachable as `datafusion::functions::all_default_functions`

```rust
fn all_default_functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

Return all default functions

---

## register_all

`function` · `datafusion_functions::register_all`

Also reachable as `datafusion::functions::register_all`

```rust
fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()>
```

Registers all enabled packages with a [`FunctionRegistry`]

---

## downcast_arg

`macro` · `datafusion_functions::downcast_arg`

Also reachable as `datafusion::functions::downcast_arg`

```rust
macro_rules! downcast_arg
```

Downcast an argument to a specific array type, returning an internal error
if the cast fails

$ARG: ArrayRef
$ARRAY_TYPE: the type of array to cast the argument to

---

## downcast_named_arg

`macro` · `datafusion_functions::downcast_named_arg`

Also reachable as `datafusion::functions::downcast_named_arg`

```rust
macro_rules! downcast_named_arg
```

Downcast a named argument to a specific array type, returning an internal error
if the cast fails

$ARG: ArrayRef
$NAME: name of the argument (for error messages)
$ARRAY_TYPE: the type of array to cast the argument to

---

## export_functions

`macro` · `datafusion_functions::export_functions`

Also reachable as `datafusion::functions::export_functions`

```rust
macro_rules! export_functions
```

macro that exports a list of function names as:
1. individual functions in an `expr_fn` module
2. a single function that returns a list of all functions

Equivalent to
```text
pub mod expr_fn {
    use super::*;
    /// Return encode(arg)
    pub fn encode(args: Vec<Expr>) -> Expr {
        super::encode().call(args)
    }
 ...
/// Return a list of all functions in this package
pub(crate) fn functions() -> Vec<Arc<ScalarUDF>> {
    vec![
      encode(),
      decode()
   ]
}
```

Exported functions accept:
- `Vec<Expr>` argument (single argument followed by a comma)
- Variable number of `Expr` arguments (zero or more arguments, must be without commas)
- Functions that require config (marked with `@config` prefix)

Note on configuration construction paths:
- The convenience wrappers generated for `@config` functions call the inner
  constructor with `ConfigOptions::default()`. These wrappers are intended
  primarily for programmatic `Expr` construction and convenience usage.
- When functions are registered in a session, DataFusion will call
  `with_updated_config()` to create a `ScalarUDF` instance using the session's
  actual `ConfigOptions`. This also happens when configuration changes at runtime
  (e.g., via `SET` statements). In short: the macro uses the default config for
  convenience constructors; the session config is applied when functions are
  registered or when configuration is updated.

---

## make_abs_function

`macro` · `datafusion_functions::make_abs_function`

Also reachable as `datafusion::functions::make_abs_function`

```rust
macro_rules! make_abs_function
```

---

## make_try_abs_function

`macro` · `datafusion_functions::make_try_abs_function`

Also reachable as `datafusion::functions::make_try_abs_function`

```rust
macro_rules! make_try_abs_function
```

---

## make_udf_function

`macro` · `datafusion_functions::make_udf_function`

Also reachable as `datafusion::functions::make_udf_function`

```rust
macro_rules! make_udf_function
```

Creates a singleton `ScalarUDF` of the `$UDF` function and a function
named `$NAME` which returns that singleton. Optionally use a custom constructor
`$CTOR` which defaults to `$UDF::new()` if not specified.

This is used to ensure creating the list of `ScalarUDF` only happens once.

---

## make_udf_function_with_config

`macro` · `datafusion_functions::make_udf_function_with_config`

Also reachable as `datafusion::functions::make_udf_function_with_config`

```rust
macro_rules! make_udf_function_with_config
```

Creates a singleton `ScalarUDF` of the `$UDF` function and a function
named `$NAME` which returns that singleton. The function takes a
configuration argument of type `$CONFIG_TYPE` to create the UDF.

---

## make_wrapping_abs_function

`macro` · `datafusion_functions::make_wrapping_abs_function`

Also reachable as `datafusion::functions::make_wrapping_abs_function`

```rust
macro_rules! make_wrapping_abs_function
```

---
