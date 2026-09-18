# `datafusion_functions_nested::concat::ArrayAppend`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.concat.ArrayAppend.json).

<a id="op-89dffee3cfdd8b477e0e42b6"></a>
## ArrayAppend

`struct` · `datafusion_functions_nested::concat::ArrayAppend` · datafusion-functions-nested 55.1.0

```rust
struct ArrayAppend
```

Source: `src/concat.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8de06572c2a796a8331fa11e"></a>
## aliases

`function` · `datafusion_functions_nested::concat::ArrayAppend::aliases` · datafusion-functions-nested 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [139, 2], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/concat.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d7e78e7830c391d0a2e6170"></a>
## default

`function` · `datafusion_functions_nested::concat::ArrayAppend::default` · datafusion-functions-nested 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [86, 2], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/concat.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5bc36b2ad9ac1f84a9a46c2"></a>
## documentation

`function` · `datafusion_functions_nested::concat::ArrayAppend::documentation` · datafusion-functions-nested 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [139, 2], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/concat.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c601b8b8ddfd44a0e3c4accd"></a>
## eq

`function` · `datafusion_functions_nested::concat::ArrayAppend::eq` · datafusion-functions-nested 55.1.0

```rust
fn eq(&self, other: &ArrayAppend) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 17], "end": [76, 26], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/concat.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3ed9549e017f4287faf5f13"></a>
## fmt

`function` · `datafusion_functions_nested::concat::ArrayAppend::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 15], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/concat.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57256d574f98eaf0d488cf79"></a>
## hash

`function` · `datafusion_functions_nested::concat::ArrayAppend::hash` · datafusion-functions-nested 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 32], "end": [76, 36], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/concat.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b37f1cbc054598c14657d5a"></a>
## invoke_with_args

`function` · `datafusion_functions_nested::concat::ArrayAppend::invoke_with_args` · datafusion-functions-nested 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [139, 2], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/concat.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dd6ffba91f95d1a800a20c6"></a>
## name

`function` · `datafusion_functions_nested::concat::ArrayAppend::name` · datafusion-functions-nested 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [139, 2], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/concat.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-625f46f8777f6bd841885394"></a>
## new

`function` · `datafusion_functions_nested::concat::ArrayAppend::new` · datafusion-functions-nested 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [99, 2], "filename": "src/concat.rs"}, "trait": null, "trait_path": null}`

Source: `src/concat.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47f12ee58e4112b156c2f9da"></a>
## return_field_from_args

`function` · `datafusion_functions_nested::concat::ArrayAppend::return_field_from_args` · datafusion-functions-nested 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [139, 2], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/concat.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f08a3b310e99c4e263df6e2"></a>
## return_type

`function` · `datafusion_functions_nested::concat::ArrayAppend::return_type` · datafusion-functions-nested 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [139, 2], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/concat.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99af74f5e1d7a38dee505698"></a>
## signature

`function` · `datafusion_functions_nested::concat::ArrayAppend::signature` · datafusion-functions-nested 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::concat::ArrayAppend", "path": "ArrayAppend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [139, 2], "filename": "src/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/concat.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
