# `tsp_types::common`

Crate `tsp_types` · 8 public items · structured records in [`model/tsp_types.common.json`](../model/tsp_types.common.json)

## TSP_PROTOCOL_VERSION

`constant` · `tsp_types::common::TSP_PROTOCOL_VERSION`

Also reachable as `tsp_types::TSP_PROTOCOL_VERSION`

```rust
const TSP_PROTOCOL_VERSION: tsp::TypeServerVersion = tsp::TypeServerVersion::Current
```

---

## GetTypeArg

`enum` · `tsp_types::common::GetTypeArg`

Also reachable as `tsp_types::GetTypeArg`

```rust
enum GetTypeArg
```

**Variants**: `Declaration`, `Node`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn end_position(&self) -> tsp::Position
fn position(&self) -> tsp::Position
fn uri(&self) -> &str
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The `arg` field in a getComputedType/getDeclaredType/getExpectedType request.

This can be either a `Node` (just `uri` + `range`) or a `Declaration`
(which contains a nested `node` with `uri` + `range`, plus extra fields).
We use `#[serde(untagged)]` so serde tries each variant in order.

---

## as_tsp_request

`function` · `tsp_types::common::as_tsp_request`

Also reachable as `tsp_types::as_tsp_request`

```rust
fn as_tsp_request<T>(x: &lsp_server::Request, method_name: &str) -> Option<Result<T, serde_json::Error>> where T: DeserializeOwned
```

Handle TypeServer Protocol (TSP) requests that don't implement the LSP Request trait

---

## error_response

`function` · `tsp_types::common::error_response`

Also reachable as `tsp_types::error_response`

```rust
fn error_response(id: lsp_server::RequestId, code: i32, message: String) -> lsp_server::Response
```

Helper to build a JSON-RPC error response for TSP handlers

---

## snapshot_outdated_error

`function` · `tsp_types::common::snapshot_outdated_error`

Also reachable as `tsp_types::snapshot_outdated_error`

```rust
fn snapshot_outdated_error() -> lsp_server::ResponseError
```

Creates a snapshot outdated error

---

## GetSupportedProtocolVersionParams

`struct` · `tsp_types::common::GetSupportedProtocolVersionParams`

Also reachable as `tsp_types::GetSupportedProtocolVersionParams`

```rust
struct GetSupportedProtocolVersionParams
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## GetTypeArgNode

`struct` · `tsp_types::common::GetTypeArgNode`

Also reachable as `tsp_types::GetTypeArgNode`

```rust
struct GetTypeArgNode
```

**Fields**: `uri`, `range`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The location fields shared by both Node and Declaration.node.

---

## GetTypeParams

`struct` · `tsp_types::common::GetTypeParams`

Also reachable as `tsp_types::GetTypeParams`

```rust
struct GetTypeParams
```

**Fields**: `arg`, `snapshot`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn end_position(&self) -> tsp::Position
fn position(&self) -> tsp::Position
fn uri(&self) -> &str
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Parameters for getComputedType, getDeclaredType, and getExpectedType
requests.

The client sends `{ "arg": Node | Declaration, "snapshot": number }`.

---
