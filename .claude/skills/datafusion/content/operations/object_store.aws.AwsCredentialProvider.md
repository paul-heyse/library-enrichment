# `object_store::aws::AwsCredentialProvider`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.AwsCredentialProvider.json).

<a id="op-c90753c8319c4a398efcba53"></a>
## AwsCredentialProvider

`type_alias` · `object_store::aws::AwsCredentialProvider` · object_store 0.13.2

```rust
type AwsCredentialProvider = std::sync::Arc<dyn CredentialProvider<Credential = AwsCredential>>
```

Source: `src/aws/mod.rs:77`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

[`CredentialProvider`](../operations/object_store.client.CredentialProvider.md#op-76004bbb5f119134cca4253a) for [`AmazonS3`](../operations/object_store.aws.AmazonS3.md#op-792ec7caebfaa6563754cd2e)
