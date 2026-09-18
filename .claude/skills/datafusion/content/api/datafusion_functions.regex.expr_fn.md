# `datafusion_functions::regex::expr_fn`

Crate `datafusion-functions` · 5 public items · structured records in [`model/datafusion_functions.regex.expr_fn.json`](../model/datafusion_functions.regex.expr_fn.json)

## regexp_count

`function` · `datafusion_functions::regex::expr_fn::regexp_count`

Also reachable as `datafusion::prelude::regexp_count`, `datafusion_functions::expr_fn::regexp_count`

```rust
fn regexp_count(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, start: Option<datafusion_expr::Expr>, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.expr_fn.regexp_count.md).


Returns the number of consecutive occurrences of a regular expression in a string.

---

## regexp_instr

`function` · `datafusion_functions::regex::expr_fn::regexp_instr`

Also reachable as `datafusion::prelude::regexp_instr`, `datafusion_functions::expr_fn::regexp_instr`

```rust
fn regexp_instr(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, start: Option<datafusion_expr::Expr>, n: Option<datafusion_expr::Expr>, endoption: Option<datafusion_expr::Expr>, flags: Option<datafusion_expr::Expr>, subexpr: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.expr_fn.regexp_instr.md).


Returns index of regular expression matches in a string.

---

## regexp_like

`function` · `datafusion_functions::regex::expr_fn::regexp_like`

Also reachable as `datafusion::prelude::regexp_like`, `datafusion_functions::expr_fn::regexp_like`

```rust
fn regexp_like(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.expr_fn.regexp_like.md).


Returns true if a regex has at least one match in a string, false otherwise.

---

## regexp_match

`function` · `datafusion_functions::regex::expr_fn::regexp_match`

Also reachable as `datafusion::prelude::regexp_match`, `datafusion_functions::expr_fn::regexp_match`

```rust
fn regexp_match(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.expr_fn.regexp_match.md).


Returns a list of regular expression matches in a string.

---

## regexp_replace

`function` · `datafusion_functions::regex::expr_fn::regexp_replace`

Also reachable as `datafusion::prelude::regexp_replace`, `datafusion_functions::expr_fn::regexp_replace`

```rust
fn regexp_replace(string: datafusion_expr::Expr, pattern: datafusion_expr::Expr, replacement: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.regex.expr_fn.regexp_replace.md).


Replaces substrings in a string that match.

---
