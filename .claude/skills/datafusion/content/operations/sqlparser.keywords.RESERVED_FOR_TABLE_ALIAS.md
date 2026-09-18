# `sqlparser::keywords::RESERVED_FOR_TABLE_ALIAS`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.keywords.RESERVED_FOR_TABLE_ALIAS.json).

<a id="op-c0faeaf69f1f5235a1d47011"></a>
## RESERVED_FOR_TABLE_ALIAS

`constant` · `sqlparser::keywords::RESERVED_FOR_TABLE_ALIAS` · sqlparser 0.62.0

```rust
const RESERVED_FOR_TABLE_ALIAS: &[Keyword] = _
```

Source: `src/keywords.rs:1188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

These keywords can't be used as a table alias, so that `FROM table_name alias`
can be parsed unambiguously without looking ahead.
