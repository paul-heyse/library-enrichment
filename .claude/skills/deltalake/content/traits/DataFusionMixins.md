# DataFusionMixins

`deltalake_core::delta_datafusion::DataFusionMixins`

```rust
trait DataFusionMixins
```

Also reachable as `deltalake::delta_datafusion::DataFusionMixins`

Prose: [`api/deltalake_core.delta_datafusion.md`](../api/deltalake_core.delta_datafusion.md#datafusionmixins) · records: [`model/deltalake_core.delta_datafusion.json`](../model/deltalake_core.delta_datafusion.json)

## Required

Every implementation must supply these.

```rust
fn input_schema(&self) -> ArrowSchemaRef
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
fn read_schema(&self) -> ArrowSchemaRef
```

## Implementors (3)

Read one before writing your own.

- `deltalake_core::kernel::snapshot::EagerSnapshot`
- `deltalake_core::kernel::snapshot::Snapshot`
- `deltalake_core::kernel::snapshot::log_data::LogDataHandler`

## Documentation

Convenience trait for calling common methods on snapshot hierarchies
