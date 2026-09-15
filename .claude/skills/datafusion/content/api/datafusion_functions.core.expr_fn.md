# `datafusion_functions::core::expr_fn`

Crate `datafusion-functions` · 23 public items · structured records in [`model/datafusion_functions.core.expr_fn.json`](../model/datafusion_functions.core.expr_fn.json)

## arrow_cast

`function` · `datafusion_functions::core::expr_fn::arrow_cast`

Also reachable as `datafusion::prelude::arrow_cast`, `datafusion_functions::expr_fn::arrow_cast`

```rust
fn arrow_cast(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Casts a value to a specific Arrow data type

---

## arrow_field

`function` · `datafusion_functions::core::expr_fn::arrow_field`

Also reachable as `datafusion::prelude::arrow_field`, `datafusion_functions::expr_fn::arrow_field`

```rust
fn arrow_field(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the Arrow field info (name, data_type, nullable, metadata) of the input expression.

---

## arrow_metadata

`function` · `datafusion_functions::core::expr_fn::arrow_metadata`

Also reachable as `datafusion::prelude::arrow_metadata`, `datafusion_functions::expr_fn::arrow_metadata`

```rust
fn arrow_metadata(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Returns the metadata of the input expression

---

## arrow_try_cast

`function` · `datafusion_functions::core::expr_fn::arrow_try_cast`

Also reachable as `datafusion::prelude::arrow_try_cast`, `datafusion_functions::expr_fn::arrow_try_cast`

```rust
fn arrow_try_cast(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Casts a value to a specific Arrow data type, returning NULL if the cast fails

---

## arrow_typeof

`function` · `datafusion_functions::core::expr_fn::arrow_typeof`

Also reachable as `datafusion::prelude::arrow_typeof`, `datafusion_functions::expr_fn::arrow_typeof`

```rust
fn arrow_typeof(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the Arrow type of the input expression.

---

## cast_to_type

`function` · `datafusion_functions::core::expr_fn::cast_to_type`

Also reachable as `datafusion::prelude::cast_to_type`, `datafusion_functions::expr_fn::cast_to_type`

```rust
fn cast_to_type(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Casts the first argument to the data type of the second argument

---

## coalesce

`function` · `datafusion_functions::core::expr_fn::coalesce`

Also reachable as `datafusion::prelude::coalesce`, `datafusion_functions::expr_fn::coalesce`

```rust
fn coalesce(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Returns `coalesce(args...)`, which evaluates to the value of the first expr which is not NULL

---

## file_row_index

`function` · `datafusion_functions::core::expr_fn::file_row_index`

Also reachable as `datafusion::prelude::file_row_index`, `datafusion_functions::expr_fn::file_row_index`

```rust
fn file_row_index() -> datafusion_expr::Expr
```

Returns the offset of the row within its source file

---

## get_field

`function` · `datafusion_functions::core::expr_fn::get_field`

Also reachable as `datafusion::prelude::get_field`, `datafusion_functions::expr_fn::get_field`

```rust
fn get_field(arg1: datafusion_expr::Expr, arg2: impl Literal) -> datafusion_expr::Expr
```

Returns the value of the field with the given name from the struct

---

## get_field_path

`function` · `datafusion_functions::core::expr_fn::get_field_path`

Also reachable as `datafusion::prelude::get_field_path`, `datafusion_functions::expr_fn::get_field_path`

```rust
fn get_field_path(base: datafusion_expr::Expr, field_names: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Returns the value of nested fields by traversing multiple field names

---

## greatest

`function` · `datafusion_functions::core::expr_fn::greatest`

Also reachable as `datafusion::prelude::greatest`, `datafusion_functions::expr_fn::greatest`

```rust
fn greatest(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Returns `greatest(args...)`, which evaluates to the greatest value in the list of expressions or NULL if all the expressions are NULL

---

## input_file_name

`function` · `datafusion_functions::core::expr_fn::input_file_name`

Also reachable as `datafusion::prelude::input_file_name`, `datafusion_functions::expr_fn::input_file_name`

```rust
fn input_file_name() -> datafusion_expr::Expr
```

Returns the path of the input file that produced the current row

---

## least

`function` · `datafusion_functions::core::expr_fn::least`

Also reachable as `datafusion::prelude::least`, `datafusion_functions::expr_fn::least`

```rust
fn least(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Returns `least(args...)`, which evaluates to the smallest value in the list of expressions or NULL if all the expressions are NULL

---

## named_struct

`function` · `datafusion_functions::core::expr_fn::named_struct`

Also reachable as `datafusion::prelude::named_struct`, `datafusion_functions::expr_fn::named_struct`

```rust
fn named_struct(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Returns a struct with the given names and arguments pairs

---

## nullif

`function` · `datafusion_functions::core::expr_fn::nullif`

Also reachable as `datafusion::prelude::nullif`, `datafusion_functions::expr_fn::nullif`

```rust
fn nullif(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns NULL if value1 equals value2; otherwise it returns value1. This can be used to perform the inverse operation of the COALESCE expression

---

## nvl

`function` · `datafusion_functions::core::expr_fn::nvl`

Also reachable as `datafusion::prelude::nvl`, `datafusion_functions::expr_fn::nvl`

```rust
fn nvl(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns value2 if value1 is NULL; otherwise it returns value1

---

## nvl2

`function` · `datafusion_functions::core::expr_fn::nvl2`

Also reachable as `datafusion::prelude::nvl2`, `datafusion_functions::expr_fn::nvl2`

```rust
fn nvl2(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, arg3: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns value2 if value1 is not NULL; otherwise, it returns value3.

---

## overlay

`function` · `datafusion_functions::core::expr_fn::overlay`

Also reachable as `datafusion::prelude::overlay`, `datafusion_functions::expr_fn::overlay`

```rust
fn overlay(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

replace the substring of string that starts at the start'th character and extends for count characters with new substring

---

## struct

`function` · `datafusion_functions::core::expr_fn::struct`

Also reachable as `datafusion::prelude::struct`, `datafusion_functions::expr_fn::struct`

```rust
fn struct(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Returns a struct with the given arguments

---

## try_cast_to_type

`function` · `datafusion_functions::core::expr_fn::try_cast_to_type`

Also reachable as `datafusion::prelude::try_cast_to_type`, `datafusion_functions::expr_fn::try_cast_to_type`

```rust
fn try_cast_to_type(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Casts the first argument to the data type of the second argument, returning NULL on failure

---

## union_extract

`function` · `datafusion_functions::core::expr_fn::union_extract`

Also reachable as `datafusion::prelude::union_extract`, `datafusion_functions::expr_fn::union_extract`

```rust
fn union_extract(arg1: datafusion_expr::Expr, arg2: impl Literal) -> datafusion_expr::Expr
```

Returns the value of the field with the given name from the union when it's selected, or NULL otherwise

---

## union_tag

`function` · `datafusion_functions::core::expr_fn::union_tag`

Also reachable as `datafusion::prelude::union_tag`, `datafusion_functions::expr_fn::union_tag`

```rust
fn union_tag(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the name of the currently selected field in the union

---

## with_metadata

`function` · `datafusion_functions::core::expr_fn::with_metadata`

Also reachable as `datafusion::prelude::with_metadata`, `datafusion_functions::expr_fn::with_metadata`

```rust
fn with_metadata(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Attaches Arrow field metadata (key/value pairs) to the input expression

---
