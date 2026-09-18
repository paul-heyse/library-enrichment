# `arrow`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.json).

<a id="op-8619314d4974dbb571ec9d30"></a>
## arrow

`module` · `arrow` · arrow 59.3.0

```rust
mod arrow
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

A complete, safe, native Rust implementation of [Apache Arrow](https://arrow.apache.org), a cross-language
development platform for in-memory data.

Please see the [arrow crates.io](https://crates.io/crates/arrow)
page for feature flags and tips to improve performance.

# Columnar Format

The [`array`] module provides statically typed implementations of all the array types as defined
by the [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html)

For example, an [`Int32Array`](array::Int32Array) represents a nullable array of `i32`

```rust
# use arrow::array::{Array, Int32Array};
let array = Int32Array::from(vec![Some(1), None, Some(3)]);
assert_eq!(array.len(), 3);
assert_eq!(array.value(0), 1);
assert_eq!(array.is_null(1), true);

let collected: Vec<_> = array.iter().collect();
assert_eq!(collected, vec![Some(1), None, Some(3)]);
assert_eq!(array.values(), &[1, 0, 3])
```

It is also possible to write generic code for different concrete types.
For example, since the following function is generic over all primitively
typed arrays, when invoked the Rust compiler will generate specialized implementations
with optimized code for each concrete type.

```rust
# use std::iter::Sum;
# use arrow::array::{Float32Array, PrimitiveArray, TimestampNanosecondArray};
# use arrow::datatypes::ArrowPrimitiveType;
#
fn sum<T: ArrowPrimitiveType>(array: &PrimitiveArray<T>) -> T::Native
where
    T: ArrowPrimitiveType,
    T::Native: Sum
{
    array.iter().map(|v| v.unwrap_or_default()).sum()
}

assert_eq!(sum(&Float32Array::from(vec![1.1, 2.9, 3.])), 7.);
assert_eq!(sum(&TimestampNanosecondArray::from(vec![1, 2, 3])), 6);
```

And the following uses [`ArrayAccessor`] to implement a generic function
over all arrays with comparable values.

[`ArrayAccessor`]: array::ArrayAccessor

```rust
# use arrow::array::{ArrayAccessor, ArrayIter, Int32Array, StringArray};
# use arrow::datatypes::ArrowPrimitiveType;
#
fn min<T: ArrayAccessor>(array: T) -> Option<T::Item>
where
    T::Item: Ord
{
    ArrayIter::new(array).filter_map(|v| v).min()
}

assert_eq!(min(&Int32Array::from(vec![4, 2, 1, 6])), Some(1));
assert_eq!(min(&StringArray::from(vec!["b", "a", "c"])), Some("a"));
```

**For more examples, and details consult the [arrow_array](../modules/arrow_array.md#op-0c2c16239480a33e475f29a9) docs.**

# Type Erasure / Trait Objects

It is common to write code that handles any type of array, without necessarily
knowing its concrete type. This is done using the [`Array`] trait and using
[`DataType`] to determine the appropriate `downcast_ref`.

[`DataType`]: datatypes::DataType

```rust
# use arrow::array::{Array, Float32Array};
# use arrow::array::StringArray;
# use arrow::datatypes::DataType;
#
fn impl_string(array: &StringArray) {}
fn impl_f32(array: &Float32Array) {}

fn impl_dyn(array: &dyn Array) {
    match array.data_type() {
        // downcast `dyn Array` to concrete `StringArray`
        DataType::Utf8 => impl_string(array.as_any().downcast_ref().unwrap()),
        // downcast `dyn Array` to concrete `Float32Array`
        DataType::Float32 => impl_f32(array.as_any().downcast_ref().unwrap()),
        _ => unimplemented!()
    }
}
```

You can use the [`AsArray`] extension trait to facilitate downcasting:

[`AsArray`]: crate::array::AsArray

```rust
# use arrow::array::{Array, Float32Array, AsArray};
# use arrow::array::StringArray;
# use arrow::datatypes::DataType;
#
fn impl_string(array: &StringArray) {}
fn impl_f32(array: &Float32Array) {}

