# `buoyant_kernel::error::Error`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.error.Error.json).

<a id="op-a6fe8c009eabf996f29109bf"></a>
## Error

`enum` · `buoyant_kernel::error::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L32).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:32`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

All the types of errors that the kernel can run into

<a id="op-1dfd346043afb342350a8eba"></a>
## Arrow

`variant` · `buoyant_kernel::error::Error::Arrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Arrow
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L46).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An error performing operations on arrow data

<a id="op-a1147676dc06962bc8d25b7d"></a>
## Backtraced

`variant` · `buoyant_kernel::error::Error::Backtraced` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Backtraced
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L38).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:38`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This is an error that includes a backtrace. To have a particular type of error include such
backtrace (when RUST_BACKTRACE=1), annotate the error with `#[error(transparent)]` and then
add the error type and enum variant to the `from_with_backtrace!` macro invocation
below. See IOError for an example.

<a id="op-a70ba46f5d47284a413d8ddd"></a>
## ChangeDataFeedIncompatibleSchema

`variant` · `buoyant_kernel::error::Error::ChangeDataFeedIncompatibleSchema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ChangeDataFeedIncompatibleSchema
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L219).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:219`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d2e7bd92d1850ad6cae4841"></a>
## ChangeDataFeedUnsupported

`variant` · `buoyant_kernel::error::Error::ChangeDataFeedUnsupported` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ChangeDataFeedUnsupported
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L216).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:216`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8942a40db5a0ab7ef37719a4"></a>
## CheckpointWrite

`variant` · `buoyant_kernel::error::Error::CheckpointWrite` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CheckpointWrite
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L49).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c832660ea706887c1e606db"></a>
## ChecksumWriteUnsupported

`variant` · `buoyant_kernel::error::Error::ChecksumWriteUnsupported` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ChecksumWriteUnsupported
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L209).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:209`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Cannot write a version checksum (CRC) file for this snapshot

<a id="op-0736fdc67e2528180b97a2da"></a>
## DeletionVector

`variant` · `buoyant_kernel::error::Error::DeletionVector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DeletionVector
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L126).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:126`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An error occurred while working with deletion vectors

<a id="op-41db85b445e85a3b6daade5f"></a>
## EngineDataType

`variant` · `buoyant_kernel::error::Error::EngineDataType` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
EngineDataType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L53).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

User tried to convert engine data to the wrong type

<a id="op-b6ba53633972c7bac66cf174"></a>
## Extract

`variant` · `buoyant_kernel::error::Error::Extract` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Extract
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L57).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:57`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Could not extract the specified type

<a id="op-8f0d9c95adb6e6e26cad1dcb"></a>
## FileAlreadyExists

`variant` · `buoyant_kernel::error::Error::FileAlreadyExists` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
FileAlreadyExists
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L201).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:201`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The file already exists at the path, prohibiting a non-overwrite write

<a id="op-f58dcfa1d09ba8ac4cbba335"></a>
## FileNotFound

`variant` · `buoyant_kernel::error::Error::FileNotFound` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
FileNotFound
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L101).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:101`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A specified file could not be found

<a id="op-5448ac938c976df2479cb9fb"></a>
## Generic

`variant` · `buoyant_kernel::error::Error::Generic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Generic
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L61).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:61`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A generic error with a message

<a id="op-791a05c9f8801d85c5e5f51c"></a>
## GenericError

`variant` · `buoyant_kernel::error::Error::GenericError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
GenericError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L65).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:65`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A generic error wrapping another error

<a id="op-93adf5fd1d842c00848b2aa6"></a>
## IOError

`variant` · `buoyant_kernel::error::Error::IOError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
IOError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L72).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:72`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Some kind of [`std::io::Error`]

Unresolved upstream links (retained, not inferred): ``std::io::Error``.

<a id="op-5428dfc3b47255b5ea18d039"></a>
## InternalError

`variant` · `buoyant_kernel::error::Error::InternalError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InternalError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L76).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An internal error that means kernel found an unexpected situation, which is likely a bug

<a id="op-f4d1af917da5faa5d0c5f8f9"></a>
## InvalidCheckpoint

`variant` · `buoyant_kernel::error::Error::InvalidCheckpoint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidCheckpoint
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L223).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:223`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Invalid checkpoint files

<a id="op-4fe42cef46d855607e991af2"></a>
## InvalidColumnMappingMode

`variant` · `buoyant_kernel::error::Error::InvalidColumnMappingMode` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidColumnMappingMode
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L177).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:177`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a91b9ad46665b2107bfe0e72"></a>
## InvalidDecimal

`variant` · `buoyant_kernel::error::Error::InvalidDecimal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidDecimal
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L185).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:185`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Precision or scale not compliant with delta specification

<a id="op-ef7a9ad0d26e424dd5a64b9a"></a>
## InvalidExpressionEvaluation

`variant` · `buoyant_kernel::error::Error::InvalidExpressionEvaluation` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidExpressionEvaluation
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L193).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:193`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Expressions did not parse or evaluate correctly

