# `arrow_array::builder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.json).

<a id="op-aacfbf3413ace5e290c06216"></a>
## builder

`module` · `arrow_array::builder` · arrow-array 59.3.0

```rust
mod builder
```

Source: `src/builder/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Defines push-based APIs for constructing arrays

# Basic Usage

Builders can be used to build simple, non-nested arrays

```
# use arrow_array::builder::Int32Builder;
# use arrow_array::PrimitiveArray;
let mut a = Int32Builder::new();
a.append_value(1);
a.append_null();
a.append_value(2);
let a = a.finish();

assert_eq!(a, PrimitiveArray::from(vec![Some(1), None, Some(2)]));
```

```
# use arrow_array::builder::StringBuilder;
# use arrow_array::{Array, StringArray};
let mut a = StringBuilder::new();
a.append_value("foo");
a.append_value("bar");
a.append_null();
let a = a.finish();

assert_eq!(a, StringArray::from_iter([Some("foo"), Some("bar"), None]));
```

# Nested Usage

Builders can also be used to build more complex nested arrays, such as lists

```
# use arrow_array::builder::{Int32Builder, ListBuilder};
# use arrow_array::ListArray;
# use arrow_array::types::Int32Type;
let mut a = ListBuilder::new(Int32Builder::new());
// [1, 2]
a.values().append_value(1);
a.values().append_value(2);
a.append(true);
// null
a.append(false);
// []
a.append(true);
// [3, null]
a.values().append_value(3);
a.values().append_null();
a.append(true);

// [[1, 2], null, [], [3, null]]
let a = a.finish();

assert_eq!(a, ListArray::from_iter_primitive::<Int32Type, _, _>([
    Some(vec![Some(1), Some(2)]),
    None,
    Some(vec![]),
    Some(vec![Some(3), None])]
))
```

# Using the [`Extend`] trait to append values from an iterable:

```
# use arrow_array::{Array};
# use arrow_array::builder::{ArrayBuilder, StringBuilder};

let mut builder = StringBuilder::new();
builder.extend(vec![Some("🍐"), Some("🍎"), None]);
assert_eq!(builder.finish().len(), 3);
```

# Using the [`Extend`] trait to write generic functions:

```
# use arrow_array::{Array, ArrayRef, StringArray};
# use arrow_array::builder::{ArrayBuilder, Int32Builder, ListBuilder, StringBuilder};

// For generic methods that fill a list of values for an [`ArrayBuilder`], use the [`Extend`] trait.
fn filter_and_fill<V, I: IntoIterator<Item = V>>(builder: &mut impl Extend<V>, values: I, filter: V)
where V: PartialEq
{
    builder.extend(values.into_iter().filter(|v| *v == filter));
}
let mut string_builder = StringBuilder::new();
filter_and_fill(
    &mut string_builder,
    vec![Some("🍐"), Some("🍎"), None],
    Some("🍎"),
);
assert_eq!(string_builder.finish().len(), 1);

let mut int_builder = Int32Builder::new();
filter_and_fill(
    &mut int_builder,
    vec![Some(11), Some(42), None],
    Some(42),
);
assert_eq!(int_builder.finish().len(), 1);

// For generic methods that fill lists-of-lists for an [`ArrayBuilder`], use the [`Extend`] trait.
fn filter_and_fill_if_contains<T, V, I: IntoIterator<Item = Option<V>>>(
    list_builder: &mut impl Extend<Option<V>>,
    values: I,
    filter: Option<T>,
) where
    T: PartialEq,
    for<'a> &'a V: IntoIterator<Item = &'a Option<T>>,
{
    list_builder.extend(values.into_iter().filter(|string: &Option<V>| {
        string
            .as_ref()
            .map(|str: &V| str.into_iter().any(|ch: &Option<T>| ch == &filter))
            .unwrap_or(false)
    }));
 }
let builder = StringBuilder::new();
let mut list_builder = ListBuilder::new(builder);
let pear_pear = vec![Some("🍐"),Some("🍐")];
let pear_app = vec![Some("🍐"),Some("🍎")];
filter_and_fill_if_contains(
    &mut list_builder,
    vec![Some(pear_pear), Some(pear_app), None],
    Some("🍎"),
);
assert_eq!(list_builder.finish().len(), 1);
```

# Custom Builders

It is common to have a collection of statically defined Rust types that
you want to convert to Arrow arrays.

An example of doing so is below

```
# use std::any::Any;
# use arrow_array::builder::{ArrayBuilder, Int32Builder, ListBuilder, StringBuilder};
# use arrow_array::{ArrayRef, RecordBatch, StructArray};
# use arrow_schema::{DataType, Field};
# use std::sync::Arc;
/// A custom row representation
struct MyRow {
    i32: i32,
    optional_i32: Option<i32>,
    string: Option<String>,
    i32_list: Option<Vec<Option<i32>>>,
}

/// Converts `Vec<Row>` into `StructArray`
#[derive(Debug, Default)]
struct MyRowBuilder {
    i32: Int32Builder,
    string: StringBuilder,
    i32_list: ListBuilder<Int32Builder>,
}

impl MyRowBuilder {
    fn append(&mut self, row: &MyRow) {
        self.i32.append_value(row.i32);
        self.string.append_option(row.string.as_ref());
        self.i32_list.append_option(row.i32_list.as_ref().map(|x| x.iter().copied()));
    }

    /// Note: returns StructArray to allow nesting within another array if desired
    fn finish(&mut self) -> StructArray {
        let i32 = Arc::new(self.i32.finish()) as ArrayRef;
        let i32_field = Arc::new(Field::new("i32", DataType::Int32, false));

        let string = Arc::new(self.string.finish()) as ArrayRef;
        let string_field = Arc::new(Field::new("i32", DataType::Utf8, false));

        let i32_list = Arc::new(self.i32_list.finish()) as ArrayRef;
        let value_field = Arc::new(Field::new_list_field(DataType::Int32, true));
        let i32_list_field = Arc::new(Field::new("i32_list", DataType::List(value_field), true));

        StructArray::from(vec![
            (i32_field, i32),
            (string_field, string),
            (i32_list_field, i32_list),
        ])
    }
}

/// For building arrays in generic code, use Extend instead of the append_* methods
/// e.g. append_value, append_option, append_null
impl<'a> Extend<&'a MyRow> for MyRowBuilder {
    fn extend<T: IntoIterator<Item = &'a MyRow>>(&mut self, iter: T) {
        iter.into_iter().for_each(|row| self.append(row));
    }
}

/// Converts a slice of [`MyRow`] to a [`RecordBatch`]
fn rows_to_batch(rows: &[MyRow]) -> RecordBatch {
    let mut builder = MyRowBuilder::default();
    builder.extend(rows);
    RecordBatch::from(&builder.finish())
}
```

# Null / Validity Masks

The [`NullBufferBuilder`](../operations/arrow_buffer.builder.null.NullBufferBuilder.md#op-f629e701548be729fb00ab9e) is optimized for creating the null mask for an array.

```
# use arrow_array::builder::NullBufferBuilder;
let mut builder = NullBufferBuilder::new(8);
let mut builder = NullBufferBuilder::new(8);
builder.append_n_non_nulls(7);
builder.append_null();
let buffer = builder.finish().unwrap();
assert_eq!(buffer.len(), 8);
assert_eq!(buffer.iter().collect::<Vec<_>>(), vec![true, true, true, true, true, true, true, false]);
```

Unresolved upstream links (retained, not inferred): ``Extend``.
