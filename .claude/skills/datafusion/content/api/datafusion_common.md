# `datafusion_common`

Crate `datafusion-common` · 39 public items · structured records in [`model/datafusion_common.json`](../model/datafusion_common.json)

## arrow_datafusion_err

`macro` · `datafusion_common::arrow_datafusion_err`

Also reachable as `datafusion::common::arrow_datafusion_err`

```rust
macro_rules! arrow_datafusion_err
```

---

## arrow_err

`macro` · `datafusion_common::arrow_err`

Also reachable as `datafusion::common::arrow_err`

```rust
macro_rules! arrow_err
```

---

## assert_batches_eq

`macro` · `datafusion_common::assert_batches_eq`

Also reachable as `datafusion::assert_batches_eq`, `datafusion::common::assert_batches_eq`

```rust
macro_rules! assert_batches_eq
```

Compares formatted output of a record batch with an expected
vector of strings, with the result of pretty formatting record
batches. This is a macro so errors appear on the correct line

Designed so that failure output can be directly copy/pasted
into the test code as expected results.

Expects to be called about like this:

`assert_batches_eq!(expected_lines: &[&str], batches: &[RecordBatch])`

# Example
```
# use std::sync::Arc;
# use arrow::record_batch::RecordBatch;
# use arrow::array::{ArrayRef, Int32Array};
# use datafusion_common::assert_batches_eq;
let col: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));
let batch = RecordBatch::try_from_iter([("column", col)]).unwrap();
// Expected output is a vec of strings
let expected = vec![
    "+--------+",
    "| column |",
    "+--------+",
    "| 1      |",
    "| 2      |",
    "+--------+",
];
// compare the formatted output of the record batch with the expected output
assert_batches_eq!(expected, &[batch]);
```

---

## assert_batches_sorted_eq

`macro` · `datafusion_common::assert_batches_sorted_eq`

Also reachable as `datafusion::assert_batches_sorted_eq`, `datafusion::common::assert_batches_sorted_eq`

```rust
macro_rules! assert_batches_sorted_eq
```

Compares formatted output of a record batch with an expected
vector of strings in a way that order does not matter.
This is a macro so errors appear on the correct line

See [`assert_batches_eq`] for more details and example.

Expects to be called about like this:

`assert_batch_sorted_eq!(expected_lines: &[&str], batches: &[RecordBatch])`

---

## assert_contains

`macro` · `datafusion_common::assert_contains`

Also reachable as `datafusion::common::assert_contains`

```rust
macro_rules! assert_contains
```

A macro to assert that one string is contained within another with
a nice error message if they are not.

Usage: `assert_contains!(actual, expected)`

Is a macro so test error
messages are on the same line as the failure;

Both arguments must be convertible into Strings ([`Into`]<[`String`]>)

---

## assert_eq_or_internal_err

`macro` · `datafusion_common::assert_eq_or_internal_err`

Also reachable as `datafusion::common::assert_eq_or_internal_err`

```rust
macro_rules! assert_eq_or_internal_err
```

Assert equality, returning `DataFusionError::Internal` on failure.

# Examples

```text
assert_eq_or_internal_err!(actual, expected);
assert_eq_or_internal_err!(left_expr, right_expr, "values must match");
assert_eq_or_internal_err!(lhs, rhs, "metadata: {}", extra);
```

---

## assert_ne_or_internal_err

`macro` · `datafusion_common::assert_ne_or_internal_err`

Also reachable as `datafusion::common::assert_ne_or_internal_err`

```rust
macro_rules! assert_ne_or_internal_err
```

Assert inequality, returning `DataFusionError::Internal` on failure.

# Examples

```text
assert_ne_or_internal_err!(left, right);
assert_ne_or_internal_err!(lhs_expr, rhs_expr, "values must differ");
assert_ne_or_internal_err!(a, b, "context {}", info);
```

---

## assert_not_contains

`macro` · `datafusion_common::assert_not_contains`

Also reachable as `datafusion::common::assert_not_contains`

```rust
macro_rules! assert_not_contains
```

