# `arrow_csv::writer::WriterBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.writer.WriterBuilder.json).

<a id="op-44e2845dc1cd29d45ec5dba4"></a>
## WriterBuilder

`struct` · `arrow_csv::writer::WriterBuilder` · arrow-csv 59.3.0

```rust
struct WriterBuilder
```

Source: `src/writer.rs:352`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

A CSV writer builder

<a id="op-68ad3bc09f82bd5fc9e74c54"></a>
## build

`function` · `arrow_csv::writer::WriterBuilder::build` · arrow-csv 59.3.0

```rust
fn build<W: Write>(self, writer: W) -> Writer<W>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:636`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Create a new `Writer`

<a id="op-eadaf151511a5fba5dfe5b36"></a>
## clone

`function` · `arrow_csv::writer::WriterBuilder::clone` · arrow-csv 59.3.0

```rust
fn clone(&self) -> WriterBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 10], "end": [351, 15], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer.rs:351`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a780d0668c36492f9a18acce"></a>
## date_format

`function` · `arrow_csv::writer::WriterBuilder::date_format` · arrow-csv 59.3.0

```rust
fn date_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:516`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's date format if set, defaults to RFC3339

<a id="op-14d6b0681c0b898d012340f1"></a>
## datetime_format

`function` · `arrow_csv::writer::WriterBuilder::datetime_format` · arrow-csv 59.3.0

```rust
fn datetime_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:527`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's datetime format if set, defaults to RFC3339

<a id="op-6ab6a6697d85ecd53c67933c"></a>
## default

