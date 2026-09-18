# `arrow_string::like`

Crate `arrow-string` · 8 public items · structured records in [`model/arrow_string.like.json`](../model/arrow_string.like.json)

## contains

`function` · `arrow_string::like::contains`

Also reachable as `arrow::compute::contains`, `arrow::compute::kernels::comparison::contains`

```rust
fn contains(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.like.contains.md).


Perform SQL `CONTAINS(left, right)`

# Supported DataTypes

`left` and `right` must be the same type, and one of
- Utf8
- LargeUtf8
- Utf8View
- Binary
- LargeBinary
- BinaryView

# Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::like::contains;
let strings = StringArray::from(vec!["arrow-rs", "arrow-rs", "arrow-rs", "Parquet"]);
let patterns = StringArray::from(vec!["arr", "-rs", "arrow-cpp", "X"]);

let result = contains(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, true, false, false]));
```

---

## ends_with

`function` · `arrow_string::like::ends_with`

Also reachable as `arrow::compute::ends_with`, `arrow::compute::kernels::comparison::ends_with`

```rust
fn ends_with(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.like.ends_with.md).


Perform SQL `ENDSWITH(left, right)`

# Supported DataTypes

`left` and `right` must be the same type, and one of
- Utf8
- LargeUtf8
- Utf8View
- Binary
- LargeBinary
- BinaryView

# Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::like::ends_with;
let strings = StringArray::from(vec!["arrow-rs", "arrow-rs",  "Parquet"]);
let patterns = StringArray::from(vec!["arr", "-rs", "t"]);

let result = ends_with(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![false, true, true]));
```

---

## eq_ignore_ascii_case

`function` · `arrow_string::like::eq_ignore_ascii_case`

Also reachable as `arrow::compute::eq_ignore_ascii_case`, `arrow::compute::kernels::comparison::eq_ignore_ascii_case`

```rust
fn eq_ignore_ascii_case(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.like.eq_ignore_ascii_case.md).


Perform equality check on two arrays using an ASCII case-insensitive match.

`left` and `right` must be the same type, and one of
- Utf8
- LargeUtf8
- Utf8View

# Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::like::eq_ignore_ascii_case;
let strings = StringArray::from(vec!["arrow", "rs", "arrow-rS", "Parquet"]);
let patterns = StringArray::from(vec!["ARROW", "rS", "ARROW-rs", "arrow"]);

let result = eq_ignore_ascii_case(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, true, true, false]));
```

---

## ilike

`function` · `arrow_string::like::ilike`

Also reachable as `arrow::compute::ilike`, `arrow::compute::kernels::comparison::ilike`

```rust
fn ilike(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.like.ilike.md).


Perform SQL `left ILIKE right`

# Notes
- This is a case-insensitive version of [`like`]
- See the documentation on [`like`] for more details
- Implements loose matching as defined by the Unicode standard. For example,
  the `ﬀ` ligature is not equivalent to `FF` and `ß` is not equivalent to `SS`

---

## like

`function` · `arrow_string::like::like`

Also reachable as `arrow::compute::kernels::comparison::like`, `arrow::compute::like`

```rust
fn like(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.like.like.md).


Perform SQL `left LIKE right`

# Supported DataTypes

`left` and `right` must be the same type, and one of
- Utf8
- LargeUtf8
- Utf8View

There are two wildcards supported with the LIKE operator:

1. `%` - The percent sign represents zero, one, or multiple characters
2. `_` - The underscore represents a single character

Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::like::like;
let strings = StringArray::from(vec!["Arrow", "Arrow", "Arrow", "Ar"]);
let patterns = StringArray::from(vec!["A%", "B%", "A.", "A_"]);

let result = like(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, false, false, true]));
```

---

## nilike

`function` · `arrow_string::like::nilike`

Also reachable as `arrow::compute::kernels::comparison::nilike`, `arrow::compute::nilike`

```rust
fn nilike(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.like.nilike.md).


Perform SQL `left NOT ILIKE right`

# Notes
- This is a negative of [`like`]
- See the documentation on [`ilike`] for more details

---

## nlike

`function` · `arrow_string::like::nlike`

Also reachable as `arrow::compute::kernels::comparison::nlike`, `arrow::compute::nlike`

```rust
fn nlike(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.like.nlike.md).


Perform SQL `left NOT LIKE right`

# Notes
- This is a negative of [`like`]
- See the documentation on [`like`] for more details

---

## starts_with

`function` · `arrow_string::like::starts_with`

Also reachable as `arrow::compute::kernels::comparison::starts_with`, `arrow::compute::starts_with`

```rust
fn starts_with(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.like.starts_with.md).


Perform SQL `STARTSWITH(left, right)`

# Supported DataTypes

`left` and `right` must be the same type, and one of
- Utf8
- LargeUtf8
- Utf8View
- Binary
- LargeBinary
- BinaryView

# Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::like::starts_with;
let strings = StringArray::from(vec!["arrow-rs", "arrow-rs", "arrow-rs", "Parquet"]);
let patterns = StringArray::from(vec!["arr", "arrow", "arrow-cpp", "p"]);

let result = starts_with(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, true, false, false]));
```

---
