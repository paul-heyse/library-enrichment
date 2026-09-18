# `datafusion_functions::core::file_row_index::FileRowIndexFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.file_row_index.FileRowIndexFunc.json).

<a id="op-3693ac97957c4259d0e82ca7"></a>
## FileRowIndexFunc

`struct` · `datafusion_functions::core::file_row_index::FileRowIndexFunc` · datafusion-functions 55.1.0

```rust
struct FileRowIndexFunc
```

Source: `src/core/file_row_index.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Scalar UDF implementation for `file_row_index()`.

File sources that can expose per-file row indexes rewrite this placeholder
function into a source-provided physical expression. Direct evaluation
returns an error because there is no file context outside a scan.

<a id="op-8d9d4f7a0ec5018f75074fc5"></a>
## default

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [60, 2], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/file_row_index.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c0ad618afdddb64f152de27"></a>
## documentation

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [96, 2], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/file_row_index.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8a9975fccd247fa30f7a185"></a>
## eq

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &FileRowIndexFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 26], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/file_row_index.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55cc8f99ade4676d8f3fe736"></a>
## fmt

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/file_row_index.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6df684c0ac0e70f0f5b98e35"></a>
## hash

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 32], "end": [51, 36], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/file_row_index.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2fce39588628112413866ac"></a>
## invoke_with_args

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [96, 2], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/file_row_index.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22108ad248923ee2fc249196"></a>
## name

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [96, 2], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/file_row_index.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe7e69dd5db63fc8ccfbc67f"></a>
## new

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [68, 2], "filename": "src/core/file_row_index.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/file_row_index.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6631cf39a5c8c77e912748c5"></a>
## placement

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::placement` · datafusion-functions 55.1.0

```rust
fn placement(&self, _args: &[ExpressionPlacement]) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [96, 2], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/file_row_index.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1057948e605a5b8c91b633b9"></a>
## return_type

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, args: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [96, 2], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/file_row_index.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f085f4ac119dac565c947c9"></a>
## signature

`function` · `datafusion_functions::core::file_row_index::FileRowIndexFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::file_row_index::FileRowIndexFunc", "path": "FileRowIndexFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [96, 2], "filename": "src/core/file_row_index.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/file_row_index.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
