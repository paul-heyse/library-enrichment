# `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.cdf.scan.DeltaCdfTableProvider.json).

<a id="op-f23cb1f3d3e62a6ec4598c3a"></a>
## DeltaCdfTableProvider

`struct` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaCdfTableProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L28).

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A DataFusion [`TableProvider`](datafusion::catalog::TableProvider) that exposes a Delta
table's Change Data Feed (CDF) as a queryable relation.

Wraps a [`CdfLoadBuilder`](../operations/deltalake_core.operations.load_cdf.CdfLoadBuilder.md#op-f0e83221d70bc885239ff83c) together with the resolved output schema so the CDF stream
(insertions, updates and deletions across versions) can be scanned through the normal
DataFusion planning machinery.

Unresolved upstream links (retained, not inferred): `datafusion::catalog::TableProvider`.

<a id="op-74befa4042867f2094ce2411"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider", "path": "DeltaCdfTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "crates/core/src/delta_datafusion/cdf/scan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c264e3e1089f70d878cd5b3f"></a>
## scan

`function` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::scan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn scan(&self, session: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> DataFusionResult<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L65).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider", "path": "DeltaCdfTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [125, 2], "filename": "crates/core/src/delta_datafusion/cdf/scan.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccab0392e6ba934b7d4aa652"></a>
## schema

`function` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L57).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider", "path": "DeltaCdfTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [125, 2], "filename": "crates/core/src/delta_datafusion/cdf/scan.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09a62190ed0b801076e6b78d"></a>
## supports_filters_pushdown

`function` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::supports_filters_pushdown` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn supports_filters_pushdown(&self, filter: &[&Expr]) -> DataFusionResult<Vec<TableProviderFilterPushDown>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L116).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider", "path": "DeltaCdfTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [125, 2], "filename": "crates/core/src/delta_datafusion/cdf/scan.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:116`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00c0941a21d41a7071c53fea"></a>
## table_type

`function` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::table_type` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_type(&self) -> TableType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L61).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider", "path": "DeltaCdfTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [125, 2], "filename": "crates/core/src/delta_datafusion/cdf/scan.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProvider", "path": "TableProvider"}, "trait_path": "datafusion_session::table::TableProvider"}`

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:61`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76590db94c2c43f9d137eeea"></a>
## try_new

`function` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(cdf_builder: CdfLoadBuilder) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L35).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider", "path": "DeltaCdfTableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [53, 2], "filename": "crates/core/src/delta_datafusion/cdf/scan.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build a DeltaCDFTableProvider

<a id="op-02bbea9688bcf8900df474c1"></a>
## cdf_builder

`struct_field` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::cdf_builder` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
cdf_builder: operations::load_cdf::CdfLoadBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L29).

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc5937267881a7be3df0de62"></a>
## schema

`struct_field` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
schema: arrow::datatypes::SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan.rs#L30).

Source: `crates/core/src/delta_datafusion/cdf/scan.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
