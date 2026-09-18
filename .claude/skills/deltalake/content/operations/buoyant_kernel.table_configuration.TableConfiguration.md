# `buoyant_kernel::table_configuration::TableConfiguration`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_configuration.TableConfiguration.json).

<a id="op-4c1bb6e90ea642d0fb312bae"></a>
## TableConfiguration

`struct` · `buoyant_kernel::table_configuration::TableConfiguration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TableConfiguration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L104).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Holds all the configuration for a table at a specific version. This includes the supported
reader and writer features, table properties, schema, version, and table root. This can be used
to check whether a table supports a feature or has it enabled. For example, deletion vector
support can be checked with [`TableConfiguration::is_feature_supported`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c2699329afaa083ef39ce68) and deletion
vector write enablement can be checked with [`TableConfiguration::is_feature_enabled`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-f23193c28035dc8db17c102f).

[`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae) performs checks upon construction with `TableConfiguration::try_new`
to validate that Metadata and Protocol are correctly formatted and mutually compatible.
After construction, call `ensure_operation_supported` to verify that the kernel supports the
required operations for the table's protocol features.

<a id="op-d519c20619bd17680897feb4"></a>
## build_expected_stats_schemas

`function` · `buoyant_kernel::table_configuration::TableConfiguration::build_expected_stats_schemas` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build_expected_stats_schemas(&self, required_physical_columns: Option<&[ColumnName]>, requested_physical_columns: Option<&[ColumnName]>) -> DeltaResult<ExpectedStatsSchemas>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L315).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:315`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Generates the expected schema for file statistics.

Engines can provide statistics for files written to the delta table, enabling
data skipping and other optimizations. Returns the physical stats schema wrapped in
an `ExpectedStatsSchemas`.

The schema is structured as:
```text
{
  numRecords: long,
  nullCount: { <columns with LONG type> },
  minValues: { <columns with original types> },
  maxValues: { <columns with original types> },
}
```

The schemas are affected by:
- **Column mapping mode**: Physical schema field names use physical names from column
  mapping metadata.
- **`delta.dataSkippingStatsColumns`**: If set, only specified columns are included.
- **`delta.dataSkippingNumIndexedCols`**: Otherwise, includes the first N leaf columns
  (default 32).
- **Required columns** (e.g. clustering columns): Per the Delta protocol, always included in
  statistics, regardless of the above settings.
- **Requested columns**: Optional output filter that limits which columns appear in the
  schema without affecting column counting.

See the Delta protocol for more details on per-file statistics:
<https://github.com/delta-io/delta/blob/master/PROTOCOL.md#per-file-statistics>

<a id="op-9ccf9e29639f3a65ce589efb"></a>
## clone

`function` · `buoyant_kernel::table_configuration::TableConfiguration::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TableConfiguration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 17], "end": [103, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2716df0636cdccf57a0b26e"></a>
## column_mapping_mode

`function` · `buoyant_kernel::table_configuration::TableConfiguration::column_mapping_mode` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn column_mapping_mode(&self) -> ColumnMappingMode
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L529).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:529`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`ColumnMappingMode`](../operations/buoyant_kernel.table_features.column_mapping.ColumnMappingMode.md#op-757f64913cb951ae565079da) for this table at this version.

<a id="op-e971c1eb417fedd9c0d9429b"></a>
## ensure_operation_supported

`function` · `buoyant_kernel::table_configuration::TableConfiguration::ensure_operation_supported` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ensure_operation_supported(&self, operation: Operation) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L654).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:654`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `Ok` if the kernel supports the given operation on this table. This checks that
the protocol's features are all supported for the requested operation type.

- For `Scan` and `Cdf` operations: checks reader version and reader features
- For `Write` operations: checks writer version and writer features

<a id="op-ce1e013dd09d6004fb27a24d"></a>
## eq

`function` · `buoyant_kernel::table_configuration::TableConfiguration::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &TableConfiguration) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 24], "end": [103, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81a3b5bc3b013255cbd7f304"></a>
## fmt

`function` · `buoyant_kernel::table_configuration::TableConfiguration::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 10], "end": [103, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08bcc76bb9d3587e2c91e829"></a>
## is_catalog_managed

`function` · `buoyant_kernel::table_configuration::TableConfiguration::is_catalog_managed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_catalog_managed(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L522).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:522`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether this table is catalog-managed (has the CatalogManaged or CatalogOwnedPreview
table feature).

<a id="op-f23193c28035dc8db17c102f"></a>
## is_feature_enabled

`function` · `buoyant_kernel::table_configuration::TableConfiguration::is_feature_enabled` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_feature_enabled(&self, feature: &TableFeature) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L837).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:837`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Generic method to check if a feature is enabled.

A feature is enabled if:
1. It is supported in the protocol
2. The enablement check passes

<a id="op-4c2699329afaa083ef39ce68"></a>
## is_feature_supported

`function` · `buoyant_kernel::table_configuration::TableConfiguration::is_feature_supported` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_feature_supported(&self, feature: &TableFeature) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L791).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:791`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Helper method to check if a feature is supported.
This checks protocol versions and feature lists but does NOT check enablement properties.

<a id="op-ccc8d642559aa0f8d77869c4"></a>
## logical_schema

