# `arrow_flight::sql::metadata::table_types::GetTableTypesBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.table_types.GetTableTypesBuilder.json).

<a id="op-2be3187ac4a20e3f087861aa"></a>
## GetTableTypesBuilder

`struct` · `arrow_flight::sql::metadata::table_types::GetTableTypesBuilder` · arrow-flight 59.3.0

```rust
struct GetTableTypesBuilder
```

Source: `src/sql/metadata/table_types.rs:40`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A builder for a [`CommandGetTableTypes`] response.

Builds rows like this:

* table_type: utf8,
