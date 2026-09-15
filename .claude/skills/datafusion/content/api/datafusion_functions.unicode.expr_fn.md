# `datafusion_functions::unicode::expr_fn`

Crate `datafusion-functions` · 17 public items · structured records in [`model/datafusion_functions.unicode.expr_fn.json`](../model/datafusion_functions.unicode.expr_fn.json)

## char_length

`function` · `datafusion_functions::unicode::expr_fn::char_length`

Also reachable as `datafusion::prelude::char_length`, `datafusion_functions::expr_fn::char_length`

```rust
fn char_length(string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

the number of characters in the `string`

---

## character_length

`function` · `datafusion_functions::unicode::expr_fn::character_length`

Also reachable as `datafusion::prelude::character_length`, `datafusion_functions::expr_fn::character_length`

```rust
fn character_length(string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

the number of characters in the `string`

---

## find_in_set

`function` · `datafusion_functions::unicode::expr_fn::find_in_set`

Also reachable as `datafusion::prelude::find_in_set`, `datafusion_functions::expr_fn::find_in_set`

```rust
fn find_in_set(string: datafusion_expr::Expr, strlist: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns a value in the range of 1 to N if the string `str` is in the string list `strlist` consisting of N substrings

---

## initcap

`function` · `datafusion_functions::unicode::expr_fn::initcap`

Also reachable as `datafusion::prelude::initcap`, `datafusion_functions::expr_fn::initcap`

```rust
fn initcap(string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

converts the first letter of each word in `string` in uppercase and the remaining characters in lowercase

---

## instr

`function` · `datafusion_functions::unicode::expr_fn::instr`

Also reachable as `datafusion::prelude::instr`, `datafusion_functions::expr_fn::instr`

```rust
fn instr(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr
```

finds the position from where the `substring` matches the `string`

---

## left

`function` · `datafusion_functions::unicode::expr_fn::left`

Also reachable as `datafusion::prelude::left`, `datafusion_functions::expr_fn::left`

```rust
fn left(string: datafusion_expr::Expr, n: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the first `n` characters in the `string`

---

## length

`function` · `datafusion_functions::unicode::expr_fn::length`

Also reachable as `datafusion::prelude::length`, `datafusion_functions::expr_fn::length`

```rust
fn length(string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

the number of characters in the `string`

---

## lpad

`function` · `datafusion_functions::unicode::expr_fn::lpad`

Also reachable as `datafusion::prelude::lpad`, `datafusion_functions::expr_fn::lpad`

```rust
fn lpad(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

fill up a string to the length by prepending the characters

---

## position

`function` · `datafusion_functions::unicode::expr_fn::position`

Also reachable as `datafusion::prelude::position`, `datafusion_functions::expr_fn::position`

```rust
fn position(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr
```

finds the position from where the `substring` matches the `string`

---

## reverse

`function` · `datafusion_functions::unicode::expr_fn::reverse`

Also reachable as `datafusion::prelude::reverse`, `datafusion_functions::expr_fn::reverse`

```rust
fn reverse(string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

reverses the `string`

---

## right

`function` · `datafusion_functions::unicode::expr_fn::right`

Also reachable as `datafusion::prelude::right`, `datafusion_functions::expr_fn::right`

```rust
fn right(string: datafusion_expr::Expr, n: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the last `n` characters in the `string`

---

## rpad

`function` · `datafusion_functions::unicode::expr_fn::rpad`

Also reachable as `datafusion::prelude::rpad`, `datafusion_functions::expr_fn::rpad`

```rust
fn rpad(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

fill up a string to the length by appending the characters

---

## strpos

`function` · `datafusion_functions::unicode::expr_fn::strpos`

Also reachable as `datafusion::prelude::strpos`, `datafusion_functions::expr_fn::strpos`

```rust
fn strpos(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr
```

finds the position from where the `substring` matches the `string`

---

## substr

`function` · `datafusion_functions::unicode::expr_fn::substr`

Also reachable as `datafusion::prelude::substr`, `datafusion_functions::expr_fn::substr`

```rust
fn substr(string: datafusion_expr::Expr, position: datafusion_expr::Expr) -> datafusion_expr::Expr
```

substring from the `position` to the end

---

## substr_index

`function` · `datafusion_functions::unicode::expr_fn::substr_index`

Also reachable as `datafusion::prelude::substr_index`, `datafusion_functions::expr_fn::substr_index`

```rust
fn substr_index(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, count: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the substring from str before count occurrences of the delimiter

---

## substring

`function` · `datafusion_functions::unicode::expr_fn::substring`

Also reachable as `datafusion::prelude::substring`, `datafusion_functions::expr_fn::substring`

```rust
fn substring(string: datafusion_expr::Expr, position: datafusion_expr::Expr, length: datafusion_expr::Expr) -> datafusion_expr::Expr
```

substring from the `position` with `length` characters

---

## translate

`function` · `datafusion_functions::unicode::expr_fn::translate`

Also reachable as `datafusion::prelude::translate`, `datafusion_functions::expr_fn::translate`

```rust
fn translate(string: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr
```

replaces the characters in `from` with the counterpart in `to`

---
