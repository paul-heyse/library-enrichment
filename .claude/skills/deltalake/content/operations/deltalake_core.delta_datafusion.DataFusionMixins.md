# `deltalake_core::delta_datafusion::DataFusionMixins`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.DataFusionMixins.json).

<a id="op-a3e7d3071c58054430f5d9d3"></a>
## DataFusionMixins

`trait` · `deltalake_core::delta_datafusion::DataFusionMixins` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DataFusionMixins
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L131).

Source: `crates/core/src/delta_datafusion/mod.rs:131`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Convenience trait for calling common methods on snapshot hierarchies

<a id="op-52e91e8ff52525da9d07f4da"></a>
## input_schema

`function` · `deltalake_core::delta_datafusion::DataFusionMixins::input_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn input_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L136).

Source: `crates/core/src/delta_datafusion/mod.rs:136`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table schema as an [`ArrowSchemaRef`]

Unresolved upstream links (retained, not inferred): ``ArrowSchemaRef``.

<a id="op-b5a9a4a67ebd07d36e943d36"></a>
## parse_predicate_expression

`function` · `deltalake_core::delta_datafusion::DataFusionMixins::parse_predicate_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L139).

Source: `crates/core/src/delta_datafusion/mod.rs:139`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse an expression string into a datafusion [`Expr`]

Unresolved upstream links (retained, not inferred): ``Expr``.

<a id="op-322d9e654642aeadb83aff95"></a>
## read_schema

`function` · `deltalake_core::delta_datafusion::DataFusionMixins::read_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L133).

Source: `crates/core/src/delta_datafusion/mod.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The physical datafusion schema of a table
