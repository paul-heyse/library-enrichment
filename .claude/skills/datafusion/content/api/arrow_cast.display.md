# `arrow_cast::display`

Crate `arrow-cast` · 10 public items · structured records in [`model/arrow_cast.display.json`](../model/arrow_cast.display.json)

## DurationFormat

`enum` · `arrow_cast::display::DurationFormat`

Also reachable as `arrow::util::display::DurationFormat`

```rust
enum DurationFormat
```

**Variants**: `ISO8601`, `Pretty`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.DurationFormat.md).


Format for displaying durations

---

## FormatError

`enum` · `arrow_cast::display::FormatError`

Also reachable as `arrow::util::display::FormatError`

```rust
enum FormatError
```

**Variants**: `Format`, `Arrow`

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(value: ArrowError) -> Self
fn from(value: std::fmt::Error) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.FormatError.md).


Either an [`ArrowError`] or [`std::fmt::Error`]

---

## array_value_to_string

`function` · `arrow_cast::display::array_value_to_string`

Also reachable as `arrow::util::display::array_value_to_string`

```rust
fn array_value_to_string(column: &dyn Array, row: usize) -> Result<String, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.array_value_to_string.md).


Get the value at the given row in an array as a String.

Note this function is quite inefficient and is unlikely to be
suitable for converting large arrays or record batches.

Please see [`ArrayFormatter`] for a more performant interface

---

## lexical_to_string

`function` · `arrow_cast::display::lexical_to_string`

Also reachable as `arrow::util::display::lexical_to_string`

```rust
fn lexical_to_string<N: lexical_core::ToLexical>(n: N) -> String
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.lexical_to_string.md).


Converts numeric type to a `String`

---

## ArrayFormatter

`struct` · `arrow_cast::display::ArrayFormatter`

Also reachable as `arrow::util::display::ArrayFormatter`

```rust
struct ArrayFormatter<'a>
```

**Methods** (3)

```rust
fn new(format: Box<dyn DisplayIndex + 'a>, safe: bool) -> Self
fn try_new(array: &'a dyn Array, options: &FormatOptions<'a>) -> Result<Self, ArrowError>
fn value(&self, idx: usize) -> ValueFormatter<'_>
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.ArrayFormatter.md).


A string formatter for an [`Array`]

This can be used with [`std::write`] to write type-erased `dyn Array`

```
# use std::fmt::{Display, Formatter, Write};
# use arrow_array::{Array, ArrayRef, Int32Array};
# use arrow_cast::display::{ArrayFormatter, FormatOptions};
# use arrow_schema::ArrowError;
struct MyContainer {
    values: ArrayRef,
}

