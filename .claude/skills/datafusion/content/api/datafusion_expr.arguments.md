# `datafusion_expr::arguments`

Crate `datafusion-expr` · 2 public items · structured records in [`model/datafusion_expr.arguments.json`](../model/datafusion_expr.arguments.json)

## resolve_function_arguments

`function` · `datafusion_expr::arguments::resolve_function_arguments`

```rust
fn resolve_function_arguments(param_names: &[String], args: Vec<Expr>, arg_names: Vec<Option<ArgumentName>>) -> datafusion_common::Result<Vec<Expr>>
```

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

---

## ArgumentName

`struct` · `datafusion_expr::arguments::ArgumentName`

```rust
struct ArgumentName
```

**Fields**: `value`, `is_quoted`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Represents a named function argument with its original case and quote information.

This struct preserves whether an identifier was quoted in the SQL, which determines
whether case-sensitive or case-insensitive matching should be used per SQL standards.

---
