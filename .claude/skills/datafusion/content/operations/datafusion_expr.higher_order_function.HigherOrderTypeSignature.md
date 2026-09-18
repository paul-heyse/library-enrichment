# `datafusion_expr::higher_order_function::HigherOrderTypeSignature`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.HigherOrderTypeSignature.json).

<a id="op-fbe23e3717bc225a891eae94"></a>
## HigherOrderTypeSignature

`enum` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature` · datafusion-expr 55.1.0

```rust
enum HigherOrderTypeSignature
```

Source: `src/higher_order_function.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The types of arguments for which a function has implementations.

[`HigherOrderTypeSignature`](../operations/datafusion_expr.higher_order_function.HigherOrderTypeSignature.md#op-fbe23e3717bc225a891eae94) **DOES NOT** define the types that a user query could call the
function with. DataFusion will automatically coerce (cast) argument types to
one of the supported function signatures, if possible.

# Overview
Functions typically provide implementations for a small number of different
argument [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)s, rather than all possible combinations. If a user
calls a function with arguments that do not match any of the declared types,
DataFusion will attempt to automatically coerce (add casts to) function
arguments so they match the [`HigherOrderTypeSignature`](../operations/datafusion_expr.higher_order_function.HigherOrderTypeSignature.md#op-fbe23e3717bc225a891eae94). See the [`type_coercion`] module
for more details

[`type_coercion`]: crate::type_coercion

<a id="op-929f410e231d1d8c5520b112"></a>
## Any

`variant` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::Any` · datafusion-expr 55.1.0

```rust
Any
```

Source: `src/higher_order_function.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The specified number of lambdas or arguments with arbitrary types.

<a id="op-09aa874d6152f7bfb4e57b14"></a>
## Exact

`variant` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::Exact` · datafusion-expr 55.1.0

```rust
Exact
```

Source: `src/higher_order_function.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Exactly the specified arguments in the given order, with arbitrary types.
DataFusion will call [`HigherOrderUDFImpl::coerce_value_types`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-f20fe249c57b53787b057f56) to prepare the value
argument types.

<a id="op-d95a9ba1813a6aef4c1595be"></a>
## UserDefined

`variant` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::UserDefined` · datafusion-expr 55.1.0

```rust
UserDefined
```

Source: `src/higher_order_function.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The acceptable signature and coercions rules are special for this
function.

If this signature is specified,
DataFusion will call [`HigherOrderUDFImpl::coerce_value_types`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-f20fe249c57b53787b057f56) to prepare argument types.

<a id="op-889ba301747d52963c604935"></a>
## VariadicAny

`variant` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::VariadicAny` · datafusion-expr 55.1.0

```rust
VariadicAny
```

Source: `src/higher_order_function.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

One or more lambdas or arguments with arbitrary types

<a id="op-6d955bd007121f7904bd65f7"></a>
## clone

`function` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> HigherOrderTypeSignature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderTypeSignature", "path": "HigherOrderTypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 17], "end": [65, 22], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/higher_order_function.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33c34713ab29e66a694b83e6"></a>
## eq

`function` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &HigherOrderTypeSignature) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderTypeSignature", "path": "HigherOrderTypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 24], "end": [65, 33], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/higher_order_function.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daae43dd9623152cd16374ad"></a>
## fmt

`function` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderTypeSignature", "path": "HigherOrderTypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/higher_order_function.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94df708709b19e3c43e52c1c"></a>
## hash

`function` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderTypeSignature", "path": "HigherOrderTypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 51], "end": [65, 55], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/higher_order_function.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e10bd815dff5e94e863d268f"></a>
## partial_cmp

`function` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &HigherOrderTypeSignature) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderTypeSignature", "path": "HigherOrderTypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 39], "end": [65, 49], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/higher_order_function.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
