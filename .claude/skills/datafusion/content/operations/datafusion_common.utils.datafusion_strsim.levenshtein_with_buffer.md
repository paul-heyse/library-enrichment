# `datafusion_common::utils::datafusion_strsim::levenshtein_with_buffer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.datafusion_strsim.levenshtein_with_buffer.json).

<a id="op-a3423a78ed944bd025ec5568"></a>
## levenshtein_with_buffer

`function` · `datafusion_common::utils::datafusion_strsim::levenshtein_with_buffer` · datafusion-common 55.1.0

```rust
fn levenshtein_with_buffer(a: &str, b: &str, cache: &mut Vec<usize>) -> usize
```

Source: `src/utils/mod.rs:996`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the Levenshtein distance using a reusable cache buffer.
This avoids allocating a new Vec for each call, improving performance
when computing many distances.

The `cache` buffer will be resized as needed and reused across calls.
