# `sqlparser::keywords`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.keywords.json).

<a id="op-14ca4d6700c70d02c395c279"></a>
## keywords

`module` · `sqlparser::keywords` · sqlparser 0.62.0

```rust
mod keywords
```

Source: `src/keywords.rs:18`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This module defines:
1) a list of constants for every keyword
2) an `ALL_KEYWORDS` array with every keyword in it
   This is not a list of *reserved* keywords: some of these can be
   parsed as identifiers if the parser decides so. This means that
   new keywords can be added here without affecting the parse result.

   As a matter of fact, most of these keywords are not used at all
   and could be removed.
3) a `RESERVED_FOR_TABLE_ALIAS` array with keywords reserved in a
   "table alias" context.
