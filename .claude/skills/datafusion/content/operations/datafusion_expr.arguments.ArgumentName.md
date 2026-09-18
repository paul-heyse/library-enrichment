# `datafusion_expr::arguments::ArgumentName`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.arguments.ArgumentName.json).

<a id="op-d6ca12420e8a680287c2e456"></a>
## ArgumentName

`struct` · `datafusion_expr::arguments::ArgumentName` · datafusion-expr 55.1.0

```rust
struct ArgumentName
```

Source: `src/arguments.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represents a named function argument with its original case and quote information.

This struct preserves whether an identifier was quoted in the SQL, which determines
whether case-sensitive or case-insensitive matching should be used per SQL standards.

<a id="op-e962281ba1e51d5b8d62adcb"></a>
## clone

`function` · `datafusion_expr::arguments::ArgumentName::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ArgumentName
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::arguments::ArgumentName", "path": "ArgumentName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 17], "end": [27, 22], "filename": "src/arguments.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arguments.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c5c9f8742d56a1d34e54903"></a>
## eq

`function` · `datafusion_expr::arguments::ArgumentName::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &ArgumentName) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::arguments::ArgumentName", "path": "ArgumentName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 24], "end": [27, 33], "filename": "src/arguments.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arguments.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d166b6971eeae27e277d692"></a>
## fmt

`function` · `datafusion_expr::arguments::ArgumentName::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::arguments::ArgumentName", "path": "ArgumentName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "src/arguments.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arguments.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2501793a5e943a9f59d849e4"></a>
## is_quoted

`struct_field` · `datafusion_expr::arguments::ArgumentName::is_quoted` · datafusion-expr 55.1.0

```rust
is_quoted: bool
```

Source: `src/arguments.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the identifier was quoted (e.g., "STR" vs STR)
- true: quoted identifier, requires case-sensitive matching
- false: unquoted identifier, uses case-insensitive matching

<a id="op-1ee34242c577f56c03b02ea4"></a>
## value

`struct_field` · `datafusion_expr::arguments::ArgumentName::value` · datafusion-expr 55.1.0

```rust
value: String
```

Source: `src/arguments.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The argument name in its original case as it appeared in the SQL
