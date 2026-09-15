# `datafusion_functions::string::expr_fn`

Crate `datafusion-functions` · 21 public items · structured records in [`model/datafusion_functions.string.expr_fn.json`](../model/datafusion_functions.string.expr_fn.json)

## ascii

`function` · `datafusion_functions::string::expr_fn::ascii`

Also reachable as `datafusion::prelude::ascii`, `datafusion_functions::expr_fn::ascii`

```rust
fn ascii(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the numeric code of the first character of the argument.

---

## bit_length

`function` · `datafusion_functions::string::expr_fn::bit_length`

Also reachable as `datafusion::prelude::bit_length`, `datafusion_functions::expr_fn::bit_length`

```rust
fn bit_length(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the number of bits in the `string`

---

## btrim

`function` · `datafusion_functions::string::expr_fn::btrim`

Also reachable as `datafusion::prelude::btrim`, `datafusion_functions::expr_fn::btrim`

```rust
fn btrim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Removes all characters, spaces by default, from both sides of a string

---

## chr

`function` · `datafusion_functions::string::expr_fn::chr`

Also reachable as `datafusion::prelude::chr`, `datafusion_functions::expr_fn::chr`

```rust
fn chr(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Converts the Unicode code point to a UTF8 character

---

## concat

`function` · `datafusion_functions::string::expr_fn::concat`

Also reachable as `datafusion::prelude::concat`, `datafusion_functions::expr_fn::concat`

```rust
fn concat(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Concatenates the text representations of all the arguments. NULL arguments are ignored

---

## concat_ws

`function` · `datafusion_functions::string::expr_fn::concat_ws`

Also reachable as `datafusion::prelude::concat_ws`, `datafusion_functions::expr_fn::concat_ws`

```rust
fn concat_ws(delimiter: datafusion_expr::Expr, args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Concatenates all but the first argument, with separators. The first argument is used as the separator string, and should not be NULL. Other NULL arguments are ignored.

---

## contains

`function` · `datafusion_functions::string::expr_fn::contains`

Also reachable as `datafusion::prelude::contains`, `datafusion_functions::expr_fn::contains`

```rust
fn contains(string: datafusion_expr::Expr, search_string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Return true if `search_string` is found within `string`.

---

## ends_with

`function` · `datafusion_functions::string::expr_fn::ends_with`

Also reachable as `datafusion::prelude::ends_with`, `datafusion_functions::expr_fn::ends_with`

```rust
fn ends_with(string: datafusion_expr::Expr, suffix: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns true if the `string` ends with the `suffix`, false otherwise.

---

## levenshtein

`function` · `datafusion_functions::string::expr_fn::levenshtein`

Also reachable as `datafusion::prelude::levenshtein`, `datafusion_functions::expr_fn::levenshtein`

```rust
fn levenshtein(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the Levenshtein distance between the two given strings

---

## lower

`function` · `datafusion_functions::string::expr_fn::lower`

Also reachable as `datafusion::prelude::lower`, `datafusion_functions::expr_fn::lower`

```rust
fn lower(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Converts a string to lowercase.

---

## ltrim

`function` · `datafusion_functions::string::expr_fn::ltrim`

Also reachable as `datafusion::prelude::ltrim`, `datafusion_functions::expr_fn::ltrim`

```rust
fn ltrim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Removes all characters, spaces by default, from the beginning of a string

---

## octet_length

`function` · `datafusion_functions::string::expr_fn::octet_length`

Also reachable as `datafusion::prelude::octet_length`, `datafusion_functions::expr_fn::octet_length`

```rust
fn octet_length(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the number of bytes of a string

---

## repeat

`function` · `datafusion_functions::string::expr_fn::repeat`

Also reachable as `datafusion::prelude::repeat`, `datafusion_functions::expr_fn::repeat`

```rust
fn repeat(string: datafusion_expr::Expr, n: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Repeats the `string` to `n` times

---

## replace

`function` · `datafusion_functions::string::expr_fn::replace`

Also reachable as `datafusion::prelude::replace`, `datafusion_functions::expr_fn::replace`

```rust
fn replace(string: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Replaces all occurrences of `from` with `to` in the `string`

---

## rtrim

`function` · `datafusion_functions::string::expr_fn::rtrim`

Also reachable as `datafusion::prelude::rtrim`, `datafusion_functions::expr_fn::rtrim`

```rust
fn rtrim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Removes all characters, spaces by default, from the end of a string

---

## split_part

`function` · `datafusion_functions::string::expr_fn::split_part`

Also reachable as `datafusion::prelude::split_part`, `datafusion_functions::expr_fn::split_part`

```rust
fn split_part(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, index: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Splits a string based on a delimiter and picks out the desired field based on the index.

---

## starts_with

`function` · `datafusion_functions::string::expr_fn::starts_with`

Also reachable as `datafusion::prelude::starts_with`, `datafusion_functions::expr_fn::starts_with`

```rust
fn starts_with(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns true if string starts with prefix.

---

## to_hex

`function` · `datafusion_functions::string::expr_fn::to_hex`

Also reachable as `datafusion::prelude::to_hex`, `datafusion_functions::expr_fn::to_hex`

```rust
fn to_hex(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Converts an integer to a hexadecimal string.

---

## trim

`function` · `datafusion_functions::string::expr_fn::trim`

Also reachable as `datafusion::prelude::trim`, `datafusion_functions::expr_fn::trim`

```rust
fn trim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Removes all characters, spaces by default, from both sides of a string

---

## upper

`function` · `datafusion_functions::string::expr_fn::upper`

Also reachable as `datafusion::prelude::upper`, `datafusion_functions::expr_fn::upper`

```rust
fn upper(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Converts a string to uppercase.

---

## uuid

`function` · `datafusion_functions::string::expr_fn::uuid`

Also reachable as `datafusion::prelude::uuid`, `datafusion_functions::expr_fn::uuid`

```rust
fn uuid() -> datafusion_expr::Expr
```

returns uuid v4 as a string value

---
