# `arrow_json::JsonSerializable`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.JsonSerializable.json).

<a id="op-e044e24629c465e0b47dcd8a"></a>
## JsonSerializable

`trait` · `arrow_json::JsonSerializable` · arrow-json 59.3.0

```rust
trait JsonSerializable: 'static
```

Source: `src/lib.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Trait declaring any type that is serializable to JSON. This includes all primitive types (bool, i32, etc.).

<a id="op-addcc65a282599affe780bb4"></a>
## into_json_value

`function` · `arrow_json::JsonSerializable::into_json_value` · arrow-json 59.3.0

```rust
fn into_json_value(self) -> Option<Value>
```

Source: `src/lib.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Converts self into json value if its possible
