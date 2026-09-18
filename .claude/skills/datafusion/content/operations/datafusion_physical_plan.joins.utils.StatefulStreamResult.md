# `datafusion_physical_plan::joins::utils::StatefulStreamResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.StatefulStreamResult.json).

<a id="op-b64e0b757967c34f940d7549"></a>
## StatefulStreamResult

`enum` · `datafusion_physical_plan::joins::utils::StatefulStreamResult` · datafusion-physical-plan 55.1.0

```rust
enum StatefulStreamResult<T>
```

Source: `src/joins/utils.rs:1889`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Represents the result of a stateful operation.

This enumeration indicates whether the state produced a result that is
ready for use (`Ready`) or if the operation requires continuation (`Continue`).

Variants:
- `Ready(T)`: Indicates that the operation is complete with a result of type `T`.
- `Continue`: Indicates that the operation is not yet complete and requires further
  processing or more data. When this variant is returned, it typically means that the
  current invocation of the state did not produce a final result, and the operation
  should be invoked again later with more data and possibly with a different state.

<a id="op-a4943561e2e2d21b1de978d5"></a>
## Continue

`variant` · `datafusion_physical_plan::joins::utils::StatefulStreamResult::Continue` · datafusion-physical-plan 55.1.0

```rust
Continue
```

Source: `src/joins/utils.rs:1891`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca6a7338cc7d78946706c260"></a>
## Ready

`variant` · `datafusion_physical_plan::joins::utils::StatefulStreamResult::Ready` · datafusion-physical-plan 55.1.0

```rust
Ready
```

Source: `src/joins/utils.rs:1890`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
