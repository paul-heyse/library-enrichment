# `datafusion_functions_aggregate_common::tdigest::TDigest`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.tdigest.TDigest.json).

<a id="op-df5dff1c945937fa245a5a76"></a>
## TDigest

`struct` · `datafusion_functions_aggregate_common::tdigest::TDigest` · datafusion-functions-aggregate-common 55.1.0

```rust
struct TDigest
```

Source: `src/tdigest.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

T-Digest to be operated on.

<a id="op-883900af3ca9afbd35640c94"></a>
## centroids

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::centroids` · datafusion-functions-aggregate-common 55.1.0

```rust
fn centroids(&self) -> &[Centroid]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The centroids that make up this digest, ordered by mean.

Together with the [`Self::sum()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-d0097daf09bcf0a8c3338ccc), [`Self::max_size()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-55cd766524dfd6c8d0aa2c9f),
[`Self::count()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-95b6ba8bbce9cb1e436d830f), [`Self::max()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-532809935e68e016ec334dc0), and [`Self::min()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-a21e2745a12f9f99d45da0e9) accessors this
exposes the full serialized state of the digest without packing it into
a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) list. See [`Self::try_from_parts()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-45ab21b41dacf7f5dd9d78df) for the inverse.

<a id="op-e6be275edbf11efd514fcd1f"></a>
## clone

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::clone` · datafusion-functions-aggregate-common 55.1.0

```rust
fn clone(&self) -> TDigest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 28], "end": [97, 33], "filename": "src/tdigest.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tdigest.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95b6ba8bbce9cb1e436d830f"></a>
## count

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::count` · datafusion-functions-aggregate-common 55.1.0

```rust
fn count(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9ace0fa67665952860f9a35"></a>
## default

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [185, 2], "filename": "src/tdigest.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/tdigest.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4736c3e5dc50b98b7931a9b9"></a>
## eq

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::eq` · datafusion-functions-aggregate-common 55.1.0

```rust
fn eq(&self, other: &TDigest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 17], "end": [97, 26], "filename": "src/tdigest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tdigest.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efccfaf9f57cb28200f9f007"></a>
## estimate_quantile

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::estimate_quantile` · datafusion-functions-aggregate-common 55.1.0

```rust
fn estimate_quantile(&self, q: f64) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [699, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

To estimate the value located at `q` quantile

<a id="op-cbf117f2f0330c7ddbed6a05"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 10], "end": [97, 15], "filename": "src/tdigest.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tdigest.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46497474b38817dc27a1bc8e"></a>
## from_scalar_state

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::from_scalar_state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn from_scalar_state(state: &[ScalarValue]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [699, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Unpack the serialized state of a [`TDigest`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-df5dff1c945937fa245a5a76) produced by
[`Self::to_scalar_state()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-eb540ff260eaa5f0b70907c4).

# Correctness

