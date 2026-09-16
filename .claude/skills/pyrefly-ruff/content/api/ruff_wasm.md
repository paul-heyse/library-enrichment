# `ruff_wasm`

Crate `ruff_wasm` · 14 public items · structured records in [`model/ruff_wasm.json`](../model/ruff_wasm.json)

## ExpandedDiagnosticTag

`enum` · `ruff_wasm::ExpandedDiagnosticTag`

```rust
enum ExpandedDiagnosticTag
```

**Variants**: `Unnecessary`, `Deprecated`

**Implements**: `core::convert::From`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &diagnostic::DiagnosticTag) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## LogLevel

`enum` · `ruff_wasm::LogLevel`

```rust
enum LogLevel
```

**Variants**: `Trace`, `Debug`, `Info`, `Warn`, `Error`

**Implements**: `wasm_bindgen::convert::traits::FromWasmAbi`, `wasm_bindgen::convert::traits::IntoWasmAbi`, `wasm_bindgen::convert::traits::OptionFromWasmAbi`, `wasm_bindgen::convert::traits::OptionIntoWasmAbi`, `wasm_bindgen::convert::traits::TryFromJsValue`, `wasm_bindgen::convert::traits::VectorFromWasmAbi`, `wasm_bindgen::convert::traits::VectorIntoWasmAbi`, `wasm_bindgen::describe::WasmDescribe`, `wasm_bindgen::describe::WasmDescribeVector`

**Derives**: Clone, Copy, Debug

**via `wasm_bindgen::convert::traits::FromWasmAbi`**

```rust
unsafe fn from_abi(js: u32) -> Self
```

**via `wasm_bindgen::convert::traits::IntoWasmAbi`**

```rust
fn into_abi(self) -> u32
```

**via `wasm_bindgen::convert::traits::OptionFromWasmAbi`**

```rust
fn is_none(val: &Self::Abi) -> bool
```

**via `wasm_bindgen::convert::traits::OptionIntoWasmAbi`**

```rust
fn none() -> Self::Abi
```

**via `wasm_bindgen::convert::traits::TryFromJsValue`**

```rust
fn try_from_js_value_ref(value: &wasm_bindgen::JsValue) -> wasm_bindgen::__rt::core::option::Option<Self>
```

**via `wasm_bindgen::convert::traits::VectorFromWasmAbi`**

```rust
unsafe fn vector_from_abi(js: Self::Abi) -> wasm_bindgen::__rt::alloc::boxed::Box<[LogLevel]>
```

**via `wasm_bindgen::convert::traits::VectorIntoWasmAbi`**

```rust
fn vector_into_abi(vector: wasm_bindgen::__rt::alloc::boxed::Box<[LogLevel]>) -> Self::Abi
```

**via `wasm_bindgen::describe::WasmDescribe`**

```rust
fn describe()
```

**via `wasm_bindgen::describe::WasmDescribeVector`**

```rust
fn describe_vector()
```

---

## PositionEncoding

`enum` · `ruff_wasm::PositionEncoding`

```rust
enum PositionEncoding
```

**Variants**: `Utf8`, `Utf16`, `Utf32`

**Implements**: `wasm_bindgen::convert::traits::FromWasmAbi`, `wasm_bindgen::convert::traits::IntoWasmAbi`, `wasm_bindgen::convert::traits::OptionFromWasmAbi`, `wasm_bindgen::convert::traits::OptionIntoWasmAbi`, `wasm_bindgen::convert::traits::TryFromJsValue`, `wasm_bindgen::convert::traits::VectorFromWasmAbi`, `wasm_bindgen::convert::traits::VectorIntoWasmAbi`, `wasm_bindgen::describe::WasmDescribe`, `wasm_bindgen::describe::WasmDescribeVector`

**Derives**: Clone, Copy, Default

**via `wasm_bindgen::convert::traits::FromWasmAbi`**

```rust
unsafe fn from_abi(js: u32) -> Self
```

**via `wasm_bindgen::convert::traits::IntoWasmAbi`**

```rust
fn into_abi(self) -> u32
```

**via `wasm_bindgen::convert::traits::OptionFromWasmAbi`**

```rust
fn is_none(val: &Self::Abi) -> bool
```

**via `wasm_bindgen::convert::traits::OptionIntoWasmAbi`**

```rust
fn none() -> Self::Abi
```

**via `wasm_bindgen::convert::traits::TryFromJsValue`**

```rust
fn try_from_js_value_ref(value: &wasm_bindgen::JsValue) -> wasm_bindgen::__rt::core::option::Option<Self>
```

**via `wasm_bindgen::convert::traits::VectorFromWasmAbi`**

```rust
unsafe fn vector_from_abi(js: Self::Abi) -> wasm_bindgen::__rt::alloc::boxed::Box<[PositionEncoding]>
```

**via `wasm_bindgen::convert::traits::VectorIntoWasmAbi`**

```rust
fn vector_into_abi(vector: wasm_bindgen::__rt::alloc::boxed::Box<[PositionEncoding]>) -> Self::Abi
```

**via `wasm_bindgen::describe::WasmDescribe`**

```rust
fn describe()
```

**via `wasm_bindgen::describe::WasmDescribeVector`**

```rust
fn describe_vector()
```

---

## SubDiagnosticSeverity

`enum` · `ruff_wasm::SubDiagnosticSeverity`

```rust
enum SubDiagnosticSeverity
```

