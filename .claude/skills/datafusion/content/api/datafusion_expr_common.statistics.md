# `datafusion_expr_common::statistics`

Crate `datafusion-expr-common` · 14 public items · structured records in [`model/datafusion_expr_common.statistics.json`](../model/datafusion_expr_common.statistics.json)

## Distribution

`enum` · `datafusion_expr_common::statistics::Distribution`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::Distribution`

```rust
enum Distribution
```

**Variants**: `Uniform`, `Exponential`, `Gaussian`, `Bernoulli`, `Generic`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn data_type(&self) -> DataType
fn mean(&self) -> Result<ScalarValue>
fn median(&self) -> Result<ScalarValue>
fn new_bernoulli(p: ScalarValue) -> Result<Self>
fn new_exponential(rate: ScalarValue, offset: ScalarValue, positive_tail: bool) -> Result<Self>
fn new_from_interval(range: Interval) -> Result<Self>
fn new_gaussian(mean: ScalarValue, variance: ScalarValue) -> Result<Self>
fn new_generic(mean: ScalarValue, median: ScalarValue, variance: ScalarValue, range: Interval) -> Result<Self>
fn new_uniform(interval: Interval) -> Result<Self>
fn range(&self) -> Result<Interval>
fn target_type(args: &[&ScalarValue]) -> Result<DataType>
fn variance(&self) -> Result<ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.Distribution.md).


This object defines probabilistic distributions that encode uncertain
information about a single, scalar value. Currently, we support five core
statistical distributions. New variants will be added over time.

This object is the lowest-level object in the statistics hierarchy, and it
is the main unit of calculus when evaluating expressions in a statistical
context. Notions like column and table statistics are built on top of this
object and the operations it supports.

---

## StatisticsRequest

`enum` · `datafusion_expr_common::statistics::StatisticsRequest`

Also reachable as `datafusion_expr::statistics::StatisticsRequest`

```rust
enum StatisticsRequest
```

**Variants**: `Min`, `Max`, `NullCount`, `DistinctCount`, `Sum`, `ByteSize`, `RowCount`, `TotalByteSize`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.StatisticsRequest.md).


A statistic a caller would like a provider to supply, if it can do so
cheaply.

A small, query-aware extension to the existing `Statistics` model: instead
of "give me everything you have for every column", a caller can ask for a
specific list of stats by name. `StatisticsRequest` is just that vocabulary
— DataFusion itself does not populate or consume it. It exists so a request
can be threaded from a `TableScan` (see `TableScan::statistics_requests`)
through `ScanArgs::statistics_requests` to a `TableProvider`, which is enough
for a query-aware statistics feature to be implemented outside of DataFusion.

Each variant maps onto a field of [`datafusion_common::Statistics`] /
[`datafusion_common::ColumnStatistics`], so a provider that already
populates one can answer the request trivially.

The per-column variants hold an `Arc<Column>` rather than an owned
[`Column`] (which carries owned strings) so cloning a request — and the
`BTreeSet<StatisticsRequest>` stored on `TableScan`, which is cloned with
the plan during optimization — stays cheap.

---

## combine_bernoullis

`function` · `datafusion_expr_common::statistics::combine_bernoullis`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::combine_bernoullis`

```rust
fn combine_bernoullis(op: &operator::Operator, left: &BernoulliDistribution, right: &BernoulliDistribution) -> datafusion_common::Result<BernoulliDistribution>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.combine_bernoullis.md).


This function takes a logical operator and two Bernoulli distributions,
and it returns a new Bernoulli distribution that represents the result of
the operation. Currently, only `AND` and `OR` operations are supported.

---

## combine_gaussians

`function` · `datafusion_expr_common::statistics::combine_gaussians`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::combine_gaussians`

```rust
fn combine_gaussians(op: &operator::Operator, left: &GaussianDistribution, right: &GaussianDistribution) -> datafusion_common::Result<Option<GaussianDistribution>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.combine_gaussians.md).


Applies the given operation to the given Gaussian distributions. Currently,
this function handles only addition and subtraction operations. If the
result is not a Gaussian random variable, it returns `None`. For details,
see:

<https://en.wikipedia.org/wiki/Sum_of_normally_distributed_random_variables>

---

## compute_mean

`function` · `datafusion_expr_common::statistics::compute_mean`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::compute_mean`

```rust
fn compute_mean(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.compute_mean.md).


Computes the mean value for the result of the given binary operation on
two unknown quantities represented by their [`Distribution`] objects.

---

## compute_median

`function` · `datafusion_expr_common::statistics::compute_median`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::compute_median`

