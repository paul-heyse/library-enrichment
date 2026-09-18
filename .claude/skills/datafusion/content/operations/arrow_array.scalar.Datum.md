# `arrow_array::scalar::Datum`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.scalar.Datum.json).

<a id="op-8fd206fb21d47e60f8e9716f"></a>
## Datum

`trait` · `arrow_array::scalar::Datum` · arrow-array 59.3.0

```rust
trait Datum
```

Source: `src/scalar.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A possibly [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)

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

<a id="op-3c1ed2782c3717e61b865b18"></a>
## get

`function` · `arrow_array::scalar::Datum::get` · arrow-array 59.3.0

```rust
fn get(&self) -> (&dyn Array, bool)
```

Source: `src/scalar.rs:80`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the value for this [`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f) and a boolean indicating if the value is scalar
