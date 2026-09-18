# `parquet_variant`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.json).

<a id="op-6469913707cd0592bc49d8dc"></a>
## parquet_variant

`module` · `parquet_variant` · parquet-variant 59.3.0

```rust
mod parquet_variant
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Implementation of [Variant Binary Encoding] from [Apache Parquet].

[Variant Binary Encoding]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md
[Apache Parquet]: https://parquet.apache.org/

## Main APIs
- [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d): Represents a variant value, which can be an object, list, or primitive.
- [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6): For building `Variant` values.

## 🚧 Work In Progress

This crate is under active development and is not yet ready for production use.
If you are interested in helping, you can find more information on the GitHub [Variant issue]

[Variant issue]: https://github.com/apache/arrow-rs/issues/6736
