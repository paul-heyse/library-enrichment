# `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.order.AggregateOrderSensitivity.json).

<a id="op-8b6a26410e0f5b4d59a68b96"></a>
## AggregateOrderSensitivity

`enum` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity` · datafusion-functions-aggregate-common 55.1.0

```rust
enum AggregateOrderSensitivity
```

Source: `src/order.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Represents the sensitivity of an aggregate expression to ordering.

<a id="op-822350206b0073e10a7979ae"></a>
## Beneficial

`variant` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::Beneficial` · datafusion-functions-aggregate-common 55.1.0

```rust
Beneficial
```

Source: `src/order.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Indicates that ordering is beneficial for the aggregate expression in terms
of evaluation efficiency. The aggregator can produce its result efficiently
when its required ordering is satisfied; however, it can still produce the
correct result (albeit less efficiently) when its required ordering is not met.

<a id="op-f0208ea2c00e38314e7b8c3f"></a>
## HardRequirement

`variant` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::HardRequirement` · datafusion-functions-aggregate-common 55.1.0

```rust
HardRequirement
```

Source: `src/order.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Indicates that the aggregate expression has a hard requirement on ordering.
The aggregator cannot produce a correct result unless its ordering
requirement is satisfied.

<a id="op-781ad4a8bc18f697896f005a"></a>
## Insensitive

`variant` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::Insensitive` · datafusion-functions-aggregate-common 55.1.0

```rust
Insensitive
```

Source: `src/order.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Indicates that the aggregate expression is insensitive to ordering.
Ordering at the input is not important for the result of the aggregator.

<a id="op-a0c714d7de4775dd8e95f08d"></a>
## SoftRequirement

`variant` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::SoftRequirement` · datafusion-functions-aggregate-common 55.1.0

```rust
SoftRequirement
```

Source: `src/order.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Indicates that the aggregator is more efficient when the input is ordered
but can still produce its result correctly regardless of the input ordering.
This is similar to, but stronger than, [`Self::Beneficial`](../operations/datafusion_functions_aggregate_common.order.AggregateOrderSensitivity.md#op-822350206b0073e10a7979ae).

Similarly to [`Self::HardRequirement`](../operations/datafusion_functions_aggregate_common.order.AggregateOrderSensitivity.md#op-f0208ea2c00e38314e7b8c3f), when possible DataFusion will insert
a `SortExec`, to reorder the input to match the SoftRequirement. However,
when such a `SortExec` cannot be inserted, (for example, due to conflicting
[`Self::HardRequirement`](../operations/datafusion_functions_aggregate_common.order.AggregateOrderSensitivity.md#op-f0208ea2c00e38314e7b8c3f) with other ordered aggregates in the query),
the aggregate function will still execute, without the preferred order, unlike
with [`Self::HardRequirement`](../operations/datafusion_functions_aggregate_common.order.AggregateOrderSensitivity.md#op-f0208ea2c00e38314e7b8c3f)

<a id="op-12a8a42acaae4d0399535702"></a>
## clone

`function` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::clone` · datafusion-functions-aggregate-common 55.1.0

```rust
fn clone(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::order::AggregateOrderSensitivity", "path": "AggregateOrderSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 32], "end": [19, 37], "filename": "src/order.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/order.rs:19`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3c0e7610de9d406475e5fe3"></a>
## eq

`function` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::eq` · datafusion-functions-aggregate-common 55.1.0

```rust
fn eq(&self, other: &AggregateOrderSensitivity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::order::AggregateOrderSensitivity", "path": "AggregateOrderSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 17], "end": [19, 26], "filename": "src/order.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/order.rs:19`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9245c13f8032b49b2215295"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::order::AggregateOrderSensitivity", "path": "AggregateOrderSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 10], "end": [19, 15], "filename": "src/order.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/order.rs:19`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83afb2b9fdce7549abfa4845"></a>
## hard_requires

`function` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::hard_requires` · datafusion-functions-aggregate-common 55.1.0

```rust
fn hard_requires(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::order::AggregateOrderSensitivity", "path": "AggregateOrderSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [58, 2], "filename": "src/order.rs"}, "trait": null, "trait_path": null}`

Source: `src/order.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90908ec7bbb134fd272a0df2"></a>
## is_beneficial

`function` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::is_beneficial` · datafusion-functions-aggregate-common 55.1.0

```rust
fn is_beneficial(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::order::AggregateOrderSensitivity", "path": "AggregateOrderSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [58, 2], "filename": "src/order.rs"}, "trait": null, "trait_path": null}`

Source: `src/order.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-094b6daffc5e83ee4b99bfa5"></a>
## is_insensitive

`function` · `datafusion_functions_aggregate_common::order::AggregateOrderSensitivity::is_insensitive` · datafusion-functions-aggregate-common 55.1.0

```rust
fn is_insensitive(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::order::AggregateOrderSensitivity", "path": "AggregateOrderSensitivity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [58, 2], "filename": "src/order.rs"}, "trait": null, "trait_path": null}`

Source: `src/order.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