<a id="op-4d372124b4421a75bd9a3e2f"></a>
## InvalidLogPath

`variant` · `buoyant_kernel::error::Error::InvalidLogPath` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidLogPath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L197).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:197`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Unable to parse the name of a log path

<a id="op-846fcc86c789863be8037914"></a>
## InvalidPartitionValues

`variant` · `buoyant_kernel::error::Error::InvalidPartitionValues` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidPartitionValues
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L110).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:110`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The connector-provided partition values are invalid (missing/extra/duplicate keys,
or a value type does not match the schema column type).

<a id="op-9ccc81558598c1225ed73912"></a>
## InvalidProtocol

`variant` · `buoyant_kernel::error::Error::InvalidProtocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidProtocol
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L154).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:154`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Invalid protocol action was read from the log

<a id="op-2e1cb840440643bea6559dfa"></a>
## InvalidSelectionVector

`variant` · `buoyant_kernel::error::Error::InvalidSelectionVector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidSelectionVector
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L130).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:130`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A selection vector is larger than data length

<a id="op-3a0b135d8aba66b302de9688"></a>
## InvalidStructData

`variant` · `buoyant_kernel::error::Error::InvalidStructData` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidStructData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L189).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:189`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Inconsistent data passed to struct scalar

<a id="op-c4a493f794624ecb259bf23c"></a>
## InvalidTableLocation

`variant` · `buoyant_kernel::error::Error::InvalidTableLocation` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidTableLocation
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L181).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:181`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Asked for a table at an invalid location

<a id="op-d9c74a3464ee1d859ddeb5ce"></a>
## InvalidTransactionState

`variant` · `buoyant_kernel::error::Error::InvalidTransactionState` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidTransactionState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L134).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:134`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Transaction state is invalid for the requested operation

<a id="op-84f78f70ea0faa8c1931b649"></a>
## InvalidUrl

`variant` · `buoyant_kernel::error::Error::InvalidUrl` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidUrl
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L138).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:138`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A specified URL was invalid

<a id="op-1df716b79e98f4a28d2e40e9"></a>
## JoinFailure

`variant` · `buoyant_kernel::error::Error::JoinFailure` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
JoinFailure
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L166).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:166`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A tokio executor failed to join a task

<a id="op-0f145ad7b1953e68131869a2"></a>
## LiteralExpressionTransformError

`variant` · `buoyant_kernel::error::Error::LiteralExpressionTransformError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
LiteralExpressionTransformError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L227).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:227`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Error while transforming a schema + leaves into an Expression of literals

<a id="op-bc2841648d44d970e21f8c14"></a>
## LogHistory

`variant` · `buoyant_kernel::error::Error::LogHistory` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
LogHistory
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L241).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:241`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Error during log history operations (timestamp queries, version lookups)

<a id="op-04360595e83972213a8f7b40"></a>
## MalformedJson

`variant` · `buoyant_kernel::error::Error::MalformedJson` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MalformedJson
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L142).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:142`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

serde encountered malformed json

<a id="op-ce8e99fd89b74882b0058643"></a>
## MissingColumn

`variant` · `buoyant_kernel::error::Error::MissingColumn` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MissingColumn
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L105).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:105`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A column was requested, but not found

<a id="op-bd92d2873b699d105fad303c"></a>
## MissingData

`variant` · `buoyant_kernel::error::Error::MissingData` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MissingData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L118).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:118`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Data was expected, but not found

<a id="op-a30f864ee91a8f768951a476"></a>
## MissingMetadata

`variant` · `buoyant_kernel::error::Error::MissingMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MissingMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L146).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:146`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

There was no metadata action in the delta log

<a id="op-028d22c1c20d39cd9d9f5d93"></a>
## MissingMetadataAndProtocol

`variant` · `buoyant_kernel::error::Error::MissingMetadataAndProtocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MissingMetadataAndProtocol
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L158).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:158`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Neither metadata nor protocol could be found in the delta log

<a id="op-2ade9b0cbce9871cd47bf880"></a>
## MissingProtocol

`variant` · `buoyant_kernel::error::Error::MissingProtocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MissingProtocol
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L150).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:150`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

There was no protocol action in the delta log

<a id="op-3bd0cce2c3c9124bef25ffd2"></a>
## MissingVersion

`variant` · `buoyant_kernel::error::Error::MissingVersion` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MissingVersion
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L122).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:122`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A version for the delta table could not be found in the log

<a id="op-27222d9c0d00af0d33fdc816"></a>
## ObjectStore

