# `arrow_avro::compression`

Crate `arrow-avro` · 2 public items · structured records in [`model/arrow_avro.compression.json`](../model/arrow_avro.compression.json)

## CODEC_METADATA_KEY

`constant` · `arrow_avro::compression::CODEC_METADATA_KEY`

```rust
const CODEC_METADATA_KEY: &str = "avro.codec"
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.compression.CODEC_METADATA_KEY.md).


The metadata key used for storing the JSON encoded [`CompressionCodec`]

---

## CompressionCodec

`enum` · `arrow_avro::compression::CompressionCodec`

```rust
enum CompressionCodec
```

**Variants**: `Deflate`, `Snappy`, `ZStandard`, `Bzip2`, `Xz`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/arrow_avro.compression.CompressionCodec.md).


Supported compression codecs for Avro data

Avro supports multiple compression formats for data blocks.
This enum represents the compression codecs available in this implementation.

---
