# `datafusion_proto::bytes`

Crate `datafusion-proto` · 17 public items · structured records in [`model/datafusion_proto.bytes.json`](../model/datafusion_proto.bytes.json)

## logical_plan_from_bytes

`function` · `datafusion_proto::bytes::logical_plan_from_bytes`

```rust
fn logical_plan_from_bytes(bytes: &[u8], ctx: &datafusion_execution::TaskContext) -> datafusion_common::Result<datafusion_expr::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.logical_plan_from_bytes.md).


Deserialize a LogicalPlan from bytes

---

## logical_plan_from_bytes_with_extension_codec

`function` · `datafusion_proto::bytes::logical_plan_from_bytes_with_extension_codec`

```rust
fn logical_plan_from_bytes_with_extension_codec(bytes: &[u8], ctx: &datafusion_execution::TaskContext, extension_codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.logical_plan_from_bytes_with_extension_codec.md).


Deserialize a LogicalPlan from bytes

---

## logical_plan_from_json

`function` · `datafusion_proto::bytes::logical_plan_from_json`

```rust
fn logical_plan_from_json(json: &str, ctx: &datafusion_execution::TaskContext) -> datafusion_common::Result<datafusion_expr::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.logical_plan_from_json.md).


Deserialize a LogicalPlan from JSON

---

## logical_plan_from_json_with_extension_codec

`function` · `datafusion_proto::bytes::logical_plan_from_json_with_extension_codec`

```rust
fn logical_plan_from_json_with_extension_codec(json: &str, ctx: &datafusion_execution::TaskContext, extension_codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.logical_plan_from_json_with_extension_codec.md).


Deserialize a LogicalPlan from JSON

---

## logical_plan_to_bytes

`function` · `datafusion_proto::bytes::logical_plan_to_bytes`

```rust
fn logical_plan_to_bytes(plan: &datafusion_expr::LogicalPlan) -> datafusion_common::Result<prost::bytes::Bytes>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.logical_plan_to_bytes.md).


Serialize a LogicalPlan as bytes

---

## logical_plan_to_bytes_with_extension_codec

`function` · `datafusion_proto::bytes::logical_plan_to_bytes_with_extension_codec`

```rust
fn logical_plan_to_bytes_with_extension_codec(plan: &datafusion_expr::LogicalPlan, extension_codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<prost::bytes::Bytes>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.logical_plan_to_bytes_with_extension_codec.md).


Serialize a LogicalPlan as bytes, using the provided extension codec

---

## logical_plan_to_json

`function` · `datafusion_proto::bytes::logical_plan_to_json`

```rust
fn logical_plan_to_json(plan: &datafusion_expr::LogicalPlan) -> datafusion_common::Result<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.logical_plan_to_json.md).


Serialize a LogicalPlan as JSON

---

## logical_plan_to_json_with_extension_codec

`function` · `datafusion_proto::bytes::logical_plan_to_json_with_extension_codec`

```rust
fn logical_plan_to_json_with_extension_codec(plan: &datafusion_expr::LogicalPlan, extension_codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.logical_plan_to_json_with_extension_codec.md).


Serialize a LogicalPlan as JSON using the provided extension codec

---

## physical_plan_from_bytes

`function` · `datafusion_proto::bytes::physical_plan_from_bytes`

```rust
fn physical_plan_from_bytes(bytes: &[u8], ctx: &datafusion_execution::TaskContext) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.physical_plan_from_bytes.md).


Deserialize a PhysicalPlan from bytes

---

## physical_plan_from_bytes_with_extension_codec

`function` · `datafusion_proto::bytes::physical_plan_from_bytes_with_extension_codec`

```rust
fn physical_plan_from_bytes_with_extension_codec(bytes: &[u8], ctx: &datafusion_execution::TaskContext, extension_codec: &dyn PhysicalExtensionCodec) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.physical_plan_from_bytes_with_extension_codec.md).


Deserialize a PhysicalPlan from bytes

---

## physical_plan_from_bytes_with_proto_converter

`function` · `datafusion_proto::bytes::physical_plan_from_bytes_with_proto_converter`

```rust
fn physical_plan_from_bytes_with_proto_converter(bytes: &[u8], ctx: &datafusion_execution::TaskContext, extension_codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.physical_plan_from_bytes_with_proto_converter.md).


Deserialize a PhysicalPlan from bytes

---

## physical_plan_from_json

`function` · `datafusion_proto::bytes::physical_plan_from_json`

```rust
fn physical_plan_from_json(json: &str, ctx: &datafusion_execution::TaskContext) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.physical_plan_from_json.md).


Deserialize a PhysicalPlan from JSON

---

## physical_plan_to_bytes

`function` · `datafusion_proto::bytes::physical_plan_to_bytes`

```rust
fn physical_plan_to_bytes(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<prost::bytes::Bytes>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.physical_plan_to_bytes.md).


Serialize a PhysicalPlan as bytes

---

## physical_plan_to_bytes_with_extension_codec

`function` · `datafusion_proto::bytes::physical_plan_to_bytes_with_extension_codec`

```rust
fn physical_plan_to_bytes_with_extension_codec(plan: std::sync::Arc<dyn ExecutionPlan>, extension_codec: &dyn PhysicalExtensionCodec) -> datafusion_common::Result<prost::bytes::Bytes>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.physical_plan_to_bytes_with_extension_codec.md).


Serialize a PhysicalPlan as bytes, using the provided extension codec

---

## physical_plan_to_bytes_with_proto_converter

`function` · `datafusion_proto::bytes::physical_plan_to_bytes_with_proto_converter`

```rust
fn physical_plan_to_bytes_with_proto_converter(plan: std::sync::Arc<dyn ExecutionPlan>, extension_codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<prost::bytes::Bytes>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.physical_plan_to_bytes_with_proto_converter.md).


Serialize a PhysicalPlan as bytes, using the provided extension codec
and protobuf converter.

---

## physical_plan_to_json

`function` · `datafusion_proto::bytes::physical_plan_to_json`

```rust
fn physical_plan_to_json(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.physical_plan_to_json.md).


Serialize a PhysicalPlan as JSON

---

## Serializeable

`trait` · `datafusion_proto::bytes::Serializeable`

```rust
trait Serializeable: Sized
```

**Implementors** (1)

- `datafusion_expr::expr::Expr`

**Methods** (3)

```rust
fn from_bytes(bytes: &[u8]) -> Result<Self>
fn from_bytes_with_ctx(bytes: &[u8], ctx: &TaskContext) -> Result<Self>
fn to_bytes(&self) -> Result<Bytes>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.bytes.Serializeable.md).


Encodes something (such as [`Expr`]) to/from a stream of
bytes.

```
use datafusion_expr::{col, lit, Expr};
use datafusion_proto::bytes::Serializeable;

// Create a new `Expr` a < 32
let expr = col("a").lt(lit(5i32));

// Convert it to an opaque form
let bytes = expr.to_bytes().unwrap();

// Decode bytes from somewhere (over network, etc.)
let decoded_expr = Expr::from_bytes(&bytes).unwrap();
assert_eq!(expr, decoded_expr);
```

---
