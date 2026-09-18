# `datafusion_physical_plan::test::exec::assert_strong_count_converges_to_zero`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.assert_strong_count_converges_to_zero.json).

<a id="op-cf35cbca5e4d1e38c0560486"></a>
## assert_strong_count_converges_to_zero

`function` · `datafusion_physical_plan::test::exec::assert_strong_count_converges_to_zero` · datafusion-physical-plan 55.1.0

```rust
async fn assert_strong_count_converges_to_zero<T>(refs: std::sync::Weak<T>)
```

Source: `src/test/exec.rs:911`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Asserts that the strong count of the given [`Weak`] pointer converges to zero.

This might take a while but has a timeout.

Unresolved upstream links (retained, not inferred): ``Weak``.