Providing input to this method that was not obtained from
[`Self::to_scalar_state()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-eb540ff260eaa5f0b70907c4) results in undefined behaviour and may
panic.

<a id="op-532809935e68e016ec334dc0"></a>
## max

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::max` · datafusion-functions-aggregate-common 55.1.0

```rust
fn max(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55cd766524dfd6c8d0aa2c9f"></a>
## max_size

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::max_size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn max_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4257448378f07863c2b4e53e"></a>
## merge_digests

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::merge_digests` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_digests<'a>(digests: impl IntoIterator<Item = &'a TDigest>) -> TDigest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [699, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:351`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-321e84a0f2d40af1f4ece115"></a>
## merge_sorted_f64

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::merge_sorted_f64` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_sorted_f64(&self, sorted_values: &[f64]) -> TDigest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [699, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3982cbd7e0c243e5af2c3a49"></a>
## merge_unsorted_f64

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::merge_unsorted_f64` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_unsorted_f64(&self, unsorted_values: Vec<f64>) -> TDigest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [699, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a21e2745a12f9f99d45da0e9"></a>
## min

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::min` · datafusion-functions-aggregate-common 55.1.0

```rust
fn min(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dbd401e379942de0a81a464"></a>
## new

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(max_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b089207dc018068500a71376"></a>
## new_with_centroid

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::new_with_centroid` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new_with_centroid(max_size: usize, centroid: Centroid) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d99950eeacc3c0cf1c579cd"></a>
## size

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Size in bytes including `Self`.

<a id="op-d0097daf09bcf0a8c3338ccc"></a>
## sum

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::sum` · datafusion-functions-aggregate-common 55.1.0

```rust
fn sum(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [172, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The sum of all values ingested into this digest.

<a id="op-eb540ff260eaa5f0b70907c4"></a>
## to_scalar_state

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::to_scalar_state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn to_scalar_state(&self) -> Vec<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [699, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:563`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

This method decomposes the [`TDigest`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-df5dff1c945937fa245a5a76) and its [`Centroid`](../operations/datafusion_functions_aggregate_common.tdigest.Centroid.md#op-a41984a513b5d982d464b95b) instances
into a series of primitive scalar values.

First the values of the TDigest are packed, followed by the variable
number of centroids packed into a [`ScalarValue::List`](../operations/datafusion_common.scalar.ScalarValue.md#op-e02ff167c86d13c10b64e274) of
[`ScalarValue::Float64`](../operations/datafusion_common.scalar.ScalarValue.md#op-800b5418e91d3bb2bc873b8e):

```text

   ┌────────┬────────┬────────┬───────┬────────┬────────┐
   │max_size│  sum   │ count  │  max  │  min   │centroid│
   └────────┴────────┴────────┴───────┴────────┴────────┘
                                                    │
                              ┌─────────────────────┘
                              ▼
                         ┌ List ───┐
                         │┌ ─ ─ ─ ┐│
                         │  mean   │
                         │├ ─ ─ ─ ┼│─ ─ Centroid 1
                         │ weight  │
                         │└ ─ ─ ─ ┘│
                         │         │
                         │┌ ─ ─ ─ ┐│
                         │  mean   │
                         │├ ─ ─ ─ ┼│─ ─ Centroid 2
                         │ weight  │
                         │└ ─ ─ ─ ┘│
                         │         │
                             ...
```

The [`TDigest::from_scalar_state()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-46497474b38817dc27a1bc8e) method reverses this processes,
consuming the output of this method and returning an unpacked
[`TDigest`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-df5dff1c945937fa245a5a76).

<a id="op-45ab21b41dacf7f5dd9d78df"></a>
## try_from_parts

`function` · `datafusion_functions_aggregate_common::tdigest::TDigest::try_from_parts` · datafusion-functions-aggregate-common 55.1.0

```rust
fn try_from_parts(max_size: usize, sum: f64, count: f64, max: f64, min: f64, centroids: Vec<Centroid>) -> Result<Self, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::TDigest", "path": "TDigest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [699, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Construct a [`TDigest`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-df5dff1c945937fa245a5a76) directly from its constituent parts, validating
the inputs.

Together with the [`Self::centroids()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-883900af3ca9afbd35640c94), [`Self::sum()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-d0097daf09bcf0a8c3338ccc),
[`Self::max_size()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-55cd766524dfd6c8d0aa2c9f), [`Self::count()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-95b6ba8bbce9cb1e436d830f), [`Self::max()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-532809935e68e016ec334dc0), and
[`Self::min()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-a21e2745a12f9f99d45da0e9) accessors, this allows a digest to be serialized into and
restored from a caller's own format without round-tripping through a
[`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) list (the non-Arrow counterpart to
[`Self::from_scalar_state()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-46497474b38817dc27a1bc8e)).

Unlike [`Self::from_scalar_state()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-46497474b38817dc27a1bc8e), this validates its inputs, returning
an error rather than a silently wrong digest when handed corrupt state.
Callers who trust their data can `unwrap()`.

# Errors

Returns an error if:
- `min` and `max` are both finite but `max < min`;
- the `centroids` are not sorted in non-decreasing order by mean (the
  order produced by [`Self::centroids()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-883900af3ca9afbd35640c94)); or
- any centroid weight is not finite and strictly positive
  ([`Self::estimate_quantile()`](../operations/datafusion_functions_aggregate_common.tdigest.TDigest.md#op-efccfaf9f57cb28200f9f007) divides by a centroid's weight, so a
  zero, negative, or non-finite weight yields silently wrong results).
