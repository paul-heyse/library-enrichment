# `datafusion_common::null_equality::NullEquality`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.null_equality.NullEquality.json).

<a id="op-f799a818b3305012370a4a5e"></a>
## NullEquality

`enum` · `datafusion_common::null_equality::NullEquality` · datafusion-common 55.1.0

```rust
enum NullEquality
```

Source: `src/null_equality.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents the behavior for null values when evaluating equality. Currently, its primary use
case is to define the behavior of joins for null values.

# Examples

The following table shows the expected equality behavior for `NullEquality`.

| A    | B    | NullEqualsNothing | NullEqualsNull |
|------|------|-------------------|----------------|
| NULL | NULL | false             | true           |
| NULL | 'b'  | false             | false          |
| 'a'  | NULL | false             | false          |
| 'a'  | 'b'  | false             | false          |

# Order

The order on this type represents the "restrictiveness" of the behavior. The more restrictive
a behavior is, the fewer elements are considered to be equal to null.
[NullEquality::NullEqualsNothing](../operations/datafusion_common.null_equality.NullEquality.md#op-c116c80db5727dfee9e298d7) represents the most restrictive behavior.

This mirrors the old order with `null_equals_null` booleans, as `false` indicated that
`null != null`.

<a id="op-c116c80db5727dfee9e298d7"></a>
## NullEqualsNothing

`variant` · `datafusion_common::null_equality::NullEquality::NullEqualsNothing` · datafusion-common 55.1.0

```rust
NullEqualsNothing
```

Source: `src/null_equality.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Null is *not* equal to anything (`null != null`)

<a id="op-15654c5c90b569c147800782"></a>
## NullEqualsNull

`variant` · `datafusion_common::null_equality::NullEquality::NullEqualsNull` · datafusion-common 55.1.0

```rust
NullEqualsNull
```

Source: `src/null_equality.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Null is equal to null (`null == null`)

<a id="op-178b12da71084e5d82ff5379"></a>
## clone

`function` · `datafusion_common::null_equality::NullEquality::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> NullEquality
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::null_equality::NullEquality", "path": "NullEquality"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 17], "end": [40, 22], "filename": "src/null_equality.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/null_equality.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fec566995c924524face3c6d"></a>
## eq

`function` · `datafusion_common::null_equality::NullEquality::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &NullEquality) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::null_equality::NullEquality", "path": "NullEquality"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 30], "end": [40, 39], "filename": "src/null_equality.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/null_equality.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e927b9ea673d9e6c15bde18"></a>
## fmt

`function` · `datafusion_common::null_equality::NullEquality::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::null_equality::NullEquality", "path": "NullEquality"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 10], "end": [40, 15], "filename": "src/null_equality.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/null_equality.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b44655effbbdd4a386681619"></a>
## hash

`function` · `datafusion_common::null_equality::NullEquality::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::null_equality::NullEquality", "path": "NullEquality"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 57], "end": [40, 61], "filename": "src/null_equality.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/null_equality.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6007ecb55ee9cc20a892695c"></a>
## partial_cmp

`function` · `datafusion_common::null_equality::NullEquality::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &NullEquality) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::null_equality::NullEquality", "path": "NullEquality"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 45], "end": [40, 55], "filename": "src/null_equality.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/null_equality.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