`variant` · `buoyant_kernel::error::Error::ObjectStore` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ObjectStore
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L88).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:88`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An error interacting with the object_store crate

<a id="op-eb57325e261bcaa87ed5a8a1"></a>
## ObjectStorePath

`variant` · `buoyant_kernel::error::Error::ObjectStorePath` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ObjectStorePath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L93).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:93`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An error working with paths from the object_store crate

<a id="op-b343c91c092653cd07f52e80"></a>
## Parquet

`variant` · `buoyant_kernel::error::Error::Parquet` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Parquet
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L81).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:81`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An error enountered while working with parquet data

<a id="op-fd6af2cb49766d49b3cf752a"></a>
## ParseError

`variant` · `buoyant_kernel::error::Error::ParseError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ParseError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L162).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:162`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A string failed to parse as the specified data type

<a id="op-03cdd6947a10aaa6d25c578e"></a>
## ParseIntError

`variant` · `buoyant_kernel::error::Error::ParseIntError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ParseIntError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L174).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:174`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Could not parse an integer

<a id="op-56a744094fc75dbebf683c70"></a>
## ParseIntervalError

`variant` · `buoyant_kernel::error::Error::ParseIntervalError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ParseIntervalError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L213).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:213`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Parsing error when attempting to deserialize an interval

<a id="op-5a61a4cc1945adaf549f656a"></a>
## Reqwest

`variant` · `buoyant_kernel::error::Error::Reqwest` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Reqwest
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L97).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21112dfcdc517d4caeb65148"></a>
## Schema

`variant` · `buoyant_kernel::error::Error::Schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Schema
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L233).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:233`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Schema mismatch has occurred or invalid schema used somewhere

<a id="op-7e0118cd2b29c520af8ea2c7"></a>
## StatsValidation

`variant` · `buoyant_kernel::error::Error::StatsValidation` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
StatsValidation
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L237).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:237`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Validation error for file statistics (e.g., missing required clustering column stats)

<a id="op-344ad45fe6781d4cb0697909"></a>
## UnexpectedColumnType

`variant` · `buoyant_kernel::error::Error::UnexpectedColumnType` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnexpectedColumnType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L114).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A column was specified with a specific type, but it is not of that type

<a id="op-91b3a148979b4b0fa8419672"></a>
## Unsupported

`variant` · `buoyant_kernel::error::Error::Unsupported` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Unsupported
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L205).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:205`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Some functionality is currently unsupported

<a id="op-0b826da4b13dbf4ffa2e9b97"></a>
## Utf8Error

`variant` · `buoyant_kernel::error::Error::Utf8Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Utf8Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L170).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:170`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Could not convert to string from utf-8

<a id="op-9c01a58f15e46d48ea01d703"></a>
## change_data_feed_unsupported

`function` · `buoyant_kernel::error::Error::change_data_feed_unsupported` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn change_data_feed_unsupported(version: impl Into<Version>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L323).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:323`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44e1344e2d257e3db6493e77"></a>
## deletion_vector

`function` · `buoyant_kernel::error::Error::deletion_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deletion_vector(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L280).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:280`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbbf8bcb431a430ad173a7e8"></a>
## engine_data_type

`function` · `buoyant_kernel::error::Error::engine_data_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn engine_data_type(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L283).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:283`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d905210bf9cbd6cf36ad7b0b"></a>
## file_not_found

`function` · `buoyant_kernel::error::Error::file_not_found` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_not_found(path: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L265).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:265`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d78c87a27779af71d59cdeb4"></a>
## fmt

`function` · `buoyant_kernel::error::Error::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4cb8883d25c626e09e1a07c"></a>
## fmt

`function` · `buoyant_kernel::error::Error::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 28], "end": [31, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02648bdea3d6654a0321319d"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: serde_json::Error) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [379, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "serde_json::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:376`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03151ffb6618c8b73d42c625"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: object_store::path::Error) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 21], "end": [93, 28], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::path::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-081960a6d003c0900d41db3c"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: std::io::Error) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L376).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [379, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::io::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:376`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24d15a6f228d0cc176531244"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: reqwest::Error) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 13], "end": [97, 20], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "reqwest::error::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ad8bf8713917b55aa7e584a"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: Utf8Error) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 15], "end": [170, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::str::error::Utf8Error", "path": "Utf8Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32eafc99d0f130bba6c515d6"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: ParseIntError) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 19], "end": [174, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "core::num::error::ParseIntError", "path": "ParseIntError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32f81797a97c3c2cae22fe6e"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: Box<history_manager::error::LogHistoryError>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 16], "end": [241, 23], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::error::LogHistoryError", "path": "LogHistoryError"}}}], "constraints": []}}, "id": "alloc::boxed::Box", "path": "Box"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bebcc9a8d563812335f93cb"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: url::ParseError) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 16], "end": [138, 23], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "url::parser::ParseError", "path": "ParseError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70f938bf4d58d72f0dc5eec7"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: ArrowError) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L383).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [386, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:383`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-806e21be22e6cacecf7b3a8b"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(e: LogHistoryError) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "crate::Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [141, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::error::LogHistoryError", "path": "LogHistoryError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:138`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98828de7f6a28ce4cb19e37c"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: parquet::errors::ParquetError) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 13], "end": [81, 20], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::errors::ParquetError", "path": "ParquetError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4e3840d4642b4bfff9ef253"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: object_store::Error) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L390).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [396, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:390`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db5d8bf6b34c36d453906c4c"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: Infallible) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L403).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [402, 1], "end": [406, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "never"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:403`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9ff976d35332bcdaf30cb7d"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: ParseIntervalError) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 24], "end": [213, 31], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::deserialize::ParseIntervalError", "path": "ParseIntervalError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cde4aa0adaeed768b793c89"></a>
## generic

