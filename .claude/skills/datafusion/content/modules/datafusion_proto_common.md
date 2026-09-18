# `datafusion_proto_common`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.json).

<a id="op-d6413286518a7310fe84c9dd"></a>
## datafusion_proto_common

`module` · `datafusion_proto_common` · datafusion-proto-common 55.1.0

```rust
mod datafusion_proto_common
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Serialize / Deserialize DataFusion Primitive Types to bytes

This crate provides support for serializing and deserializing the
following structures to and from bytes:

1. [`ScalarValue`]'s

[`ScalarValue`]: datafusion_common::ScalarValue

Internally, this crate is implemented by converting the common types to [protocol
buffers] using [prost].

[protocol buffers]: https://developers.google.com/protocol-buffers
[prost]: https://docs.rs/prost/latest/prost/

# Version Compatibility

The serialized form are not guaranteed to be compatible across
DataFusion versions. A plan serialized with one version of DataFusion
may not be able to deserialized with a different version.

# See Also

The binary format created by this crate supports the full range of DataFusion
plans, but is DataFusion specific. See [datafusion-substrait] for a crate
which can encode many DataFusion plans using the [substrait.io] standard.

[datafusion-substrait]: https://docs.rs/datafusion-substrait/latest/datafusion_substrait
[substrait.io]: https://substrait.io

# Example: Serializing [`ScalarValue`]s
```
# use datafusion_common::{ScalarValue, Result};
# use prost::{bytes::{Bytes, BytesMut}};
# use datafusion_common::plan_datafusion_err;
# use datafusion_proto_common::protobuf_common;
# use prost::Message;
# fn main() -> Result<()>{
// Create a new ScalarValue
let val = ScalarValue::UInt64(Some(3));
let mut buffer = BytesMut::new();
let protobuf: protobuf_common::ScalarValue = match val {
    ScalarValue::UInt64(Some(val)) => protobuf_common::ScalarValue {
        value: Some(protobuf_common::scalar_value::Value::Uint64Value(val)),
    },
    _ => unreachable!(),
};

protobuf
    .encode(&mut buffer)
    .map_err(|e| plan_datafusion_err!("Error encoding protobuf as bytes: {e}"))?;
// Convert it to bytes (for sending over the network, etc.)
let bytes: Bytes = buffer.into();

let protobuf = protobuf_common::ScalarValue::decode(bytes).map_err(|e| {
    plan_datafusion_err!("Error decoding ScalarValue as protobuf: {e}")
})?;
// Decode bytes from somewhere (over network, etc.) back to ScalarValue
let decoded_val: ScalarValue = match protobuf.value {
    Some(protobuf_common::scalar_value::Value::Uint64Value(val)) => {
        ScalarValue::UInt64(Some(val))
    }
    _ => unreachable!(),
};
assert_eq!(val, decoded_val);
# Ok(())
# }
```
