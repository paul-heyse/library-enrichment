# `datafusion_functions_nested::reverse::ArrayReverse`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.reverse.ArrayReverse.json).

<a id="op-0bd7ad50ae50a3521fd32e21"></a>
## ArrayReverse

`struct` · `datafusion_functions_nested::reverse::ArrayReverse` · datafusion-functions-nested 55.1.0

```rust
struct ArrayReverse
```

Source: `src/reverse.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a962c5b2aa29d3bd48dad5e"></a>
## aliases

`function` · `datafusion_functions_nested::reverse::ArrayReverse::aliases` · datafusion-functions-nested 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [114, 2], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/reverse.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d5e7feacc2160566e14ca4e"></a>
## default

`function` · `datafusion_functions_nested::reverse::ArrayReverse::default` · datafusion-functions-nested 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [79, 2], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/reverse.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-493290e4cd5ea092437e0854"></a>
## documentation

`function` · `datafusion_functions_nested::reverse::ArrayReverse::documentation` · datafusion-functions-nested 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [114, 2], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/reverse.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59b970c2ebf3ef630ced6cba"></a>
## eq

`function` · `datafusion_functions_nested::reverse::ArrayReverse::eq` · datafusion-functions-nested 55.1.0

```rust
fn eq(&self, other: &ArrayReverse) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 17], "end": [69, 26], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/reverse.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ae1d8244c7d0b6063bf3e62"></a>
## fmt

`function` · `datafusion_functions_nested::reverse::ArrayReverse::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 10], "end": [69, 15], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reverse.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3294a998ae2ac3a601b947e"></a>
## hash

`function` · `datafusion_functions_nested::reverse::ArrayReverse::hash` · datafusion-functions-nested 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 32], "end": [69, 36], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/reverse.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bba211c62fc03f187ef7d14"></a>
## invoke_with_args

`function` · `datafusion_functions_nested::reverse::ArrayReverse::invoke_with_args` · datafusion-functions-nested 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [114, 2], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/reverse.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8401c2404652f2029a026ebb"></a>
## name

`function` · `datafusion_functions_nested::reverse::ArrayReverse::name` · datafusion-functions-nested 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [114, 2], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/reverse.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7870da6533c053c51d80822d"></a>
## new

`function` · `datafusion_functions_nested::reverse::ArrayReverse::new` · datafusion-functions-nested 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [88, 2], "filename": "src/reverse.rs"}, "trait": null, "trait_path": null}`

Source: `src/reverse.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f34bedbad10fa2ed919213d7"></a>
## return_type

`function` · `datafusion_functions_nested::reverse::ArrayReverse::return_type` · datafusion-functions-nested 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [114, 2], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/reverse.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-350aa8e3dd72f55af9117b43"></a>
## signature

`function` · `datafusion_functions_nested::reverse::ArrayReverse::signature` · datafusion-functions-nested 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::reverse::ArrayReverse", "path": "ArrayReverse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [114, 2], "filename": "src/reverse.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/reverse.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
