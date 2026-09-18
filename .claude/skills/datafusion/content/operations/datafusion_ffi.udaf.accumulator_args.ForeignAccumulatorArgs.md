# `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udaf.accumulator_args.ForeignAccumulatorArgs.json).

<a id="op-1c4726610b622276669150f6"></a>
## ForeignAccumulatorArgs

`struct` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs` · datafusion-ffi 55.1.0

```rust
struct ForeignAccumulatorArgs
```

Source: `src/udaf/accumulator_args.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This struct mirrors AccumulatorArgs except that it contains owned data.
It is necessary to create this struct so that we can parse the protobuf
data across the FFI boundary and turn it into owned data that
AccumulatorArgs can then reference.

<a id="op-bea174bed0e283459bbeac84"></a>
## expr_fields

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::expr_fields` · datafusion-ffi 55.1.0

```rust
expr_fields: Vec<arrow_schema::FieldRef>
```

Source: `src/udaf/accumulator_args.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b642cb3a2e46db31f5d5325"></a>
## exprs

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::exprs` · datafusion-ffi 55.1.0

```rust
exprs: Vec<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/udaf/accumulator_args.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7baaaf1be67b7a46af308c0"></a>
## ignore_nulls

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::ignore_nulls` · datafusion-ffi 55.1.0

```rust
ignore_nulls: bool
```

Source: `src/udaf/accumulator_args.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65eba7eecd674bb71ca6cbe6"></a>
## is_distinct

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::is_distinct` · datafusion-ffi 55.1.0

```rust
is_distinct: bool
```

Source: `src/udaf/accumulator_args.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e94803c72c34bbbcb7ed9fd"></a>
## is_reversed

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::is_reversed` · datafusion-ffi 55.1.0

```rust
is_reversed: bool
```

Source: `src/udaf/accumulator_args.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d07c0dc8717826b714b5bb2"></a>
## name

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::name` · datafusion-ffi 55.1.0

```rust
name: String
```

Source: `src/udaf/accumulator_args.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc63f03496ca098d5bfee020"></a>
## order_bys

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::order_bys` · datafusion-ffi 55.1.0

```rust
order_bys: Vec<datafusion_physical_expr::PhysicalSortExpr>
```

Source: `src/udaf/accumulator_args.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-854ad8df8c2d3b4e5bab2987"></a>
## return_field

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::return_field` · datafusion-ffi 55.1.0

```rust
return_field: arrow_schema::FieldRef
```

Source: `src/udaf/accumulator_args.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-714f5196dfa3af70621644ad"></a>
## schema

`struct_field` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs::schema` · datafusion-ffi 55.1.0

```rust
schema: arrow::datatypes::Schema
```

Source: `src/udaf/accumulator_args.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
