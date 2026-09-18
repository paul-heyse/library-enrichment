# `sqlparser::ast::CopyTarget`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CopyTarget.json).

<a id="op-a2228bd3ef68192d9316924f"></a>
## CopyTarget

`enum` · `sqlparser::ast::CopyTarget` · sqlparser 0.62.0

```rust
enum CopyTarget
```

Source: `src/ast/mod.rs:9271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target for the `COPY` command: STDIN, STDOUT, a file, or a program.

<a id="op-79ae03999608d554e353f86b"></a>
## File

`variant` · `sqlparser::ast::CopyTarget::File` · sqlparser 0.62.0

```rust
File
```

Source: `src/ast/mod.rs:9277`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Read from or write to a file.

<a id="op-5bdeb881c6eca188bd13089f"></a>
## Program

`variant` · `sqlparser::ast::CopyTarget::Program` · sqlparser 0.62.0

```rust
Program
```

Source: `src/ast/mod.rs:9282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use a program as the source or target (shell command).

<a id="op-c01fd2649b3bc7fab7bd51f8"></a>
## Stdin

`variant` · `sqlparser::ast::CopyTarget::Stdin` · sqlparser 0.62.0

```rust
Stdin
```

Source: `src/ast/mod.rs:9273`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use standard input as the source.

<a id="op-d8050c2c90ca69dced0bf03b"></a>
## Stdout

`variant` · `sqlparser::ast::CopyTarget::Stdout` · sqlparser 0.62.0

```rust
Stdout
```

Source: `src/ast/mod.rs:9275`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use standard output as the target.

<a id="op-28ab983a464e472d009831ed"></a>
## clone

`function` · `sqlparser::ast::CopyTarget::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CopyTarget
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9267, 17], "end": [9267, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4ac8b20f6eef7126dfbffde"></a>
## cmp

`function` · `sqlparser::ast::CopyTarget::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CopyTarget) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9267, 51], "end": [9267, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33cc818b0091760559191f5d"></a>
## deserialize

`function` · `sqlparser::ast::CopyTarget::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9268, 49], "end": [9268, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d58ebecdada370b2f3b9098"></a>
## eq

`function` · `sqlparser::ast::CopyTarget::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CopyTarget) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9267, 24], "end": [9267, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15023f9adf429c3dde226065"></a>
## fmt

`function` · `sqlparser::ast::CopyTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9288, 1], "end": [9302, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9289`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56a65dea8d1c05c824828297"></a>
## fmt

`function` · `sqlparser::ast::CopyTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9267, 10], "end": [9267, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cd06058c9a019271dd24232"></a>
## hash

`function` · `sqlparser::ast::CopyTarget::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9267, 56], "end": [9267, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b32e3ba361f774de2dc8e384"></a>
## partial_cmp

`function` · `sqlparser::ast::CopyTarget::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CopyTarget) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9267, 35], "end": [9267, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3501e9e773f3b9fb06928ff6"></a>
## serialize

`function` · `sqlparser::ast::CopyTarget::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9268, 38], "end": [9268, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d6d8d7c3c13693bce8d0430"></a>
## visit

`function` · `sqlparser::ast::CopyTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9269, 40], "end": [9269, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9269`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1355fba36e4b2f107e72a8b"></a>
## visit

`function` · `sqlparser::ast::CopyTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyTarget", "path": "CopyTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9269, 47], "end": [9269, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9269`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
