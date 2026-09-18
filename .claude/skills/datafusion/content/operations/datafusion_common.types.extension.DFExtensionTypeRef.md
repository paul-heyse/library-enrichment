# `datafusion_common::types::extension::DFExtensionTypeRef`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.extension.DFExtensionTypeRef.json).

<a id="op-24969730b079964af558647f"></a>
## DFExtensionTypeRef

`type_alias` · `datafusion_common::types::extension::DFExtensionTypeRef` · datafusion-common 55.1.0

```rust
type DFExtensionTypeRef = std::sync::Arc<dyn DFExtensionType>
```

Source: `src/types/extension.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A cheaply cloneable pointer to a [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300).
