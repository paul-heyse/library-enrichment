# `datafusion_functions_nested::set_ops::ArrayUnion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.set_ops.ArrayUnion.json).

<a id="op-37e51944dac90b73c48207b7"></a>
## ArrayUnion

`struct` · `datafusion_functions_nested::set_ops::ArrayUnion` · datafusion-functions-nested 55.1.0

```rust
struct ArrayUnion
```

Source: `src/set_ops.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4119508ec666a0ecca0d5135"></a>
## aliases

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::aliases` · datafusion-functions-nested 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [150, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db75c3186ab276c65f15e323"></a>
## default

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::default` · datafusion-functions-nested 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [106, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/set_ops.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e032352149f0893fc645fc2"></a>
## documentation

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::documentation` · datafusion-functions-nested 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [150, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76b8b670daa6740ec914f87d"></a>
## eq

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::eq` · datafusion-functions-nested 55.1.0

```rust
fn eq(&self, other: &ArrayUnion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 17], "end": [96, 26], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/set_ops.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e104bb202161f7b2c31a65ba"></a>
## fmt

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 10], "end": [96, 15], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/set_ops.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-804d88ae09e990b94024b020"></a>
## hash

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::hash` · datafusion-functions-nested 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 32], "end": [96, 36], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/set_ops.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbe2feeb59f3da5437d1aa9d"></a>
## invoke_with_args

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::invoke_with_args` · datafusion-functions-nested 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [150, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6be42f677bdf076867e59b4"></a>
## name

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::name` · datafusion-functions-nested 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [150, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d53ac857e8e2685ec5c859d2"></a>
## new

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::new` · datafusion-functions-nested 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [119, 2], "filename": "src/set_ops.rs"}, "trait": null, "trait_path": null}`

Source: `src/set_ops.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3d4fd2ed5930e79805d763d"></a>
## return_type

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::return_type` · datafusion-functions-nested 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [150, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef65ecf065960a0124911743"></a>
## signature

`function` · `datafusion_functions_nested::set_ops::ArrayUnion::signature` · datafusion-functions-nested 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::set_ops::ArrayUnion", "path": "ArrayUnion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [150, 2], "filename": "src/set_ops.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/set_ops.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