impl Display for MyContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let options = FormatOptions::default();
        let formatter = ArrayFormatter::try_new(self.values.as_ref(), &options)
            .map_err(|_| std::fmt::Error)?;

        let mut iter = 0..self.values.len();
        if let Some(idx) = iter.next() {
            write!(f, "{}", formatter.value(idx))?;
        }
        for idx in iter {
            write!(f, ", {}", formatter.value(idx))?;
        }
        Ok(())
    }
}
```

[`ValueFormatter::write`] can also be used to get a semantic error, instead of the
opaque [`std::fmt::Error`]

```
# use std::fmt::Write;
# use arrow_array::Array;
# use arrow_cast::display::{ArrayFormatter, FormatOptions};
# use arrow_schema::ArrowError;
fn format_array(
    f: &mut dyn Write,
    array: &dyn Array,
    options: &FormatOptions,
) -> Result<(), ArrowError> {
    let formatter = ArrayFormatter::try_new(array, options)?;
    for i in 0..array.len() {
        formatter.value(i).write(f)?
    }
    Ok(())
}
```

---

## FormatOptions

`struct` · `arrow_cast::display::FormatOptions`

Also reachable as `arrow::util::display::FormatOptions`

```rust
struct FormatOptions<'a>
```

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq

**Methods** (23)

```rust
const fn date_format(&self) -> Option<&'a str>
const fn datetime_format(&self) -> Option<&'a str>
const fn duration_format(&self) -> DurationFormat
const fn formatter_factory(&self) -> Option<&'a dyn ArrayFormatterFactory>
const fn new() -> Self
const fn null(&self) -> &'a str
const fn quoted_strings(&self) -> bool
const fn safe(&self) -> bool
const fn time_format(&self) -> Option<&'a str>
const fn timestamp_format(&self) -> Option<&'a str>
const fn timestamp_tz_format(&self) -> Option<&'a str>
const fn types_info(&self) -> bool
const fn with_date_format(self, date_format: Option<&'a str>) -> Self
const fn with_datetime_format(self, datetime_format: Option<&'a str>) -> Self
const fn with_display_error(self, safe: bool) -> Self
const fn with_duration_format(self, duration_format: DurationFormat) -> Self
const fn with_formatter_factory(self, formatter_factory: Option<&'a dyn ArrayFormatterFactory>) -> Self
const fn with_null(self, null: &'a str) -> Self
const fn with_quoted_strings(self, quoted_strings: bool) -> Self
const fn with_time_format(self, time_format: Option<&'a str>) -> Self
const fn with_timestamp_format(self, timestamp_format: Option<&'a str>) -> Self
const fn with_timestamp_tz_format(self, timestamp_tz_format: Option<&'a str>) -> Self
const fn with_types_info(self, types_info: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.FormatOptions.md).


Options for formatting arrays

By default nulls are formatted as `""` and temporal types formatted
according to RFC3339

# Equality

Most fields in [`FormatOptions`] are compared by value, except `formatter_factory`. As the trait
does not require an [`Eq`] and [`Hash`] implementation, this struct only compares the pointer of
the factories.

---

## ValueFormatter

`struct` · `arrow_cast::display::ValueFormatter`

Also reachable as `arrow::util::display::ValueFormatter`

```rust
struct ValueFormatter<'a>
```

**Implements**: `core::fmt::Display`

**Methods** (2)

```rust
fn try_to_string(&self) -> Result<String, ArrowError>
fn write(&self, s: &mut dyn Write) -> Result<(), ArrowError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.ValueFormatter.md).


Implements [`Display`] for a specific array value

---

## ArrayFormatterFactory

`trait` · `arrow_cast::display::ArrayFormatterFactory`

Also reachable as `arrow::util::display::ArrayFormatterFactory`

```rust
trait ArrayFormatterFactory: Debug + Send + Sync
```

**Implementors** (1)

- `datafusion_expr::extension_types::array_formatter_factory::DFArrayFormatterFactory`

**Methods** (1)

```rust
fn create_array_formatter<'formatter>(&self, array: &'formatter dyn Array, options: &FormatOptions<'formatter>, field: Option<&'formatter Field>) -> Result<Option<ArrayFormatter<'formatter>>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.ArrayFormatterFactory.md).


Allows creating a new [`ArrayFormatter`] for a given [`Array`] and an optional [`Field`].

# Example

The example below shows how to create a custom formatter for a custom type `my_money`. Note that
this example requires the `prettyprint` feature.

```rust
# #[cfg(feature = "prettyprint")]{
use std::fmt::Write;
use arrow_array::{cast::AsArray, Array, Int32Array};
use arrow_cast::display::{ArrayFormatter, ArrayFormatterFactory, DisplayIndex, FormatOptions, FormatResult};
use arrow_cast::pretty::pretty_format_batches_with_options;
use arrow_schema::{ArrowError, Field};

/// A custom formatter factory that can create a formatter for the special type `my_money`.
///
/// This struct could have access to some kind of extension type registry that can lookup the
/// correct formatter for an extension type on-demand.
#[derive(Debug)]
struct MyFormatters {}

impl ArrayFormatterFactory for MyFormatters {
    fn create_array_formatter<'formatter>(
        &self,
        array: &'formatter dyn Array,
        options: &FormatOptions<'formatter>,
        field: Option<&'formatter Field>,
    ) -> Result<Option<ArrayFormatter<'formatter>>, ArrowError> {
        // check if this is the money type
        if field
            .map(|f| f.extension_type_name() == Some("my_money"))
            .unwrap_or(false)
        {
            // We assume that my_money always is an Int32.
            let array = array.as_primitive();
            let display_index = Box::new(MyMoneyFormatter { array, options: options.clone() });
            return Ok(Some(ArrayFormatter::new(display_index, options.safe())));
        }

        Ok(None) // None indicates that the default formatter should be used.
    }
}

/// A formatter for the type `my_money` that wraps a specific array and has access to the
/// formatting options.
struct MyMoneyFormatter<'a> {
    array: &'a Int32Array,
    options: FormatOptions<'a>,
}

impl<'a> DisplayIndex for MyMoneyFormatter<'a> {
    fn write(&self, idx: usize, f: &mut dyn Write) -> FormatResult {
        match self.array.is_valid(idx) {
            true => write!(f, "{} €", self.array.value(idx))?,
            false => write!(f, "{}", self.options.null())?,
        }

        Ok(())
    }
}

// Usually, here you would provide your record batches.
let my_batches = vec![];

// Call the pretty printer with the custom formatter factory.
pretty_format_batches_with_options(
       &my_batches,
       &FormatOptions::new().with_formatter_factory(Some(&MyFormatters {}))
);
# }
```

---

## DisplayIndex

`trait` · `arrow_cast::display::DisplayIndex`

Also reachable as `arrow::util::display::DisplayIndex`

```rust
trait DisplayIndex
```

**Methods** (1)

```rust
fn write(&self, idx: usize, f: &mut dyn Write) -> FormatResult
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.DisplayIndex.md).


[`Display`] but accepting an index

---

## FormatResult

`type_alias` · `arrow_cast::display::FormatResult`

Also reachable as `arrow::util::display::FormatResult`

```rust
type FormatResult = Result<(), FormatError>
```

[Full member, field, variant and typed contracts](../operations/arrow_cast.display.FormatResult.md).


The result of formatting an array element via [`DisplayIndex::write`].

---
