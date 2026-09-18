# `sqlparser::ast::DisplaySeparated`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DisplaySeparated.json).

<a id="op-ed9d7605574ca412b7746beb"></a>
## DisplaySeparated

`struct` · `sqlparser::ast::DisplaySeparated` · sqlparser 0.62.0

```rust
struct DisplaySeparated<'a, T> where T: fmt::Display
```

Source: `src/ast/mod.rs:157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Helper used to format a slice using a separator string (e.g., `", "`).

<a id="op-73f65de4180c5cfb7a2f10e2"></a>
## fmt

`function` · `sqlparser::ast::DisplaySeparated::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::DisplaySeparated", "path": "DisplaySeparated"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Display", "path": "fmt::Display"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [165, 1], "end": [178, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:169`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
