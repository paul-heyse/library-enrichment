# `datafusion_physical_plan::test::assert_is_pending`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.assert_is_pending.json).

<a id="op-3d78a77433cba7bf2a76986f"></a>
## assert_is_pending

`function` · `datafusion_physical_plan::test::assert_is_pending` · datafusion-physical-plan 55.1.0

```rust
fn assert_is_pending<'a, T>(fut: &mut std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>)
```

Source: `src/test.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Asserts that given future is pending.