fn impl_dyn(array: &dyn Array) {
    match array.data_type() {
        DataType::Utf8 => impl_string(array.as_string()),
        DataType::Float32 => impl_f32(array.as_primitive()),
        _ => unimplemented!()
    }
}
```

It is also common to want to write a function that returns one of a number of possible
array implementations. [`ArrayRef`] is a type-alias for [`Arc<dyn Array>`](array::Array)
which is frequently used for this purpose

```rust
# use std::str::FromStr;
# use std::sync::Arc;
# use arrow::array::{ArrayRef, Int32Array, PrimitiveArray};
# use arrow::datatypes::{ArrowPrimitiveType, DataType, Int32Type, UInt32Type};
# use arrow::compute::cast;
#
fn parse_to_primitive<'a, T, I>(iter: I) -> PrimitiveArray<T>
where
    T: ArrowPrimitiveType,
    T::Native: FromStr,
    I: IntoIterator<Item=&'a str>,
{
    PrimitiveArray::from_iter(iter.into_iter().map(|val| T::Native::from_str(val).ok()))
}

fn parse_strings<'a, I>(iter: I, to_data_type: DataType) -> ArrayRef
where
    I: IntoIterator<Item=&'a str>,
{
   match to_data_type {
       DataType::Int32 => Arc::new(parse_to_primitive::<Int32Type, _>(iter)) as _,
       DataType::UInt32 => Arc::new(parse_to_primitive::<UInt32Type, _>(iter)) as _,
       _ => unimplemented!()
   }
}

let array = parse_strings(["1", "2", "3"], DataType::Int32);
let integers = array.as_any().downcast_ref::<Int32Array>().unwrap();
assert_eq!(integers.values(), &[1, 2, 3])
```

# Compute Kernels

The [`compute`](../modules/arrow.compute.md#op-8d4f889defd695def0b7582b) module provides optimised implementations of many common operations,
for example the `parse_strings` operation above could also be implemented as follows:

```
# use std::sync::Arc;
# use arrow::error::Result;
# use arrow::array::{ArrayRef, StringArray, UInt32Array};
# use arrow::datatypes::DataType;
#
fn parse_strings<'a, I>(iter: I, to_data_type: &DataType) -> Result<ArrayRef>
where
    I: IntoIterator<Item=&'a str>,
{
    let array = StringArray::from_iter(iter.into_iter().map(Some));
    arrow::compute::cast(&array, to_data_type)
}

let array = parse_strings(["1", "2", "3"], &DataType::UInt32).unwrap();
let integers = array.as_any().downcast_ref::<UInt32Array>().unwrap();
assert_eq!(integers.values(), &[1, 2, 3])
```

This module also implements many common vertical operations:

* All mathematical binary operators, such as [`sub`](compute::kernels::numeric::sub)
* All boolean binary operators such as [`equality`](compute::kernels::cmp::eq)
* [`cast`](compute::kernels::cast::cast)
* [`filter`](compute::kernels::filter::filter)
* [`take`](compute::kernels::take::take)
* [`sort`](compute::kernels::sort::sort)
* some string operators such as [`substring`](compute::kernels::substring::substring) and [`length`](compute::kernels::length::length)

```
# use arrow::compute::kernels::cmp::gt;
# use arrow_array::cast::AsArray;
# use arrow_array::Int32Array;
# use arrow_array::types::Int32Type;
# use arrow_select::filter::filter;
let array = Int32Array::from_iter(0..100);
// Create a 32-bit integer scalar (single) value:
let scalar = Int32Array::new_scalar(60);
// find all rows in the array that are greater than 60
let predicate = gt(&array, &scalar).unwrap();
// copy all matching rows into a new array
let filtered = filter(&array, &predicate).unwrap();