`function` · `arrow_csv::writer::WriterBuilder::default` · arrow-csv 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 1], "end": [414, 2], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer.rs:395`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6230be8ad1114b459c668444"></a>
## delimiter

`function` · `arrow_csv::writer::WriterBuilder::delimiter` · arrow-csv 59.3.0

```rust
fn delimiter(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:460`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's column delimiter as a byte character

<a id="op-9899c6b92a5ec4c8c534e1c9"></a>
## double_quote

`function` · `arrow_csv::writer::WriterBuilder::double_quote` · arrow-csv 59.3.0

```rust
fn double_quote(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:505`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get whether double quote escapes are enabled

<a id="op-899b682a75843c006502cdfb"></a>
## escape

`function` · `arrow_csv::writer::WriterBuilder::escape` · arrow-csv 59.3.0

```rust
fn escape(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:488`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's escape character as a byte character

<a id="op-42d8567a2b3cf2b8809658c7"></a>
## fmt

`function` · `arrow_csv::writer::WriterBuilder::fmt` · arrow-csv 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 17], "end": [351, 22], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer.rs:351`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01c04639a1c72e59c283913f"></a>
## header

`function` · `arrow_csv::writer::WriterBuilder::header` · arrow-csv 59.3.0

```rust
fn header(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:449`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Returns `true` if this writer is configured to write a header

<a id="op-91080e79b71b15fbc8ae6561"></a>
## ignore_leading_whitespace

`function` · `arrow_csv::writer::WriterBuilder::ignore_leading_whitespace` · arrow-csv 59.3.0

```rust
fn ignore_leading_whitespace(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:583`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get whether to ignore leading whitespace in string values

<a id="op-ddc874934d0496c020877c0b"></a>
## ignore_trailing_whitespace

`function` · `arrow_csv::writer::WriterBuilder::ignore_trailing_whitespace` · arrow-csv 59.3.0

```rust
fn ignore_trailing_whitespace(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:595`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get whether to ignore trailing whitespace in string values

<a id="op-dabc94b88a2586646d078f09"></a>
## line_terminator

`function` · `arrow_csv::writer::WriterBuilder::line_terminator` · arrow-csv 59.3.0

```rust
fn line_terminator(&self) -> &Terminator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:631`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's line terminator, defaults to `LF` (`\n`)

<a id="op-aa7092823d34bc850b2d6f39"></a>
## new

`function` · `arrow_csv::writer::WriterBuilder::new` · arrow-csv 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:438`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Create a new builder for configuring CSV [`Writer`](../operations/arrow_csv.writer.Writer.md#op-9112f918255e66827bb62542) options.

To convert a builder into a writer, call [`WriterBuilder::build`](../operations/arrow_csv.writer.WriterBuilder.md#op-68ad3bc09f82bd5fc9e74c54). See
the [module documentation](crate::writer) for more examples.

# Example

```
# use arrow_csv::{Writer, WriterBuilder};
# use std::fs::File;

fn example() -> Writer<File> {
    let file = File::create("target/out.csv").unwrap();

    // create a builder that doesn't write headers
    let builder = WriterBuilder::new().with_header(false);
    let writer = builder.build(file);

    writer
}
```

<a id="op-087372e15835745a86406762"></a>
## null

`function` · `arrow_csv::writer::WriterBuilder::null` · arrow-csv 59.3.0

```rust
fn null(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:571`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the value to represent null in output

<a id="op-172f322d0611b2a95a2ce844"></a>
## quote

`function` · `arrow_csv::writer::WriterBuilder::quote` · arrow-csv 59.3.0

```rust
fn quote(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:471`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's quote character as a byte character

<a id="op-e85fd89ae0024e7efeae102e"></a>
## quote_style

`function` · `arrow_csv::writer::WriterBuilder::quote_style` · arrow-csv 59.3.0

```rust
fn quote_style(&self) -> QuoteStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:620`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the configured quoting style

<a id="op-dda82d47326348258d2bb5af"></a>
## time_format

`function` · `arrow_csv::writer::WriterBuilder::time_format` · arrow-csv 59.3.0

```rust
fn time_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:538`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's datetime time if set, defaults to RFC3339

<a id="op-8edc70796ef8e76892e605b8"></a>
## timestamp_format

`function` · `arrow_csv::writer::WriterBuilder::timestamp_format` · arrow-csv 59.3.0

```rust
fn timestamp_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:549`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's timestamp format if set, defaults to RFC3339

<a id="op-d7a9d27c2737be50328c6297"></a>
## timestamp_tz_format

`function` · `arrow_csv::writer::WriterBuilder::timestamp_tz_format` · arrow-csv 59.3.0

```rust
fn timestamp_tz_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:560`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Get the CSV file's timestamp tz format if set, defaults to RFC3339

<a id="op-93466d41ed2313c6346e4111"></a>
## with_date_format

`function` · `arrow_csv::writer::WriterBuilder::with_date_format` · arrow-csv 59.3.0

```rust
fn with_date_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's date format

<a id="op-a5bc6f722256967cd720fed2"></a>
## with_datetime_format

`function` · `arrow_csv::writer::WriterBuilder::with_datetime_format` · arrow-csv 59.3.0

```rust
fn with_datetime_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:521`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's datetime format

<a id="op-9fe6dfd650b104723f7078d5"></a>
## with_delimiter

`function` · `arrow_csv::writer::WriterBuilder::with_delimiter` · arrow-csv 59.3.0

```rust
fn with_delimiter(self, delimiter: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:454`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's column delimiter as a byte character

<a id="op-0010c18e29b3702198a66a19"></a>
## with_double_quote

`function` · `arrow_csv::writer::WriterBuilder::with_double_quote` · arrow-csv 59.3.0

```rust
fn with_double_quote(self, double_quote: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:499`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set whether to enable double quote escapes

When enabled (which is the default), quotes are escaped by doubling
them. e.g., `"` escapes to `""`.

When disabled, quotes are escaped with the escape character (which
is `\\` by default).

<a id="op-e8b8344dec3159108f1db6dd"></a>
## with_escape

`function` · `arrow_csv::writer::WriterBuilder::with_escape` · arrow-csv 59.3.0

```rust
fn with_escape(self, escape: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:482`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's escape character as a byte character

In some variants of CSV, quotes are escaped using a special escape
character like `\` (instead of escaping quotes by doubling them).

By default, writing these idiosyncratic escapes is disabled, and is
only used when `double_quote` is disabled.

<a id="op-0febf81cffc55f9c25fa9e22"></a>
## with_header

`function` · `arrow_csv::writer::WriterBuilder::with_header` · arrow-csv 59.3.0

```rust
fn with_header(self, header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:443`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set whether to write the CSV file with a header

<a id="op-f03e00afcb321ce44e684c77"></a>
## with_ignore_leading_whitespace

`function` · `arrow_csv::writer::WriterBuilder::with_ignore_leading_whitespace` · arrow-csv 59.3.0

```rust
fn with_ignore_leading_whitespace(self, ignore: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:577`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set whether to ignore leading whitespace in string values
For example, a string value such as "   foo" will be written as "foo"

<a id="op-7c15e0f47e7daa6481ab2a4d"></a>
## with_ignore_trailing_whitespace

`function` · `arrow_csv::writer::WriterBuilder::with_ignore_trailing_whitespace` · arrow-csv 59.3.0

```rust
fn with_ignore_trailing_whitespace(self, ignore: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:589`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set whether to ignore trailing whitespace in string values
For example, a string value such as "foo    " will be written as "foo"

<a id="op-8e75ce61d96b1e4a38ad4077"></a>
## with_line_terminator

`function` · `arrow_csv::writer::WriterBuilder::with_line_terminator` · arrow-csv 59.3.0

```rust
fn with_line_terminator(self, terminator: Terminator) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:625`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's line terminator

<a id="op-c89b18271afb00f5f86b068f"></a>
## with_null

`function` · `arrow_csv::writer::WriterBuilder::with_null` · arrow-csv 59.3.0

```rust
fn with_null(self, null_value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:565`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the value to represent null in output

<a id="op-6b16294e64f6abfa9a9c7170"></a>
## with_quote

`function` · `arrow_csv::writer::WriterBuilder::with_quote` · arrow-csv 59.3.0

```rust
fn with_quote(self, quote: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:465`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's quote character as a byte character

<a id="op-7b196c4e12b392ebea59af38"></a>
## with_quote_style

`function` · `arrow_csv::writer::WriterBuilder::with_quote_style` · arrow-csv 59.3.0

```rust
fn with_quote_style(self, quote_style: QuoteStyle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:614`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the quoting style for writing CSV files

# Example

```
use arrow_csv::{WriterBuilder, QuoteStyle};

// Quote all fields (equivalent to Spark's quoteAll=true)
let builder = WriterBuilder::new()
    .with_quote_style(QuoteStyle::Always);

// Only quote when necessary (default)
let builder = WriterBuilder::new()
    .with_quote_style(QuoteStyle::Necessary);
```

<a id="op-886a5946a628e8de262adb0d"></a>
## with_time_format

`function` · `arrow_csv::writer::WriterBuilder::with_time_format` · arrow-csv 59.3.0

```rust
fn with_time_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:532`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's time format

<a id="op-36864c99ad09fcd7a143987f"></a>
## with_timestamp_format

`function` · `arrow_csv::writer::WriterBuilder::with_timestamp_format` · arrow-csv 59.3.0

```rust
fn with_timestamp_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:543`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's timestamp format

<a id="op-ae060241791ce56ac37b87bc"></a>
## with_timestamp_tz_format

`function` · `arrow_csv::writer::WriterBuilder::with_timestamp_tz_format` · arrow-csv 59.3.0

```rust
fn with_timestamp_tz_format(self, tz_format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [666, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:554`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's timestamp tz format
