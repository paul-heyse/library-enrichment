# `arrow_array::array::primitive_array::TimestampSecondArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.TimestampSecondArray.json).

<a id="op-cd3c04f65a314f4d084ab6eb"></a>
## TimestampSecondArray

`type_alias` · `arrow_array::array::primitive_array::TimestampSecondArray` · arrow-array 59.3.0

```rust
type TimestampSecondArray = PrimitiveArray<TimestampSecondType>
```

Source: `src/array/primitive_array.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of seconds since UNIX epoch stored as `i64`

This type is similar to the [`chrono::DateTime`] type and can hold
values such as `1970-05-09 14:25:11 +01:00`

See also [`Timestamp`](arrow_schema::DataType::Timestamp).

# Example: UTC timestamps post epoch
```
# use arrow_array::TimestampSecondArray;
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1970-05-09T14:25:11+0:00
let arr = TimestampSecondArray::from(vec![11111111]);
// OR
let arr = TimestampSecondArray::from(vec![Some(11111111)]);
let utc_tz: Tz = "+00:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, utc_tz).map(|v| v.to_string()).unwrap(), "1970-05-09 14:25:11 +00:00")
```

# Example: UTC timestamps pre epoch
```
# use arrow_array::TimestampSecondArray;
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1969-08-25T09:34:49+0:00
let arr = TimestampSecondArray::from(vec![-11111111]);
// OR
let arr = TimestampSecondArray::from(vec![Some(-11111111)]);
let utc_tz: Tz = "+00:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, utc_tz).map(|v| v.to_string()).unwrap(), "1969-08-25 09:34:49 +00:00")
```

# Example: With timezone specified
```
# use arrow_array::TimestampSecondArray;
use arrow_array::timezone::Tz;
// Corresponds to single element array with entry 1970-05-10T00:25:11+10:00
let arr = TimestampSecondArray::from(vec![11111111]).with_timezone("+10:00".to_string());
// OR
let arr = TimestampSecondArray::from(vec![Some(11111111)]).with_timezone("+10:00".to_string());
let sydney_tz: Tz = "+10:00".parse().unwrap();

assert_eq!(arr.value_as_datetime_with_tz(0, sydney_tz).map(|v| v.to_string()).unwrap(), "1970-05-10 00:25:11 +10:00")
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples

Unresolved upstream links (retained, not inferred): ``chrono::DateTime``.
