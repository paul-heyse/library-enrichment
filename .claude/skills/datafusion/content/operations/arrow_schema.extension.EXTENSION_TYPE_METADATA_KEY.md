# `arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.EXTENSION_TYPE_METADATA_KEY.json).

<a id="op-c38aeda07fc1041ad3e5ea8f"></a>
## EXTENSION_TYPE_METADATA_KEY

`constant` · `arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY` · arrow-schema 59.3.0

```rust
const EXTENSION_TYPE_METADATA_KEY: &str = "ARROW:extension:metadata"
```

Source: `src/extension/mod.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The metadata key for a serialized representation of the [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f)
necessary to reconstruct the custom type.
