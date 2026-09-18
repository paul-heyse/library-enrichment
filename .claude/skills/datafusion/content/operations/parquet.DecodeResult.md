# `parquet::DecodeResult`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.DecodeResult.json).

<a id="op-672a46d318916cbad7299962"></a>
## DecodeResult

`enum` · `parquet::DecodeResult` · parquet 59.3.0

```rust
enum DecodeResult<T: Debug>
```

Source: `src/lib.rs:218`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

What data is needed to read the next item from a decoder.

This is used to communicate between the decoder and the caller
to indicate what data is needed next, or what the result of decoding is.

<a id="op-3c7f0e9903175b57a81366a4"></a>
## Data

`variant` · `parquet::DecodeResult::Data` · parquet 59.3.0

```rust
Data
```

Source: `src/lib.rs:223`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The decoder produced an output item

<a id="op-02bc7c2cc99638391ccb2d5b"></a>
## Finished

`variant` · `parquet::DecodeResult::Finished` · parquet 59.3.0

```rust
Finished
```

Source: `src/lib.rs:225`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The decoder finished processing

<a id="op-d08060afa7628b341aa4497a"></a>
## NeedsData

`variant` · `parquet::DecodeResult::NeedsData` · parquet 59.3.0

```rust
NeedsData
```

Source: `src/lib.rs:221`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The ranges of data necessary to proceed

<a id="op-53b6ca6f26565de23a8c6c78"></a>
## fmt

`function` · `parquet::DecodeResult::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::DecodeResult", "path": "DecodeResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 10], "end": [217, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:217`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
