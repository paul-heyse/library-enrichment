# `ruff_formatter::group_id`

Crate `ruff_formatter` · 3 public items · structured records in [`model/ruff_formatter.group_id.json`](../model/ruff_formatter.group_id.json)

## DebugGroupId

`struct` · `ruff_formatter::group_id::DebugGroupId`

```rust
struct DebugGroupId
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ReleaseGroupId

`struct` · `ruff_formatter::group_id::ReleaseGroupId`

```rust
struct ReleaseGroupId
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Unique identification for a group.

See [`crate::Formatter::group_id`] on how to get a unique id.

---

## GroupId

`type_alias` · `ruff_formatter::group_id::GroupId`

Also reachable as `ruff_formatter::GroupId`

```rust
type GroupId = DebugGroupId
```

---
