# `datafusion_common::stats::Precision`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.stats.Precision.json).

<a id="op-a1a8c2b723a3a45d5d93d37d"></a>
## Precision

`enum` · `datafusion_common::stats::Precision` · datafusion-common 55.1.0

```rust
enum Precision<T: Debug + Clone + PartialEq + Eq + PartialOrd>
```

Source: `src/stats.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents a value with a degree of certainty. `Precision` is used to
propagate information the precision of statistical values.

<a id="op-63bfbf88237f08ceac333448"></a>
## Absent

`variant` · `datafusion_common::stats::Precision::Absent` · datafusion-common 55.1.0

```rust
Absent
```

Source: `src/stats.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Nothing is known about the value. This is the default state.

Acts as an absorbing element in arithmetic -> any operation
involving `Absent` yields `Absent`. [`Precision::to_inexact`](../operations/datafusion_common.stats.Precision.md#op-412672b47f566cd1e9b16279)
on `Absent` returns `Absent`, not `Inexact` — it represents
a fundamentally different state.

Common sources include:
- Data sources without statistics
- Parquet columns missing from file metadata
- Statistics that cannot be derived for an operation (e.g.,
  `distinct_count` after a union, `total_byte_size` for joins)

<a id="op-36222f8a97e753f63952f546"></a>
## Exact

`variant` · `datafusion_common::stats::Precision::Exact` · datafusion-common 55.1.0

```rust
Exact
```

Source: `src/stats.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The exact value is known. Used for guaranteeing correctness.

Comes from definitive sources such as:
- Parquet file metadata (row counts, byte sizes)
- In-memory RecordBatch data (actual row counts, byte sizes, null counts)
- and more...

<a id="op-5d87555abc98461943c3ed02"></a>
## Inexact

`variant` · `datafusion_common::stats::Precision::Inexact` · datafusion-common 55.1.0

```rust
Inexact
```

Source: `src/stats.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The value is not known exactly, but is likely close to this value.
Used for cost-based optimizations.

Some operations that would result in `Inexact(T)` would be:
- Applying a filter (selectivity is unknown)
- Mixing exact and inexact values in arithmetic
- and more...

<a id="op-b4334e1f1337b8e0549cfdb7"></a>
## add

`function` · `datafusion_common::stats::Precision::add` · datafusion-common 55.1.0

```rust
fn add(&self, other: &Precision<usize>) -> Precision<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [207, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the sum of two (possibly inexact) [`usize`] values,
conservatively propagating exactness information. If one of the input
values is [`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448), the result is `Absent` too.

Unresolved upstream links (retained, not inferred): ``usize``.

<a id="op-de17797ac173f9108ba9555b"></a>
## add

`function` · `datafusion_common::stats::Precision::add` · datafusion-common 55.1.0

```rust
fn add(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "crate::ScalarValue"}}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [323, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the sum of two (possibly inexact) [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) values,
conservatively propagating exactness information. If one of the input
values is [`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448), the result is `Absent` too.

Uses [`ScalarValue::add_checked`](../operations/datafusion_common.scalar.ScalarValue.md#op-9a563df51ff426220d0d2592) so that integer overflow returns
an error (mapped to `Absent`) instead of silently wrapping.

For performance-sensitive paths prefer `precision_add` which
avoids the Arrow array round-trip.

<a id="op-0e066186dd7732034421f898"></a>
## add_for_sum

`function` · `datafusion_common::stats::Precision::add_for_sum` · datafusion-common 55.1.0

```rust
fn add_for_sum(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "crate::ScalarValue"}}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [323, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

SUM-style addition with integer widening to match SQL `SUM` return
types for smaller integral inputs.

<a id="op-f4438beb0cff445f08838abb"></a>
## cast_to

`function` · `datafusion_common::stats::Precision::cast_to` · datafusion-common 55.1.0

```rust
fn cast_to(&self, data_type: &DataType) -> Result<Precision<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "crate::ScalarValue"}}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [323, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Casts the value to the given data type, propagating exactness information.

<a id="op-e08d62f219feb17e175161b4"></a>
## cast_to_sum_type

`function` · `datafusion_common::stats::Precision::cast_to_sum_type` · datafusion-common 55.1.0

```rust
fn cast_to_sum_type(&self) -> Precision<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "crate::ScalarValue"}}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [323, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Casts integer values to the wider SQL `SUM` return type.

This narrows overflow risk when `sum_value` statistics are merged:
`Int8/Int16/Int32 -> Int64` and `UInt8/UInt16/UInt32 -> UInt64`.

<a id="op-c8140bf63d87239d68d1b2b2"></a>
## clone

`function` · `datafusion_common::stats::Precision::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Precision<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/stats.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-789fbe601fedb9d090147be5"></a>
## default

`function` · `datafusion_common::stats::Precision::default` · datafusion-common 55.1.0

```rust
fn default() -> Precision<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 32], "end": [30, 39], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/stats.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0139cd3947562ef644c90445"></a>
## eq

`function` · `datafusion_common::stats::Precision::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Precision<T>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 26], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/stats.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17ef0f34d6032879510b1a63"></a>
## fmt

`function` · `datafusion_common::stats::Precision::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [349, 2], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/stats.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a97ca7e549f923a9468fd19b"></a>
## fmt

`function` · `datafusion_common::stats::Precision::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 1], "end": [339, 2], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stats.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f58d8addae831f6ca13fca6"></a>
## from

