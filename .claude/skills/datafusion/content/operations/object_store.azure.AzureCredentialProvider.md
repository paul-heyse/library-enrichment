# `object_store::azure::AzureCredentialProvider`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.azure.AzureCredentialProvider.json).

<a id="op-83656ccde14f028af0230a2a"></a>
## AzureCredentialProvider

`type_alias` · `object_store::azure::AzureCredentialProvider` · object_store 0.13.2

```rust
type AzureCredentialProvider = std::sync::Arc<dyn CredentialProvider<Credential = AzureCredential>>
```

Source: `src/azure/mod.rs:52`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

[`CredentialProvider`](../operations/object_store.client.CredentialProvider.md#op-76004bbb5f119134cca4253a) for [`MicrosoftAzure`](../operations/object_store.azure.MicrosoftAzure.md#op-a50607143b9beb78c537c27a)
