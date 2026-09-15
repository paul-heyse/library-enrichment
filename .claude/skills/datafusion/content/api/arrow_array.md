# `arrow_array`

Crate `arrow-array` · 12 public items · structured records in [`model/arrow_array.json`](../model/arrow_array.json)

## create_array

`macro` · `arrow_array::create_array`

Also reachable as `arrow::array::create_array`

```rust
macro_rules! create_array
```

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

---

## downcast_dictionary_array

`macro` · `arrow_array::downcast_dictionary_array`

Also reachable as `arrow::array::downcast_dictionary_array`, `arrow::downcast_dictionary_array`, `datafusion::arrow::downcast_dictionary_array`, `datafusion_common::arrow::downcast_dictionary_array`

```rust
macro_rules! downcast_dictionary_array
```

Downcast an [`Array`] to a [`DictionaryArray`] based on its [`DataType`], accepts
a number of subsequent patterns to match the data type

```
# use arrow_array::{Array, StringArray, downcast_dictionary_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_strings(array: &dyn Array) {
    downcast_dictionary_array!(
        array => match array.values().data_type() {
            DataType::Utf8 => {
                for v in array.downcast_dict::<StringArray>().unwrap() {
                    println!("{:?}", v);
                }
            }
            t => println!("Unsupported dictionary value type {}", t),
        },
        DataType::Utf8 => {
            for v in as_string_array(array) {
                println!("{:?}", v);
            }
        }
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => {
            for v in as_largestring_array(array) {
                println!("{:?}", v);
            }
        }
        t => println!("Unsupported datatype {}", t)
    )
}
```

[`DataType`]: arrow_schema::DataType

---

## downcast_integer

`macro` · `arrow_array::downcast_integer`

Also reachable as `arrow::array::downcast_integer`

```rust
macro_rules! downcast_integer
```

Given one or more expressions evaluating to an integer [`DataType`] invokes the provided macro
`m` with the corresponding integer [`ArrowPrimitiveType`], followed by any additional arguments

```
# use arrow_array::{downcast_primitive, ArrowPrimitiveType, downcast_integer};
# use arrow_schema::DataType;

macro_rules! dictionary_key_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn dictionary_key_size(t: &DataType) -> u8 {
    match t {
        DataType::Dictionary(k, _) => downcast_integer! {
            k.as_ref() => (dictionary_key_size_helper, u8),
            _ => unreachable!(),
        },
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX,
    }
}

assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8))), 4);
assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::Int64), Box::new(DataType::Utf8))), 8);
assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::UInt16), Box::new(DataType::Utf8))), 2);
```

[`DataType`]: arrow_schema::DataType

---

## downcast_integer_array

`macro` · `arrow_array::downcast_integer_array`

Also reachable as `arrow::array::downcast_integer_array`

```rust
macro_rules! downcast_integer_array
```

Given one or more expressions evaluating to an integer [`PrimitiveArray`] invokes the provided macro
with the corresponding array, along with match statements for any non integer array types

```
# use arrow_array::{Array, downcast_integer_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_integer(array: &dyn Array) {
    downcast_integer_array!(
        array => {
            for v in array {
                println!("{:?}", v);
            }
        }
        DataType::Utf8 => {
            for v in as_string_array(array) {
                println!("{:?}", v);
            }
        }
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => {
            for v in as_largestring_array(array) {
                println!("{:?}", v);
            }
        }
        t => println!("Unsupported datatype {}", t)
    )
}
```

[`DataType`]: arrow_schema::DataType

---

## downcast_primitive

`macro` · `arrow_array::downcast_primitive`

Also reachable as `arrow::array::downcast_primitive`

```rust
macro_rules! downcast_primitive
```

Given one or more expressions evaluating to primitive [`DataType`] invokes the provided macro
`m` with the corresponding [`ArrowPrimitiveType`], followed by any additional arguments

```
# use arrow_array::{downcast_primitive, ArrowPrimitiveType};
# use arrow_schema::DataType;

macro_rules! primitive_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn primitive_size(t: &DataType) -> u8 {
    downcast_primitive! {
        t => (primitive_size_helper, u8),
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX
    }
}

assert_eq!(primitive_size(&DataType::Int32), 4);
assert_eq!(primitive_size(&DataType::Int64), 8);
assert_eq!(primitive_size(&DataType::Float16), 2);
assert_eq!(primitive_size(&DataType::Decimal128(38, 10)), 16);
assert_eq!(primitive_size(&DataType::Decimal256(76, 20)), 32);
```

[`DataType`]: arrow_schema::DataType

---

## downcast_primitive_array

`macro` · `arrow_array::downcast_primitive_array`

Also reachable as `arrow::array::downcast_primitive_array`, `arrow::downcast_primitive_array`, `datafusion::arrow::downcast_primitive_array`, `datafusion_common::arrow::downcast_primitive_array`

```rust
macro_rules! downcast_primitive_array
```

Downcast an [`Array`] to a [`PrimitiveArray`] based on its [`DataType`]
accepts a number of subsequent patterns to match the data type

```
# use arrow_array::{Array, downcast_primitive_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_primitive(array: &dyn Array) {
    downcast_primitive_array!(
        array => {
            for v in array {
                println!("{:?}", v);
            }
        }
        DataType::Utf8 => {
            for v in as_string_array(array) {
                println!("{:?}", v);
            }
        }
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => {
            for v in as_largestring_array(array) {
                println!("{:?}", v);
            }
        }
        t => println!("Unsupported datatype {}", t)
    )
}
```

