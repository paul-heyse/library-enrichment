# `arrow_cast::display::ArrayFormatterFactory`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.display.ArrayFormatterFactory.json).

<a id="op-981732bb15cd351972b89354"></a>
## ArrayFormatterFactory

`trait` · `arrow_cast::display::ArrayFormatterFactory` · arrow-cast 59.3.0

```rust
trait ArrayFormatterFactory: Debug + Send + Sync
```

Source: `src/display.rs:378`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Allows creating a new [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992) for a given [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) and an optional [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf).

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

<a id="op-84219aff82348f6fb0446fdf"></a>
## create_array_formatter

`function` · `arrow_cast::display::ArrayFormatterFactory::create_array_formatter` · arrow-cast 59.3.0

```rust
fn create_array_formatter<'formatter>(&self, array: &'formatter dyn Array, options: &FormatOptions<'formatter>, field: Option<&'formatter Field>) -> Result<Option<ArrayFormatter<'formatter>>, ArrowError>
```

Source: `src/display.rs:385`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Creates a new [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992) for the given [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) and an optional [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf). If the
default implementation should be used, return [`None`].

The field shall be used to look up metadata about the `array` while `options` provide
information on formatting, for example, dates and times which should be considered by an
implementor.

Unresolved upstream links (retained, not inferred): ``None``.
