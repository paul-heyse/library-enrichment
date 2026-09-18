# `arrow_avro::schema::MAX_PREFIX_LEN`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.schema.MAX_PREFIX_LEN.json).

<a id="op-c05d5c815fb5dea37d9746f3"></a>
## MAX_PREFIX_LEN

`constant` · `arrow_avro::schema::MAX_PREFIX_LEN` · arrow-avro 59.3.0

```rust
const MAX_PREFIX_LEN: usize = 34
```

Source: `src/schema.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

The maximum possible length of a prefix.
SHA256 (32) + single-object magic (2)
