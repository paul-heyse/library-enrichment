# `object_store::gcp::GcpSigningCredentialProvider`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.GcpSigningCredentialProvider.json).

<a id="op-ebbf5f6d923926254ad656ac"></a>
## GcpSigningCredentialProvider

`type_alias` · `object_store::gcp::GcpSigningCredentialProvider` · object_store 0.13.2

```rust
type GcpSigningCredentialProvider = std::sync::Arc<dyn CredentialProvider<Credential = GcpSigningCredential>>
```

Source: `src/gcp/mod.rs:73`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

[`GcpSigningCredential`](../operations/object_store.gcp.credential.GcpSigningCredential.md#op-e7b2777de2728adb0f286741) for [`GoogleCloudStorage`](../operations/object_store.gcp.GoogleCloudStorage.md#op-4b3bbb82170d36762c713ba6)
