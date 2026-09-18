# `arrow_array::scalar`

Crate `arrow-array` · 2 public items · structured records in [`model/arrow_array.scalar.json`](../model/arrow_array.scalar.json)

## Scalar

`struct` · `arrow_array::scalar::Scalar`

```rust
struct Scalar<T: Array>
```

**Implements**: `arrow_array::scalar::Datum`

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
fn into_inner(self) -> T
fn new(array: T) -> Self
```

**via `arrow_array::scalar::Datum`**

```rust
fn get(&self) -> (&dyn Array, bool)
```

[Full member, field, variant and typed contracts](../operations/arrow_array.scalar.Scalar.md).


A wrapper around a single value [`Array`] that implements
[`Datum`] and indicates [compute] kernels should treat this array
as a scalar value (a single value).

Using a [`Scalar`] is often much more efficient than creating an
[`Array`] with the same (repeated) value.

See [`Datum`] for more information.

# Example

```rust
# use arrow_array::{Scalar, Int32Array, ArrayRef};
# fn get_array() -> ArrayRef { std::sync::Arc::new(Int32Array::from(vec![42])) }
// Create a (typed) scalar for Int32Array for the value 42
let scalar = Scalar::new(Int32Array::from(vec![42]));

// Create a scalar using PrimtiveArray::scalar
let scalar = Int32Array::new_scalar(42);

// create a scalar from an ArrayRef (for dynamic typed Arrays)
let array: ArrayRef = get_array();
let scalar = Scalar::new(array);
```

[compute]: https://docs.rs/arrow/latest/arrow/compute/index.html

---

## Datum

`trait` · `arrow_array::scalar::Datum`

```rust
trait Datum
```

**Implementors** (1)

- `arrow_array::scalar::Scalar`

**Methods** (1)

```rust
fn get(&self) -> (&dyn Array, bool)
```

[Full member, field, variant and typed contracts](../operations/arrow_array.scalar.Datum.md).


A possibly [`Scalar`] [`Array`]

This allows optimised binary kernels where one or more arguments are constant

```
# use arrow_array::*;
# use arrow_buffer::{BooleanBuffer, MutableBuffer, NullBuffer};
# use arrow_schema::ArrowError;
#
fn eq_impl<T: ArrowPrimitiveType>(
    a: &PrimitiveArray<T>,
    a_scalar: bool,
    b: &PrimitiveArray<T>,
    b_scalar: bool,
) -> BooleanArray {
    let (array, scalar) = match (a_scalar, b_scalar) {
        (true, true) | (false, false) => {
            let len = a.len().min(b.len());
            let nulls = NullBuffer::union(a.nulls(), b.nulls());
            let buffer = BooleanBuffer::collect_bool(len, |idx| a.value(idx) == b.value(idx));
            return BooleanArray::new(buffer, nulls);
        }
        (true, false) => (b, (a.null_count() == 0).then(|| a.value(0))),
        (false, true) => (a, (b.null_count() == 0).then(|| b.value(0))),
    };
    match scalar {
        Some(v) => {
            let len = array.len();
            let nulls = array.nulls().cloned();
            let buffer = BooleanBuffer::collect_bool(len, |idx| array.value(idx) == v);
            BooleanArray::new(buffer, nulls)
        }
        None => BooleanArray::new_null(array.len()),
    }
}

pub fn eq(l: &dyn Datum, r: &dyn Datum) -> Result<BooleanArray, ArrowError> {
    let (l_array, l_scalar) = l.get();
    let (r_array, r_scalar) = r.get();
    downcast_primitive_array!(
        (l_array, r_array) => Ok(eq_impl(l_array, l_scalar, r_array, r_scalar)),
        (a, b) => Err(ArrowError::NotYetImplemented(format!("{a} == {b}"))),
    )
}

// Comparison of two arrays
let a = Int32Array::from(vec![1, 2, 3, 4, 5]);
let b = Int32Array::from(vec![1, 2, 4, 7, 3]);
let r = eq(&a, &b).unwrap();
let values: Vec<_> = r.values().iter().collect();
assert_eq!(values, &[true, true, false, false, false]);

// Comparison of an array and a scalar
let a = Int32Array::from(vec![1, 2, 3, 4, 5]);
let b = Int32Array::new_scalar(1);
let r = eq(&a, &b).unwrap();
let values: Vec<_> = r.values().iter().collect();
assert_eq!(values, &[true, false, false, false, false]);

---
