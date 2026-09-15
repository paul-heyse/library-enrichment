# `datafusion_spark::function::string::expr_fn`

Crate `datafusion-spark` · 18 public items · structured records in [`model/datafusion_spark.function.string.expr_fn.json`](../model/datafusion_spark.function.string.expr_fn.json)

## ascii

`function` · `datafusion_spark::function::string::expr_fn::ascii`

Also reachable as `datafusion_spark::expr_fn::ascii`

```rust
fn ascii(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the ASCII code point of the first character of string.

---

## base64

`function` · `datafusion_spark::function::string::expr_fn::base64`

Also reachable as `datafusion_spark::expr_fn::base64`

```rust
fn base64(bin: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Encodes the input binary `bin` into a base64 string.

---

## char

`function` · `datafusion_spark::function::string::expr_fn::char`

Also reachable as `datafusion_spark::expr_fn::char`

```rust
fn char(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the ASCII character having the binary equivalent to col. If col is larger than 256 the result is equivalent to char(col % 256).

---

## concat

`function` · `datafusion_spark::function::string::expr_fn::concat`

Also reachable as `datafusion_spark::expr_fn::concat`

```rust
fn concat(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Concatenates multiple input strings into a single string. Returns NULL if any input is NULL.

---

## concat_ws

`function` · `datafusion_spark::function::string::expr_fn::concat_ws`

Also reachable as `datafusion_spark::expr_fn::concat_ws`

```rust
fn concat_ws(sep: datafusion_expr::Expr, args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Concatenates strings with separator. Supports arrays. Null values are skipped.

---

## elt

`function` · `datafusion_spark::function::string::expr_fn::elt`

Also reachable as `datafusion_spark::expr_fn::elt`

```rust
fn elt(select_col: datafusion_expr::Expr, arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, argn: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the n-th input (1-indexed), e.g. returns 2nd input when n is 2. The function returns NULL if the index is 0 or exceeds the length of the array.

---

## format_string

`function` · `datafusion_spark::function::string::expr_fn::format_string`

Also reachable as `datafusion_spark::expr_fn::format_string`

```rust
fn format_string(strfmt: datafusion_expr::Expr, args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns a formatted string from printf-style format strings.

---

## ilike

`function` · `datafusion_spark::function::string::expr_fn::ilike`

Also reachable as `datafusion_spark::expr_fn::ilike`

```rust
fn ilike(str: datafusion_expr::Expr, pattern: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns true if str matches pattern (case insensitive).

---

## is_valid_utf8

`function` · `datafusion_spark::function::string::expr_fn::is_valid_utf8`

Also reachable as `datafusion_spark::expr_fn::is_valid_utf8`

```rust
fn is_valid_utf8(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns true if str is a valid UTF-8 string, otherwise returns false

---

## length

`function` · `datafusion_spark::function::string::expr_fn::length`

Also reachable as `datafusion_spark::expr_fn::length`

```rust
fn length(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the character length of string data or number of bytes of binary data. The length of string data includes the trailing spaces. The length of binary data includes binary zeros.

---

## like

`function` · `datafusion_spark::function::string::expr_fn::like`

Also reachable as `datafusion_spark::expr_fn::like`

```rust
fn like(str: datafusion_expr::Expr, pattern: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns true if str matches pattern (case sensitive).

---

## luhn_check

`function` · `datafusion_spark::function::string::expr_fn::luhn_check`

Also reachable as `datafusion_spark::expr_fn::luhn_check`

```rust
fn luhn_check(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns whether the input string of digits is valid according to the Luhn algorithm.

---

## make_valid_utf8

`function` · `datafusion_spark::function::string::expr_fn::make_valid_utf8`

Also reachable as `datafusion_spark::expr_fn::make_valid_utf8`

```rust
fn make_valid_utf8(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the original string if str is a valid UTF-8 string, otherwise returns a new string whose invalid UTF8 byte sequences are replaced using the UNICODE replacement character U+FFFD.

---

## quote

`function` · `datafusion_spark::function::string::expr_fn::quote`

Also reachable as `datafusion_spark::expr_fn::quote`

```rust
fn quote(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns str enclosed by single quotes and each instance of single quote in it is preceded by a backslash

---

## soundex

`function` · `datafusion_spark::function::string::expr_fn::soundex`

Also reachable as `datafusion_spark::expr_fn::soundex`

```rust
fn soundex(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns Soundex code of the string.

---

## space

`function` · `datafusion_spark::function::string::expr_fn::space`

Also reachable as `datafusion_spark::expr_fn::space`

```rust
fn space(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns a string consisting of n spaces.

---

## substring

`function` · `datafusion_spark::function::string::expr_fn::substring`

Also reachable as `datafusion_spark::expr_fn::substring`

```rust
fn substring(str: datafusion_expr::Expr, pos: datafusion_expr::Expr, length: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the substring from string `str` starting at position `pos` with length `length.

---

## unbase64

`function` · `datafusion_spark::function::string::expr_fn::unbase64`

Also reachable as `datafusion_spark::expr_fn::unbase64`

```rust
fn unbase64(str: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Decodes the input string `str` from a base64 string into binary data.

---
