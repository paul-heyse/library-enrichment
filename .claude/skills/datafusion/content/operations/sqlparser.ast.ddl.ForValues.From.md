# `sqlparser::ast::ddl::ForValues::From`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ForValues.From.json).

<a id="op-f53043a2a06772212c47231a"></a>
## from

`struct_field` · `sqlparser::ast::ddl::ForValues::From::from` · sqlparser 0.62.0

```rust
from: Vec<PartitionBoundValue>
```

Source: `src/ast/ddl.rs:3400`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The lower bound values for the partition.

<a id="op-63771f3ff128f09dead00700"></a>
## to

`struct_field` · `sqlparser::ast::ddl::ForValues::From::to` · sqlparser 0.62.0

```rust
to: Vec<PartitionBoundValue>
```

Source: `src/ast/ddl.rs:3402`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The upper bound values for the partition.
