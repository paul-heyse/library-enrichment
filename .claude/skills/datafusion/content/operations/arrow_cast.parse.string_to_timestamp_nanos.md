# `arrow_cast::parse::string_to_timestamp_nanos`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.string_to_timestamp_nanos.json).

<a id="op-ca6af032bdb822be5bba613a"></a>
## string_to_timestamp_nanos

`function` · `arrow_cast::parse::string_to_timestamp_nanos` · arrow-cast 59.3.0

```rust
fn string_to_timestamp_nanos(s: &str) -> Result<i64, arrow_schema::ArrowError>
```

Source: `src/parse.rs:272`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Accepts a string in RFC3339 / ISO8601 standard format and some
variants and converts it to a nanosecond precision timestamp.

See [`string_to_datetime`](../operations/arrow_cast.parse.string_to_datetime.md#op-85f0bec14a3d565f3c9986a6) for the full set of supported formats

Implements the `to_timestamp` function to convert a string to a
timestamp, following the model of spark SQL’s to_`timestamp`.

Internally, this function uses the `chrono` library for the
datetime parsing

We hope to extend this function in the future with a second
parameter to specifying the format string.

## Timestamp Precision

Function uses the maximum precision timestamps supported by
Arrow (nanoseconds stored as a 64-bit integer) timestamps. This
means the range of dates that timestamps can represent is ~1677 AD
to 2262 AM

## Timezone / Offset Handling

Numerical values of timestamps are stored compared to offset UTC.

This function interprets string without an explicit time zone as timestamps
relative to UTC, see [`string_to_datetime`](../operations/arrow_cast.parse.string_to_datetime.md#op-85f0bec14a3d565f3c9986a6) for alternative semantics

In particular:

```
# use arrow_cast::parse::string_to_timestamp_nanos;
// Note all three of these timestamps are parsed as the same value
let a = string_to_timestamp_nanos("1997-01-31 09:26:56.123Z").unwrap();
let b = string_to_timestamp_nanos("1997-01-31T09:26:56.123").unwrap();
let c = string_to_timestamp_nanos("1997-01-31T14:26:56.123+05:00").unwrap();

assert_eq!(a, b);
assert_eq!(b, c);
```

