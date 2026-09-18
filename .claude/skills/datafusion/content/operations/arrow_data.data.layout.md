# `arrow_data::data::layout`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.data.layout.json).

<a id="op-01a5e1a8500477eeae9a4a6b"></a>
## layout

`function` · `arrow_data::data::layout` · arrow-data 59.3.0

```rust
fn layout(data_type: &arrow_schema::DataType) -> DataTypeLayout
```

Source: `src/data.rs:1787`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Return the expected [`DataTypeLayout`](../operations/arrow_data.data.DataTypeLayout.md#op-16644228ac7c0d2756bb3d40) Arrays of this data
type are expected to have
