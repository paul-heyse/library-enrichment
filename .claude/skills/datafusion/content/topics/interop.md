# Cross-process interop

Four separate mechanisms for getting a plan or a provider across a boundary. Substrait is the cross-engine plan format; `datafusion-proto` is DataFusion's own plan serialization; `datafusion-ffi` moves providers and UDFs across a stable ABI so a plugin can be compiled separately; Flight carries record batches over the network. Custom nodes need an explicit codec in the serialization paths — they do not round-trip for free.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_expr::logical_plan::plan::LogicalPlan` | enum | 52 | [prose](../api/datafusion_expr.logical_plan.plan.md#logicalplan) | [records](../model/datafusion_expr.logical_plan.plan.json) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | trait | 35 | [prose](../api/datafusion_physical_plan.execution_plan.md#executionplan) | [records](../model/datafusion_physical_plan.execution_plan.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_proto::physical_plan::PhysicalExtensionCodec` | 2 | 10 | 3 | [PhysicalExtensionCodec](../traits/PhysicalExtensionCodec.md) |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode` | 11 | 3 | 0 | [UserDefinedLogicalNode](../traits/UserDefinedLogicalNode.md) |

## Settings (1)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.spark.map_key_dedup_policy` | EXCEPTION |

## Runnable examples (7)

- [`corpus/examples/flight/client.rs`](../corpus/examples/flight/client.rs)
- [`corpus/examples/flight/main.rs`](../corpus/examples/flight/main.rs)
- [`corpus/examples/flight/server.rs`](../corpus/examples/flight/server.rs)
- [`corpus/examples/flight/sql_server.rs`](../corpus/examples/flight/sql_server.rs)
- [`corpus/examples/proto/composed_extension_codec.rs`](../corpus/examples/proto/composed_extension_codec.rs)
- [`corpus/examples/proto/expression_deduplication.rs`](../corpus/examples/proto/expression_deduplication.rs)
- [`corpus/examples/proto/main.rs`](../corpus/examples/proto/main.rs)

## Upstream guides

- [`corpus/guides/library-user-guide/extensions.md`](../corpus/guides/library-user-guide/extensions.md)

## Decision rules

- Logical proto extensions use LogicalExtensionCodec; physical extensions use PhysicalExtensionCodec. Substrait has separate extension/registry requirements; verify support and round trips.
- Use the documented FFI compatibility and ownership contract for separately compiled plugins. Neither arbitrary DataFusion-version compatibility nor serialization follows from a stable ABI claim.

## Anti-patterns

- Expecting a custom extension node to survive a proto or Substrait round trip without a codec.
- Treating Substrait as lossless for DataFusion-specific constructs.

## Agent checklist

- Do all custom nodes in the plan have codecs?
- Was the round trip tested, rather than assumed?
