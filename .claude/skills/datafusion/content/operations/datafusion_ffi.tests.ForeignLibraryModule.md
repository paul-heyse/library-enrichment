# `datafusion_ffi::tests::ForeignLibraryModule`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.tests.ForeignLibraryModule.json).

<a id="op-84e5a90c5d822045a904d2e3"></a>
## ForeignLibraryModule

`struct` · `datafusion_ffi::tests::ForeignLibraryModule` · datafusion-ffi 55.1.0

```rust
struct ForeignLibraryModule
```

Source: `src/tests/mod.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This struct defines the module interfaces. It is to be shared by
both the module loading program and library that implements the
module.

<a id="op-548cdedc9db086a5463234cd"></a>
## create_catalog

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_catalog` · datafusion-ffi 55.1.0

```rust
create_catalog: fn(proto::logical_extension_codec::FFI_LogicalExtensionCodec) -> catalog_provider::FFI_CatalogProvider
```

Source: `src/tests/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Construct an opinionated catalog provider

<a id="op-bb84162e013247dfa4fa2af2"></a>
## create_catalog_list

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_catalog_list` · datafusion-ffi 55.1.0

```rust
create_catalog_list: fn(proto::logical_extension_codec::FFI_LogicalExtensionCodec) -> catalog_provider_list::FFI_CatalogProviderList
```

Source: `src/tests/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Construct an opinionated catalog provider list

<a id="op-7ce0213ce39a757ee8487a32"></a>
## create_context_aware_optimizer_rule

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_context_aware_optimizer_rule` · datafusion-ffi 55.1.0

```rust
create_context_aware_optimizer_rule: fn() -> physical_optimizer::FFI_PhysicalOptimizerRule
```

Source: `src/tests/mod.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09f5f683f8c8a9490eb12169"></a>
## create_empty_exec

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_empty_exec` · datafusion-ffi 55.1.0

```rust
create_empty_exec: fn() -> execution_plan::FFI_ExecutionPlan
```

Source: `src/tests/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1a7c264795eae38abf31a74"></a>
## create_exec_with_dynamic_expressions

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_exec_with_dynamic_expressions` · datafusion-ffi 55.1.0

```rust
create_exec_with_dynamic_expressions: fn() -> execution_plan::FFI_ExecutionPlan
```

Source: `src/tests/mod.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b82f8d527e175779b78c1c6e"></a>
## create_exec_with_expressions

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_exec_with_expressions` · datafusion-ffi 55.1.0

```rust
create_exec_with_expressions: fn() -> execution_plan::FFI_ExecutionPlan
```

Source: `src/tests/mod.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35159734ef1433a9819e6267"></a>
## create_exec_with_statistics

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_exec_with_statistics` · datafusion-ffi 55.1.0

```rust
create_exec_with_statistics: fn() -> execution_plan::FFI_ExecutionPlan
```

Source: `src/tests/mod.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cd744c29bd00ec4c811c05c"></a>
## create_extension_options

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_extension_options` · datafusion-ffi 55.1.0

```rust
create_extension_options: fn() -> config::extension_options::FFI_ExtensionOptions
```

Source: `src/tests/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Create extension options, for either ConfigOptions or TableOptions

<a id="op-0395117aec9787a2f1831c72"></a>
## create_first_value_udaf

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_first_value_udaf` · datafusion-ffi 55.1.0

```rust
create_first_value_udaf: fn() -> udaf::FFI_AggregateUDF
```

Source: `src/tests/mod.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Create an aggregate UDAF using first_value

<a id="op-3530fb114193c7b6c146a7ce"></a>
## create_nullary_udf

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_nullary_udf` · datafusion-ffi 55.1.0

```rust
create_nullary_udf: fn() -> udf::FFI_ScalarUDF
```

Source: `src/tests/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8020c97d1a9fb13400fd87cc"></a>
## create_physical_optimizer_rule

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_physical_optimizer_rule` · datafusion-ffi 55.1.0

```rust
create_physical_optimizer_rule: fn() -> physical_optimizer::FFI_PhysicalOptimizerRule
```

