# `datafusion_expr::async_udf::AsyncScalarUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.async_udf.AsyncScalarUDF.json).

<a id="op-a0ba9951c2f69da915facf1a"></a>
## AsyncScalarUDF

`struct` · `datafusion_expr::async_udf::AsyncScalarUDF` · datafusion-expr 55.1.0

```rust
struct AsyncScalarUDF
```

Source: `src/async_udf.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A scalar UDF that must be invoked using async methods

Note this is not meant to be used directly, but is meant to be an implementation detail
for AsyncUDFImpl.

<a id="op-2d8f4467fb62468ef789faf4"></a>
## eq

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [68, 2], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/async_udf.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-396d6a0871d6b91e80f11734"></a>
## fmt

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [130, 2], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/async_udf.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1bc9923b130a31c3bf2fa4f"></a>
## fmt

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 10], "end": [57, 15], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/async_udf.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed1e82e37839a00072666a94"></a>
## hash

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [77, 2], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/async_udf.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46c6aca75514ca2e5785febb"></a>
## ideal_batch_size

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::ideal_batch_size` · datafusion-expr 55.1.0

```rust
fn ideal_batch_size(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [102, 2], "filename": "src/async_udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_udf.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The ideal batch size for this function

<a id="op-f9fc04ed413e6a681e05afd5"></a>
## into_scalar_udf

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::into_scalar_udf` · datafusion-expr 55.1.0

```rust
fn into_scalar_udf(self) -> ScalarUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [102, 2], "filename": "src/async_udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_udf.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Turn this AsyncUDF into a ScalarUDF, suitable for
registering in the context

<a id="op-cb07866b0653e61fc90f5b97"></a>
## invoke_async_with_args

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::invoke_async_with_args` · datafusion-expr 55.1.0

```rust
async fn invoke_async_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [102, 2], "filename": "src/async_udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_udf.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke the function asynchronously with the async arguments

<a id="op-5b257d31a92d2f7b1c90146a"></a>
## invoke_with_args

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::invoke_with_args` · datafusion-expr 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [124, 2], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/async_udf.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-139682f325587cfaec5ffb1f"></a>
## name

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [124, 2], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/async_udf.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5eebc5608629b09c230d757d"></a>
## new

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::new` · datafusion-expr 55.1.0

```rust
fn new(inner: Arc<dyn AsyncScalarUDFImpl>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [102, 2], "filename": "src/async_udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_udf.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dd45d3109ac3cbf85f9ecaf"></a>
## return_field_from_args

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::return_field_from_args` · datafusion-expr 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [124, 2], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/async_udf.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be7e1b21c7c840b1e93d834d"></a>
## return_type

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [124, 2], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/async_udf.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edeab9f7812e16830cd58631"></a>
## signature

`function` · `datafusion_expr::async_udf::AsyncScalarUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::async_udf::AsyncScalarUDF", "path": "AsyncScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [124, 2], "filename": "src/async_udf.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/async_udf.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