`function` · `datafusion_common::stats::Precision::from` · datafusion-common 55.1.0

```rust
fn from(option: Option<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [329, 2], "filename": "src/stats.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/stats.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cceb46d1d985aeccaab0e60e"></a>
## from

`function` · `datafusion_common::stats::Precision::from` · datafusion-common 55.1.0

```rust
fn from(value: Precision<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "crate::ScalarValue"}}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [361, 2], "filename": "src/stats.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/stats.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eb27a1d1bf2058d87bfd4c0"></a>
## get_value

`function` · `datafusion_common::stats::Precision::get_value` · datafusion-common 55.1.0

```rust
fn get_value(&self) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If we have some value (exact or inexact), it returns that value.
Otherwise, it returns `None`.

<a id="op-c4b555c101827411acaa3333"></a>
## heap_size

`function` · `datafusion_common::stats::Precision::heap_size` · datafusion-common 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "crate::stats::Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [115, 2], "filename": "src/heap_size.rs"}, "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}, "trait_path": "datafusion_common::heap_size::DFHeapSize"}`

Source: `src/heap_size.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b394a82305557db057c1a34e"></a>
## is_exact

`function` · `datafusion_common::stats::Precision::is_exact` · datafusion-common 55.1.0

```rust
fn is_exact(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns `Some(true)` if we have an exact value, `Some(false)` if we
have an inexact value, and `None` if there is no value.

<a id="op-b497987d938cc37fe450bf5a"></a>
## map

`function` · `datafusion_common::stats::Precision::map` · datafusion-common 55.1.0

```rust
fn map<U, F>(self, f: F) -> Precision<U> where F: Fn(T) -> U, U: Debug + Clone + PartialEq + Eq + PartialOrd
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Transform the value in this [`Precision`](../operations/datafusion_common.stats.Precision.md#op-a1a8c2b723a3a45d5d93d37d) object, if one exists, using
the given function. Preserves the exactness state.

<a id="op-55d77c0aae29c7e41a62f2cd"></a>
## max

`function` · `datafusion_common::stats::Precision::max` · datafusion-common 55.1.0

```rust
fn max(&self, other: &Precision<T>) -> Precision<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the maximum of two (possibly inexact) values, conservatively
propagating exactness information. If one of the input values is
[`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448), the result is `Absent` too.

<a id="op-4294b186741b104d0ad283e3"></a>
## min

`function` · `datafusion_common::stats::Precision::min` · datafusion-common 55.1.0

```rust
fn min(&self, other: &Precision<T>) -> Precision<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the minimum of two (possibly inexact) values, conservatively
propagating exactness information. If one of the input values is
[`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448), the result is `Absent` too.

<a id="op-51409011e029a31ba59521da"></a>
## multiply

`function` · `datafusion_common::stats::Precision::multiply` · datafusion-common 55.1.0

```rust
fn multiply(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "crate::ScalarValue"}}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [323, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the multiplication of two (possibly inexact) [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) values,
conservatively propagating exactness information. If one of the input
values is [`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448), the result is `Absent` too.

<a id="op-bfbef05c534f5c63c584b330"></a>
## multiply

`function` · `datafusion_common::stats::Precision::multiply` · datafusion-common 55.1.0

```rust
fn multiply(&self, other: &Precision<usize>) -> Precision<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [207, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the multiplication of two (possibly inexact) [`usize`] values,
conservatively propagating exactness information. If one of the input
values is [`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448), the result is `Absent` too.

Unresolved upstream links (retained, not inferred): ``usize``.

<a id="op-71ec97fdd06c8c39797f40e1"></a>
## sub

`function` · `datafusion_common::stats::Precision::sub` · datafusion-common 55.1.0

```rust
fn sub(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "crate::ScalarValue"}}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [323, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the difference of two (possibly inexact) [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) values,
conservatively propagating exactness information. If one of the input
values is [`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448), the result is `Absent` too.

<a id="op-bd10c959df318f006990304f"></a>
## sub

`function` · `datafusion_common::stats::Precision::sub` · datafusion-common 55.1.0

```rust
fn sub(&self, other: &Precision<usize>) -> Precision<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [207, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the difference of two (possibly inexact) [`usize`] values,
conservatively propagating exactness information. If one of the input
values is [`Precision::Absent`](../operations/datafusion_common.stats.Precision.md#op-63bfbf88237f08ceac333448), the result is `Absent` too.

Unresolved upstream links (retained, not inferred): ``usize``.

<a id="op-412672b47f566cd1e9b16279"></a>
## to_inexact

`function` · `datafusion_common::stats::Precision::to_inexact` · datafusion-common 55.1.0

```rust
fn to_inexact(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [138, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Demotes the precision state from exact to inexact (if present).

<a id="op-b404a1c17520b36a1afa9a97"></a>
## with_estimated_selectivity

`function` · `datafusion_common::stats::Precision::with_estimated_selectivity` · datafusion-common 55.1.0

```rust
fn with_estimated_selectivity(self, selectivity: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "datafusion_common::stats::Precision", "path": "Precision"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [207, 2], "filename": "src/stats.rs"}, "trait": null, "trait_path": null}`

Source: `src/stats.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the estimate of applying a filter with estimated selectivity
`selectivity` to this Precision. A selectivity of `1.0` means that all
rows are selected. A selectivity of `0.5` means half the rows are
selected. An exact zero is preserved, since filtering an empty input
cannot produce rows; any other known value is demoted to inexact.
