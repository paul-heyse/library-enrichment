# `object_store::aws::checksum::Checksum`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.checksum.Checksum.json).

<a id="op-3623194a981c2a8be403e9b3"></a>
## Checksum

`enum` · `object_store::aws::checksum::Checksum` · object_store 0.13.2

```rust
enum Checksum
```

Source: `src/aws/checksum.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Enum representing checksum algorithm supported by S3.

<a id="op-42c51092fd03ecc2cc291b76"></a>
## Err

`assoc_type` · `object_store::aws::checksum::Checksum::Err` · object_store 0.13.2

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::checksum::Checksum", "path": "Checksum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [46, 2], "filename": "src/aws/checksum.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/aws/checksum.rs:38`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4efb4700296ed5e8bd8f3b5"></a>
## Error

`assoc_type` · `object_store::aws::checksum::Checksum::Error` · object_store 0.13.2

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::checksum::Checksum", "path": "Checksum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [54, 2], "filename": "src/aws/checksum.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/aws/checksum.rs:49`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3e96c770ecf9a730cc410ec"></a>
## SHA256

`variant` · `object_store::aws::checksum::Checksum::SHA256` · object_store 0.13.2

```rust
SHA256
```

Source: `src/aws/checksum.rs:26`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

SHA-256 algorithm.

<a id="op-cd7d0645a6a06df362bb7cdd"></a>
## clone

`function` · `object_store::aws::checksum::Checksum::clone` · object_store 0.13.2

```rust
fn clone(&self) -> Checksum
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::checksum::Checksum", "path": "Checksum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 17], "end": [22, 22], "filename": "src/aws/checksum.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aws/checksum.rs:22`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b1a6c6c376bb9c6b86f037b"></a>
## eq

`function` · `object_store::aws::checksum::Checksum::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &Checksum) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::checksum::Checksum", "path": "Checksum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 30], "end": [22, 39], "filename": "src/aws/checksum.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aws/checksum.rs:22`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34ddadcd503d936c7d8c6943"></a>
## fmt

`function` · `object_store::aws::checksum::Checksum::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::checksum::Checksum", "path": "Checksum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [35, 2], "filename": "src/aws/checksum.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/aws/checksum.rs:30`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a54b302dd9eb863b9e7c0290"></a>
## fmt

`function` · `object_store::aws::checksum::Checksum::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::checksum::Checksum", "path": "Checksum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 15], "filename": "src/aws/checksum.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aws/checksum.rs:22`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d87f62056eea55906a4750e"></a>
## from_str

`function` · `object_store::aws::checksum::Checksum::from_str` · object_store 0.13.2

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::checksum::Checksum", "path": "Checksum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [46, 2], "filename": "src/aws/checksum.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/aws/checksum.rs:40`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf2231f2578e9ee8a7eaea30"></a>
## try_from

`function` · `object_store::aws::checksum::Checksum::try_from` · object_store 0.13.2

```rust
fn try_from(value: &String) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::checksum::Checksum", "path": "Checksum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [54, 2], "filename": "src/aws/checksum.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/aws/checksum.rs:51`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