[`DataType`]: arrow_schema::DataType

---

## downcast_run_array

`macro` · `arrow_array::downcast_run_array`

Also reachable as `arrow::array::downcast_run_array`

```rust
macro_rules! downcast_run_array
```

Downcast an [`Array`] to a [`RunArray`] based on its [`DataType`], accepts
a number of subsequent patterns to match the data type

```
# use arrow_array::{Array, StringArray, downcast_run_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_strings(array: &dyn Array) {
    downcast_run_array!(
        array => match array.values().data_type() {
            DataType::Utf8 => {
                for v in array.downcast::<StringArray>().unwrap() {
                    println!("{:?}", v);
                }
            }
            t => println!("Unsupported run array value type {}", t),
        },
        DataType::Utf8 => {
            for v in as_string_array(array) {
                println!("{:?}", v);
            }
        }
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => {
            for v in as_largestring_array(array) {
                println!("{:?}", v);
            }
        }
        t => println!("Unsupported datatype {}", t)
    )
}
```

[`DataType`]: arrow_schema::DataType

---

## downcast_run_end_index

`macro` · `arrow_array::downcast_run_end_index`

Also reachable as `arrow::array::downcast_run_end_index`

```rust
macro_rules! downcast_run_end_index
```

Given one or more expressions evaluating to an integer [`DataType`] invokes the provided macro
`m` with the corresponding integer [`RunEndIndexType`], followed by any additional arguments

```
# use std::sync::Arc;
# use arrow_array::{downcast_primitive, ArrowPrimitiveType, downcast_run_end_index};
# use arrow_schema::{DataType, Field};

macro_rules! run_end_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn run_end_index_size(t: &DataType) -> u8 {
    match t {
        DataType::RunEndEncoded(k, _) => downcast_run_end_index! {
            k.data_type() => (run_end_size_helper, u8),
            _ => unreachable!(),
        },
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX,
    }
}

assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int32, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 4);
assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int64, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 8);
assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int16, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 2);
```

[`DataType`]: arrow_schema::DataType

---

## downcast_temporal

`macro` · `arrow_array::downcast_temporal`

Also reachable as `arrow::array::downcast_temporal`

```rust
macro_rules! downcast_temporal
```

Given one or more expressions evaluating to primitive [`DataType`] invokes the provided macro
`m` with the corresponding [`ArrowPrimitiveType`], followed by any additional arguments

```
# use arrow_array::{downcast_temporal, ArrowPrimitiveType};
# use arrow_schema::DataType;

macro_rules! temporal_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn temporal_size(t: &DataType) -> u8 {
    downcast_temporal! {
        t => (temporal_size_helper, u8),
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX
    }
}

assert_eq!(temporal_size(&DataType::Date32), 4);
assert_eq!(temporal_size(&DataType::Date64), 8);
```

[`DataType`]: arrow_schema::DataType

---

## downcast_temporal_array

`macro` · `arrow_array::downcast_temporal_array`

Also reachable as `arrow::array::downcast_temporal_array`

```rust
macro_rules! downcast_temporal_array
```

Downcast an [`Array`] to a temporal [`PrimitiveArray`] based on its [`DataType`]
accepts a number of subsequent patterns to match the data type

```
# use arrow_array::{Array, downcast_temporal_array, cast::as_string_array, cast::as_largestring_array};
# use arrow_schema::DataType;

fn print_temporal(array: &dyn Array) {
    downcast_temporal_array!(
        array => {
            for v in array {
                println!("{:?}", v);
            }
        }
        DataType::Utf8 => {
            for v in as_string_array(array) {
                println!("{:?}", v);
            }
        }
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => {
            for v in as_largestring_array(array) {
                println!("{:?}", v);
            }
        }
        t => println!("Unsupported datatype {}", t)
    )
}
```

[`DataType`]: arrow_schema::DataType

---

## record_batch

`macro` · `arrow_array::record_batch`

Also reachable as `arrow::array::record_batch`

```rust
macro_rules! record_batch
```

Creates a record batch from literal slice of values, suitable for rapid
testing and development.

Example:

```rust
use arrow_array::record_batch;
use arrow_schema;

let batch = record_batch!(
    ("a", Int32, [1, 2, 3]),
    ("b", Float64, [Some(4.0), None, Some(5.0)]),
    ("c", Utf8, ["alpha", "beta", "gamma"])
);
```

Variables and expressions are also supported:

```rust
use arrow_array::record_batch;

let values = vec![1, 2, 3];
let batch = record_batch!(
    ("a", Int32, values),
    ("b", Float64, vec![Some(4.0), None, Some(5.0)])
);
```
Due to limitation of [`create_array!`] macro, support for limited data types is available.

---

## ree_map

`macro` · `arrow_array::ree_map`

Also reachable as `arrow::array::ree_map`

```rust
macro_rules! ree_map
```

Recursively applies a function to the values of a RunEndEncoded array, preserving the run structure.

# Example

```ignore
let result = ree_recurse!(array, Int32Type, my_function)?;
```

This macro is useful for implementing functions that should work on the logical values
of a REE array while preserving the run-end encoding structure.

---
