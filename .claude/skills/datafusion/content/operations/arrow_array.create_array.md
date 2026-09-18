# `arrow_array::create_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.create_array.json).

<a id="op-44a423b9f37b4517fb89d1cc"></a>
## create_array

`macro` · `arrow_array::create_array` · arrow-array 59.3.0

```rust
macro_rules! create_array
```

Source: `src/record_batch.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates an array from a literal slice of values,
suitable for rapid testing and development.

Example:

```rust

use arrow_array::create_array;

let array = create_array!(Int32, [1, 2, 3, 4, 5]);
let array = create_array!(Utf8, [Some("a"), Some("b"), None, Some("e")]);
```
Support for limited data types is available. The macro will return a compile error if an unsupported data type is used.
Presently supported data types are:
- `Boolean`, `Null`
- `Decimal32`, `Decimal64`, `Decimal128`, `Decimal256`
- `Float16`, `Float32`, `Float64`
- `Int8`, `Int16`, `Int32`, `Int64`
- `UInt8`, `UInt16`, `UInt32`, `UInt64`
- `IntervalDayTime`, `IntervalYearMonth`
- `Second`, `Millisecond`, `Microsecond`, `Nanosecond`
- `Second32`, `Millisecond32`, `Microsecond64`, `Nanosecond64`
- `DurationSecond`, `DurationMillisecond`, `DurationMicrosecond`, `DurationNanosecond`
- `TimestampSecond`, `TimestampMillisecond`, `TimestampMicrosecond`, `TimestampNanosecond`
- `Utf8`, `Utf8View`, `LargeUtf8`, `Binary`, `LargeBinary`
