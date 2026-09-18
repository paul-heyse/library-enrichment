# `datafusion_common::param_value::ParamValues`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.param_value.ParamValues.json).

<a id="op-da42ffa15ab2c27aa37b6cf0"></a>
## ParamValues

`enum` · `datafusion_common::param_value::ParamValues` · datafusion-common 55.1.0

```rust
enum ParamValues
```

Source: `src/param_value.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The parameter value corresponding to the placeholder

<a id="op-fa918a74c00f5529dfafb09a"></a>
## List

`variant` · `datafusion_common::param_value::ParamValues::List` · datafusion-common 55.1.0

```rust
List
```

Source: `src/param_value.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

For positional query parameters, like `SELECT * FROM test WHERE a > $1 AND b = $2`

<a id="op-9a56c1358624047f302dcaf1"></a>
## Map

`variant` · `datafusion_common::param_value::ParamValues::Map` · datafusion-common 55.1.0

```rust
Map
```

Source: `src/param_value.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

For named query parameters, like `SELECT * FROM test WHERE a > $foo AND b = $goo`

<a id="op-2cedb6096070c6359fbd3806"></a>
## clone

`function` · `datafusion_common::param_value::ParamValues::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ParamValues
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::param_value::ParamValues", "path": "ParamValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 17], "end": [25, 22], "filename": "src/param_value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/param_value.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b57a66639418e14eb18b17a"></a>
## fmt

`function` · `datafusion_common::param_value::ParamValues::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::param_value::ParamValues", "path": "ParamValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 10], "end": [25, 15], "filename": "src/param_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/param_value.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53e4f7ee03830e538402ce1b"></a>
## from

`function` · `datafusion_common::param_value::ParamValues::from` · datafusion-common 55.1.0

```rust
fn from(value: Vec<(K, ScalarValue)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::param_value::ParamValues", "path": "ParamValues"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [122, 1], "end": [133, 2], "filename": "src/param_value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "K"}, {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}]}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/param_value.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59119857363bd9b7a78a09b6"></a>
## from

`function` · `datafusion_common::param_value::ParamValues::from` · datafusion-common 55.1.0

```rust
fn from(value: Vec<ScalarValue>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::param_value::ParamValues", "path": "ParamValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [120, 2], "filename": "src/param_value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/param_value.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2af6b6f3663e9ba071309d4"></a>
## from

`function` · `datafusion_common::param_value::ParamValues::from` · datafusion-common 55.1.0

```rust
fn from(value: HashMap<K, ScalarValue>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::param_value::ParamValues", "path": "ParamValues"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}]}, "is_negative": false, "span": {"begin": [135, 1], "end": [146, 2], "filename": "src/param_value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/param_value.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-810d04aaeffdf8a195a5dc54"></a>
## get_placeholders_with_values

`function` · `datafusion_common::param_value::ParamValues::get_placeholders_with_values` · datafusion-common 55.1.0

```rust
fn get_placeholders_with_values(&self, id: &str) -> Result<ScalarAndMetadata>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::param_value::ParamValues", "path": "ParamValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [114, 2], "filename": "src/param_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/param_value.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06a9105606c632deca33ec6f"></a>
## verify

`function` · `datafusion_common::param_value::ParamValues::verify` · datafusion-common 55.1.0

```rust
fn verify(&self, expect: &[DataType]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::param_value::ParamValues", "path": "ParamValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [114, 2], "filename": "src/param_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/param_value.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Verify parameter list length and DataType

Use [`ParamValues::verify_fields`](../operations/datafusion_common.param_value.ParamValues.md#op-cd1c52eb9b203c88de93fd2f) to ensure field metadata is considered when
computing type equality.

<a id="op-cd1c52eb9b203c88de93fd2f"></a>
## verify_fields

`function` · `datafusion_common::param_value::ParamValues::verify_fields` · datafusion-common 55.1.0

```rust
fn verify_fields(&self, expect: &[FieldRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::param_value::ParamValues", "path": "ParamValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [114, 2], "filename": "src/param_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/param_value.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Verify parameter list length and type
