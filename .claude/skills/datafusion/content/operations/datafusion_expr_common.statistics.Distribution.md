# `datafusion_expr_common::statistics::Distribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.Distribution.json).

<a id="op-01128278dc758957f40620ba"></a>
## Distribution

`enum` · `datafusion_expr_common::statistics::Distribution` · datafusion-expr-common 55.1.0

```rust
enum Distribution
```

Source: `src/statistics.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

This object defines probabilistic distributions that encode uncertain
information about a single, scalar value. Currently, we support five core
statistical distributions. New variants will be added over time.

This object is the lowest-level object in the statistics hierarchy, and it
is the main unit of calculus when evaluating expressions in a statistical
context. Notions like column and table statistics are built on top of this
object and the operations it supports.

<a id="op-f104de3c93a2bdf3cb167e87"></a>
## Bernoulli

`variant` · `datafusion_expr_common::statistics::Distribution::Bernoulli` · datafusion-expr-common 55.1.0

```rust
Bernoulli
```

Source: `src/statistics.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-880b47f4989129a20b4a40d3"></a>
## Exponential

`variant` · `datafusion_expr_common::statistics::Distribution::Exponential` · datafusion-expr-common 55.1.0

```rust
Exponential
```

Source: `src/statistics.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb391024bc9ca20f52803716"></a>
## Gaussian

`variant` · `datafusion_expr_common::statistics::Distribution::Gaussian` · datafusion-expr-common 55.1.0

```rust
Gaussian
```

Source: `src/statistics.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2f187e0006172f198bef833"></a>
## Generic

`variant` · `datafusion_expr_common::statistics::Distribution::Generic` · datafusion-expr-common 55.1.0

```rust
Generic
```

Source: `src/statistics.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-526d899958dbd74fbf3ef9bb"></a>
## Uniform

`variant` · `datafusion_expr_common::statistics::Distribution::Uniform` · datafusion-expr-common 55.1.0

```rust
Uniform
```

Source: `src/statistics.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e905650d6962c0e140e0f25"></a>
## clone

`function` · `datafusion_expr_common::statistics::Distribution::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> Distribution
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88878f0cb5cee01c0fcea27a"></a>
## data_type

`function` · `datafusion_expr_common::statistics::Distribution::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns the data type of the statistical parameters comprising this
distribution.

<a id="op-23374e6263c12d5826fb84cb"></a>
## eq

`function` · `datafusion_expr_common::statistics::Distribution::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &Distribution) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 24], "end": [54, 33], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/statistics.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-227980f8dd60db8a5b1fc890"></a>
## fmt

`function` · `datafusion_expr_common::statistics::Distribution::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 22], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8a68b4826c28f3860b996f9"></a>
## mean

`function` · `datafusion_expr_common::statistics::Distribution::mean` · datafusion-expr-common 55.1.0

