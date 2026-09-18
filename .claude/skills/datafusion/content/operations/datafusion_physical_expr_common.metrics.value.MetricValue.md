# `datafusion_physical_expr_common::metrics::value::MetricValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.MetricValue.json).

<a id="op-0ea6822ed6077a467ded8740"></a>
## MetricValue

`enum` · `datafusion_physical_expr_common::metrics::value::MetricValue` · datafusion-physical-expr-common 55.1.0

```rust
enum MetricValue
```

Source: `src/metrics/value.rs:626`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Possible values for a [super::Metric](../operations/datafusion_physical_expr_common.metrics.Metric.md#op-f3d3e8659b0526690e35cebe).

Among other differences, the metric types have different ways to
logically interpret their underlying values and some metrics are
so common they are given special treatment.

<a id="op-1c47be737ae689f8fe0cdafd"></a>
## Count

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::Count` · datafusion-physical-expr-common 55.1.0

```rust
Count
```

Source: `src/metrics/value.rs:662`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Operator defined count.

<a id="op-b12e3125e4753202882de271"></a>
## CurrentMemoryUsage

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::CurrentMemoryUsage` · datafusion-physical-expr-common 55.1.0

```rust
CurrentMemoryUsage
```

Source: `src/metrics/value.rs:660`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Current memory used

<a id="op-18b4cb9ab0036bddac7ddeeb"></a>
## Custom

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::Custom` · datafusion-physical-expr-common 55.1.0

```rust
Custom
```

Source: `src/metrics/value.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-666fffec2a5d01c00a74f760"></a>
## ElapsedCompute

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::ElapsedCompute` · datafusion-physical-expr-common 55.1.0

```rust
ElapsedCompute
```

Source: `src/metrics/value.rs:648`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Elapsed Compute Time: the wall clock time spent in "cpu
intensive" work.

This measurement represents, roughly:
```
use std::time::Instant;
let start = Instant::now();
// ...CPU intensive work here...
let elapsed_compute = (Instant::now() - start).as_nanos();
```

Note 1: Does *not* include time other operators spend
computing input.

Note 2: *Does* includes time when the thread could have made
progress but the OS did not schedule it (e.g. due to CPU
contention), thus making this value different than the
classical definition of "cpu_time", which is the time reported
from `clock_gettime(CLOCK_THREAD_CPUTIME_ID, ..)`.

<a id="op-3c679f8b80cda81a36405228"></a>
## EndTimestamp

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::EndTimestamp` · datafusion-physical-expr-common 55.1.0

```rust
EndTimestamp
```

Source: `src/metrics/value.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The time at which execution ended

<a id="op-218967907010fe425f6b2352"></a>
## Gauge

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::Gauge` · datafusion-physical-expr-common 55.1.0

```rust
Gauge
```

Source: `src/metrics/value.rs:669`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Operator defined gauge.

<a id="op-d3d9a45097119169e7c2f827"></a>
## OutputBatches

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::OutputBatches` · datafusion-physical-expr-common 55.1.0

```rust
OutputBatches
```

Source: `src/metrics/value.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Total number of output batches produced: "output_batches" metric

<a id="op-b844d540eb539877f094d9c3"></a>
## OutputBytes

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::OutputBytes` · datafusion-physical-expr-common 55.1.0

```rust
OutputBytes
```

Source: `src/metrics/value.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Total size of output bytes produced: "output_bytes" metric

<a id="op-51c1616e897ed43761775ef8"></a>
## OutputRows

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::OutputRows` · datafusion-physical-expr-common 55.1.0

```rust
OutputRows
```

Source: `src/metrics/value.rs:628`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Number of output rows produced: "output_rows" metric

<a id="op-4b58c958a8236536f1d19cf0"></a>
## PeakMemoryUsage

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::PeakMemoryUsage` · datafusion-physical-expr-common 55.1.0

```rust
PeakMemoryUsage
```

Source: `src/metrics/value.rs:676`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Operator defined peak memory usage in bytes.

<a id="op-d9bc16cd86e72c4b406dbccf"></a>
## PruningMetrics

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::PruningMetrics` · datafusion-physical-expr-common 55.1.0

```rust
PruningMetrics
```

Source: `src/metrics/value.rs:694`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Metrics related to scan pruning

<a id="op-591f686f8a7dd711da9df50a"></a>
## Ratio

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::Ratio` · datafusion-physical-expr-common 55.1.0

```rust
Ratio
```

Source: `src/metrics/value.rs:699`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Metrics that should be displayed as ratio like (42%)

<a id="op-c3273bb6126fdcf1914f5f11"></a>
## SpillCount

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::SpillCount` · datafusion-physical-expr-common 55.1.0

```rust
SpillCount
```

Source: `src/metrics/value.rs:650`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Number of spills produced: "spill_count" metric

<a id="op-99579ff392afa81deba399eb"></a>
## SpilledBytes

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::SpilledBytes` · datafusion-physical-expr-common 55.1.0

```rust
SpilledBytes
```

Source: `src/metrics/value.rs:652`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Total size of spilled bytes produced: "spilled_bytes" metric

<a id="op-3b415e81703e00c2e79bdf55"></a>
## SpilledRows

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::SpilledRows` · datafusion-physical-expr-common 55.1.0

```rust
SpilledRows
```

Source: `src/metrics/value.rs:658`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Total size of spilled rows produced: "spilled_rows" metric

<a id="op-544ed367e707ddc7bca1ae3a"></a>
## StartTimestamp

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::StartTimestamp` · datafusion-physical-expr-common 55.1.0

```rust
StartTimestamp
```

Source: `src/metrics/value.rs:690`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The time at which execution started

<a id="op-7903209cd77dab757f41a1c7"></a>
## Time

`variant` · `datafusion_physical_expr_common::metrics::value::MetricValue::Time` · datafusion-physical-expr-common 55.1.0

```rust
Time
```

Source: `src/metrics/value.rs:683`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Operator defined time

<a id="op-3f43f51e982dc21ad2f3066e"></a>
## aggregate

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::aggregate` · datafusion-physical-expr-common 55.1.0

```rust
fn aggregate(&mut self, other: &Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [1074, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:938`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Aggregates the value of other to `self`. panic's if the types
are mismatched or aggregating does not make sense for this
value

Note this is purposely marked `mut` (even though atomics are
used) so Rust's type system can be used to ensure the
appropriate API access. `MetricValues` should be modified
using the original [`Count`](../operations/datafusion_physical_expr_common.metrics.value.Count.md#op-2921678004049ad87901f32b) or [`Time`](../operations/datafusion_physical_expr_common.metrics.value.Time.md#op-0552155cccfca2434f2fc732) they were created
from.

<a id="op-911f211002968d6e2b796c35"></a>
## as_usize

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::as_usize` · datafusion-physical-expr-common 55.1.0

```rust
fn as_usize(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [1074, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:841`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the value of the metric as a usize value, used to aggregate metric
value across partitions.

<a id="op-8b99e821a5a644809f460ce4"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> MetricValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [625, 17], "end": [625, 22], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/value.rs:625`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be49867a255b42178a80bdce"></a>
## display_sort_key

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::display_sort_key` · datafusion-physical-expr-common 55.1.0

```rust
fn display_sort_key(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [1074, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:1025`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a number by which to sort metrics by display. Lower
numbers are "more useful" (and displayed first)

<a id="op-35009dfdb3473d0dc8e49fe9"></a>
## eq

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [713, 1], "end": [812, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metrics/value.rs:714`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5527db6da671951ecc7a11b0"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1076, 1], "end": [1127, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/value.rs:1078`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Prints the value of this metric

<a id="op-5b19519ba7aba1bb4a7300c8"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [625, 10], "end": [625, 15], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/value.rs:625`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2439e882b9d1ecde046f4a90"></a>
## is_timestamp

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::is_timestamp` · datafusion-physical-expr-common 55.1.0

```rust
fn is_timestamp(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [1074, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:1071`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

returns true if this metric has a timestamp value

<a id="op-9f070075610b06291f0f5938"></a>
## name

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::name` · datafusion-physical-expr-common 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [1074, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:816`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the name of this SQL metric

<a id="op-e25b51822f110e08f957a1ab"></a>
## new_empty

`function` · `datafusion_physical_expr_common::metrics::value::MetricValue::new_empty` · datafusion-physical-expr-common 55.1.0

```rust
fn new_empty(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::MetricValue", "path": "MetricValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [1074, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:878`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

create a new MetricValue with the same type as `self` suitable
for accumulating