Source: `src/tests/mod.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81b960af0587bdef8e9a09ac"></a>
## create_placement_udf

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_placement_udf` · datafusion-ffi 55.1.0

```rust
create_placement_udf: fn() -> udf::FFI_ScalarUDF
```

Source: `src/tests/mod.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff13daf0b89c880703101ea5"></a>
## create_query_planner

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_query_planner` · datafusion-ffi 55.1.0

```rust
create_query_planner: fn(proto::logical_extension_codec::FFI_LogicalExtensionCodec, proto::physical_extension_codec::FFI_PhysicalExtensionCodec, util::FFI_Option<query_planner::FFI_QueryPlanner>) -> query_planner::FFI_QueryPlanner
```

Source: `src/tests/mod.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Construct a query planner. When `library_a_planner` is provided the
planner delegates to it, as library C does after library A swaps planners.

<a id="op-d5cd260cc14679c0b1221778"></a>
## create_rank_udwf

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_rank_udwf` · datafusion-ffi 55.1.0

```rust
create_rank_udwf: fn() -> udwf::FFI_WindowUDF
```

Source: `src/tests/mod.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bc259ed8defed5176f3c05e"></a>
## create_scalar_udf

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_scalar_udf` · datafusion-ffi 55.1.0

```rust
create_scalar_udf: fn() -> udf::FFI_ScalarUDF
```

Source: `src/tests/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Create a scalar UDF

<a id="op-6eddba091f943ce3c9dbc524"></a>
## create_stddev_udaf

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_stddev_udaf` · datafusion-ffi 55.1.0

```rust
create_stddev_udaf: fn() -> udaf::FFI_AggregateUDF
```

Source: `src/tests/mod.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Create  grouping UDAF using stddev

<a id="op-81613e23de82a12f65e3e0fa"></a>
## create_sum_udaf

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_sum_udaf` · datafusion-ffi 55.1.0

```rust
create_sum_udaf: fn() -> udaf::FFI_AggregateUDF
```

Source: `src/tests/mod.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Create an aggregate UDAF using sum

<a id="op-2e7fede0e6fd234ddd880158"></a>
## create_table

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_table` · datafusion-ffi 55.1.0

```rust
create_table: fn(bool, proto::logical_extension_codec::FFI_LogicalExtensionCodec) -> table_provider::FFI_TableProvider
```

Source: `src/tests/mod.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Constructs the table provider

<a id="op-855ab4c452476f8cd421ba21"></a>
## create_table_factory

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_table_factory` · datafusion-ffi 55.1.0

```rust
create_table_factory: fn(proto::logical_extension_codec::FFI_LogicalExtensionCodec) -> table_provider_factory::FFI_TableProviderFactory
```

Source: `src/tests/mod.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Constructs the table provider factory

<a id="op-6ad060d029f88d19ae3fe3dd"></a>
## create_table_function

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_table_function` · datafusion-ffi 55.1.0

```rust
create_table_function: fn(proto::logical_extension_codec::FFI_LogicalExtensionCodec) -> udtf::FFI_TableFunction
```

Source: `src/tests/mod.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adba56c9313f66f29bf2c198"></a>
## create_table_with_statistics

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_table_with_statistics` · datafusion-ffi 55.1.0

```rust
create_table_with_statistics: fn(proto::logical_extension_codec::FFI_LogicalExtensionCodec) -> table_provider::FFI_TableProvider
```

Source: `src/tests/mod.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2af8dbd18b4c55973306bbd7"></a>
## create_timezone_udf

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::create_timezone_udf` · datafusion-ffi 55.1.0

```rust
create_timezone_udf: fn() -> udf::FFI_ScalarUDF
```

Source: `src/tests/mod.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3d5a2e54b1cbf3982194e31"></a>
## version

`struct_field` · `datafusion_ffi::tests::ForeignLibraryModule::version` · datafusion-ffi 55.1.0

```rust
version: fn() -> u64
```

Source: `src/tests/mod.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
