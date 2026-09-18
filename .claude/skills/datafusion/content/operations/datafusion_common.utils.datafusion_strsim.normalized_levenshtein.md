# `datafusion_common::utils::datafusion_strsim::normalized_levenshtein`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.datafusion_strsim.normalized_levenshtein.json).

<a id="op-5e89fd9a440a9aa56a8fff39"></a>
## normalized_levenshtein

`function` · `datafusion_common::utils::datafusion_strsim::normalized_levenshtein` · datafusion-common 55.1.0

```rust
fn normalized_levenshtein(a: &str, b: &str) -> f64
```

Source: `src/utils/mod.rs:1013`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the normalized Levenshtein distance between two strings.
The normalized distance is a value between 0.0 and 1.0, where 1.0 indicates
that the strings are identical and 0.0 indicates no similarity.

```
use datafusion_common::utils::datafusion_strsim::normalized_levenshtein;

assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);

assert!(normalized_levenshtein("", "second").abs() < 0.00001);

assert!((normalized_levenshtein("kitten", "sitten") - 0.833).abs() < 0.001);
```
