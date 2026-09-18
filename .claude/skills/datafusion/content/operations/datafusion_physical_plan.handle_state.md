# `datafusion_physical_plan::handle_state`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.handle_state.json).

<a id="op-c474405296f80d6d6f7e3001"></a>
## handle_state

`macro` · `datafusion_physical_plan::handle_state` · datafusion-physical-plan 55.1.0

```rust
macro_rules! handle_state
```

Source: `src/joins/utils.rs:1866`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The `handle_state` macro is designed to process the result of a state-changing
operation. It operates on a `StatefulStreamResult` by matching its variants and
executing corresponding actions. This macro is used to streamline code that deals
with state transitions, reducing boilerplate and improving readability.

# Cases

- `Ok(StatefulStreamResult::Continue)`: Continues the loop, indicating the
  stream join operation should proceed to the next step.
- `Ok(StatefulStreamResult::Ready(result))`: Returns a `Poll::Ready` with the
  result, either yielding a value or indicating the stream is awaiting more
  data.
- `Err(e)`: Returns a `Poll::Ready` containing an error, signaling an issue
  during the stream join operation.

# Arguments

* `$match_case`: An expression that evaluates to a `Result<StatefulStreamResult<_>>`.