**Variants**: `Help`, `Info`, `Warning`, `Error`, `Fatal`

**Implements**: `core::convert::From`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: diagnostic::SubDiagnosticSeverity) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## before_main

`function` · `ruff_wasm::before_main`

```rust
fn before_main()
```

---

## init_logging

`function` · `ruff_wasm::init_logging`

```rust
fn init_logging(level: LogLevel)
```

Initializes the logger with the given log level.

## Panics
If this function is called more than once.

---

## run

`function` · `ruff_wasm::run`

```rust
fn run()
```

---

## ExpandedDiagnosticAnnotation

`struct` · `ruff_wasm::ExpandedDiagnosticAnnotation`

```rust
struct ExpandedDiagnosticAnnotation
```

**Fields**: `primary`, `message`, `location`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ExpandedDiagnosticLocation

`struct` · `ruff_wasm::ExpandedDiagnosticLocation`

```rust
struct ExpandedDiagnosticLocation
```

**Fields**: `path`, `start_location`, `end_location`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ExpandedFix

`struct` · `ruff_wasm::ExpandedFix`

```rust
struct ExpandedFix
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ExpandedMessage

`struct` · `ruff_wasm::ExpandedMessage`

```rust
struct ExpandedMessage
```

**Fields**: `code`, `message`, `tags`, `annotations`, `sub_diagnostics`, `start_location`, `end_location`, `fix`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ExpandedSubDiagnostic

`struct` · `ruff_wasm::ExpandedSubDiagnostic`

```rust
struct ExpandedSubDiagnostic
```

**Fields**: `severity`, `message`, `location`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Location

`struct` · `ruff_wasm::Location`

```rust
struct Location
```

**Fields**: `row`, `column`

**Implements**: `core::convert::From`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: SourceLocation) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Workspace

`struct` · `ruff_wasm::Workspace`

```rust
struct Workspace
```

**Implements**: `wasm_bindgen::__rt::marker::SupportsConstructor`, `wasm_bindgen::__rt::marker::SupportsInstanceProperty`, `wasm_bindgen::__rt::marker::SupportsStaticProperty`, `wasm_bindgen::convert::traits::FromWasmAbi`, `wasm_bindgen::convert::traits::IntoWasmAbi`, `wasm_bindgen::convert::traits::LongRefFromWasmAbi`, `wasm_bindgen::convert::traits::OptionFromWasmAbi`, `wasm_bindgen::convert::traits::OptionIntoWasmAbi`, `wasm_bindgen::convert::traits::RefFromWasmAbi`, `wasm_bindgen::convert::traits::RefMutFromWasmAbi`, `wasm_bindgen::convert::traits::TryFromJsValue`, `wasm_bindgen::convert::traits::VectorFromWasmAbi`, `wasm_bindgen::convert::traits::VectorIntoWasmAbi`, `wasm_bindgen::describe::WasmDescribe`, `wasm_bindgen::describe::WasmDescribeVector`

**Methods** (9)

```rust
fn check(&self, contents: &str) -> Result<JsValue, Error>
fn comments(&self, contents: &str) -> Result<String, Error>
fn default_settings() -> Result<JsValue, Error>
fn format(&self, contents: &str) -> Result<String, Error>
fn format_ir(&self, contents: &str) -> Result<String, Error>
fn new(options: JsValue, position_encoding: PositionEncoding) -> Result<Workspace, Error>
fn parse(&self, contents: &str) -> Result<String, Error>
fn tokens(&self, contents: &str) -> Result<String, Error>
fn version() -> String
```

**via `wasm_bindgen::convert::traits::FromWasmAbi`**

```rust
unsafe fn from_abi(js: Self::Abi) -> Self
```

**via `wasm_bindgen::convert::traits::IntoWasmAbi`**

```rust
fn into_abi(self) -> Self::Abi
```

**via `wasm_bindgen::convert::traits::LongRefFromWasmAbi`**

```rust
unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor
```

**via `wasm_bindgen::convert::traits::OptionFromWasmAbi`**

```rust
fn is_none(abi: &Self::Abi) -> bool
```

**via `wasm_bindgen::convert::traits::OptionIntoWasmAbi`**

```rust
fn none() -> Self::Abi
```

**via `wasm_bindgen::convert::traits::RefFromWasmAbi`**

```rust
unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor
```

**via `wasm_bindgen::convert::traits::RefMutFromWasmAbi`**

```rust
unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor
```

**via `wasm_bindgen::convert::traits::TryFromJsValue`**

```rust
fn try_from_js_value(value: wasm_bindgen::JsValue) -> wasm_bindgen::__rt::core::result::Result<Self, wasm_bindgen::JsValue>
fn try_from_js_value_ref(value: &wasm_bindgen::JsValue) -> wasm_bindgen::__rt::core::option::Option<Self>
```

**via `wasm_bindgen::convert::traits::VectorFromWasmAbi`**

```rust
unsafe fn vector_from_abi(js: Self::Abi) -> wasm_bindgen::__rt::alloc::boxed::Box<[Workspace]>
```

**via `wasm_bindgen::convert::traits::VectorIntoWasmAbi`**

```rust
fn vector_into_abi(vector: wasm_bindgen::__rt::alloc::boxed::Box<[Workspace]>) -> Self::Abi
```

**via `wasm_bindgen::describe::WasmDescribe`**

```rust
fn describe()
```

**via `wasm_bindgen::describe::WasmDescribeVector`**

```rust
fn describe_vector()
```

---
