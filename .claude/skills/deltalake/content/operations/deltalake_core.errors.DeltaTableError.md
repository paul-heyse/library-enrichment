# `deltalake_core::errors::DeltaTableError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.json).

<a id="op-814fa8781a3d87b190447426"></a>
## DeltaTableError

`enum` · `deltalake_core::errors::DeltaTableError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L32).

Source: `crates/core/src/errors.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta Table specific error

<a id="op-79fa5ca04ee8d687ca2bfa66"></a>
## Arrow

`variant` · `deltalake_core::errors::DeltaTableError::Arrow` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Arrow
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L54).

Source: `crates/core/src/errors.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when converting the schema in Arrow format failed.

<a id="op-48eb7f1be7d39da07c014c24"></a>
## ChangeDataInvalidVersionRange

`variant` · `deltalake_core::errors::DeltaTableError::ChangeDataInvalidVersionRange` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ChangeDataInvalidVersionRange
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L225).

Source: `crates/core/src/errors.rs:225`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d3e2aeeb8bf3d70b26cf0b1"></a>
## ChangeDataNotEnabled

`variant` · `deltalake_core::errors::DeltaTableError::ChangeDataNotEnabled` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ChangeDataNotEnabled
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L222).

Source: `crates/core/src/errors.rs:222`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3464f3fa0dafd896844e6b32"></a>
## ChangeDataNotRecorded

`variant` · `deltalake_core::errors::DeltaTableError::ChangeDataNotRecorded` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ChangeDataNotRecorded
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L215).

Source: `crates/core/src/errors.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb99484011b94b9c659928e1"></a>
## ChangeDataTimestampGreaterThanCommit

`variant` · `deltalake_core::errors::DeltaTableError::ChangeDataTimestampGreaterThanCommit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ChangeDataTimestampGreaterThanCommit
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L228).

Source: `crates/core/src/errors.rs:228`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49c7f16a5829ff93151fc185"></a>
## CommitValidation

`variant` · `deltalake_core::errors::DeltaTableError::CommitValidation` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CommitValidation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L148).

Source: `crates/core/src/errors.rs:148`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error raised while preparing a commit

<a id="op-91abbdd51a8e896d4cfead49"></a>
## Generic

`variant` · `deltalake_core::errors::DeltaTableError::Generic` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Generic
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L190).

Source: `crates/core/src/errors.rs:190`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generic Delta Table error

<a id="op-e71498b8e68e8ef8af891034"></a>
## GenericError

`variant` · `deltalake_core::errors::DeltaTableError::GenericError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
GenericError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L194).

Source: `crates/core/src/errors.rs:194`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generic Delta Table error

<a id="op-4f49890dde81661f4f48264c"></a>
## InvalidData

`variant` · `deltalake_core::errors::DeltaTableError::InvalidData` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidData
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L103).

Source: `crates/core/src/errors.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when attempting to write bad data to the table

<a id="op-a1234fcc8146c20b77c9b099"></a>
## InvalidDateTimeString

`variant` · `deltalake_core::errors::DeltaTableError::InvalidDateTimeString` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidDateTimeString
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L95).

Source: `crates/core/src/errors.rs:95`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when the datetime string is invalid for a conversion.

<a id="op-af124963cbc18327df476a76"></a>
## InvalidJsonLog

`variant` · `deltalake_core::errors::DeltaTableError::InvalidJsonLog` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidJsonLog
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L62).

Source: `crates/core/src/errors.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when the log record has an invalid JSON.

<a id="op-54e2079e3ee7c247c33ccd5c"></a>
## InvalidPartitionFilter

`variant` · `deltalake_core::errors::DeltaTableError::InvalidPartitionFilter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidPartitionFilter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L133).

Source: `crates/core/src/errors.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when a invalid partition filter was found.

<a id="op-f15c56a852fdf7f902670e64"></a>
## InvalidStatsJson

`variant` · `deltalake_core::errors::DeltaTableError::InvalidStatsJson` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidStatsJson
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L73).

Source: `crates/core/src/errors.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when the log contains invalid stats JSON.

<a id="op-02e2ad790a6d7f7e90bf1b66"></a>
## InvalidTableLocation

`variant` · `deltalake_core::errors::DeltaTableError::InvalidTableLocation` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidTableLocation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L179).

Source: `crates/core/src/errors.rs:179`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A Feature is missing to perform operation

<a id="op-4d3f821e4363c169a3e67f22"></a>
## InvalidVersion

`variant` · `deltalake_core::errors::DeltaTableError::InvalidVersion` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L80).

Source: `crates/core/src/errors.rs:80`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when the DeltaTable has an invalid version.

<a id="op-120d7ff9b4434933b38dff8b"></a>
## Io

`variant` · `deltalake_core::errors::DeltaTableError::Io` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Io
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L140).

Source: `crates/core/src/errors.rs:140`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when a line from log record is invalid.

<a id="op-c1658be975dda24a82660c8a"></a>
## Kernel

`variant` · `deltalake_core::errors::DeltaTableError::Kernel` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Kernel
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L200).

Source: `crates/core/src/errors.rs:200`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5009faf82ed1001b2069dd5d"></a>
## KernelError

`variant` · `deltalake_core::errors::DeltaTableError::KernelError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
KernelError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L34).

