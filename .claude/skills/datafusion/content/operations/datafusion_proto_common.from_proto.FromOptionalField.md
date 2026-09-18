# `datafusion_proto_common::from_proto::FromOptionalField`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.from_proto.FromOptionalField.json).

<a id="op-8e8d248c503bcb1140068b49"></a>
## FromOptionalField

`trait` · `datafusion_proto_common::from_proto::FromOptionalField` · datafusion-proto-common 55.1.0

```rust
trait FromOptionalField<T>
```

Source: `src/from_proto/mod.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

An extension trait that adds the methods `optional` and `required` to any
Option containing a type implementing `TryInto<U, Error = Error>`

<a id="op-9c2abd26ba5472ca1a4eac8f"></a>
## optional

`function` · `datafusion_proto_common::from_proto::FromOptionalField::optional` · datafusion-proto-common 55.1.0

```rust
fn optional(self) -> datafusion_common::Result<Option<T>, Error>
```

Source: `src/from_proto/mod.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Converts an optional protobuf field to an option of a different type

Returns None if the option is None, otherwise calls [`TryInto::try_into`]
on the contained data, returning any error encountered

Unresolved upstream links (retained, not inferred): ``TryInto::try_into``.

<a id="op-46999a91532ad0330ef8c3b9"></a>
## required

`function` · `datafusion_proto_common::from_proto::FromOptionalField::required` · datafusion-proto-common 55.1.0

```rust
fn required(self, field: impl Into<String>) -> datafusion_common::Result<T, Error>
```

Source: `src/from_proto/mod.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Converts an optional protobuf field to a different type, returning an error if None

Returns `Error::MissingRequiredField` if None, otherwise calls [`TryInto::try_into`]
on the contained data, returning any error encountered

Unresolved upstream links (retained, not inferred): ``TryInto::try_into``.
