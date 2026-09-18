# `datafusion_expr::expr::Like`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Like.json).

<a id="op-10ef141f611366b10bb76318"></a>
## Like

`struct` · `datafusion_expr::expr::Like` · datafusion-expr 55.1.0

```rust
struct Like
```

Source: `src/expr.rs:892`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

LIKE expression

<a id="op-a34f86923c95fa5fa76ffb8a"></a>
## case_insensitive

`struct_field` · `datafusion_expr::expr::Like::case_insensitive` · datafusion-expr 55.1.0

```rust
case_insensitive: bool
```

Source: `src/expr.rs:898`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether to ignore case on comparing

<a id="op-b3a4191af3cc68686a246b62"></a>
## clone

`function` · `datafusion_expr::expr::Like::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Like
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Like", "path": "Like"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 10], "end": [891, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:891`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2abc3a0ae5e12968da6502a5"></a>
## eq

`function` · `datafusion_expr::expr::Like::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Like) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Like", "path": "Like"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 17], "end": [891, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:891`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a17b053342af0da38f34b1e9"></a>
## escape_char

`struct_field` · `datafusion_expr::expr::Like::escape_char` · datafusion-expr 55.1.0

```rust
escape_char: Option<char>
```

Source: `src/expr.rs:896`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42a09876689a5e9d8dd4642a"></a>
## expr

`struct_field` · `datafusion_expr::expr::Like::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:894`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85c3b1b060593b803e55c75d"></a>
## fmt

`function` · `datafusion_expr::expr::Like::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Like", "path": "Like"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 50], "end": [891, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:891`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51abbbea12fbb15a7e6b5ad9"></a>
## hash

`function` · `datafusion_expr::expr::Like::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Like", "path": "Like"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 44], "end": [891, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:891`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0053d43e4f079ca0d1485cc1"></a>
## negated

`struct_field` · `datafusion_expr::expr::Like::negated` · datafusion-expr 55.1.0

```rust
negated: bool
```

Source: `src/expr.rs:893`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99fa28f4ab816f4c9a836260"></a>
## new

`function` · `datafusion_expr::expr::Like::new` · datafusion-expr 55.1.0

```rust
fn new(negated: bool, expr: Box<Expr>, pattern: Box<Expr>, escape_char: Option<char>, case_insensitive: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Like", "path": "Like"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [901, 1], "end": [918, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:903`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Like expression

<a id="op-1b5b24ca70c1d8cb46b03504"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Like::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Like) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Like", "path": "Like"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 32], "end": [891, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:891`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53fd9e526a26e6f899976e19"></a>
## pattern

`struct_field` · `datafusion_expr::expr::Like::pattern` · datafusion-expr 55.1.0

```rust
pattern: Box<Expr>
```

Source: `src/expr.rs:895`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
