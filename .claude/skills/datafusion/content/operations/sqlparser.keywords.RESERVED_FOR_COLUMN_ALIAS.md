# `sqlparser::keywords::RESERVED_FOR_COLUMN_ALIAS`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.keywords.RESERVED_FOR_COLUMN_ALIAS.json).

<a id="op-f11a6f0e7e4d4424a601974b"></a>
## RESERVED_FOR_COLUMN_ALIAS

`constant` · `sqlparser::keywords::RESERVED_FOR_COLUMN_ALIAS` · sqlparser 0.62.0

```rust
const RESERVED_FOR_COLUMN_ALIAS: &[Keyword] = _
```

Source: `src/keywords.rs:1259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Can't be used as a column alias, so that `SELECT <expr> alias`
can be parsed unambiguously without looking ahead.
