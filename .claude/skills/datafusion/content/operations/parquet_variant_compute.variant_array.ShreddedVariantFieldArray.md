# `parquet_variant_compute::variant_array::ShreddedVariantFieldArray`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array.ShreddedVariantFieldArray.json).

<a id="op-87433f5e5843ee17c6ce4390"></a>
## ShreddedVariantFieldArray

`struct` · `parquet_variant_compute::variant_array::ShreddedVariantFieldArray` · parquet-variant-compute 59.3.0

```rust
struct ShreddedVariantFieldArray
```

Source: `src/variant_array.rs:667`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

One shredded field of a partially or perfectly shredded variant. For example, suppose the
shredding schema for variant `v` treats it as an object with a single field `a`, where `a` is
itself a struct with the single field `b` of type INT. Then the physical layout of the column
is:

```text
v: VARIANT {
    metadata: BINARY,
    value: BINARY,
    typed_value: STRUCT {
        a: SHREDDED_VARIANT_FIELD {
            value: BINARY,
            typed_value: STRUCT {
                a: SHREDDED_VARIANT_FIELD {
                    value: BINARY,
                    typed_value: INT,
                },
            },
        },
    },
}
```

In the above, each row of `v.value` is either a variant value (shredding failed, `v` was not an
object at all) or a variant object (partial shredding, `v` was an object but included unexpected
fields other than `a`), or is NULL (perfect shredding, `v` was an object containing only the
single expected field `a`).

A similar story unfolds for each `v.typed_value.a.value` -- a variant value if shredding failed
(`v:a` was not an object at all), or a variant object (`v:a` was an object with unexpected
additional fields), or NULL (`v:a` was an object containing only the single expected field `b`).

Finally, `v.typed_value.a.typed_value.b.value` is either NULL (`v:a.b` was an integer) or else a
variant value (which could be `Variant::Null`).
