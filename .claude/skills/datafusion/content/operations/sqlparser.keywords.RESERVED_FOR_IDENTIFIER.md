# `sqlparser::keywords::RESERVED_FOR_IDENTIFIER`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.keywords.RESERVED_FOR_IDENTIFIER.json).

<a id="op-af0f93d0f82f5518d294e6dc"></a>
## RESERVED_FOR_IDENTIFIER

`constant` · `sqlparser::keywords::RESERVED_FOR_IDENTIFIER` · sqlparser 0.62.0

```rust
const RESERVED_FOR_IDENTIFIER: &[Keyword] = _
```

Source: `src/keywords.rs:1304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Global list of reserved keywords that cannot be parsed as identifiers
without special handling like quoting. Parser should call `Dialect::is_reserved_for_identifier`
to allow for each dialect to customize the list.
