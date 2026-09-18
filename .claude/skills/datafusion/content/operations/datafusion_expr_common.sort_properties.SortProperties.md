# `datafusion_expr_common::sort_properties::SortProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.sort_properties.SortProperties.json).

<a id="op-a357ca132b9df8290c59f5bf"></a>
## SortProperties

`enum` · `datafusion_expr_common::sort_properties::SortProperties` · datafusion-expr-common 55.1.0

```rust
enum SortProperties
```

Source: `src/sort_properties.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

To propagate [`SortOptions`](../operations/arrow_schema.SortOptions.md#op-78d98c3e0c6da432658d0949) across the `PhysicalExpr`, it is insufficient
to simply use `Option<SortOptions>`: There must be a differentiation between
unordered columns and literal values, since literals may not break the ordering
when they are used as a child of some binary expression when the other child has
some ordering. On the other hand, unordered columns cannot maintain ordering when
they take part in such operations.

Example: ((a_ordered + b_unordered) + c_ordered) expression cannot end up with
sorted data; however the ((a_ordered + 999) + c_ordered) expression can. Therefore,
we need two different variants for literals and unordered columns as literals are
often more ordering-friendly under most mathematical operations.

<a id="op-e89500d77010715fb1b1d77f"></a>
## Ordered

`variant` · `datafusion_expr_common::sort_properties::SortProperties::Ordered` · datafusion-expr-common 55.1.0

```rust
Ordered
```

Source: `src/sort_properties.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Use the ordinary [`SortOptions`](../operations/arrow_schema.SortOptions.md#op-78d98c3e0c6da432658d0949) struct to represent ordered data:

<a id="op-42809f81d168516d4d3210d4"></a>
## Output

`assoc_type` · `datafusion_expr_common::sort_properties::SortProperties::Output` · datafusion-expr-common 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [130, 2], "filename": "src/sort_properties.rs"}, "trait": {"args": null, "id": "core::ops::arith::Neg", "path": "Neg"}, "trait_path": "core::ops::arith::Neg"}`

Source: `src/sort_properties.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c67ffed0f24b7c9140b77fbe"></a>
## Singleton

`variant` · `datafusion_expr_common::sort_properties::SortProperties::Singleton` · datafusion-expr-common 55.1.0

```rust
Singleton
```

Source: `src/sort_properties.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-921f855387cf41894ee220e2"></a>
## Unordered

`variant` · `datafusion_expr_common::sort_properties::SortProperties::Unordered` · datafusion-expr-common 55.1.0

```rust
Unordered
```

Source: `src/sort_properties.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6254f835e6018df69283595"></a>
## add

`function` · `datafusion_expr_common::sort_properties::SortProperties::add` · datafusion-expr-common 55.1.0

```rust
fn add(&self, rhs: &Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [119, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-870db4cebc2b539e8bedde0f"></a>
## and_or

`function` · `datafusion_expr_common::sort_properties::SortProperties::and_or` · datafusion-expr-common 55.1.0

```rust
fn and_or(&self, rhs: &Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [119, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3edb7d8fa96226c3738a07c"></a>
## clone

`function` · `datafusion_expr_common::sort_properties::SortProperties::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> SortProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 28], "end": [36, 33], "filename": "src/sort_properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort_properties.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b9c98bc2799fd043276cc4f"></a>
## default

`function` · `datafusion_expr_common::sort_properties::SortProperties::default` · datafusion-expr-common 55.1.0

```rust
fn default() -> SortProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 41], "end": [36, 48], "filename": "src/sort_properties.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sort_properties.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd132c441d33b41b4a65658a"></a>
## eq

`function` · `datafusion_expr_common::sort_properties::SortProperties::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &SortProperties) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 19], "filename": "src/sort_properties.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sort_properties.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9eccdd9bce24baf7c5a3b2ff"></a>
## fmt

`function` · `datafusion_expr_common::sort_properties::SortProperties::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 21], "end": [36, 26], "filename": "src/sort_properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort_properties.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-def8105bb61a1987387aca18"></a>
## gt_or_gteq

`function` · `datafusion_expr_common::sort_properties::SortProperties::gt_or_gteq` · datafusion-expr-common 55.1.0

```rust
fn gt_or_gteq(&self, rhs: &Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [119, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d425c08a54bb901f0f27e036"></a>
## neg

`function` · `datafusion_expr_common::sort_properties::SortProperties::neg` · datafusion-expr-common 55.1.0

```rust
fn neg(self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [130, 2], "filename": "src/sort_properties.rs"}, "trait": {"args": null, "id": "core::ops::arith::Neg", "path": "Neg"}, "trait_path": "core::ops::arith::Neg"}`

Source: `src/sort_properties.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e472eb3a23dc52fbe890ccb7"></a>
## sub

`function` · `datafusion_expr_common::sort_properties::SortProperties::sub` · datafusion-expr-common 55.1.0

```rust
fn sub(&self, rhs: &Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [119, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
