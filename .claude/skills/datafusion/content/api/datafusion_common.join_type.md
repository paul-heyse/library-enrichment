# `datafusion_common::join_type`

Crate `datafusion-common` · 3 public items · structured records in [`model/datafusion_common.join_type.json`](../model/datafusion_common.join_type.json)

## JoinConstraint

`enum` · `datafusion_common::join_type::JoinConstraint`

Also reachable as `datafusion::common::JoinConstraint`, `datafusion::logical_expr::JoinConstraint`, `datafusion_common::JoinConstraint`, `datafusion_expr::JoinConstraint`, `datafusion_expr::logical_plan::JoinConstraint`

```rust
enum JoinConstraint
```

**Variants**: `On`, `Using`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_common.join_type.JoinConstraint.md).


Join constraint

---

## JoinSide

`enum` · `datafusion_common::join_type::JoinSide`

Also reachable as `datafusion::common::JoinSide`, `datafusion_common::JoinSide`

```rust
enum JoinSide
```

**Variants**: `Left`, `Right`, `None`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn negate(&self) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.join_type.JoinSide.md).


Join side.
Stores the referred table side during calculations

---

## JoinType

`enum` · `datafusion_common::join_type::JoinType`

Also reachable as `datafusion::common::JoinType`, `datafusion::logical_expr::JoinType`, `datafusion::prelude::JoinType`, `datafusion_common::JoinType`, `datafusion_expr::JoinType`, `datafusion_expr::logical_plan::JoinType`

```rust
enum JoinType
```

**Variants**: `Inner`, `Left`, `Right`, `Full`, `LeftSemi`, `RightSemi`, `LeftAnti`, `RightAnti`, `LeftMark`, `RightMark`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn empty_build_side_produces_empty_result(self) -> bool
fn empty_map_produces_empty_result(self) -> bool
fn is_outer(self) -> bool
fn on_lr_is_preserved(&self) -> (bool, bool)
fn supports_swap(&self) -> bool
fn swap(&self) -> JoinType
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.join_type.JoinType.md).


Join type

---