A macro to assert that one string is NOT contained within another with
a nice error message if they are.

Usage: `assert_not_contains!(actual, unexpected)`

Is a macro so test error
messages are on the same line as the failure;

Both arguments must be convertible into Strings ([`Into`]<[`String`]>)

---

## assert_or_internal_err

`macro` · `datafusion_common::assert_or_internal_err`

Also reachable as `datafusion::common::assert_or_internal_err`

```rust
macro_rules! assert_or_internal_err
```

Assert a condition, returning `DataFusionError::Internal` on failure.

# Examples

```text
assert_or_internal_err!(predicate);
assert_or_internal_err!(predicate, "human readable message");
assert_or_internal_err!(predicate, format!("details: {}", value));
```

---

## config_datafusion_err

`macro` · `datafusion_common::config_datafusion_err`

Also reachable as `datafusion::common::config_datafusion_err`

```rust
macro_rules! config_datafusion_err
```

Macro wraps `$ERR` to add backtrace feature

---

## config_err

`macro` · `datafusion_common::config_err`

Also reachable as `datafusion::common::config_err`

```rust
macro_rules! config_err
```

Macro wraps Err(`$ERR`) to add backtrace feature

---

## config_field

`macro` · `datafusion_common::config_field`

Also reachable as `datafusion::common::config_field`

```rust
macro_rules! config_field
```

Macro that generates [`ConfigField`] for a given type.

# Usage
This always requires [`Display`] to be implemented for the given type.

There are two ways to invoke this macro. The first one uses
[`default_config_transform`]/[`FromStr`] to parse the data:

```ignore
config_field(MyType);
```

Note that the parsing error MUST implement [`std::error::Error`]!

Or you can specify how you want to parse an [`str`] into the type:

```ignore
fn parse_it(s: &str) -> Result<MyType> {
    ...
}

config_field(
    MyType,
    value => parse_it(value)
);
```

---

## config_namespace

`macro` · `datafusion_common::config_namespace`

Also reachable as `datafusion::common::config_namespace`

```rust
macro_rules! config_namespace
```

A macro that wraps a configuration struct and automatically derives
[`Default`] and [`ConfigField`] for it, allowing it to be used
in the [`ConfigOptions`] configuration tree.

`transform` is used to normalize values before parsing.

For example,

```ignore
config_namespace! {
   /// Amazing config
   pub struct MyConfig {
       /// Field 1 doc
       field1: String, transform = str::to_lowercase, default = "".to_string()

       /// Field 2 doc
       field2: usize, default = 232

       /// Field 3 doc
       field3: Option<usize>, default = None
   }
}
```

Will generate

```ignore
/// Amazing config
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MyConfig {
    /// Field 1 doc
    field1: String,
    /// Field 2 doc
    field2: usize,
    /// Field 3 doc
    field3: Option<usize>,
}
impl ConfigField for MyConfig {
    fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let (key, rem) = key.split_once('.').unwrap_or((key, ""));
        match key {
            "field1" => {
                let value = str::to_lowercase(value);
                self.field1.set(rem, value.as_ref())
            },
            "field2" => self.field2.set(rem, value.as_ref()),
            "field3" => self.field3.set(rem, value.as_ref()),
            _ => _internal_err!(
                "Config value \"{}\" not found on MyConfig",
                key
            ),
        }
    }

    fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str) {
        let key = format!("{}.field1", key_prefix);
        let desc = "Field 1 doc";
        self.field1.visit(v, key.as_str(), desc);
        let key = format!("{}.field2", key_prefix);
        let desc = "Field 2 doc";
        self.field2.visit(v, key.as_str(), desc);
        let key = format!("{}.field3", key_prefix);
        let desc = "Field 3 doc";
        self.field3.visit(v, key.as_str(), desc);
    }
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            field1: "".to_string(),
            field2: 232,
            field3: None,
        }
    }
}
```

NB: Misplaced commas may result in nonsensical errors

---

## context

`macro` · `datafusion_common::context`

Also reachable as `datafusion::common::context`

```rust
macro_rules! context
```

---

## create_array

