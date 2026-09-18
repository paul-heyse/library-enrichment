# `datafusion_expr::arguments::resolve_function_arguments`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.arguments.resolve_function_arguments.json).

<a id="op-bc300e1b32e32002ae55cd61"></a>
## resolve_function_arguments

`function` · `datafusion_expr::arguments::resolve_function_arguments` · datafusion-expr 55.1.0

```rust
fn resolve_function_arguments(param_names: &[String], args: Vec<Expr>, arg_names: Vec<Option<ArgumentName>>) -> datafusion_common::Result<Vec<Expr>>
```

Source: `src/arguments.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Resolves function arguments, handling named and positional notation.

This function validates and reorders arguments to match the function's parameter names
when named arguments are used.

# Rules
- All positional arguments must come before named arguments
- Named arguments can be in any order after positional arguments
- Parameter names follow SQL identifier rules: unquoted names are case-insensitive
  (normalized to lowercase), quoted names are case-sensitive
- No duplicate parameter names allowed

# Arguments
* `param_names` - The function's parameter names in order
* `args` - The argument expressions
* `arg_names` - Optional parameter name for each argument

# Returns
A vector of expressions in the correct order matching the parameter names

# Examples
```text
Given parameters ["a", "b", "c"]
And call: func(10, c => 30, b => 20)
Returns: [Expr(10), Expr(20), Expr(30)]
```
