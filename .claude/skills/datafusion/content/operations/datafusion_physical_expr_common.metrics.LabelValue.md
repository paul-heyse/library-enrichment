# `datafusion_physical_expr_common::metrics::LabelValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.LabelValue.json).

<a id="op-23c44f864f6fba78ef3a6c26"></a>
## LabelValue

`struct` · `datafusion_physical_expr_common::metrics::LabelValue` · datafusion-physical-expr-common 55.1.0

```rust
struct LabelValue
```

Source: `src/metrics/mod.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

A label name or value.

String literals preserve the existing allocation-free path. Dynamic strings
can be stored behind [`Arc<str>`], so cloning a [`Label`](../operations/datafusion_physical_expr_common.metrics.Label.md#op-32cbece590fe957187c93920) only increments an
atomic reference count and does not allocate or copy the underlying string
data.

Unresolved upstream links (retained, not inferred): ``Arc<str>``.

<a id="op-2c84206f289c521aefc72352"></a>
## as_str

`function` · `datafusion_physical_expr_common::metrics::LabelValue::as_str` · datafusion-physical-expr-common 55.1.0

```rust
fn as_str(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [594, 1], "end": [602, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:596`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return this label value as a string slice.

<a id="op-35c98286a55a597502eae778"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::LabelValue::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> LabelValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 10], "end": [580, 15], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/mod.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cca1680f0e97279f0e4f5f1"></a>
## eq

`function` · `datafusion_physical_expr_common::metrics::LabelValue::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [635, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metrics/mod.rs:632`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-593943bb6c1d3a9ac79acd39"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::LabelValue::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 1], "end": [649, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/mod.rs:646`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c615f041db4cbfff56b7fb3d"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::LabelValue::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [651, 1], "end": [655, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/mod.rs:652`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36a18f870c124465055d3cfe"></a>
## from

`function` · `datafusion_physical_expr_common::metrics::LabelValue::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: Cow<'static, str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [622, 1], "end": [629, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'static"}, {"type": {"primitive": "str"}}], "constraints": []}}, "id": "alloc::borrow::Cow", "path": "Cow"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metrics/mod.rs:623`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a7690f547f88703b372d5d6"></a>
## from

`function` · `datafusion_physical_expr_common::metrics::LabelValue::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: Arc<str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [616, 1], "end": [620, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metrics/mod.rs:617`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecec8498018b95c10b1689f1"></a>
## from

`function` · `datafusion_physical_expr_common::metrics::LabelValue::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 1], "end": [614, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metrics/mod.rs:611`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4b61729923a133e8163ce83"></a>
## from

`function` · `datafusion_physical_expr_common::metrics::LabelValue::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(value: &'static str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [604, 1], "end": [608, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'static", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metrics/mod.rs:605`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95f82df486b82aae35b1e89a"></a>
## hash

`function` · `datafusion_physical_expr_common::metrics::LabelValue::hash` · datafusion-physical-expr-common 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::LabelValue", "path": "LabelValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [639, 1], "end": [643, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/metrics/mod.rs:640`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
