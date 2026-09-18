# `datafusion_functions_aggregate_common::utils::DecimalAverager`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.utils.DecimalAverager.json).

<a id="op-158eefd28c1fbfa46a5c3340"></a>
## DecimalAverager

`struct` · `datafusion_functions_aggregate_common::utils::DecimalAverager` · datafusion-functions-aggregate-common 55.1.0

```rust
struct DecimalAverager<T: DecimalType>
```

Source: `src/utils.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Computes averages for `Decimal128`/`Decimal256` values, checking for overflow

This is needed because different precisions for Decimal128/Decimal256 can
store different ranges of values and thus sum/count may not fit in
the target type.

For example, the precision is 3, the max of value is `999` and the min
value is `-999`

<a id="op-536dacd333389650b9e56680"></a>
## avg

`function` · `datafusion_functions_aggregate_common::utils::DecimalAverager::avg` · datafusion-functions-aggregate-common 55.1.0

```rust
fn avg(&self, sum: T::Native, count: T::Native) -> Result<T::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::DecimalAverager", "path": "DecimalAverager"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [177, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Returns the `sum`/`count` as a i128/i256 Decimal128/Decimal256 with
target_scale and target_precision and reporting overflow.

* sum: The total sum value stored as Decimal128 with sum_scale
  (passed to `Self::try_new`)
* count: total count, stored as a i128/i256 (*NOT* a Decimal128/Decimal256 value)

<a id="op-77b6f2dae9527d1963b1ad95"></a>
## try_new

`function` · `datafusion_functions_aggregate_common::utils::DecimalAverager::try_new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn try_new(sum_scale: i8, target_precision: u8, target_scale: i8) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::DecimalAverager", "path": "DecimalAverager"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [177, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Create a new `DecimalAverager`:

* sum_scale: the scale of `sum` values passed to [`Self::avg`](../operations/datafusion_functions_aggregate_common.utils.DecimalAverager.md#op-536dacd333389650b9e56680)
* target_precision: the output precision
* target_scale: the output scale

Errors if the resulting data can not be stored