Source: `crates/core/src/errors.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03596317c9ee464ea2929ad9"></a>
## MetadataError

`variant` · `deltalake_core::errors::DeltaTableError::MetadataError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MetadataError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L206).

Source: `crates/core/src/errors.rs:206`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b5ef02e795c6887632ba4a9"></a>
## MissingFeature

`variant` · `deltalake_core::errors::DeltaTableError::MissingFeature` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MissingFeature
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L170).

Source: `crates/core/src/errors.rs:170`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A Feature is missing to perform operation

<a id="op-7ea2f56f3140deb9d9a6c25c"></a>
## NoSchema

`variant` · `deltalake_core::errors::DeltaTableError::NoSchema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NoSchema
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L114).

Source: `crates/core/src/errors.rs:114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when no schema was found in the DeltaTable.

<a id="op-5c77277a742427c4a2874265"></a>
## NoStartingVersionOrTimestamp

`variant` · `deltalake_core::errors::DeltaTableError::NoStartingVersionOrTimestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NoStartingVersionOrTimestamp
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L231).

Source: `crates/core/src/errors.rs:231`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8374096853bddd42b5e18584"></a>
## NotATable

`variant` · `deltalake_core::errors::DeltaTableError::NotATable` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NotATable
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L110).

Source: `crates/core/src/errors.rs:110`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when it is not a DeltaTable.

<a id="op-90ad2982ba37415ab72b17c7"></a>
## NotInitialized

`variant` · `deltalake_core::errors::DeltaTableError::NotInitialized` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NotInitialized
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L209).

Source: `crates/core/src/errors.rs:209`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7648082d6b4f48a778b4913"></a>
## NotInitializedWithFiles

`variant` · `deltalake_core::errors::DeltaTableError::NotInitializedWithFiles` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NotInitializedWithFiles
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L212).

Source: `crates/core/src/errors.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4228bcd7b9ce3b5b3e4b4698"></a>
## ObjectStore

`variant` · `deltalake_core::errors::DeltaTableError::ObjectStore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ObjectStore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L38).

Source: `crates/core/src/errors.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when reading the delta log object failed.

<a id="op-3643a2adf165385ebcc6a112"></a>
## Parquet

`variant` · `deltalake_core::errors::DeltaTableError::Parquet` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Parquet
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L46).

Source: `crates/core/src/errors.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when parsing checkpoint parquet.

<a id="op-dda3a84a83b969abd2bf7f53"></a>
## PartitionError

`variant` · `deltalake_core::errors::DeltaTableError::PartitionError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
PartitionError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L126).

Source: `crates/core/src/errors.rs:126`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when a partition is not formatted as a Hive Partition.

<a id="op-494b0bb2b3d75ccf7421a74f"></a>
## SchemaMismatch

`variant` · `deltalake_core::errors::DeltaTableError::SchemaMismatch` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
SchemaMismatch
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L119).

Source: `crates/core/src/errors.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when writes are attempted with data that doesn't match the schema of the
table

<a id="op-752534cf0f82727228d47216"></a>
## SerializeLogJson

`variant` · `deltalake_core::errors::DeltaTableError::SerializeLogJson` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
SerializeLogJson
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L183).

Source: `crates/core/src/errors.rs:183`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generic Delta Table error

<a id="op-378f5c6fb9ab718f5e70d407"></a>
## Transaction

`variant` · `deltalake_core::errors::DeltaTableError::Transaction` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Transaction
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L155).

Source: `crates/core/src/errors.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error raised while commititng transaction

<a id="op-819662020b8c1cefa3c38472"></a>
## UnsupportedColumnMapping

`variant` · `deltalake_core::errors::DeltaTableError::UnsupportedColumnMapping` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnsupportedColumnMapping
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L236).

Source: `crates/core/src/errors.rs:236`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when an operation is attempted on a column-mapped table that does not
yet support column mapping.

<a id="op-f68c064464baf44ac8fe9429"></a>
## VersionAlreadyExists

`variant` · `deltalake_core::errors::DeltaTableError::VersionAlreadyExists` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VersionAlreadyExists
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L162).

Source: `crates/core/src/errors.rs:162`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when transaction is failed to be committed because given version already exists.

<a id="op-4ac3033c4031d69ad9d508e3"></a>
## VersionDowngrade

`variant` · `deltalake_core::errors::DeltaTableError::VersionDowngrade` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VersionDowngrade
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L86).

Source: `crates/core/src/errors.rs:86`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when an operation requests an older version than the currently loaded one.

<a id="op-30262ffa21617903960ed75e"></a>
## VersionMismatch

`variant` · `deltalake_core::errors::DeltaTableError::VersionMismatch` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VersionMismatch
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L166).

