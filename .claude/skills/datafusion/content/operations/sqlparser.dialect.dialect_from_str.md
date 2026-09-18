# `sqlparser::dialect::dialect_from_str`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.dialect_from_str.json).

<a id="op-add97c0ec536c79089d1ffe0"></a>
## dialect_from_str

`function` · `sqlparser::dialect::dialect_from_str` · sqlparser 0.62.0

```rust
fn dialect_from_str(dialect_name: impl AsRef<str>) -> Option<Box<dyn Dialect>>
```

Source: `src/dialect/mod.rs:1845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns the built in [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) corresponding to `dialect_name`.

See [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) documentation for an example.
