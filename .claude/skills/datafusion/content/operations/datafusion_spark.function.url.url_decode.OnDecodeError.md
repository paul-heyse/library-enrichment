# `datafusion_spark::function::url::url_decode::OnDecodeError`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.url.url_decode.OnDecodeError.json).

<a id="op-e335abb25835e94a33778abb"></a>
## OnDecodeError

`enum` · `datafusion_spark::function::url::url_decode::OnDecodeError` · datafusion-spark 55.1.0

```rust
enum OnDecodeError
```

Source: `src/function/url/url_decode.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

How [`spark_handled_url_decode`](../operations/datafusion_spark.function.url.url_decode.spark_handled_url_decode.md#op-3f6836ea63b4dd9538312e63) reacts to a malformed input value.

<a id="op-401f3b978d7808517416d58c"></a>
## Fail

`variant` · `datafusion_spark::function::url::url_decode::OnDecodeError::Fail` · datafusion-spark 55.1.0

```rust
Fail
```

Source: `src/function/url/url_decode.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Propagate the error, as `url_decode` does.

<a id="op-16fa23df61887ce86f76ef0b"></a>
## Null

`variant` · `datafusion_spark::function::url::url_decode::OnDecodeError::Null` · datafusion-spark 55.1.0

```rust
Null
```

Source: `src/function/url/url_decode.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Return NULL for that row, as `try_url_decode` does.

<a id="op-eb38e88b9ab4395023b96568"></a>
## clone

`function` · `datafusion_spark::function::url::url_decode::OnDecodeError::clone` · datafusion-spark 55.1.0

```rust
fn clone(&self) -> OnDecodeError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::url_decode::OnDecodeError", "path": "OnDecodeError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 10], "end": [168, 15], "filename": "src/function/url/url_decode.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/function/url/url_decode.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97f58c64c5c9d2fa1c3e8ba5"></a>
## eq

`function` · `datafusion_spark::function::url::url_decode::OnDecodeError::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &OnDecodeError) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::url_decode::OnDecodeError", "path": "OnDecodeError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 30], "end": [168, 39], "filename": "src/function/url/url_decode.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/url/url_decode.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0055023c7a47ee92827fdfd1"></a>
## fmt

`function` · `datafusion_spark::function::url::url_decode::OnDecodeError::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::url::url_decode::OnDecodeError", "path": "OnDecodeError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 23], "end": [168, 28], "filename": "src/function/url/url_decode.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/url/url_decode.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