Source: `crates/core/src/errors.rs:166`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when user attempts to commit actions that don't belong to the next version.

<a id="op-e1c9b5309a13b37c0dffdc1b"></a>
## fmt

`function` · `deltalake_core::errors::DeltaTableError::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 28], "end": [31, 33], "filename": "crates/core/src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edcc076d45e79819ea39d39a"></a>
## fmt

`function` · `deltalake_core::errors::DeltaTableError::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 26], "filename": "crates/core/src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fce0c4e33a6396ac8fd0d0d"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: delta_kernel::error::Error) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 24], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78fa3f671e5cf3d485fba957"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: ObjectStoreError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 9], "end": [40, 16], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8974b28a07ca56d425bf23f2"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: serde_json::Error) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L253).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [256, 2], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "serde_json::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:253`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-923128d6018ccbdf8af7ac35"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: object_store::path::Error) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L245).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [250, 2], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::path::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:245`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96da5dfe84db852e4797e5a1"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: DataFusionError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L119).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::errors::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [128, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/mod.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2a5b78b0f9d519d284517d5"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: kernel::Error) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 9], "end": [201, 16], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a542d58d1628b1740e598f72"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: CommitBuilderError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L245).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::errors::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [248, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitBuilderError", "path": "CommitBuilderError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/kernel/transaction/mod.rs:245`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bff3f7998aba1a056e250358"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: chrono::ParseError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 9], "end": [97, 16], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "chrono::format::ParseError", "path": "ParseError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d040bdb9921e35c556118d1c"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: TransactionError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L226).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::errors::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [238, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::TransactionError", "path": "TransactionError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/kernel/transaction/mod.rs:226`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e26dbd9c68847ff475443c0c"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: parquet::errors::ParquetError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 9], "end": [48, 16], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6b56a62a08e940b89bb7d55"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: arrow::error::ArrowError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 9], "end": [56, 16], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb91dae21d368aa880da7910"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: std::io::Error) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 9], "end": [142, 16], "filename": "crates/core/src/errors.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::io::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e78168333b01292f32545c7"></a>
## generic

`function` · `deltalake_core::errors::DeltaTableError::generic` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn generic(msg: impl ToString) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L269).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [283, 2], "filename": "crates/core/src/errors.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/errors.rs:269`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a [Generic](DeltaTableError::Generic) error with the given message.

<a id="op-947b274f2b1e6248359f31ea"></a>
## not_a_table

`function` · `deltalake_core::errors::DeltaTableError::not_a_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn not_a_table(path: impl AsRef<str>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L260).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [283, 2], "filename": "crates/core/src/errors.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/errors.rs:260`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Crate a NotATable Error with message for given path.

<a id="op-3bd4f673d07bc4762f3253c1"></a>
## source

`function` · `deltalake_core::errors::DeltaTableError::source` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 26], "filename": "crates/core/src/errors.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `crates/core/src/errors.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd38d4f5ae592a151715b5ea"></a>
## unsupported_column_mapping

`function` · `deltalake_core::errors::DeltaTableError::unsupported_column_mapping` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unsupported_column_mapping(mode: ColumnMappingOperation, operation: impl ToString) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L274).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [283, 2], "filename": "crates/core/src/errors.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/errors.rs:274`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Construct an [`UnsupportedColumnMapping`](DeltaTableError::UnsupportedColumnMapping) error.

<a id="op-0894d6a4a4a84ba68e2893a4"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: WriteError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/writer.rs#L186).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::errors::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [197, 2], "filename": "crates/core/src/datafile/writer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::writer::WriteError", "path": "WriteError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/datafile/writer.rs:186`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fe0bafb86994e48b06e3427"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: CreateError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/create.rs#L47).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::errors::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [52, 2], "filename": "crates/core/src/operations/create.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::operations::create::CreateError", "path": "CreateError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/operations/create.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a881e0f5cebc9d01de44fd6"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: WriteError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::errors::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [105, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::WriteError", "path": "WriteError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/operations/write/mod.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-974bdacc8e7fe6b14f17fa76"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: DeltaWriterError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L125).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::errors::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [138, 2], "filename": "crates/core/src/writer/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::writer::DeltaWriterError", "path": "DeltaWriterError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/writer/mod.rs:125`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc06d5348fd051de9cb42e0e"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: Error) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/convert_to_delta.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [81, 2], "filename": "crates/core/src/operations/convert_to_delta.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::operations::convert_to_delta::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/operations/convert_to_delta.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e19521eecb2ece024f7bfd63"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: RestoreError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/restore.rs#L62).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [67, 2], "filename": "crates/core/src/operations/restore.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::operations::restore::RestoreError", "path": "RestoreError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/operations/restore.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5c3c5852a8ff8ac90148c56"></a>
## from

`function` · `deltalake_core::errors::DeltaTableError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(err: VacuumError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L157).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::DeltaTableError", "path": "crate::errors::DeltaTableError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [162, 2], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumError", "path": "VacuumError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/operations/vacuum.rs:157`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
