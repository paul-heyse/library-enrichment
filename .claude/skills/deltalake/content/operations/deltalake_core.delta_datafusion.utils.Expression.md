# `deltalake_core::delta_datafusion::utils::Expression`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.utils.Expression.json).

<a id="op-692eba028643ec245da47450"></a>
## Expression

`enum` · `deltalake_core::delta_datafusion::utils::Expression` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
enum Expression
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/utils.rs#L17).

Source: `crates/core/src/delta_datafusion/utils.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Used to represent user input of either a Datafusion expression or string expression

<a id="op-9ed3cbc977775a7f223e148e"></a>
## DataFusion

`variant` · `deltalake_core::delta_datafusion::utils::Expression::DataFusion` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
DataFusion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/utils.rs#L19).

Source: `crates/core/src/delta_datafusion/utils.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion Expression

<a id="op-830f1cb6873e88140a8c9f59"></a>
## String

`variant` · `deltalake_core::delta_datafusion::utils::Expression::String` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/utils.rs#L21).

Source: `crates/core/src/delta_datafusion/utils.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

String Expression

<a id="op-e9ae16ce961170cd63506470"></a>
## clone

`function` · `deltalake_core::delta_datafusion::utils::Expression::clone` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Expression
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/utils.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::utils::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 17], "end": [16, 22], "filename": "crates/core/src/delta_datafusion/utils.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/utils.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c23965125a58254f15a49745"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::utils::Expression::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/utils.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::utils::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 10], "end": [16, 15], "filename": "crates/core/src/delta_datafusion/utils.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/utils.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7ccc3d951c69485d45099fc"></a>
## from

`function` · `deltalake_core::delta_datafusion::utils::Expression::from` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from(val: &str) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/utils.rs#L55).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::utils::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [58, 2], "filename": "crates/core/src/delta_datafusion/utils.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/utils.rs:55`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd4aed5f78eb19d52b1603a6"></a>
## from

`function` · `deltalake_core::delta_datafusion::utils::Expression::from` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from(val: String) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/utils.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::utils::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [63, 2], "filename": "crates/core/src/delta_datafusion/utils.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/utils.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e36f4443ba2f9b0c5fc43d13"></a>
## from

`function` · `deltalake_core::delta_datafusion::utils::Expression::from` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from(val: Expr) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/utils.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::utils::Expression", "path": "Expression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [52, 2], "filename": "crates/core/src/delta_datafusion/utils.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/utils.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
