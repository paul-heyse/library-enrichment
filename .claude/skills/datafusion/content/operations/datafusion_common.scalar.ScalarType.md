# `datafusion_common::scalar::ScalarType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.ScalarType.json).

<a id="op-862819debdda220e39b62ea5"></a>
## ScalarType

`trait` · `datafusion_common::scalar::ScalarType` · datafusion-common 55.1.0

```rust
trait ScalarType<T: ArrowNativeType>
```

Source: `src/scalar/mod.rs:5964`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Trait used to map a NativeType to a ScalarValue

<a id="op-3e85cc8d7088a98c186019ff"></a>
## scalar

`function` · `datafusion_common::scalar::ScalarType::scalar` · datafusion-common 55.1.0

```rust
fn scalar(r: Option<T>) -> ScalarValue
```

Source: `src/scalar/mod.rs:5966`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

returns a scalar from an optional T
