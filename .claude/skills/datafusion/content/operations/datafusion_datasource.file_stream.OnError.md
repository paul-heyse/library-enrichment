# `datafusion_datasource::file_stream::OnError`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_stream.OnError.json).

<a id="op-c1681855f4dc46368c926ae0"></a>
## OnError

`enum` · `datafusion_datasource::file_stream::OnError` · datafusion-datasource 55.1.0

```rust
enum OnError
```

Source: `src/file_stream/mod.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Describes the behavior of the `FileStream` if file opening or scanning fails

<a id="op-5c640c78b3c0bf4f4797669f"></a>
## Fail

`variant` · `datafusion_datasource::file_stream::OnError::Fail` · datafusion-datasource 55.1.0

```rust
Fail
```

Source: `src/file_stream/mod.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Fail the entire stream and return the underlying error

<a id="op-fd242e0c5f2b8331049a1b5e"></a>
## Skip

`variant` · `datafusion_datasource::file_stream::OnError::Skip` · datafusion-datasource 55.1.0

```rust
Skip
```

Source: `src/file_stream/mod.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Continue scanning, ignoring the failed file

<a id="op-d3c628bd87d1147d89c680de"></a>
## default

`function` · `datafusion_datasource::file_stream::OnError::default` · datafusion-datasource 55.1.0

```rust
fn default() -> OnError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::OnError", "path": "OnError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 10], "end": [141, 17], "filename": "src/file_stream/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_stream/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
