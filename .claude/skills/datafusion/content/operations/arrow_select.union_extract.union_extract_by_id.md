# `arrow_select::union_extract::union_extract_by_id`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.union_extract.union_extract_by_id.json).

<a id="op-74d88f1e6d3a106946fb2adb"></a>
## union_extract_by_id

`function` · `arrow_select::union_extract::union_extract_by_id` · arrow-select 59.3.0

```rust
fn union_extract_by_id(union_array: &arrow_array::UnionArray, target_type_id: i8) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Source: `src/union_extract.rs:103`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Like [`union_extract`](../operations/arrow_select.union_extract.union_extract.md#op-d34c0ba57ba2ef4680673369), but selects the child by `type_id` rather than by
field name.

This avoids ambiguity when the union contains duplicate field names.

# Errors

Returns error if `target_type_id` does not correspond to a field in the union.