let expected = Int32Array::from_iter(61..100);
assert_eq!(&expected, filtered.as_primitive::<Int32Type>());
```

As well as some horizontal operations, such as:

* [`min`](compute::kernels::aggregate::min) and [`max`](compute::kernels::aggregate::max)
* [`sum`](compute::kernels::aggregate::sum)

# Tabular Representation

It is common to want to group one or more columns together into a tabular representation. This
is provided by [`RecordBatch`] which combines a [`Schema`](datatypes::Schema)
and a corresponding list of [`ArrayRef`].


```
# use std::sync::Arc;
# use arrow::array::{Float32Array, Int32Array};
# use arrow::record_batch::RecordBatch;
#
let col_1 = Arc::new(Int32Array::from_iter([1, 2, 3])) as _;
let col_2 = Arc::new(Float32Array::from_iter([1., 6.3, 4.])) as _;

let batch = RecordBatch::try_from_iter([("col1", col_1), ("col_2", col_2)]).unwrap();
```

# Pretty Printing

See the [`util::pretty`](../modules/arrow_cast.pretty.md#op-f83724b3b9fd6b9155cf350d) module (requires the `prettyprint` crate feature)

# IO

This crate provides readers and writers for various formats to/from [`RecordBatch`]

* JSON: [`Reader`](json::reader::Reader) and [`Writer`](json::writer::Writer)
* CSV: [`Reader`](csv::reader::Reader) and [`Writer`](csv::writer::Writer)
* IPC: [`Reader`](ipc::reader::StreamReader) and [`Writer`](ipc::writer::FileWriter)

Support for [Apache Parquet] is published as a [separate parquet crate](https://crates.io/crates/parquet)

Support for [Apache Avro] is published as a [separate arrow-avro crate](https://crates.io/crates/arrow-avro)

# Serde Compatibility

[`arrow_json::reader::Decoder`](../operations/arrow_json.reader.Decoder.md#op-9a10b6ca230ecd7bed81532a) provides a mechanism to convert arbitrary, serde-compatible
structures into [`RecordBatch`].

Whilst likely less performant than implementing a custom builder, as described in
[arrow_array::builder](../modules/arrow_array.builder.md#op-aacfbf3413ace5e290c06216), this provides a simple mechanism to get up and running quickly

```
# use std::sync::Arc;
# use arrow_json::ReaderBuilder;
# use arrow_schema::{DataType, Field, Schema};
# use serde::Serialize;
# use arrow_array::cast::AsArray;
# use arrow_array::types::{Float32Type, Int32Type};
#
#[derive(Serialize)]
struct MyStruct {
    int32: i32,
    string: String,
}

let schema = Schema::new(vec![
    Field::new("int32", DataType::Int32, false),
    Field::new("string", DataType::Utf8, false),
]);

let rows = vec![
    MyStruct{ int32: 5, string: "bar".to_string() },
    MyStruct{ int32: 8, string: "foo".to_string() },
];

let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();
decoder.serialize(&rows).unwrap();

let batch = decoder.flush().unwrap().unwrap();

// Expect batch containing two columns
let int32 = batch.column(0).as_primitive::<Int32Type>();
assert_eq!(int32.values(), &[5, 8]);

