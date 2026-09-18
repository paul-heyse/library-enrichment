# `datafusion_functions_nested::sort::ArraySort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.sort.ArraySort.json).

<a id="op-b5991e8f5d624c72ee0116c3"></a>
## ArraySort

`struct` · `datafusion_functions_nested::sort::ArraySort` · datafusion-functions-nested 55.1.0

```rust
struct ArraySort
```

Source: `src/sort.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

Implementation of `array_sort` function

`array_sort` sorts the elements of an array

# Example

`array_sort([3, 1, 2])` returns `[1, 2, 3]`

<a id="op-f81ba63ec49f31763099a9ca"></a>
## aliases

`function` · `datafusion_functions_nested::sort::ArraySort::aliases` · datafusion-functions-nested 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [154, 2], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/sort.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b149cb4410a1bf9a0d0d50dd"></a>
## default

`function` · `datafusion_functions_nested::sort::ArraySort::default` · datafusion-functions-nested 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [96, 2], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sort.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bb181ad1ff8bf69c807e3d5"></a>
## documentation

`function` · `datafusion_functions_nested::sort::ArraySort::documentation` · datafusion-functions-nested 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [154, 2], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/sort.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d2cc65937a7a59cd9a425d8"></a>
## eq

`function` · `datafusion_functions_nested::sort::ArraySort::eq` · datafusion-functions-nested 55.1.0

```rust
fn eq(&self, other: &ArraySort) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 17], "end": [86, 26], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sort.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd7f5b63d5cc55b3b3a83fe2"></a>
## fmt

`function` · `datafusion_functions_nested::sort::ArraySort::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 10], "end": [86, 15], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c0dbb7a31ebdbb877862a2e"></a>
## hash

`function` · `datafusion_functions_nested::sort::ArraySort::hash` · datafusion-functions-nested 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 32], "end": [86, 36], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sort.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a61354f90763d97f346fe424"></a>
## invoke_with_args

`function` · `datafusion_functions_nested::sort::ArraySort::invoke_with_args` · datafusion-functions-nested 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [154, 2], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/sort.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bee4cf448753415131c81d5b"></a>
## name

`function` · `datafusion_functions_nested::sort::ArraySort::name` · datafusion-functions-nested 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [154, 2], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/sort.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11c37261322023438c15181a"></a>
## new

`function` · `datafusion_functions_nested::sort::ArraySort::new` · datafusion-functions-nested 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [128, 2], "filename": "src/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a23d5e01782e116b247591d0"></a>
## return_type

`function` · `datafusion_functions_nested::sort::ArraySort::return_type` · datafusion-functions-nested 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [154, 2], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/sort.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cc56671df24fe0e6001fcf4"></a>
## signature

`function` · `datafusion_functions_nested::sort::ArraySort::signature` · datafusion-functions-nested 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::sort::ArraySort", "path": "ArraySort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [154, 2], "filename": "src/sort.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/sort.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