`function` · `buoyant_kernel::table_configuration::TableConfiguration::logical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_schema(&self) -> SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L460).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:460`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The logical schema ([`SchemaRef`](../operations/buoyant_kernel.schema.SchemaRef.md#op-bcbc708676b5e88d4183ee59)) of this table at this version.

<a id="op-487eb8ab9efd9b7d56f4e16f"></a>
## metadata

`function` · `buoyant_kernel::table_configuration::TableConfiguration::metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata(&self) -> &Metadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L447).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:447`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`Metadata`](../operations/buoyant_kernel.actions.Metadata.md#op-1843fecb3566e0e5ad63d7aa) for this table at this version.

<a id="op-c3450fdac0952044513274d1"></a>
## partition_columns

`function` · `buoyant_kernel::table_configuration::TableConfiguration::partition_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partition_columns(&self) -> &[String]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L535).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:535`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The partition columns of this table (empty if non-partitioned)

<a id="op-d48e78e0b53d6e80db3e08db"></a>
## physical_schema

`function` · `buoyant_kernel::table_configuration::TableConfiguration::physical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_schema(&self) -> SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L479).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:479`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The physical schema ([`SchemaRef`](../operations/buoyant_kernel.schema.SchemaRef.md#op-bcbc708676b5e88d4183ee59)) of this table at this version.

When column mapping is disabled, this is identical to
[`logical_schema`](Self::logical_schema). Otherwise, field names are replaced with
physical column names derived from column mapping metadata.

<a id="op-8f185c2e26b02225961e6c3b"></a>
## protocol

`function` · `buoyant_kernel::table_configuration::TableConfiguration::protocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn protocol(&self) -> &Protocol
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L454).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:454`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`Protocol`](../operations/buoyant_kernel.actions.Protocol.md#op-0b0c1a8cbad46c3db514befe) of this table at this version.

<a id="op-4e5e92902cf1b75b9ee84fe4"></a>
## table_properties

`function` · `buoyant_kernel::table_configuration::TableConfiguration::table_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_properties(&self) -> &TableProperties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L515).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:515`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`TableProperties`](../operations/buoyant_kernel.table_properties.TableProperties.md#op-a6b333464916b5b04ef86dcd) of this table at this version.

<a id="op-3ff02730ca1f29e2a42e35d0"></a>
## table_root

`function` · `buoyant_kernel::table_configuration::TableConfiguration::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L541).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:541`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`Url`] of the table this [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae) belongs to

Unresolved upstream links (retained, not inferred): ``Url``.

<a id="op-d0372a1b1d50b1dd56b7c65c"></a>
## try_new

`function` · `buoyant_kernel::table_configuration::TableConfiguration::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(metadata: Metadata, protocol: Protocol, table_root: Url, version: Version) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L142).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:142`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Constructs a [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae) for a table located in `table_root` at `version`.
This validates that the [`Metadata`](../operations/buoyant_kernel.actions.Metadata.md#op-1843fecb3566e0e5ad63d7aa) and [`Protocol`](../operations/buoyant_kernel.actions.Protocol.md#op-0b0c1a8cbad46c3db514befe) are compatible with one another
and that the kernel supports reading from this table.

Note: This only returns successfully if kernel supports reading the table. It's important
to do this validation in `try_new` because all table accesses must first construct
the [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae). This ensures that developers never forget to check that kernel
supports reading the table, and that all table accesses are legal.

Note: In the future, we will perform stricter checks on the set of reader and writer
features. In particular, we will check that:
    - Non-legacy features must appear in both reader features and writer features lists. If
      such a feature is present, the reader version and writer version must be 3, and 5
      respectively.
    - Legacy reader features occur when the reader version is 3, but the writer version is
      either 5 or 6. In this case, the writer feature list must be empty.
    - Column mapping is the only legacy feature present in kernel. No future delta versions
      will introduce new legacy features.
See: <https://github.com/delta-io/delta-kernel-rs/issues/650>

<a id="op-96f5d259c341d175fe9e2ab7"></a>
## version

`function` · `buoyant_kernel::table_configuration::TableConfiguration::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L547).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::TableConfiguration", "path": "TableConfiguration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [873, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:547`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`Version`](../operations/buoyant_kernel.Version.md#op-7434cc865af7c4038dc36837) which this [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae) belongs to.

<a id="op-00c1cec8ed9ae94c4c6b85dd"></a>
## column_mapping_mode

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::column_mapping_mode` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
column_mapping_mode: table_features::ColumnMappingMode
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L116).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:116`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06ffeb5cf637bc40989db323"></a>
## logical_schema

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::logical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
logical_schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L108).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:108`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Logical schema: field names are the user-facing (logical) column names.

<a id="op-6baac29964cb38c9ce4e1728"></a>
## logical_schema_without_partition_columns

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::logical_schema_without_partition_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
logical_schema_without_partition_columns: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L110).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:110`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The subset of the logical schema that remains after excluding partition columns.

<a id="op-e85cab37c81a56ee12f68298"></a>
## metadata

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metadata: actions::Metadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L105).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:105`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8fd55fa6009039034ca49cf"></a>
## physical_data_schema_without_partition_columns

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::physical_data_schema_without_partition_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
physical_data_schema_without_partition_columns: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L114).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The subset of the physical schema that remains after excluding partition columns.

<a id="op-7b613774eb939da1213a17bf"></a>
## physical_schema

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::physical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
physical_schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L112).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:112`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Physical schema for all columns (field names respect column mapping mode).

<a id="op-8fb656947267b7feae1fe642"></a>
## protocol

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::protocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
protocol: actions::Protocol
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L106).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:106`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-816446415e5bcada92548dc9"></a>
## table_properties

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::table_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_properties: table_properties::TableProperties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L115).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48b5ebd8ec5892f735084e5b"></a>
## table_root

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_root: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L117).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:117`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99cdc691af01f3bc170844db"></a>
## version

`struct_field` · `buoyant_kernel::table_configuration::TableConfiguration::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L118).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:118`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