`macro` · `datafusion_common::create_array`

Also reachable as `datafusion::common::create_array`

```rust
macro_rules! create_array
```

---

## downcast_value

`macro` · `datafusion_common::downcast_value`

Also reachable as `datafusion::common::downcast_value`

```rust
macro_rules! downcast_value
```

Downcast an Arrow Array to a concrete type, return an `DataFusionError::Internal` if the cast is
not possible. In normal usage of DataFusion the downcast should always succeed.

Example: `let array = downcast_value!(values, Int32Array)`

---

## exec_datafusion_err

`macro` · `datafusion_common::exec_datafusion_err`

Also reachable as `datafusion::common::exec_datafusion_err`

```rust
macro_rules! exec_datafusion_err
```

Macro wraps `$ERR` to add backtrace feature

---

## exec_err

`macro` · `datafusion_common::exec_err`

Also reachable as `datafusion::common::exec_err`

```rust
macro_rules! exec_err
```

Macro wraps Err(`$ERR`) to add backtrace feature

---

## extensions_options

`macro` · `datafusion_common::extensions_options`

Also reachable as `datafusion::common::extensions_options`

```rust
macro_rules! extensions_options
```

Convenience macro to create [`ExtensionsOptions`].

The created structure implements the following traits:

- [`Clone`]
- [`Debug`]
- [`Default`]
- [`ExtensionOptions`]

# Usage
The syntax is:

```text
extensions_options! {
     /// Struct docs (optional).
    [<vis>] struct <StructName> {
        /// Field docs (optional)
        [<vis>] <field_name>: <field_type>, default = <default_value>

        ... more fields
    }
}
```

The placeholders are:
- `[<vis>]`: Optional visibility modifier like `pub` or `pub(crate)`.
- `<StructName>`: Struct name like `MyStruct`.
- `<field_name>`: Field name like `my_field`.
- `<field_type>`: Field type like `u8`.
- `<default_value>`: Default value matching the field type like `42`.

# Example
See also a full example on the [`ConfigExtension`] documentation

```
use datafusion_common::extensions_options;

extensions_options! {
    /// My own config options.
    pub struct MyConfig {
        /// Should "foo" be replaced by "bar"?
        pub foo_to_bar: bool, default = true

        /// How many "baz" should be created?
        pub baz_count: usize, default = 1337
    }
}
```


[`Debug`]: std::fmt::Debug
[`ExtensionsOptions`]: crate::config::ExtensionOptions

---

## ffi_datafusion_err

`macro` · `datafusion_common::ffi_datafusion_err`

Also reachable as `datafusion::common::ffi_datafusion_err`

```rust
macro_rules! ffi_datafusion_err
```

Macro wraps `$ERR` to add backtrace feature

---

## ffi_err

`macro` · `datafusion_common::ffi_err`

Also reachable as `datafusion::common::ffi_err`

```rust
macro_rules! ffi_err
```

Macro wraps Err(`$ERR`) to add backtrace feature

---

## internal_datafusion_err

`macro` · `datafusion_common::internal_datafusion_err`

Also reachable as `datafusion::common::internal_datafusion_err`

```rust
macro_rules! internal_datafusion_err
```

Macro wraps `$ERR` to add backtrace feature

---

## internal_err

`macro` · `datafusion_common::internal_err`

Also reachable as `datafusion::common::internal_err`, `datafusion::physical_plan::internal_err`, `datafusion_physical_plan::execution_plan::internal_err`, `datafusion_physical_plan::internal_err`

```rust
macro_rules! internal_err
```

Macro wraps Err(`$ERR`) to add backtrace feature

---

## not_impl_datafusion_err

`macro` · `datafusion_common::not_impl_datafusion_err`

Also reachable as `datafusion::common::not_impl_datafusion_err`

```rust
macro_rules! not_impl_datafusion_err
```

Macro wraps `$ERR` to add backtrace feature

---

## not_impl_err

`macro` · `datafusion_common::not_impl_err`

Also reachable as `datafusion::common::not_impl_err`

```rust
macro_rules! not_impl_err
```

