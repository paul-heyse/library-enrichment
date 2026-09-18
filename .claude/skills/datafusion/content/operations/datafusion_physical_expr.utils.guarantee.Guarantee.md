# `datafusion_physical_expr::utils::guarantee::Guarantee`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.guarantee.Guarantee.json).

<a id="op-2f7eb0649ae818e53b368cc3"></a>
## Guarantee

`enum` · `datafusion_physical_expr::utils::guarantee::Guarantee` · datafusion-physical-expr 55.1.0

```rust
enum Guarantee
```

Source: `src/utils/guarantee.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

What is guaranteed about the values for a [`LiteralGuarantee`](../operations/datafusion_physical_expr.utils.guarantee.LiteralGuarantee.md#op-c6447f3f77c11eaea6c2ddd0)?

<a id="op-ddd84e23a013a66f12f81110"></a>
## In

`variant` · `datafusion_physical_expr::utils::guarantee::Guarantee::In` · datafusion-physical-expr 55.1.0

```rust
In
```

Source: `src/utils/guarantee.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Guarantee that the expression is `true` if `column` is one of the values. If
`column` is not one of the values, the expression can not be `true`.

<a id="op-0b5b0e259a6f2c36c7ce7697"></a>
## NotIn

`variant` · `datafusion_physical_expr::utils::guarantee::Guarantee::NotIn` · datafusion-physical-expr 55.1.0

```rust
NotIn
```

Source: `src/utils/guarantee.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Guarantee that the expression is `true` if `column` is not ANY of the
values. If `column` only takes one of these values, the expression can
not be `true`.

<a id="op-ba7a09edeafce1d04638e9ff"></a>
## clone

`function` · `datafusion_physical_expr::utils::guarantee::Guarantee::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> Guarantee
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::Guarantee", "path": "Guarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 17], "end": [81, 22], "filename": "src/utils/guarantee.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/utils/guarantee.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-370131787c3216665721a913"></a>
## eq

`function` · `datafusion_physical_expr::utils::guarantee::Guarantee::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Guarantee) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::Guarantee", "path": "Guarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 30], "end": [81, 39], "filename": "src/utils/guarantee.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/utils/guarantee.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-446dc86724a79f8edb9f04fa"></a>
## fmt

`function` · `datafusion_physical_expr::utils::guarantee::Guarantee::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::Guarantee", "path": "Guarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 10], "end": [81, 15], "filename": "src/utils/guarantee.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/utils/guarantee.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c100c8f30313099a441b81d"></a>
## hash

`function` · `datafusion_physical_expr::utils::guarantee::Guarantee::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::Guarantee", "path": "Guarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 45], "end": [81, 49], "filename": "src/utils/guarantee.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/utils/guarantee.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
