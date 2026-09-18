# `arrow::datatypes`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.datatypes.json).

<a id="op-b0147cc510d5da0659809a26"></a>
## datatypes

`module` · `arrow::datatypes` · arrow 59.3.0

```rust
mod datatypes
```

Source: `src/datatypes/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Defines the logical data types of Arrow arrays.

The most important things you might be looking for are:
 * [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) to describe a schema.
 * [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) to describe one field within a schema.
 * [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) to describe the type of a field.
