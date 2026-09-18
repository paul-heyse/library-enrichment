# `datafusion_common::rounding::alter_fp_rounding_mode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.rounding.alter_fp_rounding_mode.json).

<a id="op-dad79c96786d211706369ad1"></a>
## alter_fp_rounding_mode

`function` · `datafusion_common::rounding::alter_fp_rounding_mode` · datafusion-common 55.1.0

```rust
fn alter_fp_rounding_mode<const UPPER: bool, F>(lhs: &ScalarValue, rhs: &ScalarValue, operation: F) -> Result<ScalarValue> where F: FnOnce(&ScalarValue, &ScalarValue) -> Result<ScalarValue>
```

Source: `src/rounding.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
