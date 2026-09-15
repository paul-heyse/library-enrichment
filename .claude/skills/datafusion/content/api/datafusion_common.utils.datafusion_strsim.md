# `datafusion_common::utils::datafusion_strsim`

Crate `datafusion-common` · 3 public items · structured records in [`model/datafusion_common.utils.datafusion_strsim.json`](../model/datafusion_common.utils.datafusion_strsim.json)

## levenshtein

`function` · `datafusion_common::utils::datafusion_strsim::levenshtein`

```rust
fn levenshtein(a: &str, b: &str) -> usize
```

Calculates the minimum number of insertions, deletions, and substitutions
required to change one string into the other.

```
use datafusion_common::utils::datafusion_strsim::levenshtein;

assert_eq!(3, levenshtein("kitten", "sitting"));
```

---

## levenshtein_with_buffer

`function` · `datafusion_common::utils::datafusion_strsim::levenshtein_with_buffer`

```rust
fn levenshtein_with_buffer(a: &str, b: &str, cache: &mut Vec<usize>) -> usize
```

Calculates the Levenshtein distance using a reusable cache buffer.
This avoids allocating a new Vec for each call, improving performance
when computing many distances.

The `cache` buffer will be resized as needed and reused across calls.

---

## normalized_levenshtein

`function` · `datafusion_common::utils::datafusion_strsim::normalized_levenshtein`

```rust
fn normalized_levenshtein(a: &str, b: &str) -> f64
```

Calculates the normalized Levenshtein distance between two strings.
The normalized distance is a value between 0.0 and 1.0, where 1.0 indicates
that the strings are identical and 0.0 indicates no similarity.

```
use datafusion_common::utils::datafusion_strsim::normalized_levenshtein;

assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);

assert!(normalized_levenshtein("", "second").abs() < 0.00001);

assert!((normalized_levenshtein("kitten", "sitten") - 0.833).abs() < 0.001);
```

---
