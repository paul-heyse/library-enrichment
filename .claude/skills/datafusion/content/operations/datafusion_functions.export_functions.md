# `datafusion_functions::export_functions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.export_functions.json).

<a id="op-387069f0c8c7cfb3748cd186"></a>
## export_functions

`macro` · `datafusion_functions::export_functions` · datafusion-functions 55.1.0

```rust
macro_rules! export_functions
```

Source: `src/macros.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

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
