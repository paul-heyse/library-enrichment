# `arrow_cast::parse::string_to_time_nanoseconds`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.string_to_time_nanoseconds.json).

<a id="op-d1558b37f0113b5567f62c16"></a>
## string_to_time_nanoseconds

`function` · `arrow_cast::parse::string_to_time_nanoseconds` · arrow-cast 59.3.0

```rust
fn string_to_time_nanoseconds(s: &str) -> Result<i64, arrow_schema::ArrowError>
```

Source: `src/parse.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Accepts a string in ISO8601 standard format and some
variants and converts it to nanoseconds since midnight.

Examples of accepted inputs:

* `09:26:56.123 AM`
* `23:59:59`
* `6:00 pm`

Internally, this function uses the `chrono` library for the time parsing

## Timezone / Offset Handling

This function does not support parsing strings with a timezone
or offset specified, as it considers only time since midnight.
