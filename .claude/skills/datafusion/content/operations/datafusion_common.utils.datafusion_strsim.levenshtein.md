# `datafusion_common::utils::datafusion_strsim::levenshtein`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.datafusion_strsim.levenshtein.json).

<a id="op-2ff0e0ebe17288c24d8cedff"></a>
## levenshtein

`function` · `datafusion_common::utils::datafusion_strsim::levenshtein` · datafusion-common 55.1.0

```rust
fn levenshtein(a: &str, b: &str) -> usize
```

Source: `src/utils/mod.rs:987`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the minimum number of insertions, deletions, and substitutions
required to change one string into the other.

```
use datafusion_common::utils::datafusion_strsim::levenshtein;

assert_eq!(3, levenshtein("kitten", "sitting"));
```
