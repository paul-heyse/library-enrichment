# `tracing_core::field::private`

Crate `tracing-core` · 1 public items · structured records in [`model/tracing_core.field.private.json`](../model/tracing_core.field.private.json)

## ValidLen

`trait` · `tracing_core::field::private::ValidLen`

```rust
trait ValidLen<'a>: Borrow<[(&'a Field, Option<&'a dyn Value + 'a>)]>
```

Restrictions on `ValueSet` lengths were removed in #2508 but this type remains for backwards compatibility.

---
