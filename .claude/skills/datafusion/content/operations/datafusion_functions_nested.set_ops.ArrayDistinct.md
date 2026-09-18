# `datafusion_functions_nested::set_ops::ArrayDistinct`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.set_ops.ArrayDistinct.json).

<a id="op-996c730d478bbbd96ec1e6fd"></a>
## ArrayDistinct

`struct` · `datafusion_functions_nested::set_ops::ArrayDistinct` · datafusion-functions-nested 55.1.0

```rust
struct ArrayDistinct
```

Source: `src/set_ops.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde180ab6962c18d956a640a"></a>
## aliases

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::aliases` · datafusion-functions-nested 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [297, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-745a83da86b743ad8cc2c87b"></a>
## default

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::default` · datafusion-functions-nested 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [267, 1], "end": [271, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/set_ops.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-873954081eb5e2f1fa44f4fc"></a>
## documentation

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::documentation` · datafusion-functions-nested 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [297, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81bd284d9a4ec6262220651d"></a>
## eq

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::eq` · datafusion-functions-nested 55.1.0

```rust
fn eq(&self, other: &ArrayDistinct) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 17], "end": [252, 26], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/set_ops.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfba60f042beacd3ac316789"></a>
## fmt

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 10], "end": [252, 15], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/set_ops.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee6261846e0cd3c393459a96"></a>
## hash

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::hash` · datafusion-functions-nested 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 32], "end": [252, 36], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/set_ops.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b9fb203e38055b82e00cff7"></a>
## invoke_with_args

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::invoke_with_args` · datafusion-functions-nested 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [297, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48114939e90670dc566d4509"></a>
## name

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::name` · datafusion-functions-nested 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [297, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-200117eb0b480b95a3af0878"></a>
## new

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::new` · datafusion-functions-nested 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [265, 2], "filename": "src/set_ops.rs"}, "trait": null, "trait_path": null}`

Source: `src/set_ops.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68a6492c86c05f91370cb712"></a>
## return_type

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::return_type` · datafusion-functions-nested 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [297, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:282`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-216d0ba46ce8d321d8fc936d"></a>
## signature

`function` · `datafusion_functions_nested::set_ops::ArrayDistinct::signature` · datafusion-functions-nested 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayDistinct", "path": "ArrayDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [297, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
