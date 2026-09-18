# `datafusion_common::parsers::CompressionTypeVariant`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.parsers.CompressionTypeVariant.json).

<a id="op-afa05b023e49395082f2f46e"></a>
## CompressionTypeVariant

`enum` · `datafusion_common::parsers::CompressionTypeVariant` · datafusion-common 55.1.0

```rust
enum CompressionTypeVariant
```

Source: `src/parsers.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Readable file compression type

<a id="op-41bd3090cd0e5f559294500c"></a>
## BZIP2

`variant` · `datafusion_common::parsers::CompressionTypeVariant::BZIP2` · datafusion-common 55.1.0

```rust
BZIP2
```

Source: `src/parsers.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Bzip2-ed file

<a id="op-0c31edcdb40132ff3beed1cd"></a>
## Err

`assoc_type` · `datafusion_common::parsers::CompressionTypeVariant::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [56, 2], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/parsers.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a011e5f4689686c67bcef03e"></a>
## GZIP

`variant` · `datafusion_common::parsers::CompressionTypeVariant::GZIP` · datafusion-common 55.1.0

```rust
GZIP
```

Source: `src/parsers.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Gzip-ed file

<a id="op-54fc9d7cc8cf9f8063e05160"></a>
## UNCOMPRESSED

`variant` · `datafusion_common::parsers::CompressionTypeVariant::UNCOMPRESSED` · datafusion-common 55.1.0

```rust
UNCOMPRESSED
```

Source: `src/parsers.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Uncompressed file

<a id="op-6efa01b5253846c7a70c81a2"></a>
## XZ

`variant` · `datafusion_common::parsers::CompressionTypeVariant::XZ` · datafusion-common 55.1.0

```rust
XZ
```

Source: `src/parsers.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Xz-ed file (liblzma)

<a id="op-94e80ed69dd6dd4c7d8126ff"></a>
## ZSTD

`variant` · `datafusion_common::parsers::CompressionTypeVariant::ZSTD` · datafusion-common 55.1.0

```rust
ZSTD
```

Source: `src/parsers.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Zstd-ed file,

<a id="op-f4bec4c8b33f82daec51613c"></a>
## clone

`function` · `datafusion_common::parsers::CompressionTypeVariant::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 17], "end": [26, 22], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parsers.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb31450f69a8c0c61dcd2915"></a>
## eq

`function` · `datafusion_common::parsers::CompressionTypeVariant::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &CompressionTypeVariant) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 30], "end": [26, 39], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parsers.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fbd2bf11df96d4125836b4d"></a>
## fmt

`function` · `datafusion_common::parsers::CompressionTypeVariant::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [69, 2], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parsers.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-992114eb6f1eef67bc179152"></a>
## fmt

`function` · `datafusion_common::parsers::CompressionTypeVariant::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parsers.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b1dd37d0acc8b1a57f84bc4"></a>
## from_str

`function` · `datafusion_common::parsers::CompressionTypeVariant::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [56, 2], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/parsers.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd3a9fc6df76cbef621e1b74"></a>
## hash

`function` · `datafusion_common::parsers::CompressionTypeVariant::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 45], "end": [26, 49], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/parsers.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93f52e026bb5f9c80d64dbe8"></a>
## is_compressed

`function` · `datafusion_common::parsers::CompressionTypeVariant::is_compressed` · datafusion-common 55.1.0

```rust
const fn is_compressed(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [75, 2], "filename": "src/parsers.rs"}, "trait": null, "trait_path": null}`

Source: `src/parsers.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08f996d1f97228cef7e84824"></a>
## set

`function` · `datafusion_common::parsers::CompressionTypeVariant::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "crate::parsers::CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2475, 1], "end": [2484, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:2480`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ba47cbc54eeecb3801ac296"></a>
## visit

`function` · `datafusion_common::parsers::CompressionTypeVariant::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CompressionTypeVariant", "path": "crate::parsers::CompressionTypeVariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2475, 1], "end": [2484, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:2476`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