`function` · `buoyant_kernel::error::Error::generic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn generic(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L262).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:262`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4c91e6c5a3264eb40b8df32"></a>
## generic_err

`function` · `buoyant_kernel::error::Error::generic_err` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn generic_err(source: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L257).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:257`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9990bd3c89ea49a58680ad91"></a>
## internal_error

`function` · `buoyant_kernel::error::Error::internal_error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn internal_error(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L308).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:308`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faea342c9c8162d346270570"></a>
## invalid_checkpoint

`function` · `buoyant_kernel::error::Error::invalid_checkpoint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_checkpoint(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L333).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:333`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7829d44e1a0f828a0ee31657"></a>
## invalid_column_mapping_mode

`function` · `buoyant_kernel::error::Error::invalid_column_mapping_mode` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_column_mapping_mode(mode: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L292).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:292`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf02dea5320f83b36e4608d9"></a>
## invalid_decimal

`function` · `buoyant_kernel::error::Error::invalid_decimal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_decimal(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L295).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:295`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5a015cd2b21add039601763"></a>
## invalid_expression

`function` · `buoyant_kernel::error::Error::invalid_expression` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_expression(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L301).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:301`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28abe0103a07b684b62fcd03"></a>
## invalid_partition_values

`function` · `buoyant_kernel::error::Error::invalid_partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_partition_values(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L274).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:274`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e8b9d35f853fc0d8d9f5913"></a>
## invalid_protocol

`function` · `buoyant_kernel::error::Error::invalid_protocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_protocol(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L312).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:312`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f39f16ba9d86586f80eda86"></a>
## invalid_struct_data

`function` · `buoyant_kernel::error::Error::invalid_struct_data` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_struct_data(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L298).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:298`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff87c6826c69308f73595954"></a>
## invalid_table_location

`function` · `buoyant_kernel::error::Error::invalid_table_location` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_table_location(location: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L289).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:289`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-532493a1f18a500d06fcbe6e"></a>
## invalid_transaction_state

`function` · `buoyant_kernel::error::Error::invalid_transaction_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn invalid_transaction_state(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L316).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:316`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f402e971bb98b734979b5d90"></a>
## join_failure

`function` · `buoyant_kernel::error::Error::join_failure` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn join_failure(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L286).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:286`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b031d32762be215266457e7"></a>
## missing_column

`function` · `buoyant_kernel::error::Error::missing_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn missing_column(name: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L268).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:268`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7d98d4c8d949c0c55e0f04a"></a>
## missing_data

`function` · `buoyant_kernel::error::Error::missing_data` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn missing_data(name: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L277).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:277`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47c2111dee4deaa953bcc7cf"></a>
## schema

`function` · `buoyant_kernel::error::Error::schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L337).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:337`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32ada9ded7b3551b973e9ec4"></a>
## source

`function` · `buoyant_kernel::error::Error::source` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bdb89e7a328b8573d4e606d"></a>
## stats_validation

`function` · `buoyant_kernel::error::Error::stats_validation` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn stats_validation(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L341).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:341`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea521685b5c7ebfab4c6572d"></a>
## unexpected_column_type

`function` · `buoyant_kernel::error::Error::unexpected_column_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unexpected_column_type(name: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L271).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:271`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f187fb4d5ce84a4cd7874a7"></a>
## unsupported

`function` · `buoyant_kernel::error::Error::unsupported` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unsupported(msg: impl ToString) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L320).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:320`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f029703b68c01e30d092e7d3"></a>
## with_backtrace

`function` · `buoyant_kernel::error::Error::with_backtrace` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_backtrace(self) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L352).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [362, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:352`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75059f69458f61426772f58e"></a>
## from

`function` · `buoyant_kernel::error::Error::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: expressions::literal_expression_transform::Error) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 9], "end": [228, 16], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::literal_expression_transform::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
