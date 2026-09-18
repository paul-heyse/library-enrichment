# `object_store::gcp::GcpCredentialProvider`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.GcpCredentialProvider.json).

<a id="op-db800b581a1f03156ba6e5a6"></a>
## GcpCredentialProvider

`type_alias` · `object_store::gcp::GcpCredentialProvider` · object_store 0.13.2

```rust
type GcpCredentialProvider = std::sync::Arc<dyn CredentialProvider<Credential = GcpCredential>>
```

Source: `src/gcp/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

[`CredentialProvider`](../operations/object_store.client.CredentialProvider.md#op-76004bbb5f119134cca4253a) for [`GoogleCloudStorage`](../operations/object_store.gcp.GoogleCloudStorage.md#op-4b3bbb82170d36762c713ba6)
