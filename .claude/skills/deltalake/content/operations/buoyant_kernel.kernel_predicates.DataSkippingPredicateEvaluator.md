# `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.kernel_predicates.DataSkippingPredicateEvaluator.json).

<a id="op-36668fcec46493e067d7f7e4"></a>
## DataSkippingPredicateEvaluator

`trait` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DataSkippingPredicateEvaluator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L771).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:771`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A predicate evaluator that implements data skipping semantics over various column stats. For
example, comparisons involving a column are converted into comparisons over that column's
min/max stats, and NULL checks are converted into comparisons involving the column's nullcount
and rowcount stats.

<a id="op-b4e96771ccb9514f64e2d24c"></a>
## ColumnStat

`assoc_type` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::ColumnStat` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type ColumnStat
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L775).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:775`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The type for column stats consumed by this predicate evaluator

<a id="op-c27c403032e95712243e6251"></a>
## Output

`assoc_type` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L773).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:773`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The output type produced by this predicate evaluator

<a id="op-a5db733d91eb398330cd7620"></a>
## eval_partial_cmp

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_partial_cmp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_partial_cmp(&self, ord: Ordering, col: Self::ColumnStat, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L831).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:831`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Helper method that performs a (possibly inverted) partial comparison between a typed column
stat and a scalar.

<a id="op-3fb40625cea41e8a5dbc7439"></a>
## eval_pred_binary_scalars

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_pred_binary_scalars` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_binary_scalars(&self, op: BinaryPredicateOp, left: &Scalar, right: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L805).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:805`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`KernelPredicateEvaluator::eval_pred_binary_scalars`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-1da9dd1156dc513bb7d27b4a)

<a id="op-0f9eb90ce91a72e4836adc97"></a>
## eval_pred_eq

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_pred_eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_eq(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L912).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:912`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`KernelPredicateEvaluator::eval_pred_eq`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-289effecc0f85f6dfd5fb993)

<a id="op-449a03531fb9c1f1ca3da864"></a>
## eval_pred_gt

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_pred_gt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_gt(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L890).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:890`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`KernelPredicateEvaluator::eval_pred_gt`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-68551f2464605fe81c3257e5)

<a id="op-5e0ef6418233923b7ccb53e5"></a>
## eval_pred_is_null

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_pred_is_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_is_null(&self, col: &ColumnName, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L802).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:802`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

For IS NULL (IS NOT NULL), we can only skip the file if all-null (no-null). Any other
nullcount always forces us to keep the file.

NOTE: When deletion vectors are enabled, they could produce a file that is logically
all-null or logically no-null, even tho the physical stats indicate a mix of null and
non-null values. They cannot invalidate a file's physical all-null or non-null status,
however, so the worst that can happen is we fail to skip an unnecessary file.

<a id="op-3d875207fb4368be9182456b"></a>
## eval_pred_lt

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_pred_lt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_lt(&self, col: &ColumnName, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L868).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:868`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`KernelPredicateEvaluator::eval_pred_lt`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-0651dedc7ebf7c50809f49c4)

<a id="op-99def9c5c63988388423a0e5"></a>
## eval_pred_opaque

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_pred_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_opaque(&self, op: &OpaquePredicateOpRef, exprs: &[Expr], inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L814).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:814`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`KernelPredicateEvaluator::eval_pred_opaque`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-7049b694c2404d487d9852d7).

<a id="op-0a5c3ce89fe231bd6f4d3929"></a>
## eval_pred_scalar

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_pred_scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_scalar(&self, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L790).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:790`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`KernelPredicateEvaluator::eval_pred_scalar`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-43819d3f9c8df9f0fc23a842)

<a id="op-c2b61d057802c412f66edb8b"></a>
## eval_pred_scalar_is_null

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::eval_pred_scalar_is_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eval_pred_scalar_is_null(&self, val: &Scalar, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L793).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:793`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`KernelPredicateEvaluator::eval_pred_scalar_is_null`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-279e486aa8a990294c509fe9)

<a id="op-7ad21fa31ee3cecd28b504d7"></a>
## finish_eval_pred_junction

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::finish_eval_pred_junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn finish_eval_pred_junction(&self, op: JunctionPredicateOp, preds: &mut dyn Iterator<Item = Option<Self::Output>>, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L822).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:822`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

See [`KernelPredicateEvaluator::finish_eval_pred_junction`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluator.md#op-0ee991f832c056b15d36ce28)

<a id="op-e717c38fc2c32c5ab72788d7"></a>
## get_max_stat

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::get_max_stat` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_max_stat(&self, col: &ColumnName, data_type: &DataType) -> Option<Self::ColumnStat>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L781).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:781`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Retrieves the maximum value of a column, if it exists and has the requested type.

<a id="op-ac721b895a017dadc1b8ec9b"></a>
## get_min_stat

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::get_min_stat` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_min_stat(&self, col: &ColumnName, data_type: &DataType) -> Option<Self::ColumnStat>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L778).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:778`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Retrieves the minimum value of a column, if it exists and has the requested type.

<a id="op-ccd14588391cec6778ecbb04"></a>
## get_nullcount_stat

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::get_nullcount_stat` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_nullcount_stat(&self, col: &ColumnName) -> Option<Self::ColumnStat>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L784).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:784`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Retrieves the null count of a column, if it exists.

<a id="op-b7bc5f02f2a01ea4b3796feb"></a>
## get_rowcount_stat

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::get_rowcount_stat` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_rowcount_stat(&self) -> Option<Self::ColumnStat>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L787).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:787`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Retrieves the row count of a column (parquet footers always include this stat).

<a id="op-bb1c71bbb32c9bf005587e48"></a>
## partial_cmp_max_stat

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::partial_cmp_max_stat` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partial_cmp_max_stat(&self, col: &ColumnName, val: &Scalar, ord: Ordering, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L856).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:856`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Performs a partial comparison against a column max-stat. See
[`KernelPredicateEvaluatorDefaults::partial_cmp_scalars`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluatorDefaults.md#op-35b76f0c472c3a4ca420793d) for details of the comparison
semantics.

<a id="op-b045302802bc706967c3b348"></a>
## partial_cmp_min_stat

`function` · `buoyant_kernel::kernel_predicates::DataSkippingPredicateEvaluator::partial_cmp_min_stat` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partial_cmp_min_stat(&self, col: &ColumnName, val: &Scalar, ord: Ordering, inverted: bool) -> Option<Self::Output>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/kernel_predicates/mod.rs#L842).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/kernel_predicates/mod.rs:842`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Performs a partial comparison against a column min-stat. See
[`KernelPredicateEvaluatorDefaults::partial_cmp_scalars`](../operations/buoyant_kernel.kernel_predicates.KernelPredicateEvaluatorDefaults.md#op-35b76f0c472c3a4ca420793d) for details of the comparison
semantics.
