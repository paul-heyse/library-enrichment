# `object_store::azure`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.azure.json).

<a id="op-e8cabc3351f612dca436727f"></a>
## azure

`module` · `object_store::azure` · object_store 0.13.2

```rust
mod azure
```

Source: `src/azure/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An object store implementation for Azure blob storage

## Streaming uploads

[`ObjectStore::put_multipart_opts`](../operations/object_store.ObjectStore.md#op-27dfce2ef1fd51c4e2938336) will upload data in blocks and write a blob from those blocks.

Unused blocks will automatically be dropped after 7 days.

