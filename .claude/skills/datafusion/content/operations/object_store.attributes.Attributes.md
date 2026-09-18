# `object_store::attributes::Attributes`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.attributes.Attributes.json).

<a id="op-196267bf7f6d968d4ceb156e"></a>
## Attributes

`struct` · `object_store::attributes::Attributes` · object_store 0.13.2

```rust
struct Attributes
```

Source: `src/attributes.rs:110`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Additional attributes of an object

Attributes can be specified in [PutOptions](crate::PutOptions) and retrieved
from APIs returning [GetResult](crate::GetResult).

Unlike [`ObjectMeta`](crate::ObjectMeta), [`Attributes`](../operations/object_store.attributes.Attributes.md#op-196267bf7f6d968d4ceb156e) are not returned by
listing APIs

<a id="op-1013b69fff559dfaf7f5b66e"></a>
## clone

`function` · `object_store::attributes::Attributes::clone` · object_store 0.13.2

```rust
fn clone(&self) -> Attributes
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 41], "end": [109, 46], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/attributes.rs:109`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ed8e7bd9ea54d8577f80551"></a>
## default

`function` · `object_store::attributes::Attributes::default` · object_store 0.13.2

```rust
fn default() -> Attributes
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 17], "end": [109, 24], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/attributes.rs:109`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-834a0ae5ac333ac07330dede"></a>
## eq

`function` · `object_store::attributes::Attributes::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &Attributes) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 30], "end": [109, 39], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/attributes.rs:109`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31ca1e1a6e6ce30dcfead1ca"></a>
## fmt

`function` · `object_store::attributes::Attributes::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 10], "end": [109, 15], "filename": "src/attributes.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/attributes.rs:109`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6bd8fe3d56a76ea29ff6851"></a>
## from_iter

`function` · `object_store::attributes::Attributes::from_iter` · object_store 0.13.2

```rust
fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::attributes::Attribute", "path": "Attribute"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::attributes::AttributeValue", "path": "AttributeValue"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [158, 1], "end": [170, 2], "filename": "src/attributes.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "K"}, {"generic": "V"}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/attributes.rs:163`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8ac5b5bfd1c161aea961892"></a>
## get

`function` · `object_store::attributes::Attributes::get` · object_store 0.13.2

```rust
fn get(&self, key: &Attribute) -> Option<&AttributeValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [156, 2], "filename": "src/attributes.rs"}, "trait": null, "trait_path": null}`

Source: `src/attributes.rs:131`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the [`AttributeValue`](../operations/object_store.attributes.AttributeValue.md#op-6e04dc5b3150f562335ebac2) for `key` if any

<a id="op-ccb9bd84a0e1ac962e09c18d"></a>
## insert

`function` · `object_store::attributes::Attributes::insert` · object_store 0.13.2

```rust
fn insert(&mut self, key: Attribute, value: AttributeValue) -> Option<AttributeValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [156, 2], "filename": "src/attributes.rs"}, "trait": null, "trait_path": null}`

Source: `src/attributes.rs:126`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Insert a new [`Attribute`](../operations/object_store.attributes.Attribute.md#op-ba938db385f847e7b49d9c79), [`AttributeValue`](../operations/object_store.attributes.AttributeValue.md#op-6e04dc5b3150f562335ebac2) pair

Returns the previous value for `key` if any

<a id="op-79b4280443c29991c14b53ed"></a>
## is_empty

`function` · `object_store::attributes::Attributes::is_empty` · object_store 0.13.2

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [156, 2], "filename": "src/attributes.rs"}, "trait": null, "trait_path": null}`

Source: `src/attributes.rs:153`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns true if this contains no [`Attribute`](../operations/object_store.attributes.Attribute.md#op-ba938db385f847e7b49d9c79)

<a id="op-1cd9e4cebc1857a3826f86b9"></a>
## iter

`function` · `object_store::attributes::Attributes::iter` · object_store 0.13.2

```rust
fn iter(&self) -> AttributesIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [156, 2], "filename": "src/attributes.rs"}, "trait": null, "trait_path": null}`

Source: `src/attributes.rs:141`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns an [`AttributesIter`](../operations/object_store.attributes.AttributesIter.md#op-e3425560324726f040a57f3b) over this

<a id="op-9944a4d197c065d652122b4f"></a>
## len

`function` · `object_store::attributes::Attributes::len` · object_store 0.13.2

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [156, 2], "filename": "src/attributes.rs"}, "trait": null, "trait_path": null}`

Source: `src/attributes.rs:147`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the number of [`Attribute`](../operations/object_store.attributes.Attribute.md#op-ba938db385f847e7b49d9c79) in this collection

<a id="op-0f2cf281c361ab41c30aa855"></a>
## new

`function` · `object_store::attributes::Attributes::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [156, 2], "filename": "src/attributes.rs"}, "trait": null, "trait_path": null}`

Source: `src/attributes.rs:114`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new empty [`Attributes`](../operations/object_store.attributes.Attributes.md#op-196267bf7f6d968d4ceb156e)

<a id="op-a18b0846fee238fb01deb169"></a>
## remove

`function` · `object_store::attributes::Attributes::remove` · object_store 0.13.2

```rust
fn remove(&mut self, key: &Attribute) -> Option<AttributeValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [156, 2], "filename": "src/attributes.rs"}, "trait": null, "trait_path": null}`

Source: `src/attributes.rs:136`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Removes the [`AttributeValue`](../operations/object_store.attributes.AttributeValue.md#op-6e04dc5b3150f562335ebac2) for `key` if any

<a id="op-7db79746a85589cd0a9bf9ec"></a>
## with_capacity

`function` · `object_store::attributes::Attributes::with_capacity` · object_store 0.13.2

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [156, 2], "filename": "src/attributes.rs"}, "trait": null, "trait_path": null}`

Source: `src/attributes.rs:119`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`Attributes`](../operations/object_store.attributes.Attributes.md#op-196267bf7f6d968d4ceb156e) with space for `capacity` [`Attribute`](../operations/object_store.attributes.Attribute.md#op-ba938db385f847e7b49d9c79)
