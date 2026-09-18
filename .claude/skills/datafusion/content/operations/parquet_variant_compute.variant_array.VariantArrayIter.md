# `parquet_variant_compute::variant_array::VariantArrayIter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array.VariantArrayIter.json).

<a id="op-96a7542f3c450ee2b708ac35"></a>
## VariantArrayIter

`struct` · `parquet_variant_compute::variant_array::VariantArrayIter` · parquet-variant-compute 59.3.0

```rust
struct VariantArrayIter<'a>
```

Source: `src/variant_array.rs:574`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

An iterator over [`VariantArray`]

This iterator returns `Option<Option<Variant<'a, 'a>>>` where:
- `None` indicates the end of iteration
- `Some(None)` indicates a null value at this position
- `Some(Some(variant))` indicates a valid variant value

# Example

```
# use parquet_variant::Variant;
# use parquet_variant_compute::VariantArrayBuilder;
let mut builder = VariantArrayBuilder::new(10);
builder.append_variant(Variant::from(42));
builder.append_null();
builder.append_variant(Variant::from("hello"));
let array = builder.build();

let values = array.iter().collect::<Vec<_>>();
assert_eq!(values.len(), 3);
assert_eq!(values[0], Some(Variant::from(42)));
assert_eq!(values[1], None);
assert_eq!(values[2], Some(Variant::from("hello")));
```
