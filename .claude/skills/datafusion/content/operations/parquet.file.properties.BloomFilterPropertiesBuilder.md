# `parquet::file::properties::BloomFilterPropertiesBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.BloomFilterPropertiesBuilder.json).

<a id="op-32e29ff964a10174bac103fc"></a>
## BloomFilterPropertiesBuilder

`struct` · `parquet::file::properties::BloomFilterPropertiesBuilder` · parquet 59.3.0

```rust
struct BloomFilterPropertiesBuilder
```

Source: `src/file/properties.rs:1505`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builder for [`BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c).

Use [`BloomFilterProperties::builder`](../operations/parquet.file.properties.BloomFilterProperties.md#op-fc030a452ac061a92476a2ac) or [`BloomFilterPropertiesBuilder::new`](../operations/parquet.file.properties.BloomFilterPropertiesBuilder.md#op-4781ebbf91ebcf839a4022b9)
as the entry point.

<a id="op-0900083ed81d7f54bdbb99cd"></a>
## build

`function` · `parquet::file::properties::BloomFilterPropertiesBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> BloomFilterProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPropertiesBuilder", "path": "BloomFilterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1510, 1], "end": [1565, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1553`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builds [`BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c).

Panics if the configured `fpp` is not in `(0.0, 1.0)` exclusive.
Use [`Self::try_build`](../operations/parquet.file.properties.BloomFilterPropertiesBuilder.md#op-75e80257b9acb25ac24cdae4) for a non-panicking alternative.

<a id="op-06402cd2b8a5fdc121676075"></a>
## clone

`function` · `parquet::file::properties::BloomFilterPropertiesBuilder::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BloomFilterPropertiesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPropertiesBuilder", "path": "BloomFilterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 17], "end": [1504, 22], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/properties.rs:1504`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c753b2449f84699608741ced"></a>
## default

`function` · `parquet::file::properties::BloomFilterPropertiesBuilder::default` · parquet 59.3.0

```rust
fn default() -> BloomFilterPropertiesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPropertiesBuilder", "path": "BloomFilterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 24], "end": [1504, 31], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/properties.rs:1504`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-563cebd9bb80d0ddaaafe8e2"></a>
## fmt

`function` · `parquet::file::properties::BloomFilterPropertiesBuilder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPropertiesBuilder", "path": "BloomFilterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1504, 10], "end": [1504, 15], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/properties.rs:1504`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4781ebbf91ebcf839a4022b9"></a>
## new

`function` · `parquet::file::properties::BloomFilterPropertiesBuilder::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPropertiesBuilder", "path": "BloomFilterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1510, 1], "end": [1565, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1514`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a new builder with no fields set.

Equivalent to [`BloomFilterProperties::builder`](../operations/parquet.file.properties.BloomFilterProperties.md#op-fc030a452ac061a92476a2ac).

<a id="op-75e80257b9acb25ac24cdae4"></a>
## try_build

`function` · `parquet::file::properties::BloomFilterPropertiesBuilder::try_build` · parquet 59.3.0

```rust
fn try_build(self) -> Result<BloomFilterProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPropertiesBuilder", "path": "BloomFilterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1510, 1], "end": [1565, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1559`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builds [`BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c), returning an error instead of
panicking when the configured `fpp` is not in `(0.0, 1.0)` exclusive.

<a id="op-c6add2d2f498e9834d967d1c"></a>
## with_fpp

`function` · `parquet::file::properties::BloomFilterPropertiesBuilder::with_fpp` · parquet 59.3.0

```rust
fn with_fpp(self, fpp: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPropertiesBuilder", "path": "BloomFilterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1510, 1], "end": [1565, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1523`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the target false positive probability.

The value must be in `(0.0, 1.0)` exclusively; this is validated at
build time by [`Self::build`](../operations/parquet.file.properties.BloomFilterPropertiesBuilder.md#op-0900083ed81d7f54bdbb99cd) / [`Self::try_build`](../operations/parquet.file.properties.BloomFilterPropertiesBuilder.md#op-75e80257b9acb25ac24cdae4). When unset, the
default is `0.05` (5%, see [`DEFAULT_BLOOM_FILTER_FPP`](../operations/parquet.file.properties.DEFAULT_BLOOM_FILTER_FPP.md#op-6a638a0205a9f6e75faeba63)).

<a id="op-989f3e0c7e845b1f6e798172"></a>
## with_max_ndv

`function` · `parquet::file::properties::BloomFilterPropertiesBuilder::with_max_ndv` · parquet 59.3.0

```rust
fn with_max_ndv(self, ndv: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPropertiesBuilder", "path": "BloomFilterPropertiesBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1510, 1], "end": [1565, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1544`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets the maximum expected number of distinct values used to size the
bloom filter before folding.

When unset, the default is `1_048_576` (see [`DEFAULT_BLOOM_FILTER_NDV`](../operations/parquet.file.properties.DEFAULT_BLOOM_FILTER_NDV.md#op-9c8e92e4f942656d39a72c51)),
which at the default fpp of 5% reserves roughly 1 MiB per column for the
filter bitset, derived as follows:

```text
ndv = 1,048,576, fpp = 0.05
  0.05^(1/8)                         ≈ 0.6877
  1 - 0.6877                         ≈ 0.3123
  ln(0.3123)                         ≈ -1.164
  num_bits = -8 * 1,048,576 / -1.164 ≈ 7,206,000 bits
                                     ≈   900,750 bytes (~900 KB)
  next_power_of_two(900 KB)          = 1 MiB (= 1,048,576 bytes)
```
