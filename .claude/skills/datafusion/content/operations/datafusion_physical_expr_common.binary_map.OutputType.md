# `datafusion_physical_expr_common::binary_map::OutputType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.binary_map.OutputType.json).

<a id="op-133fbc7d153a0bd46b15af1b"></a>
## OutputType

`enum` · `datafusion_physical_expr_common::binary_map::OutputType` · datafusion-physical-expr-common 55.1.0

```rust
enum OutputType
```

Source: `src/binary_map.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Should the output be a String or Binary?

<a id="op-2736dee80318330d3c99a46e"></a>
## Binary

`variant` · `datafusion_physical_expr_common::binary_map::OutputType::Binary` · datafusion-physical-expr-common 55.1.0

```rust
Binary
```

Source: `src/binary_map.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

`BinaryArray` or `LargeBinaryArray`

<a id="op-b3bce28c33e00834e38f6c03"></a>
## BinaryView

`variant` · `datafusion_physical_expr_common::binary_map::OutputType::BinaryView` · datafusion-physical-expr-common 55.1.0

```rust
BinaryView
```

Source: `src/binary_map.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

`BinaryViewArray`

<a id="op-fac842d3fb4f9cec41e35d9c"></a>
## Utf8

`variant` · `datafusion_physical_expr_common::binary_map::OutputType::Utf8` · datafusion-physical-expr-common 55.1.0

```rust
Utf8
```

Source: `src/binary_map.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

`StringArray` or `LargeStringArray`

<a id="op-9cefe8716729cf46b8e33b43"></a>
## Utf8View

`variant` · `datafusion_physical_expr_common::binary_map::OutputType::Utf8View` · datafusion-physical-expr-common 55.1.0

```rust
Utf8View
```

Source: `src/binary_map.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

`StringViewArray`

<a id="op-977c20867c5d3e852398d595"></a>
## clone

`function` · `datafusion_physical_expr_common::binary_map::OutputType::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> OutputType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_map::OutputType", "path": "OutputType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/binary_map.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/binary_map.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2844d2572e7293a7d15312ff"></a>
## eq

`function` · `datafusion_physical_expr_common::binary_map::OutputType::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &OutputType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_map::OutputType", "path": "OutputType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 30], "end": [39, 39], "filename": "src/binary_map.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/binary_map.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6198ecd43a00b422f3417d4c"></a>
## fmt

`function` · `datafusion_physical_expr_common::binary_map::OutputType::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::binary_map::OutputType", "path": "OutputType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/binary_map.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/binary_map.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
