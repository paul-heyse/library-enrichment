# `deltalake_core::kernel::models`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.kernel.models.json`](../model/deltalake_core.kernel.models.json)

## Action

`enum` · `deltalake_core::kernel::models::Action`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.models.Action.md)

Also reachable as `deltalake::kernel::Action`, `deltalake::kernel::models::Action`, `deltalake_core::kernel::Action`

```rust
enum Action
```

**Variants**: `Metadata`, `Protocol`, `Add`, `Remove`, `Cdc`, `Txn`, `CommitInfo`, `DomainMetadata`

**Implements**: `core::convert::From`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn commit_info(info: HashMap<String, serde_json::Value>) -> Self
```

**via `core::convert::From`**

```rust
fn from(a: Protocol) -> Self
fn from(a: Add) -> Self
fn from(a: Metadata) -> Self
fn from(a: CommitInfo) -> Self
fn from(a: Transaction) -> Self
fn from(a: AddCDCFile) -> Self
fn from(a: Remove) -> Self
fn from(a: DomainMetadata) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
