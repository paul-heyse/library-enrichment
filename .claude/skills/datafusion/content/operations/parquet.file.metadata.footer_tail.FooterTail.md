# `parquet::file::metadata::footer_tail::FooterTail`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.footer_tail.FooterTail.json).

<a id="op-89c094b19f6f33c809873abe"></a>
## FooterTail

`struct` · `parquet::file::metadata::footer_tail::FooterTail` · parquet 59.3.0

```rust
struct FooterTail
```

Source: `src/file/metadata/footer_tail.rs:53`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parsed Parquet footer tail (last 8 bytes of a Parquet file)

There are 8 bytes at the end of the Parquet footer with the following layout:
* 4 bytes for the metadata length
* 4 bytes for the magic bytes 'PAR1' or 'PARE' (encrypted footer)

```text
+-----+------------------+
| len | 'PAR1' or 'PARE' |
+-----+------------------+
```

# Examples
```
# use parquet::file::metadata::FooterTail;
// a non encrypted footer with 28 bytes of metadata
let last_8_bytes: [u8; 8] = [0x1C, 0x00, 0x00, 0x00, b'P', b'A', b'R', b'1'];
let footer_tail = FooterTail::try_from(last_8_bytes).unwrap();
assert_eq!(footer_tail.metadata_length(), 28);
assert_eq!(footer_tail.is_encrypted_footer(), false);
```

```
# use parquet::file::metadata::FooterTail;
// an encrypted footer with 512 bytes of metadata
let last_8_bytes = vec![0x00, 0x02, 0x00, 0x00, b'P', b'A', b'R', b'E'];
let footer_tail = FooterTail::try_from(&last_8_bytes[..]).unwrap();
assert_eq!(footer_tail.metadata_length(), 512);
assert_eq!(footer_tail.is_encrypted_footer(), true);
```


<a id="op-1881acba5714cda77ac55961"></a>
## Error

`assoc_type` · `parquet::file::metadata::footer_tail::FooterTail::Error` · parquet 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [111, 2], "filename": "src/file/metadata/footer_tail.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/metadata/footer_tail.rs:99`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b1973e60f1288c6650c19bd"></a>
## Error

`assoc_type` · `parquet::file::metadata::footer_tail::FooterTail::Error` · parquet 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [96, 2], "filename": "src/file/metadata/footer_tail.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"array": {"len": "8", "type": {"primitive": "u8"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/metadata/footer_tail.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3e21da90eee83decdbda38f"></a>
## clone

`function` · `parquet::file::metadata::footer_tail::FooterTail::clone` · parquet 59.3.0

```rust
fn clone(&self) -> FooterTail
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 17], "end": [52, 22], "filename": "src/file/metadata/footer_tail.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/metadata/footer_tail.rs:52`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e68474746d95a97ca2d9273d"></a>
## eq

`function` · `parquet::file::metadata::footer_tail::FooterTail::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &FooterTail) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 30], "end": [52, 39], "filename": "src/file/metadata/footer_tail.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/metadata/footer_tail.rs:52`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b71c36a2c295686b8af053e"></a>
## fmt

`function` · `parquet::file::metadata::footer_tail::FooterTail::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "src/file/metadata/footer_tail.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/footer_tail.rs:52`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dbb86249896c3fe4e9b5be1"></a>
## is_encrypted_footer

`function` · `parquet::file::metadata::footer_tail::FooterTail::is_encrypted_footer` · parquet 59.3.0

```rust
fn is_encrypted_footer(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [88, 2], "filename": "src/file/metadata/footer_tail.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/footer_tail.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Whether the footer metadata is encrypted

<a id="op-d05a0dc0383e6050a796f301"></a>
## metadata_length

`function` · `parquet::file::metadata::footer_tail::FooterTail::metadata_length` · parquet 59.3.0

```rust
fn metadata_length(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [88, 2], "filename": "src/file/metadata/footer_tail.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/footer_tail.rs:80`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The length of the footer metadata in bytes

<a id="op-7c095d43eae81483503c3b43"></a>
## try_from

`function` · `parquet::file::metadata::footer_tail::FooterTail::try_from` · parquet 59.3.0

```rust
fn try_from(value: &[u8]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [111, 2], "filename": "src/file/metadata/footer_tail.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/metadata/footer_tail.rs:101`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eea5b4f153fcb5dc6e6f0d4e"></a>
## try_from

`function` · `parquet::file::metadata::footer_tail::FooterTail::try_from` · parquet 59.3.0

```rust
fn try_from(value: [u8; 8]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [96, 2], "filename": "src/file/metadata/footer_tail.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"array": {"len": "8", "type": {"primitive": "u8"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/file/metadata/footer_tail.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce64d9759accf530bafd2a39"></a>
## try_new

`function` · `parquet::file::metadata::footer_tail::FooterTail::try_new` · parquet 59.3.0

```rust
fn try_new(slice: &[u8; 8]) -> Result<FooterTail>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::footer_tail::FooterTail", "path": "FooterTail"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [88, 2], "filename": "src/file/metadata/footer_tail.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/footer_tail.rs:60`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to decode the footer tail from the given 8 bytes
