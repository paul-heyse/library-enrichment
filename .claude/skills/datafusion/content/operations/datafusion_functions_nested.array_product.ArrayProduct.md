# `datafusion_functions_nested::array_product::ArrayProduct`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.array_product.ArrayProduct.json).

<a id="op-557a3bdd8ecf1098ee386321"></a>
## ArrayProduct

`struct` · `datafusion_functions_nested::array_product::ArrayProduct` · datafusion-functions-nested 55.1.0

```rust
struct ArrayProduct
```

Source: `src/array_product.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a7a7bf08ad86b66c1f130b5"></a>
## aliases

`function` · `datafusion_functions_nested::array_product::ArrayProduct::aliases` · datafusion-functions-nested 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [128, 2], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_product.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f66ceee92c6f0080e7bff6c"></a>
## coerce_types

`function` · `datafusion_functions_nested::array_product::ArrayProduct::coerce_types` · datafusion-functions-nested 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [128, 2], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_product.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20e9030bd9801ce401e46b6b"></a>
## default

`function` · `datafusion_functions_nested::array_product::ArrayProduct::default` · datafusion-functions-nested 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [76, 2], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/array_product.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-191d8c7cd2d9e865865be821"></a>
## documentation

`function` · `datafusion_functions_nested::array_product::ArrayProduct::documentation` · datafusion-functions-nested 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [128, 2], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_product.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f95776f8154cd721baf765fd"></a>
## eq

`function` · `datafusion_functions_nested::array_product::ArrayProduct::eq` · datafusion-functions-nested 55.1.0

```rust
fn eq(&self, other: &ArrayProduct) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 17], "end": [66, 26], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array_product.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccccdaec2137a6f43ce97971"></a>
## fmt

`function` · `datafusion_functions_nested::array_product::ArrayProduct::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 10], "end": [66, 15], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array_product.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81df4b235a2ef9de903cb5eb"></a>
## hash

`function` · `datafusion_functions_nested::array_product::ArrayProduct::hash` · datafusion-functions-nested 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 32], "end": [66, 36], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/array_product.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-230a87a04ac4b51e49535d77"></a>
## invoke_with_args

`function` · `datafusion_functions_nested::array_product::ArrayProduct::invoke_with_args` · datafusion-functions-nested 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [128, 2], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_product.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-425b7f1463e9051b4ebe41be"></a>
## name

`function` · `datafusion_functions_nested::array_product::ArrayProduct::name` · datafusion-functions-nested 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [128, 2], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_product.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5aba75fbfd8fff370713364"></a>
## new

`function` · `datafusion_functions_nested::array_product::ArrayProduct::new` · datafusion-functions-nested 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [85, 2], "filename": "src/array_product.rs"}, "trait": null, "trait_path": null}`

Source: `src/array_product.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce00d3b2725cf90ed940c2ec"></a>
## return_type

`function` · `datafusion_functions_nested::array_product::ArrayProduct::return_type` · datafusion-functions-nested 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [128, 2], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_product.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-070785a9b3ff98bd68543fa9"></a>
## signature

`function` · `datafusion_functions_nested::array_product::ArrayProduct::signature` · datafusion-functions-nested 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_product::ArrayProduct", "path": "ArrayProduct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [128, 2], "filename": "src/array_product.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_product.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
