# `datafusion_expr::expr::GetFieldAccess`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.GetFieldAccess.json).

<a id="op-5fe840a8e3896ea131eab832"></a>
## GetFieldAccess

`enum` · `datafusion_expr::expr::GetFieldAccess` · datafusion-expr 55.1.0

```rust
enum GetFieldAccess
```

Source: `src/expr.rs:974`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Access a sub field of a nested type, such as `Field` or `List`

<a id="op-3b1df647a9b9efd5dfa51a38"></a>
## ListIndex

`variant` · `datafusion_expr::expr::GetFieldAccess::ListIndex` · datafusion-expr 55.1.0

```rust
ListIndex
```

Source: `src/expr.rs:978`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Single list index, for example: `list[i]`

<a id="op-70933de1be5e3dc394f9274c"></a>
## ListRange

`variant` · `datafusion_expr::expr::GetFieldAccess::ListRange` · datafusion-expr 55.1.0

```rust
ListRange
```

Source: `src/expr.rs:980`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

List stride, for example `list[i:j:k]`

<a id="op-3393742e4a8e691a885d0c31"></a>
## NamedStructField

`variant` · `datafusion_expr::expr::GetFieldAccess::NamedStructField` · datafusion-expr 55.1.0

```rust
NamedStructField
```

Source: `src/expr.rs:976`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Named field, for example `struct["name"]`

<a id="op-097d3ae47be40cb0c6078851"></a>
## clone

`function` · `datafusion_expr::expr::GetFieldAccess::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> GetFieldAccess
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GetFieldAccess", "path": "GetFieldAccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [973, 10], "end": [973, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:973`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e3ca5884456b8acd84376ab"></a>
## eq

`function` · `datafusion_expr::expr::GetFieldAccess::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &GetFieldAccess) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GetFieldAccess", "path": "GetFieldAccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [973, 17], "end": [973, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:973`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f83c64c31c0870079a6e9d4a"></a>
## fmt

`function` · `datafusion_expr::expr::GetFieldAccess::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GetFieldAccess", "path": "GetFieldAccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [973, 38], "end": [973, 43], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:973`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eb3ffcdf1d5ab8d09deb7b3"></a>
## hash

`function` · `datafusion_expr::expr::GetFieldAccess::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GetFieldAccess", "path": "GetFieldAccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [973, 32], "end": [973, 36], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:973`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
