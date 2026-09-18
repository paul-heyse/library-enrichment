# `datafusion_functions::core::struct::StructFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.struct.StructFunc.json).

<a id="op-33b0f39c160d66c86a9fd609"></a>
## StructFunc

`struct` · `datafusion_functions::core::struct::StructFunc` · datafusion-functions 55.1.0

```rust
struct StructFunc
```

Source: `src/core/struct.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb5a27eea8c74b5aa390c369"></a>
## aliases

`function` · `datafusion_functions::core::struct::StructFunc::aliases` · datafusion-functions 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [139, 2], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/struct.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dce9277ea0cb886336090850"></a>
## default

`function` · `datafusion_functions::core::struct::StructFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [78, 2], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/struct.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-590c4b9d232f6200aa7b82c7"></a>
## documentation

`function` · `datafusion_functions::core::struct::StructFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [139, 2], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/struct.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95d9498b8185debc1c281a64"></a>
## eq

`function` · `datafusion_functions::core::struct::StructFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &StructFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 17], "end": [68, 26], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/struct.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-545a6b55e99968f22141b9af"></a>
## fmt

`function` · `datafusion_functions::core::struct::StructFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/struct.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a4957a0574c93f31e94ae8a"></a>
## hash

`function` · `datafusion_functions::core::struct::StructFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 32], "end": [68, 36], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/struct.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-058a7ecb821b2ce986da7ca3"></a>
## invoke_with_args

`function` · `datafusion_functions::core::struct::StructFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [139, 2], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/struct.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a09b58c9a2a24fa72b16824"></a>
## name

`function` · `datafusion_functions::core::struct::StructFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [139, 2], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/struct.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a1a1539f0c7863620fafae3"></a>
## new

`function` · `datafusion_functions::core::struct::StructFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [87, 2], "filename": "src/core/struct.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/struct.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4afdcc9cbc4e8b0110e5e7c5"></a>
## return_type

`function` · `datafusion_functions::core::struct::StructFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [139, 2], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/struct.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ca90448b0241bf636d36df5"></a>
## signature

`function` · `datafusion_functions::core::struct::StructFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::struct::StructFunc", "path": "StructFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [139, 2], "filename": "src/core/struct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/struct.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
