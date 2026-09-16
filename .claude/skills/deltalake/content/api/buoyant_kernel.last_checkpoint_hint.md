# `buoyant_kernel::last_checkpoint_hint`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.last_checkpoint_hint.json`](../model/buoyant_kernel.last_checkpoint_hint.json)

## HintAction

`enum` · `buoyant_kernel::last_checkpoint_hint::HintAction`

Also reachable as `delta_kernel::last_checkpoint_hint::HintAction`

```rust
enum HintAction
```

**Variants**: `Metadata`, `Protocol`, `Txn`, `DomainMetadata`, `CheckpointMetadata`

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

One element of [`LastCheckpointV2`]'s `non_file_actions`. A log action is exactly one action
type, so this is an externally-tagged enum keyed by the action name, reusing kernel's action
structs to yield the same types as log replay. An unrecognized action key fails the whole-hint
parse; `try_read` swallows that, so the reader falls back to reading the checkpoint.

---

## LastCheckpointHint

`struct` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointHint`

Also reachable as `delta_kernel::last_checkpoint_hint::LastCheckpointHint`

```rust
struct LastCheckpointHint
```

**Fields**: `version`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn path(log_root: &Url) -> DeltaResult<Url>
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

## LastCheckpointV2

`struct` · `buoyant_kernel::last_checkpoint_hint::LastCheckpointV2`

Also reachable as `delta_kernel::last_checkpoint_hint::LastCheckpointV2`

```rust
struct LastCheckpointV2
```

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

The `v2Checkpoint` object embedded in a `_last_checkpoint` hint for a V2 checkpoint.

Carries the V2 checkpoint file's identity and metadata plus the actions a reader would otherwise
read from the checkpoint itself -- its sidecar references and its non-file actions. Absent for
V1 / classic checkpoints.

---
