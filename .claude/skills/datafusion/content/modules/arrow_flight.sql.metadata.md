# `arrow_flight::sql::metadata`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.metadata.json).

<a id="op-6ac6534ef37e9005bc1d4fab"></a>
## metadata

`module` · `arrow_flight::sql::metadata` · arrow-flight 59.3.0

```rust
mod metadata
```

Source: `src/sql/metadata/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Builders and function for building responses to FlightSQL metadata
/ information schema requests.

- [`GetCatalogsBuilder`](../operations/arrow_flight.sql.metadata.catalogs.GetCatalogsBuilder.md#op-c8f333fc351087b2c1f0ffd0) for building responses to [`CommandGetCatalogs`] queries.
- [`GetDbSchemasBuilder`](../operations/arrow_flight.sql.metadata.db_schemas.GetDbSchemasBuilder.md#op-8ce7622b04ed745b3def9de8) for building responses to [`CommandGetDbSchemas`] queries.
- [`GetTablesBuilder`](../operations/arrow_flight.sql.metadata.tables.GetTablesBuilder.md#op-a87af5379ec74ef613b86dff)for building responses to [`CommandGetTables`] queries.
- [`SqlInfoDataBuilder`](../operations/arrow_flight.sql.metadata.sql_info.SqlInfoDataBuilder.md#op-70fe7b015f1a84c1c663a96c)for building responses to [`CommandGetSqlInfo`] queries.
- [`XdbcTypeInfoDataBuilder`](../operations/arrow_flight.sql.metadata.xdbc_info.XdbcTypeInfoDataBuilder.md#op-0294db3bf2620399969ccd2c)for building responses to [`CommandGetXdbcTypeInfo`] queries.

[`CommandGetCatalogs`]: crate::sql::CommandGetCatalogs
[`CommandGetDbSchemas`]: crate::sql::CommandGetDbSchemas
[`CommandGetTables`]: crate::sql::CommandGetTables
[`CommandGetSqlInfo`]: crate::sql::CommandGetSqlInfo
[`CommandGetXdbcTypeInfo`]: crate::sql::CommandGetXdbcTypeInfo
