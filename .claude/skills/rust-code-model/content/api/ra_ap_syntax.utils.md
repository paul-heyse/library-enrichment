# `ra_ap_syntax::utils`

Crate `ra_ap_syntax` · 2 public items · structured records in [`model/ra_ap_syntax.utils.json`](../model/ra_ap_syntax.utils.json)

## is_identifier

`function` · `ra_ap_syntax::utils::is_identifier`

```rust
fn is_identifier(name: &str, edition: parser::Edition) -> bool
```

Checks that the name is an identifier.
This also means that it is not a strict keyword.
But it may be a weak keyword.

---

## is_raw_identifier

`function` · `ra_ap_syntax::utils::is_raw_identifier`

```rust
fn is_raw_identifier(name: &str, edition: parser::Edition) -> bool
```

---
