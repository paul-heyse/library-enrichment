# `buoyant_kernel::transaction::data_layout`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.transaction.data_layout.json`](../model/buoyant_kernel.transaction.data_layout.json)

## DataLayout

`enum` · `buoyant_kernel::transaction::data_layout::DataLayout`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md)

Also reachable as `delta_kernel::transaction::data_layout::DataLayout`

```rust
enum DataLayout
```

**Variants**: `None`, `Clustered`, `Partitioned`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn clustered<I, S>(columns: I) -> Self where I: IntoIterator<Item = S>, S: AsRef<str>
fn partitioned<I, S>(columns: I) -> Self where I: IntoIterator<Item = S>, S: AsRef<str>
```

Data layout configuration for a Delta table.

Determines how data files are organized within the table:

- [`DataLayout::None`]: No special organization (default)
- [`DataLayout::Clustered`]: Data files optimized for queries on clustering columns
- [`DataLayout::Partitioned`]: Data files organized into directories by partition column values

Partitioning and clustering are mutually exclusive -- only one variant can be active at a time.

---