```rust
fn compute_median(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.compute_median.md).


Computes the median value for the result of the given binary operation on
two unknown quantities represented by its [`Distribution`] objects. Currently,
the median is calculable only for addition and subtraction operations on:
- [`Uniform`] and [`Uniform`] distributions, and
- [`Gaussian`] and [`Gaussian`] distributions.

---

## compute_variance

`function` · `datafusion_expr_common::statistics::compute_variance`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::compute_variance`

```rust
fn compute_variance(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.compute_variance.md).


Computes the variance value for the result of the given binary operation on
two unknown quantities represented by their [`Distribution`] objects.

---

## create_bernoulli_from_comparison

`function` · `datafusion_expr_common::statistics::create_bernoulli_from_comparison`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::create_bernoulli_from_comparison`

```rust
fn create_bernoulli_from_comparison(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<Distribution>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.create_bernoulli_from_comparison.md).


Creates a new `Bernoulli` distribution by computing the resulting probability.
Expects `op` to be a comparison operator, with `left` and `right` having
numeric distributions. The resulting distribution has the `Float64` data
type.

---

## new_generic_from_binary_op

`function` · `datafusion_expr_common::statistics::new_generic_from_binary_op`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::new_generic_from_binary_op`

```rust
fn new_generic_from_binary_op(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<Distribution>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.new_generic_from_binary_op.md).


Creates a new [`Generic`] distribution that represents the result of the
given binary operation on two unknown quantities represented by their
[`Distribution`] objects. The function computes the mean, median and
variance if possible.

---

## BernoulliDistribution

`struct` · `datafusion_expr_common::statistics::BernoulliDistribution`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::BernoulliDistribution`

```rust
struct BernoulliDistribution
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn data_type(&self) -> DataType
fn mean(&self) -> &ScalarValue
fn median(&self) -> Result<ScalarValue>
fn p_value(&self) -> &ScalarValue
fn range(&self) -> Interval
fn variance(&self) -> Result<ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.BernoulliDistribution.md).


Bernoulli distribution with success probability `p`. If `p` has a null value,
the success probability is unknown. For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Bernoulli_distribution>

---

## ExponentialDistribution

`struct` · `datafusion_expr_common::statistics::ExponentialDistribution`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::ExponentialDistribution`

```rust
struct ExponentialDistribution
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn data_type(&self) -> DataType
fn mean(&self) -> Result<ScalarValue>
fn median(&self) -> Result<ScalarValue>
fn offset(&self) -> &ScalarValue
fn positive_tail(&self) -> bool
fn range(&self) -> Result<Interval>
fn rate(&self) -> &ScalarValue
fn variance(&self) -> Result<ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.ExponentialDistribution.md).


Exponential distribution with an optional shift. The probability density
function (PDF) is defined as follows:

For a positive tail (when `positive_tail` is `true`):

`f(x; λ, offset) = λ exp(-λ (x - offset))    for x ≥ offset`

For a negative tail (when `positive_tail` is `false`):

`f(x; λ, offset) = λ exp(-λ (offset - x))    for x ≤ offset`


In both cases, the PDF is `0` outside the specified domain.

For more information, see:

<https://en.wikipedia.org/wiki/Exponential_distribution>

---

## GaussianDistribution

`struct` · `datafusion_expr_common::statistics::GaussianDistribution`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::GaussianDistribution`

```rust
struct GaussianDistribution
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn data_type(&self) -> DataType
fn mean(&self) -> &ScalarValue
fn median(&self) -> &ScalarValue
fn range(&self) -> Result<Interval>
fn variance(&self) -> &ScalarValue
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.GaussianDistribution.md).


Gaussian (normal) distribution, represented by its mean and variance.
For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Normal_distribution>

---

## GenericDistribution

`struct` · `datafusion_expr_common::statistics::GenericDistribution`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::GenericDistribution`

```rust
struct GenericDistribution
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn data_type(&self) -> DataType
fn mean(&self) -> &ScalarValue
fn median(&self) -> &ScalarValue
fn range(&self) -> &Interval
fn variance(&self) -> &ScalarValue
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.GenericDistribution.md).


A generic distribution whose functional form is not available, which is
approximated via some summary statistics. For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Summary_statistics>

---

## UniformDistribution

`struct` · `datafusion_expr_common::statistics::UniformDistribution`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

Also reachable as `datafusion_expr::statistics::UniformDistribution`

```rust
struct UniformDistribution
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn data_type(&self) -> DataType
fn mean(&self) -> Result<ScalarValue>
fn median(&self) -> Result<ScalarValue>
fn range(&self) -> &Interval
fn variance(&self) -> Result<ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.statistics.UniformDistribution.md).


Uniform distribution, represented by its range. If the given range extends
towards infinity, the distribution will be improper -- which is OK. For a
more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Continuous_uniform_distribution>
<https://en.wikipedia.org/wiki/Prior_probability#Improper_priors>

---
