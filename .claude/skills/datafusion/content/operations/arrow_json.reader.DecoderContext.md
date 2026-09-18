# `arrow_json::reader::DecoderContext`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.DecoderContext.json).

<a id="op-2bddba5850844d2c417c2e0c"></a>
## DecoderContext

`struct` · `arrow_json::reader::DecoderContext` · arrow-json 59.3.0

```rust
struct DecoderContext
```

Source: `src/reader/mod.rs:716`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Context for decoder creation, containing configuration.

This context is passed through the decoder creation process and contains
all the configuration needed to create decoders recursively.

<a id="op-aa1cc0a622b9da0784e85d13"></a>
## coerce_primitive

`function` · `arrow_json::reader::DecoderContext::coerce_primitive` · arrow-json 59.3.0

```rust
fn coerce_primitive(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::DecoderContext", "path": "DecoderContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [727, 1], "end": [759, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:729`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns whether to coerce primitive types (e.g., number to string)

<a id="op-91a2e4eae0fda9ac99e95110"></a>
## ignore_type_conflicts

`function` · `arrow_json::reader::DecoderContext::ignore_type_conflicts` · arrow-json 59.3.0

```rust
fn ignore_type_conflicts(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::DecoderContext", "path": "DecoderContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [727, 1], "end": [759, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:744`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns whether to treat columns with incompatible types as missing (i.e. NULL)

<a id="op-92ea4a9140acaf7a2b05cef6"></a>
## strict_mode

`function` · `arrow_json::reader::DecoderContext::strict_mode` · arrow-json 59.3.0

```rust
fn strict_mode(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::DecoderContext", "path": "DecoderContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [727, 1], "end": [759, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:734`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns whether to validate struct fields strictly

<a id="op-ed1c55da13ca60a4a0d8a949"></a>
## struct_mode

`function` · `arrow_json::reader::DecoderContext::struct_mode` · arrow-json 59.3.0

```rust
fn struct_mode(&self) -> StructMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::DecoderContext", "path": "DecoderContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [727, 1], "end": [759, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:739`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns how to decode struct fields