```rust
fn mean(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Extracts the mean value of this uncertain quantity, depending on its
distribution:
- A [`Uniform`](../operations/datafusion_expr_common.statistics.Distribution.md#op-526d899958dbd74fbf3ef9bb) distribution's interval determines its mean value, which
  is the arithmetic average of the interval endpoints.
- An [`Exponential`](../operations/datafusion_expr_common.statistics.Distribution.md#op-880b47f4989129a20b4a40d3) distribution's mean is calculable by the formula
  `offset + 1 / λ`, where `λ` is the (non-negative) rate.
- A [`Gaussian`](../operations/datafusion_expr_common.statistics.Distribution.md#op-eb391024bc9ca20f52803716) distribution contains the mean explicitly.
- A [`Bernoulli`](../operations/datafusion_expr_common.statistics.Distribution.md#op-f104de3c93a2bdf3cb167e87) distribution's mean is equal to its success probability `p`.
- A [`Generic`](../operations/datafusion_expr_common.statistics.Distribution.md#op-a2f187e0006172f198bef833) distribution _may_ have it explicitly, or this information
  may be absent.

<a id="op-0ab16b36586e4c2590e70991"></a>
## median

`function` · `datafusion_expr_common::statistics::Distribution::median` · datafusion-expr-common 55.1.0

```rust
fn median(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Extracts the median value of this uncertain quantity, depending on its
distribution:
- A [`Uniform`](../operations/datafusion_expr_common.statistics.Distribution.md#op-526d899958dbd74fbf3ef9bb) distribution's interval determines its median value, which
  is the arithmetic average of the interval endpoints.
- An [`Exponential`](../operations/datafusion_expr_common.statistics.Distribution.md#op-880b47f4989129a20b4a40d3) distribution's median is calculable by the formula
  `offset + ln(2) / λ`, where `λ` is the (non-negative) rate.
- A [`Gaussian`](../operations/datafusion_expr_common.statistics.Distribution.md#op-eb391024bc9ca20f52803716) distribution's median is equal to its mean, which is
  specified explicitly.
- A [`Bernoulli`](../operations/datafusion_expr_common.statistics.Distribution.md#op-f104de3c93a2bdf3cb167e87) distribution's median is `1` if `p > 0.5` and `0`
  otherwise, where `p` is the success probability.
- A [`Generic`](../operations/datafusion_expr_common.statistics.Distribution.md#op-a2f187e0006172f198bef833) distribution _may_ have it explicitly, or this information
  may be absent.

<a id="op-ac2bb082c1d90c455fd2b70d"></a>
## new_bernoulli

`function` · `datafusion_expr_common::statistics::Distribution::new_bernoulli` · datafusion-expr-common 55.1.0

```rust
fn new_bernoulli(p: ScalarValue) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Constructs a new [`Bernoulli`](../operations/datafusion_expr_common.statistics.Distribution.md#op-f104de3c93a2bdf3cb167e87) distribution from the given success
probability, and validates the given parameters.

<a id="op-c93ad776f72e98134da9e640"></a>
## new_exponential

`function` · `datafusion_expr_common::statistics::Distribution::new_exponential` · datafusion-expr-common 55.1.0

```rust
fn new_exponential(rate: ScalarValue, offset: ScalarValue, positive_tail: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Constructs a new [`Exponential`](../operations/datafusion_expr_common.statistics.Distribution.md#op-880b47f4989129a20b4a40d3) distribution from the given rate/offset
pair, and validates the given parameters.

<a id="op-6bccc43eda4a4996537093bc"></a>
## new_from_interval

`function` · `datafusion_expr_common::statistics::Distribution::new_from_interval` · datafusion-expr-common 55.1.0

```rust
fn new_from_interval(range: Interval) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Constructs a new [`Generic`](../operations/datafusion_expr_common.statistics.Distribution.md#op-a2f187e0006172f198bef833) distribution from the given range. Other
parameters (mean, median and variance) are initialized with null values.

<a id="op-66816908b92317d922a5e86c"></a>
## new_gaussian

`function` · `datafusion_expr_common::statistics::Distribution::new_gaussian` · datafusion-expr-common 55.1.0

```rust
fn new_gaussian(mean: ScalarValue, variance: ScalarValue) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Constructs a new [`Gaussian`](../operations/datafusion_expr_common.statistics.Distribution.md#op-eb391024bc9ca20f52803716) distribution from the given mean/variance
pair, and validates the given parameters.

<a id="op-9657e6a69b9f3e00f099ca56"></a>
## new_generic

`function` · `datafusion_expr_common::statistics::Distribution::new_generic` · datafusion-expr-common 55.1.0

```rust
fn new_generic(mean: ScalarValue, median: ScalarValue, variance: ScalarValue, range: Interval) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Constructs a new [`Generic`](../operations/datafusion_expr_common.statistics.Distribution.md#op-a2f187e0006172f198bef833) distribution from the given mean, median,
variance, and range values after validating the given parameters.

<a id="op-ade2a94d721a5aeffa2f8095"></a>
## new_uniform

`function` · `datafusion_expr_common::statistics::Distribution::new_uniform` · datafusion-expr-common 55.1.0

```rust
fn new_uniform(interval: Interval) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Constructs a new [`Uniform`](../operations/datafusion_expr_common.statistics.Distribution.md#op-526d899958dbd74fbf3ef9bb) distribution from the given [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e).

<a id="op-15b8160552ab489c5aeb64cb"></a>
## range

`function` · `datafusion_expr_common::statistics::Distribution::range` · datafusion-expr-common 55.1.0

```rust
fn range(&self) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Extracts the range of this uncertain quantity, depending on its
distribution:
- A [`Uniform`](../operations/datafusion_expr_common.statistics.Distribution.md#op-526d899958dbd74fbf3ef9bb) distribution's range is simply its interval.
- An [`Exponential`](../operations/datafusion_expr_common.statistics.Distribution.md#op-880b47f4989129a20b4a40d3) distribution's range is `[offset, +∞)`.
- A [`Gaussian`](../operations/datafusion_expr_common.statistics.Distribution.md#op-eb391024bc9ca20f52803716) distribution's range is unbounded.
- A [`Bernoulli`](../operations/datafusion_expr_common.statistics.Distribution.md#op-f104de3c93a2bdf3cb167e87) distribution's range is [`Interval::TRUE_OR_FALSE`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-b0c00003ee9ac78d4d8520da), if
  `p` is neither `0` nor `1`. Otherwise, it is [`Interval::FALSE`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-2e4d85cb662669f0171197fb)
  and [`Interval::TRUE`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-9b4481cb81123b0b6e1276b8), respectively.
- A [`Generic`](../operations/datafusion_expr_common.statistics.Distribution.md#op-a2f187e0006172f198bef833) distribution is unbounded by default, but more information
  may be present.

<a id="op-39f0fab4b554fed011b928ea"></a>
## target_type

`function` · `datafusion_expr_common::statistics::Distribution::target_type` · datafusion-expr-common 55.1.0

```rust
fn target_type(args: &[&ScalarValue]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fc7c25e6e9f1e8e60397f82"></a>
## variance

`function` · `datafusion_expr_common::statistics::Distribution::variance` · datafusion-expr-common 55.1.0

```rust
fn variance(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::statistics::Distribution", "path": "Distribution"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [223, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Extracts the variance value of this uncertain quantity, depending on
its distribution:
- A [`Uniform`](../operations/datafusion_expr_common.statistics.Distribution.md#op-526d899958dbd74fbf3ef9bb) distribution's interval determines its variance value, which
  is calculable by the formula `(upper - lower) ^ 2 / 12`.
- An [`Exponential`](../operations/datafusion_expr_common.statistics.Distribution.md#op-880b47f4989129a20b4a40d3) distribution's variance is calculable by the formula
  `1 / (λ ^ 2)`, where `λ` is the (non-negative) rate.
- A [`Gaussian`](../operations/datafusion_expr_common.statistics.Distribution.md#op-eb391024bc9ca20f52803716) distribution's variance is specified explicitly.
- A [`Bernoulli`](../operations/datafusion_expr_common.statistics.Distribution.md#op-f104de3c93a2bdf3cb167e87) distribution's median is given by the formula `p * (1 - p)`
  where `p` is the success probability.
- A [`Generic`](../operations/datafusion_expr_common.statistics.Distribution.md#op-a2f187e0006172f198bef833) distribution _may_ have it explicitly, or this information
  may be absent.
