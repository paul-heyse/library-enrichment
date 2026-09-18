# `arrow_select::merge::merge`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.merge.merge.json).

<a id="op-9664f8c4a218e3503f2ba759"></a>
## merge

`function` · `arrow_select::merge::merge` · arrow-select 59.3.0

```rust
fn merge(mask: &arrow_array::BooleanArray, truthy: &dyn Datum, falsy: &dyn Datum) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Source: `src/merge.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Merges two arrays in the order specified by a boolean mask.

This algorithm is a variant of [zip](../operations/arrow_select.zip.zip.md#op-c40a0b0a379f198055f80c56) that does not require the truthy and
falsy arrays to have the same length.

When truthy of falsy are [Scalar](arrow_array::Scalar), the single
scalar value is repeated whenever the mask array contains true or false respectively.

# Example

```text
 truthy
┌─────────┐  mask
│    A    │  ┌─────────┐                             ┌─────────┐
├─────────┤  │  true   │                             │    A    │
│    C    │  ├─────────┤                             ├─────────┤
├─────────┤  │  true   │                             │    C    │
│   NULL  │  ├─────────┤                             ├─────────┤
├─────────┤  │  false  │  merge(mask, truthy, falsy) │    B    │
│    D    │  ├─────────┤  ─────────────────────────▶ ├─────────┤
└─────────┘  │  true   │                             │   NULL  │
 falsy       ├─────────┤                             ├─────────┤
┌─────────┐  │  false  │                             │    E    │
│    B    │  ├─────────┤                             ├─────────┤
├─────────┤  │  true   │                             │    D    │
│    E    │  └─────────┘                             └─────────┘
└─────────┘
```