Macro wraps Err(`$ERR`) to add backtrace feature

---

## plan_datafusion_err

`macro` · `datafusion_common::plan_datafusion_err`

Also reachable as `datafusion::common::plan_datafusion_err`

```rust
macro_rules! plan_datafusion_err
```

Macro wraps `$ERR` to add backtrace feature

---

## plan_err

`macro` · `datafusion_common::plan_err`

Also reachable as `datafusion::common::plan_err`

```rust
macro_rules! plan_err
```

Macro wraps Err(`$ERR`) to add backtrace feature

---

## record_batch

`macro` · `datafusion_common::record_batch`

> **Deprecated** — since 55.0.0: Use `arrow::array::record_batch` instead

Also reachable as `datafusion::common::record_batch`

```rust
macro_rules! record_batch
```

Creates a record batch from literal slice of values, suitable for rapid
testing and development.

**Deprecated**: prefer the upstream macro from `arrow`,
[`arrow::array::record_batch`], which now supports both the literal slice
form shown below and a variable/expression form.

Example:
```
use arrow::array::record_batch;
let batch = record_batch!(
    ("a", Int32, vec![1, 2, 3]),
    ("b", Float64, vec![Some(4.0), None, Some(5.0)]),
    ("c", Utf8, vec!["alpha", "beta", "gamma"])
);
```

---

## resources_datafusion_err

`macro` · `datafusion_common::resources_datafusion_err`

Also reachable as `datafusion::common::resources_datafusion_err`

```rust
macro_rules! resources_datafusion_err
```

Macro wraps `$ERR` to add backtrace feature

---

## resources_err

`macro` · `datafusion_common::resources_err`

Also reachable as `datafusion::common::resources_err`

```rust
macro_rules! resources_err
```

Macro wraps Err(`$ERR`) to add backtrace feature

---

## schema_datafusion_err

`macro` · `datafusion_common::schema_datafusion_err`

Also reachable as `datafusion::common::schema_datafusion_err`

```rust
macro_rules! schema_datafusion_err
```

---

## schema_err

`macro` · `datafusion_common::schema_err`

Also reachable as `datafusion::common::schema_err`, `datafusion_common::error::_schema_err`

```rust
macro_rules! schema_err
```

---

## sql_datafusion_err

`macro` · `datafusion_common::sql_datafusion_err`

Also reachable as `datafusion::common::sql_datafusion_err`

```rust
macro_rules! sql_datafusion_err
```

---

## sql_err

`macro` · `datafusion_common::sql_err`

Also reachable as `datafusion::common::sql_err`

```rust
macro_rules! sql_err
```

---

## substrait_datafusion_err

`macro` · `datafusion_common::substrait_datafusion_err`

Also reachable as `datafusion::common::substrait_datafusion_err`

```rust
macro_rules! substrait_datafusion_err
```

Macro wraps `$ERR` to add backtrace feature

---

## substrait_err

`macro` · `datafusion_common::substrait_err`

Also reachable as `datafusion::common::substrait_err`

```rust
macro_rules! substrait_err
```

Macro wraps Err(`$ERR`) to add backtrace feature

---

## unwrap_or_internal_err

`macro` · `datafusion_common::unwrap_or_internal_err`

Also reachable as `datafusion::common::unwrap_or_internal_err`

```rust
macro_rules! unwrap_or_internal_err
```

Unwrap an `Option` if possible. Otherwise return an `DataFusionError::Internal`.
In normal usage of DataFusion the unwrap should always succeed.

Example: `let values = unwrap_or_internal_err!(values)`

---

## HashMap

`type_alias` · `datafusion_common::HashMap`

Also reachable as `datafusion::common::HashMap`

```rust
type HashMap<K, V, S = hashbrown::DefaultHashBuilder> = hashbrown::HashMap<K, V, S>
```

---

## HashSet

`type_alias` · `datafusion_common::HashSet`

Also reachable as `datafusion::common::HashSet`

```rust
type HashSet<T, S = hashbrown::DefaultHashBuilder> = hashbrown::HashSet<T, S>
```

---
