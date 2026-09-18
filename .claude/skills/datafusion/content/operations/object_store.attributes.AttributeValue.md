# `object_store::attributes::AttributeValue`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.attributes.AttributeValue.json).

<a id="op-6e04dc5b3150f562335ebac2"></a>
## AttributeValue

`struct` · `object_store::attributes::AttributeValue` · object_store 0.13.2

```rust
struct AttributeValue
```

Source: `src/attributes.rs:74`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The value of an [`Attribute`](../operations/object_store.attributes.Attribute.md#op-ba938db385f847e7b49d9c79)

Provides efficient conversion from both static and owned strings

```
# use object_store::AttributeValue;
// Can use static strings without needing an allocation
let value = AttributeValue::from("bar");
// Can also store owned strings
let value = AttributeValue::from("foo".to_string());
```

<a id="op-dab74aa394913b0c32a27c65"></a>
## Target

`assoc_type` · `object_store::attributes::AttributeValue::Target` · object_store 0.13.2

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [100, 2], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/attributes.rs:95`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1ef4fcb3f98813c618185bf"></a>
## as_ref

`function` · `object_store::attributes::AttributeValue::as_ref` · object_store 0.13.2

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [80, 2], "filename": "src/attributes.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/attributes.rs:77`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a44a501eb58639b7e3d4534d"></a>
## clone

`function` · `object_store::attributes::AttributeValue::clone` · object_store 0.13.2

```rust
fn clone(&self) -> AttributeValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 38], "end": [73, 43], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/attributes.rs:73`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba54fde8cbd7498f62e10f84"></a>
## deref

`function` · `object_store::attributes::AttributeValue::deref` · object_store 0.13.2

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [100, 2], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/attributes.rs:97`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c109a9124a0167c28fafdb3"></a>
## eq

`function` · `object_store::attributes::AttributeValue::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &AttributeValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 27], "end": [73, 36], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/attributes.rs:73`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5bc73ba098a89e37a1ad750"></a>
## fmt

`function` · `object_store::attributes::AttributeValue::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 10], "end": [73, 15], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/attributes.rs:73`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06e4395328a0f4ea7b206de5"></a>
## from

`function` · `object_store::attributes::AttributeValue::from` · object_store 0.13.2

```rust
fn from(value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [92, 2], "filename": "src/attributes.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/attributes.rs:89`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f832ccf34e0df97f7560629"></a>
## from

`function` · `object_store::attributes::AttributeValue::from` · object_store 0.13.2

```rust
fn from(value: &'static str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [86, 2], "filename": "src/attributes.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'static", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/attributes.rs:83`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75a2c9ec2c7a0a51cfa15582"></a>
## hash

`function` · `object_store::attributes::AttributeValue::hash` · object_store 0.13.2

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 17], "end": [73, 21], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/attributes.rs:73`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
