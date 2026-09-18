# `buoyant_kernel::commit_range::builder`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.commit_range.builder.json`](../model/buoyant_kernel.commit_range.builder.json)

## CommitOrdering

`enum` · `buoyant_kernel::commit_range::builder::CommitOrdering`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.commit_range.builder.CommitOrdering.md)

Also reachable as `buoyant_kernel::commit_range::CommitOrdering`, `delta_kernel::commit_range::builder::CommitOrdering`

```rust
enum CommitOrdering
```

**Variants**: `AscendingOrder`, `DescendingOrder`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Direction in which [`CommitRange::commits`] yields commits.
Default is [`CommitOrdering::AscendingOrder`]

---

## CommitRangeBuilder

`struct` · `buoyant_kernel::commit_range::builder::CommitRangeBuilder`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.commit_range.builder.CommitRangeBuilder.md)

Also reachable as `buoyant_kernel::commit_range::CommitRangeBuilder`, `delta_kernel::commit_range::builder::CommitRangeBuilder`

```rust
struct CommitRangeBuilder
```

**Methods** (3)

```rust
fn build(&self, engine: &dyn Engine) -> DeltaResult<CommitRange>
fn with_end_version(self, end_version: Version) -> Self
fn with_ordering(self, commit_ordering: CommitOrdering) -> Self
```

Builder for a [`CommitRange`].

Created via [`CommitRange::builder_for`] (path-based) or
[`CommitRange::builder_from`] (snapshot-based). Supports configuring an end version
and the commit ordering. [`Self::build`] performs delta-log listing and contiguity
validation.

---
