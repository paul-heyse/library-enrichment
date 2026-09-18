# `arrow_ipc::writer::IpcWriteOptions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.IpcWriteOptions.json).

<a id="op-7958df777a3d4faa0bb68d5a"></a>
## IpcWriteOptions

`struct` · `arrow_ipc::writer::IpcWriteOptions` · arrow-ipc 59.3.0

```rust
struct IpcWriteOptions
```

Source: `src/writer.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

IPC write options used to control the behaviour of the [`IpcDataGenerator`](../operations/arrow_ipc.writer.IpcDataGenerator.md#op-26ea3c515d0494f20ce6fa88)

<a id="op-346e7f5314d5d65cae93df44"></a>
## clone

`function` · `arrow_ipc::writer::IpcWriteOptions::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> IpcWriteOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcWriteOptions", "path": "IpcWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 22], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d15329f740499ae492e6374e"></a>
## default

`function` · `arrow_ipc::writer::IpcWriteOptions::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcWriteOptions", "path": "IpcWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [524, 1], "end": [535, 2], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer.rs:525`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df703512e8ed8205600dfc5e"></a>
## fmt

`function` · `arrow_ipc::writer::IpcWriteOptions::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcWriteOptions", "path": "IpcWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b50af71375dd4d6449b849e"></a>
## try_new

`function` · `arrow_ipc::writer::IpcWriteOptions::try_new` · arrow-ipc 59.3.0

```rust
fn try_new(alignment: usize, write_legacy_ipc_format: bool, metadata_version: MetadataVersion) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcWriteOptions", "path": "IpcWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [522, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:467`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create IpcWriteOptions, checking for incompatible settings

<a id="op-30ff8a7da150de9240b5196b"></a>
## try_with_compression

`function` · `arrow_ipc::writer::IpcWriteOptions::try_with_compression` · arrow-ipc 59.3.0

```rust
fn try_with_compression(self, batch_compression_type: Option<CompressionType>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcWriteOptions", "path": "IpcWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [522, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:394`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Configures compression when writing IPC files.

Will result in a runtime error if the corresponding feature
is not enabled

<a id="op-fe855acf3a0f9a624e6310bd"></a>
## try_with_compression_level

`function` · `arrow_ipc::writer::IpcWriteOptions::try_with_compression_level` · arrow-ipc 59.3.0

```rust
fn try_with_compression_level(self, batch_compression_level: Option<i32>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcWriteOptions", "path": "IpcWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [522, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:414`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Configures the compression level used when writing compressed IPC batches.

Compression levels require metadata V5 or newer and are currently only
supported for ZSTD compression.

<a id="op-d72c302c8db725c6c4fb833a"></a>
## with_dictionary_handling

`function` · `arrow_ipc::writer::IpcWriteOptions::with_dictionary_handling` · arrow-ipc 59.3.0

```rust
fn with_dictionary_handling(self, dictionary_handling: DictionaryHandling) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::IpcWriteOptions", "path": "IpcWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [522, 2], "filename": "src/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer.rs:518`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Configure how dictionaries are handled in IPC messages
