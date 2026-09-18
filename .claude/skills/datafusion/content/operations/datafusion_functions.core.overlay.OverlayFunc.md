# `datafusion_functions::core::overlay::OverlayFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.overlay.OverlayFunc.json).

<a id="op-7a01c59c438c4ce6e39270c6"></a>
## OverlayFunc

`struct` · `datafusion_functions::core::overlay::OverlayFunc` · datafusion-functions 55.1.0

```rust
struct OverlayFunc
```

Source: `src/core/overlay.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83ee5447be701e2d9dfa4153"></a>
## default

`function` · `datafusion_functions::core::overlay::OverlayFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [69, 2], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/overlay.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c95286556fe1520656ca3fcb"></a>
## documentation

`function` · `datafusion_functions::core::overlay::OverlayFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [118, 2], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/overlay.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a2d407d82b1e0701472d9b6"></a>
## eq

`function` · `datafusion_functions::core::overlay::OverlayFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &OverlayFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 17], "end": [60, 26], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/overlay.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fc3c11d7dfd5430a068550e"></a>
## fmt

`function` · `datafusion_functions::core::overlay::OverlayFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 15], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/overlay.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e359d984650b95f7f0caa62"></a>
## hash

`function` · `datafusion_functions::core::overlay::OverlayFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 32], "end": [60, 36], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/overlay.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f5396fb15994cb7ec59b284"></a>
## invoke_with_args

`function` · `datafusion_functions::core::overlay::OverlayFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [118, 2], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/overlay.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64c16fd8ac7d2be550818f39"></a>
## name

`function` · `datafusion_functions::core::overlay::OverlayFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [118, 2], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/overlay.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87181f2c04f4e1a694d11d87"></a>
## new

`function` · `datafusion_functions::core::overlay::OverlayFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [88, 2], "filename": "src/core/overlay.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/overlay.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d197b21c85407d73bab91fc1"></a>
## return_type

`function` · `datafusion_functions::core::overlay::OverlayFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [118, 2], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/overlay.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a268830385b87be6a751e509"></a>
## signature

`function` · `datafusion_functions::core::overlay::OverlayFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::overlay::OverlayFunc", "path": "OverlayFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [118, 2], "filename": "src/core/overlay.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/overlay.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
