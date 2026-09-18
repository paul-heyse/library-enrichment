# `arrow_string::like`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.json).

<a id="op-23a4af356550e9a7026e4093"></a>
## like

`module` · `arrow_string::like` · arrow-string 59.3.0

```rust
mod like
```

Source: `src/like.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

String predicate kernels for Arrow arrays.

Provides SQL `LIKE`/`ILIKE` kernels as well as related
string predicates such as `contains`, `starts_with`, `ends_with`, and
ASCII case-insensitive equality.
