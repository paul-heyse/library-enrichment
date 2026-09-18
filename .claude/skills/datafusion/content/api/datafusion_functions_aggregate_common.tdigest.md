# `datafusion_functions_aggregate_common::tdigest`

Crate `datafusion-functions-aggregate-common` · 3 public items · structured records in [`model/datafusion_functions_aggregate_common.tdigest.json`](../model/datafusion_functions_aggregate_common.tdigest.json)

## DEFAULT_MAX_SIZE

`constant` · `datafusion_functions_aggregate_common::tdigest::DEFAULT_MAX_SIZE`

```rust
const DEFAULT_MAX_SIZE: usize = 100
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.tdigest.DEFAULT_MAX_SIZE.md).


---

## Centroid

`struct` · `datafusion_functions_aggregate_common::tdigest::Centroid`

```rust
struct Centroid
```

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn add(&mut self, sum: f64, weight: f64) -> f64
fn cmp_mean(&self, other: &Self) -> Ordering
fn mean(&self) -> f64
fn new(mean: f64, weight: f64) -> Self
fn weight(&self) -> f64
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.tdigest.Centroid.md).


Centroid implementation to the cluster mentioned in the paper.

---

## TDigest

`struct` · `datafusion_functions_aggregate_common::tdigest::TDigest`

```rust
struct TDigest
```

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn centroids(&self) -> &[Centroid]
fn count(&self) -> f64
fn estimate_quantile(&self, q: f64) -> f64
fn from_scalar_state(state: &[ScalarValue]) -> Self
fn max(&self) -> f64
fn max_size(&self) -> usize
fn merge_digests<'a>(digests: impl IntoIterator<Item = &'a TDigest>) -> TDigest
fn merge_sorted_f64(&self, sorted_values: &[f64]) -> TDigest
fn merge_unsorted_f64(&self, unsorted_values: Vec<f64>) -> TDigest
fn min(&self) -> f64
fn new(max_size: usize) -> Self
fn new_with_centroid(max_size: usize, centroid: Centroid) -> Self
fn size(&self) -> usize
fn sum(&self) -> f64
fn to_scalar_state(&self) -> Vec<ScalarValue>
fn try_from_parts(max_size: usize, sum: f64, count: f64, max: f64, min: f64, centroids: Vec<Centroid>) -> Result<Self, DataFusionError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md).


T-Digest to be operated on.

---
