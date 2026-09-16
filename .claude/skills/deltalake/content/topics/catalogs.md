# Catalogs

A catalog resolves a table name to a storage URL, and sometimes to credentials. It is deliberately outside the table format: Delta works without one, and a catalog adds naming, discovery and access control on top. Credential vending is the part that changes the security model, because the catalog then holds the keys rather than the client.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_catalog_unity::UnityCatalogBuilderBuilder` | struct | 17 | [prose](../api/deltalake_catalog_unity.md#unitycatalogbuilderbuilder) | [records](../model/deltalake_catalog_unity.json) |
| `deltalake_catalog_unity::client::ClientOptionsBuilder` | struct | 17 | [prose](../api/deltalake_catalog_unity.client.md#clientoptionsbuilder) | [records](../model/deltalake_catalog_unity.client.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::data_catalog::DataCatalog` | 1 | 0 | 2 | [DataCatalog](../traits/DataCatalog.md) |
| `deltalake_catalog_unity::credential::TokenCredential` | 1 | 0 | 5 | [TokenCredential](../traits/TokenCredential.md) |

## Runnable examples (1)

- [`corpus/examples/catalog-unity/uc_example.rs`](../corpus/examples/catalog-unity/uc_example.rs)

## Decision rules

- Prefer catalog credential vending to long-lived static keys where the catalog supports it.
- Cache catalog lookups: they are network calls on the open path.

## Anti-patterns

- Treating a catalog as the source of truth for table state. The log is.
- Hardcoding a resolved URL and losing the catalog's ability to move the table.

## Agent checklist

- Are catalog lookups cached?
- Is credential lifetime shorter than the token refresh interval?
