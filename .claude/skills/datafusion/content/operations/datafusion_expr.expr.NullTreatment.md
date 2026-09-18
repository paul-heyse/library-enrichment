# `datafusion_expr::expr::NullTreatment`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.NullTreatment.json).

<a id="op-36c5b6167bc27d93c694467c"></a>
## NullTreatment

`enum` · `datafusion_expr::expr::NullTreatment` · datafusion-expr 55.1.0

```rust
enum NullTreatment
```

Source: `src/expr.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2016da51fb48895f609b16b"></a>
## IgnoreNulls

`variant` · `datafusion_expr::expr::NullTreatment::IgnoreNulls` · datafusion-expr 55.1.0

```rust
IgnoreNulls
```

Source: `src/expr.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c677f7a52252215798e94638"></a>
## RespectNulls

`variant` · `datafusion_expr::expr::NullTreatment::RespectNulls` · datafusion-expr 55.1.0

```rust
RespectNulls
```

Source: `src/expr.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6202ce3116098a1cc04d8fa9"></a>
## clone

`function` · `datafusion_expr::expr::NullTreatment::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> NullTreatment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 17], "end": [66, 22], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-713181032e2694609a5d76e1"></a>
## cmp

`function` · `datafusion_expr::expr::NullTreatment::cmp` · datafusion-expr 55.1.0

```rust
fn cmp(&self, other: &NullTreatment) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 51], "end": [66, 54], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/expr.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bea823727453c886e37f957"></a>
## eq

`function` · `datafusion_expr::expr::NullTreatment::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &NullTreatment) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 34], "end": [66, 43], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7689f52648472102f887f790"></a>
## fmt

`function` · `datafusion_expr::expr::NullTreatment::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [79, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expr.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2e35e705db95512ee81a429"></a>
## fmt

`function` · `datafusion_expr::expr::NullTreatment::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 10], "end": [66, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-350dc9b006e147facb7e1bb9"></a>
## from

`function` · `datafusion_expr::expr::NullTreatment::from` · datafusion-expr 55.1.0

```rust
fn from(t: protobuf::NullTreatment) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "crate::expr::NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [182, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::NullTreatment", "path": "NullTreatment"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/proto.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d382360ff0e017e717728ea7"></a>
## from

`function` · `datafusion_expr::expr::NullTreatment::from` · datafusion-expr 55.1.0

```rust
fn from(value: sqlparser::ast::NullTreatment) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [89, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e29ada16945a235e5187000"></a>
## hash

`function` · `datafusion_expr::expr::NullTreatment::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 45], "end": [66, 49], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d51786a85be6c84af478d61"></a>
## partial_cmp

`function` · `datafusion_expr::expr::NullTreatment::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &NullTreatment) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 56], "end": [66, 66], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
