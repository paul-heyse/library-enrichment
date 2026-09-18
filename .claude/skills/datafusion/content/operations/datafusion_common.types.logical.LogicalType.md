# `datafusion_common::types::logical::LogicalType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.logical.LogicalType.json).

<a id="op-a70362eb6689066b420dd5aa"></a>
## LogicalType

`trait` · `datafusion_common::types::logical::LogicalType` · datafusion-common 55.1.0

```rust
trait LogicalType: Sync + Send
```

Source: `src/types/logical.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Representation of a logical type with its signature and its native backing
type.

The logical type is meant to be used during the DataFusion logical planning
phase in order to reason about logical types without worrying about their
underlying physical implementation.

### Extension types

[`LogicalType`](../operations/datafusion_common.types.logical.LogicalType.md#op-a70362eb6689066b420dd5aa) is a trait in order to allow the possibility of declaring
extension types:

```
use datafusion_common::types::{LogicalType, NativeType, TypeSignature};

struct JSON {}

impl LogicalType for JSON {
    fn native(&self) -> &NativeType {
        &NativeType::String
    }

    fn signature(&self) -> TypeSignature<'_> {
        TypeSignature::Extension {
            name: "JSON",
            parameters: &[],
        }
    }
}
```

<a id="op-28e1c5de51649e66d45f1522"></a>
## default_cast_for

`function` · `datafusion_common::types::logical::LogicalType::default_cast_for` · datafusion-common 55.1.0

```rust
fn default_cast_for(&self, origin: &DataType) -> Result<DataType>
```

Source: `src/types/logical.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get the default physical type to cast `origin` to in order to obtain a physical type
that is logically compatible with this logical type.

<a id="op-6ece692a1c5fdb1cd2f27326"></a>
## native

`function` · `datafusion_common::types::logical::LogicalType::native` · datafusion-common 55.1.0

```rust
fn native(&self) -> &NativeType
```

Source: `src/types/logical.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get the native backing type of this logical type.

<a id="op-5eadbc82d69b598893a247c8"></a>
## signature

`function` · `datafusion_common::types::logical::LogicalType::signature` · datafusion-common 55.1.0

```rust
fn signature(&self) -> TypeSignature<'_>
```

Source: `src/types/logical.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get the unique type signature for this logical type. Logical types with identical
signatures are considered equal.