let string = batch.column(1).as_string::<i32>();
assert_eq!(string.value(0), "bar");
assert_eq!(string.value(1), "foo");
```

# Crate Topology

The [`arrow`] project is implemented as multiple sub-crates, which are then re-exported by
this top-level crate.

Crate authors can choose to depend on this top-level crate, or just
the sub-crates they need.

The current list of sub-crates is:

* [`arrow-arith`][arrow_arith](../modules/arrow_arith.md#op-363081af8fd5665be562116b) - arithmetic kernels
* [`arrow-array`][arrow_array](../modules/arrow_array.md#op-0c2c16239480a33e475f29a9) - type-safe arrow array abstractions
* [`arrow-buffer`][arrow_buffer](../modules/arrow_buffer.md#op-e147f1254e2292b9855350ca) - buffer abstractions for arrow arrays
* [`arrow-cast`][arrow_cast](../modules/arrow_cast.md#op-c44ca9952f804b390ba02fcf) - cast kernels for arrow arrays
* [`arrow-csv`][arrow_csv](../modules/arrow_csv.md#op-49561968a7082d579e4b82ba) - read/write CSV to arrow format
* [`arrow-data`][arrow_data](../modules/arrow_data.md#op-6cc5183fec74dba51c6da18e) - the underlying data of arrow arrays
* [`arrow-ipc`][arrow_ipc](../modules/arrow_ipc.md#op-318e8c625105f63dd319fd7b) - read/write IPC to arrow format
* [`arrow-json`][arrow_json](../modules/arrow_json.md#op-9cfc2f2ebe48fa33164bfd99) - read/write JSON to arrow format
* [`arrow-ord`][arrow_ord](../modules/arrow_ord.md#op-1b38ba299ad923d1743e5e00) - ordering kernels for arrow arrays
* [`arrow-row`][arrow_row](../modules/arrow_row.md#op-303caac8dd00f839fb954c1c) - comparable row format
* [`arrow-schema`][arrow_schema](../modules/arrow_schema.md#op-d7d016510f82ac213666915b) - the logical types for arrow arrays
* [`arrow-select`][arrow_select](../modules/arrow_select.md#op-e6e56e8a9a5bbf07e95f842c) - selection kernels for arrow arrays
* [`arrow-string`][arrow_string](../modules/arrow_string.md#op-307714ecf8f53f62563f6caf) - string kernels for arrow arrays

Some functionality is also distributed independently of this crate:

* [`arrow-flight`] - support for [Arrow Flight RPC]
* [`parquet`](https://docs.rs/parquet) - support for [Apache Parquet]
* [`arrow-avro`](https://docs.rs/arrow-avro) - support for [Apache Avro]

# Security

This project follows the [Apache Arrow Security Model].

Unexpected behavior (e.g., panics, crashes, or infinite loops) triggered by
malformed input is considered a **bug**, not a security vulnerability,
unless it is **exploitable** by an attacker to

* Execute arbitrary code (Remote Code Execution);
* Exfiltrate sensitive information from process memory (Information Disclosure);

If you think you have found a security vulnerability, please follow the
reporting instructions in the [security policy].

[security policy]: https://github.com/apache/arrow-rs/blob/main/SECURITY.md

# Safety

Like many crates, this crate makes use of `unsafe` where prudent. However, it endeavors to be
sound. Specifically, **it should not be possible to trigger undefined behavior using safe APIs.**

Undefined behavior using safe APIs is considered a bug, not a security
vulnerability, unless it can be exploited. Please see the [security policy]
for details.

For more information on the use of `unsafe`, see [here](https://github.com/apache/arrow-rs/tree/main/arrow#safety).

[Apache Arrow Security Model]: https://arrow.apache.org/docs/dev/format/Security.html

# Higher-level Processing

This crate aims to provide reusable, low-level primitives for operating on columnar data. For
more sophisticated query processing workloads, consider checking out [DataFusion]. This
orchestrates the primitives exported by this crate into an embeddable query engine, with
SQL and DataFrame frontends, and heavily influences this crate's roadmap.

[`arrow`]: https://github.com/apache/arrow-rs
[`array`]: mod@array
[`Array`]: array::Array
[`ArrayRef`]: array::ArrayRef
[`ArrayData`]: array::ArrayData
[`make_array`]: array::make_array
[`Buffer`]: buffer::Buffer
[`RecordBatch`]: record_batch::RecordBatch
[`arrow-flight`]: https://docs.rs/arrow-flight/latest/arrow_flight/
[`parquet`]: https://docs.rs/parquet/latest/parquet/
[Arrow Flight RPC]: https://arrow.apache.org/docs/format/Flight.html
[Arrow JSON Test Format]: https://github.com/apache/arrow/blob/master/docs/source/format/Integration.rst#json-test-data-format
[Apache Parquet]: https://parquet.apache.org/
[Apache Avro]: https://avro.apache.org/
[DataFusion]: https://github.com/apache/arrow-datafusion
[issue tracker]: https://github.com/apache/arrow-rs/issues
