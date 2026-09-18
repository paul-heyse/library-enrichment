# `buoyant_kernel::crc::state`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.crc.state.json`](../model/buoyant_kernel.crc.state.json)

## DomainMetadataState

`enum` · `buoyant_kernel::crc::state::DomainMetadataState`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.crc.state.DomainMetadataState.md)

Also reachable as `buoyant_kernel::crc::DomainMetadataState`, `delta_kernel::crc::state::DomainMetadataState`

```rust
enum DomainMetadataState
```

**Variants**: `Complete`, `Partial`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

The completeness state of cached domain metadata in a CRC.

---

## FileStatsState

`enum` · `buoyant_kernel::crc::state::FileStatsState`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.crc.state.FileStatsState.md)

Also reachable as `buoyant_kernel::crc::FileStatsState`, `delta_kernel::crc::state::FileStatsState`

```rust
enum FileStatsState
```

**Variants**: `Complete`, `Indeterminate`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn file_stats(&self) -> Option<&FileStats>
fn is_complete(&self) -> bool
```

The state of file statistics for a CRC.

# State transitions during `Crc::apply`

| Current       | + safe op     | + unsafe op or missing remove.size |
|---------------|---------------|------------------------------------|
| Complete      | Complete      | Indeterminate                      |
| Indeterminate | Indeterminate | Indeterminate                      |

---

## SetTransactionState

`enum` · `buoyant_kernel::crc::state::SetTransactionState`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.crc.state.SetTransactionState.md)

Also reachable as `buoyant_kernel::crc::SetTransactionState`, `delta_kernel::crc::state::SetTransactionState`

```rust
enum SetTransactionState
```

**Variants**: `Complete`, `Partial`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

The completeness state of cached set transactions in a CRC.

---
