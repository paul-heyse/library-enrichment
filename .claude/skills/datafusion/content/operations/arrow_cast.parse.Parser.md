# `arrow_cast::parse::Parser`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.Parser.json).

<a id="op-0973c2e1c76346a85ddd306f"></a>
## Parser

`trait` · `arrow_cast::parse::Parser` · arrow-cast 59.3.0

```rust
trait Parser: ArrowPrimitiveType
```

Source: `src/parse.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Specialized parsing implementations to convert strings to Arrow types.

This is used by csv and json reader and can be used directly as well.

# Example

To parse a string to a [`Date32Type`](../operations/arrow_array.types.Date32Type.md#op-7798afce101ac085c553ac62):

```
use arrow_cast::parse::Parser;
use arrow_array::types::Date32Type;
let date = Date32Type::parse("2021-01-01").unwrap();
assert_eq!(date, 18628);
```

To parse a string to a [`TimestampNanosecondType`](../operations/arrow_array.types.TimestampNanosecondType.md#op-2297d2d9d299c45310789b07):

```
use arrow_cast::parse::Parser;
use arrow_array::types::TimestampNanosecondType;
let ts = TimestampNanosecondType::parse("2021-01-01T00:00:00.123456789Z").unwrap();
assert_eq!(ts, 1609459200123456789);
```

<a id="op-6d2b6cf3ef53d24858ea1dac"></a>
## parse

`function` · `arrow_cast::parse::Parser::parse` · arrow-cast 59.3.0

```rust
fn parse(string: &str) -> Option<Self::Native>
```

Source: `src/parse.rs:436`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Parse a string to the native type

<a id="op-19f12513c67092bc98ece526"></a>
## parse_formatted

`function` · `arrow_cast::parse::Parser::parse_formatted` · arrow-cast 59.3.0

```rust
fn parse_formatted(string: &str, _format: &str) -> Option<Self::Native>
```

Source: `src/parse.rs:441`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Parse a string to the native type with a format string

When not implemented, the format string is unused, and this method is equivalent to [parse](#tymethod.parse)
