# `datafusion_datasource::decoder::DeserializerOutput`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.decoder.DeserializerOutput.json).

<a id="op-1bb4ce5e238ba05b311befdd"></a>
## DeserializerOutput

`enum` · `datafusion_datasource::decoder::DeserializerOutput` · datafusion-datasource 55.1.0

```rust
enum DeserializerOutput
```

Source: `src/decoder.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Possible outputs of a [`BatchDeserializer`](../operations/datafusion_datasource.decoder.BatchDeserializer.md#op-d0c1e759d47bbcbfde2d9707).

<a id="op-9140a8eaef285e592c613941"></a>
## InputExhausted

`variant` · `datafusion_datasource::decoder::DeserializerOutput::InputExhausted` · datafusion-datasource 55.1.0

```rust
InputExhausted
```

Source: `src/decoder.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The input data has been exhausted.

<a id="op-d5dc092204b9e28fef7f9eba"></a>
## RecordBatch

`variant` · `datafusion_datasource::decoder::DeserializerOutput::RecordBatch` · datafusion-datasource 55.1.0

```rust
RecordBatch
```

Source: `src/decoder.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A successfully deserialized [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

<a id="op-07e461f8c804594a6dd20fd7"></a>
## RequiresMoreData

`variant` · `datafusion_datasource::decoder::DeserializerOutput::RequiresMoreData` · datafusion-datasource 55.1.0

```rust
RequiresMoreData
```

Source: `src/decoder.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The deserializer requires more data to make progress.

<a id="op-16fa77309046c459ddd18a67"></a>
## eq

`function` · `datafusion_datasource::decoder::DeserializerOutput::eq` · datafusion-datasource 55.1.0

```rust
fn eq(&self, other: &DeserializerOutput) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::decoder::DeserializerOutput", "path": "DeserializerOutput"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 17], "end": [35, 26], "filename": "src/decoder.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/decoder.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ff70eb6c254c3942ef684e8"></a>
## fmt

`function` · `datafusion_datasource::decoder::DeserializerOutput::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::decoder::DeserializerOutput", "path": "DeserializerOutput"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/decoder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decoder.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
