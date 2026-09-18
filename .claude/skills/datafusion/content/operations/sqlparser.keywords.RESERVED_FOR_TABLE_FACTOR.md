# `sqlparser::keywords::RESERVED_FOR_TABLE_FACTOR`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.keywords.RESERVED_FOR_TABLE_FACTOR.json).

<a id="op-cfd79c4cf4740c1f75843cb0"></a>
## RESERVED_FOR_TABLE_FACTOR

`constant` · `sqlparser::keywords::RESERVED_FOR_TABLE_FACTOR` · sqlparser 0.62.0

```rust
const RESERVED_FOR_TABLE_FACTOR: &[Keyword] = _
```

Source: `src/keywords.rs:1294`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Global list of reserved keywords allowed after FROM.
Parser should call Dialect::get_reserved_keyword_after_from
to allow for each dialect to customize the list.
