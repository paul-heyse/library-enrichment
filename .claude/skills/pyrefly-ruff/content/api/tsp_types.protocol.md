# `tsp_types::protocol`

Crate `tsp_types` · 71 public items · structured records in [`model/tsp_types.protocol.json`](../model/tsp_types.protocol.json)

## ConnectionTransportKind

`enum` · `tsp_types::protocol::ConnectionTransportKind`

Also reachable as `tsp_types::ConnectionTransportKind`

```rust
enum ConnectionTransportKind
```

**Variants**: `Ipc`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## CustomIntEnum

`enum` · `tsp_types::protocol::CustomIntEnum`

Also reachable as `tsp_types::CustomIntEnum`

```rust
enum CustomIntEnum<T>
```

**Variants**: `Known`, `Custom`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This type allows extending any integer enum to support custom values.

---

## CustomStringEnum

`enum` · `tsp_types::protocol::CustomStringEnum`

Also reachable as `tsp_types::CustomStringEnum`

```rust
enum CustomStringEnum<T>
```

**Variants**: `Known`, `Custom`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This type allows extending any string enum to support custom values.

---

## Declaration

`enum` · `tsp_types::protocol::Declaration`

Also reachable as `tsp_types::Declaration`

```rust
enum Declaration
```

**Variants**: `Regular`, `Synthesized`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## DeclarationCategory

`enum` · `tsp_types::protocol::DeclarationCategory`

Also reachable as `tsp_types::DeclarationCategory`

```rust
enum DeclarationCategory
```

**Variants**: `Intrinsic`, `Variable`, `Param`, `Typeparam`, `Typealias`, `Function`, `Class`, `Import`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Represents the category of a declaration in the type system. This is used to classify declarations such as variables, functions, classes, etc.

---

## DeclarationKind

`enum` · `tsp_types::protocol::DeclarationKind`

Also reachable as `tsp_types::DeclarationKind`

```rust
enum DeclarationKind
```

**Variants**: `Regular`, `Synthesized`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Discriminator for Declaration variants.

---

## LSPId

`enum` · `tsp_types::protocol::LSPId`

Also reachable as `tsp_types::LSPId`

```rust
enum LSPId
```

**Variants**: `Int`, `String`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An identifier to denote a specific request.

---

## LSPIdOptional

`enum` · `tsp_types::protocol::LSPIdOptional`

Also reachable as `tsp_types::LSPIdOptional`

```rust
enum LSPIdOptional
```

**Variants**: `Int`, `String`, `None`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An identifier to denote a specific response.

---

## LSPNull

`enum` · `tsp_types::protocol::LSPNull`

Also reachable as `tsp_types::LSPNull`

```rust
enum LSPNull
```

**Variants**: `None`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This allows a field to always have null or empty value.

---

## LiteralValue

`enum` · `tsp_types::protocol::LiteralValue`

Also reachable as `tsp_types::LiteralValue`

```rust
enum LiteralValue
```

**Variants**: `Int`, `Bool`, `String`, `Enum`, `Sentinel`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## MessageDirection

`enum` · `tsp_types::protocol::MessageDirection`

Also reachable as `tsp_types::MessageDirection`

```rust
enum MessageDirection
```

**Variants**: `ClientToServer`, `ServerToClient`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## OR2

`enum` · `tsp_types::protocol::OR2`

Also reachable as `tsp_types::OR2`

```rust
enum OR2<T, U>
```

**Variants**: `T`, `U`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This allows a field to have two types.

---

## OR3

`enum` · `tsp_types::protocol::OR3`

Also reachable as `tsp_types::OR3`

```rust
enum OR3<T, U, V>
```

**Variants**: `T`, `U`, `V`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This allows a field to have three types.

---

## OR4

`enum` · `tsp_types::protocol::OR4`

Also reachable as `tsp_types::OR4`

```rust
enum OR4<T, U, V, W>
```

**Variants**: `T`, `U`, `V`, `W`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This allows a field to have four types.

---

## OR5

`enum` · `tsp_types::protocol::OR5`

Also reachable as `tsp_types::OR5`

```rust
enum OR5<T, U, V, W, X>
```

**Variants**: `T`, `U`, `V`, `W`, `X`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This allows a field to have five types.

---

## OR6

`enum` · `tsp_types::protocol::OR6`

Also reachable as `tsp_types::OR6`

```rust
enum OR6<T, U, V, W, X, Y>
```

**Variants**: `T`, `U`, `V`, `W`, `X`, `Y`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This allows a field to have six types.

---

## OR7

`enum` · `tsp_types::protocol::OR7`

Also reachable as `tsp_types::OR7`

```rust
enum OR7<T, U, V, W, X, Y, Z>
```

**Variants**: `T`, `U`, `V`, `W`, `X`, `Y`, `Z`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This allows a field to have seven types.

---

## TSPNotificationMethods

`enum` · `tsp_types::protocol::TSPNotificationMethods`

Also reachable as `tsp_types::TSPNotificationMethods`

```rust
enum TSPNotificationMethods
```

**Variants**: `TypeServerSnapshotChanged`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## TSPRequestMethods

`enum` · `tsp_types::protocol::TSPRequestMethods`

Also reachable as `tsp_types::TSPRequestMethods`

```rust
enum TSPRequestMethods
```

**Variants**: `TypeServerConnection`, `TypeServerGetComputedType`, `TypeServerGetDeclaredType`, `TypeServerGetExpectedType`, `TypeServerGetPythonSearchPaths`, `TypeServerGetSnapshot`, `TypeServerGetSupportedProtocolVersion`, `TypeServerResolveImport`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## TSPRequests

`enum` · `tsp_types::protocol::TSPRequests`

Also reachable as `tsp_types::TSPRequests`

```rust
enum TSPRequests
```

**Variants**: `ConnectionRequest`, `GetComputedTypeRequest`, `GetDeclaredTypeRequest`, `GetExpectedTypeRequest`, `GetPythonSearchPathsRequest`, `GetSnapshotRequest`, `GetSupportedProtocolVersionRequest`, `ResolveImportRequest`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Type

`enum` · `tsp_types::protocol::Type`

Also reachable as `tsp_types::Type`

```rust
enum Type
```

**Variants**: `BuiltInType`, `Declared`, `Function`, `Class`, `Union`, `Module`, `Var`, `Overloaded`, `Synthesized`, `Reference`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## TypeKind

`enum` · `tsp_types::protocol::TypeKind`

Also reachable as `tsp_types::TypeKind`

```rust
enum TypeKind
```

**Variants**: `Builtin`, `Declared`, `Function`, `Class`, `Union`, `Module`, `Typevar`, `Overloaded`, `Synthesized`, `Typereference`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Discriminator for the Type union, identifying which variant a type is.

---

## TypeServerVersion

`enum` · `tsp_types::protocol::TypeServerVersion`

Also reachable as `tsp_types::TypeServerVersion`

```rust
enum TypeServerVersion
```

**Variants**: `V010`, `V020`, `V030`, `V040`, `Current`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Variance

`enum` · `tsp_types::protocol::Variance`

Also reachable as `tsp_types::Variance`

```rust
enum Variance
```

**Variants**: `Auto`, `Unknown`, `Invariant`, `Covariant`, `Contravariant`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## BuiltInType

`struct` · `tsp_types::protocol::BuiltInType`

Also reachable as `tsp_types::BuiltInType`

```rust
struct BuiltInType
```

**Fields**: `declaration`, `flags`, `id`, `kind`, `name`, `possible_type`, `type_alias_info`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents special built-in types that are fundamental to Python's type system. These are not regular classes but represent special semantic meanings. Used for: - Type inference failures (unknown) - Gradual typing (any) - Uninitialized variables (unbound) - Special literals (ellipsis for ...) - Non-returning functions (never/noreturn) Examples: - `unknown`: `x` in `def foo(x):` with no type hints and no usage to infer from - `any`: Explicit `Any` annotation or from untyped imports - `unbound`: Variable declared but not yet assigned: `x: int` (before assignment) - `ellipsis`: The `...` in `def foo(...): ...` or `Tuple[int, ...]` - `never`: `def raise_error() -> Never:` or function with only raise statements

---

## ClassType

`struct` · `tsp_types::protocol::ClassType`

Also reachable as `tsp_types::ClassType`

```rust
struct ClassType
```

**Fields**: `declaration`, `flags`, `id`, `kind`, `literal_value`, `type_alias_info`, `type_args`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a class or class instance that has a declaration in the source code. Used for classes parsed from actual `class` statements. Uses TypeKind.Class for discrimination from FunctionType and other types. Used for: - User-defined classes with `class` statements - Class instances (instances of user-defined classes) - Specialized generic classes (e.g., `MyClass[int]`) - Literal instances (e.g., the number `42` is an instance of `int`) Not used for: - Built-in classes like `int`, `str`, `list` (use SynthesizedType) - Classes synthesized by decorators (use SynthesizedType) Example: ```python class Point: x: int y: int class Container[T]: value: T # point has ClassType (instance of Point) point = Point() # container has ClassType with typeArgs=[int] container: Container[int] = Container() ```

---

## ConnectionRequest

`struct` · `tsp_types::protocol::ConnectionRequest`

Also reachable as `tsp_types::ConnectionRequest`

```rust
struct ConnectionRequest
```

**Fields**: `jsonrpc`, `method`, `id`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Main-connection-only request used to open or close an extra TSP transport. Extra transports must remain read-only and must not be used for LSP traffic.

---

## ConnectionRequestParams

`struct` · `tsp_types::protocol::ConnectionRequestParams`

Also reachable as `tsp_types::ConnectionRequestParams`

```rust
struct ConnectionRequestParams
```

**Fields**: `args`, `kind`, `type_`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Main-connection-only control request used to open or close extra read-only TSP channels after LSP initialization has completed.

---

## ConnectionRequestResult

`struct` · `tsp_types::protocol::ConnectionRequestResult`

Also reachable as `tsp_types::ConnectionRequestResult`

```rust
struct ConnectionRequestResult
```

**Fields**: `message`, `success`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## DeclarationBase

`struct` · `tsp_types::protocol::DeclarationBase`

Also reachable as `tsp_types::DeclarationBase`

```rust
struct DeclarationBase
```

**Fields**: `kind`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Base interface for all declaration types. Provides the discriminator field for the Declaration union. This is a generic interface that is extended by: - RegularDeclaration (kind = Regular) - SynthesizedDeclaration (kind = Synthesized) The type parameter T ensures that the kind field matches the implementing interface. Used for type-safe discrimination: ```typescript if (declaration.kind === DeclarationKind.Regular) { // TypeScript knows this is RegularDeclaration const node = declaration.node; } ```

---

## DeclaredType

`struct` · `tsp_types::protocol::DeclaredType`

Also reachable as `tsp_types::DeclaredType`

```rust
struct DeclaredType
```

**Fields**: `declaration`, `flags`, `id`, `kind`, `type_alias_info`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Base type for symbols that have a declaration in source code. This is the common parent for FunctionType and ClassType when the type comes from an actual declaration node in the parse tree. The type parameter T allows subtypes to specify their own TypeKind (e.g., Function or Class) while sharing the common declaration field. Used for: - Functions and methods with actual `def` statements (TypeKind.Function) - Classes with actual `class` statements (TypeKind.Class) - Variables with declarations in source (TypeKind.Declared) Not used for: - Synthesized types (use SynthesizedType) - Built-in types (use BuiltInType) Example: ```python def my_function(x: int) -> str:  # FunctionType with TypeKind.Function return str(x) class MyClass:  # ClassType with TypeKind.Class pass ```

---

## EnumLiteral

`struct` · `tsp_types::protocol::EnumLiteral`

Also reachable as `tsp_types::EnumLiteral`

```rust
struct EnumLiteral
```

**Fields**: `class_name`, `item_name`, `item_type`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a literal value from an Enum. Used to track specific enum members as literal types. Fields: - className: Name of the enum class - itemName: Name of the specific enum member - itemType: Type of the enum member's value Examples: ```python from enum import Enum class Color(Enum): RED = 1 GREEN = 2 BLUE = 3 # Color.RED is an EnumLiteral: # className="Color", itemName="RED", itemType=int (for value 1) def process(color: Literal[Color.RED]) -> None: pass  # EnumLiteral tracks that it's specifically Color.RED ```

---

## FunctionType

`struct` · `tsp_types::protocol::FunctionType`

Also reachable as `tsp_types::FunctionType`

```rust
struct FunctionType
```

**Fields**: `bound_to_type`, `declaration`, `flags`, `id`, `kind`, `return_type`, `specialized_types`, `type_alias_info`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a function or method that has a declaration in the source code. Used for functions parsed from actual `def` statements. Uses TypeKind.Function for discrimination from ClassType and other types. Binding behavior: - boundToType: Contains the class/instance the method is bound to. Used for: - User-defined functions with `def` statements - Methods declared in source classes - Lambda functions (though simple ones) Not used for: - Built-in functions like `len`, `print` (use SynthesizedType) - Synthesized methods from decorators like @dataclass (use SynthesizedType) Example: ```python def calculate(x: int, y: int) -> int: return x + y class MyClass: def method(self, value: str) -> None: pass ```

---

## GetComputedTypeRequest

`struct` · `tsp_types::protocol::GetComputedTypeRequest`

Also reachable as `tsp_types::GetComputedTypeRequest`

```rust
struct GetComputedTypeRequest
```

**Fields**: `method`, `id`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Requests and notifications for the type server protocol. Request for the computed type of a declaration or node. Computed type is the type that is inferred based on the code flow. Example: def foo(a: int | str): if instanceof(a, int): b = a + 1  # Computed type of 'b' is 'int'

---

## GetDeclaredTypeRequest

`struct` · `tsp_types::protocol::GetDeclaredTypeRequest`

Also reachable as `tsp_types::GetDeclaredTypeRequest`

```rust
struct GetDeclaredTypeRequest
```

**Fields**: `method`, `id`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Request for the declared type of a declaration or node. Declared type is the type that is explicitly declared in the source code. Example: def foo(a: int | str): # Declared type of parameter 'a' is 'int | str' pass

---

## GetExpectedTypeRequest

`struct` · `tsp_types::protocol::GetExpectedTypeRequest`

Also reachable as `tsp_types::GetExpectedTypeRequest`

```rust
struct GetExpectedTypeRequest
```

**Fields**: `method`, `id`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Request for the expected type of a declaration or node. Expected type is the type that the context expects. Example: def foo(a: int | str): pass foo(4)  # Expected type of argument 'a' is 'int | str'

---

## GetPythonSearchPathsParams

`struct` · `tsp_types::protocol::GetPythonSearchPathsParams`

Also reachable as `tsp_types::GetPythonSearchPathsParams`

```rust
struct GetPythonSearchPathsParams
```

**Fields**: `from_uri`, `snapshot`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Parameters for the GetPythonSearchPathsRequest. Requests the list of directories that Python searches for modules and packages. The search paths include: - Standard library directories - Site-packages directories (third-party packages) - Virtual environment paths (if active) - Project-specific paths (PYTHONPATH, src directories) Used for: - Resolving import statements to find module files - Auto-import suggestions - Determining which packages are available Example search paths: ``` [ "/usr/lib/python3.11",              # Standard library "/venv/lib/python3.11/site-packages",  # Virtual env packages "/project/src"                       # Project source ] ```

---

## GetPythonSearchPathsRequest

`struct` · `tsp_types::protocol::GetPythonSearchPathsRequest`

Also reachable as `tsp_types::GetPythonSearchPathsRequest`

```rust
struct GetPythonSearchPathsRequest
```

**Fields**: `method`, `id`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Request to get the search paths that the type server uses for Python modules.

---

## GetSnapshotRequest

`struct` · `tsp_types::protocol::GetSnapshotRequest`

Also reachable as `tsp_types::GetSnapshotRequest`

```rust
struct GetSnapshotRequest
```

**Fields**: `method`, `id`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Request from client to get the current snapshot of the type server. A snapshot is a point-in-time representation of the type server's state, including all loaded files and their types. A type server should change its snapshot whenever any type it might have returned is no longer valid. Meaning types are only usable for the snapshot they were returned with. Snapshots are not meant to survive any changes that would make the type server throw away its internal cache. They are merely an identifier to indicate to the client that the type server will accept requests for types from that snapshot.

---

## GetSupportedProtocolVersionRequest

`struct` · `tsp_types::protocol::GetSupportedProtocolVersionRequest`

Also reachable as `tsp_types::GetSupportedProtocolVersionRequest`

```rust
struct GetSupportedProtocolVersionRequest
```

**Fields**: `method`, `id`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Request to get the version of the protocol the type server supports. Returns a string representation of the protocol version (should be semver format)

---

## ModuleName

`struct` · `tsp_types::protocol::ModuleName`

Also reachable as `tsp_types::ModuleName`

```rust
struct ModuleName
```

**Fields**: `leading_dots`, `name_parts`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a Python module name, handling both absolute and relative imports. Used for: - Import statement resolution - Tracking module dependencies - Resolving relative imports (from . import, from .. import) Examples: - `import os.path`: leadingDots=0, nameParts=['os', 'path'] - `from . import utils`: leadingDots=1, nameParts=['utils'] - `from ...parent import module`: leadingDots=3, nameParts=['parent', 'module'] - `import mymodule`: leadingDots=0, nameParts=['mymodule']

---

## ModuleType

`struct` · `tsp_types::protocol::ModuleType`

Also reachable as `tsp_types::ModuleType`

```rust
struct ModuleType
```

**Fields**: `flags`, `id`, `kind`, `module_name`, `type_alias_info`, `uri`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a Python module as a type. Used when a module object itself is referenced (not its contents). Used for: - Module imports: `import os` makes `os` a ModuleType - Module attributes accessed via __file__, __name__, etc. - Submodule references: `os.path` is also a ModuleType The loaderFields contain all the symbols exported by the module that would be accessible via attribute access (module.symbol_name). Examples: ```python import os import os.path as path from typing import Protocol # `os` has ModuleType with loaderFields containing {"path": ..., "getcwd": ..., etc.} # `path` has ModuleType for the os.path module # In type stubs, Protocol is a module symbol that gets loaded ```

---

## Node

`struct` · `tsp_types::protocol::Node`

Also reachable as `tsp_types::Node`

```rust
struct Node
```

**Fields**: `range`, `uri`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a location in source code (a node in the AST). Used to point to specific declarations, expressions, or statements in Python source files. Used for: - Pointing to where a type is declared - Identifying the location of expressions for type inference - Error reporting and diagnostics - Linking types back to their source definitions Examples: - For `def foo():`, the node points to the function declaration - For a variable `x = 42`, the node points to the assignment - For default parameter values in functions

---

## OverloadedType

`struct` · `tsp_types::protocol::OverloadedType`

Also reachable as `tsp_types::OverloadedType`

```rust
struct OverloadedType
```

**Fields**: `flags`, `id`, `implementation`, `kind`, `overloads`, `type_alias_info`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents an overloaded function with multiple signatures. Used when a function has multiple `@overload` decorators defining different call signatures. Used for: - Functions with @overload decorators - Built-in functions with multiple signatures (e.g., `range(stop)` vs `range(start, stop, step)`) - Methods with different signatures for different argument types The `overloads` array contains all the @overload signatures, and `implementation` contains the actual implementation (if present). Examples: ```python from typing import overload @overload def process(value: int) -> str: ... @overload def process(value: str) -> int: ... def process(value: int | str) -> int | str: if isinstance(value, int): return str(value) return len(value) # The type of `process` is OverloadedType with: # - overloads = [signature for (int)->str, signature for (str)->int] # - implementation = signature for (int|str)->(int|str) ```

---

## Position

`struct` · `tsp_types::protocol::Position`

Also reachable as `tsp_types::Position`

```rust
struct Position
```

**Fields**: `character`, `line`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Position in a text document expressed as zero-based line and character offset.

---

## Range

`struct` · `tsp_types::protocol::Range`

Also reachable as `tsp_types::Range`

```rust
struct Range
```

**Fields**: `end`, `start`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A range in a text document expressed as (zero-based) start and end positions.

---

## RegularDeclaration

`struct` · `tsp_types::protocol::RegularDeclaration`

Also reachable as `tsp_types::RegularDeclaration`

```rust
struct RegularDeclaration
```

**Fields**: `category`, `kind`, `name`, `node`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a declaration that exists in source code. Points to the actual AST node where a symbol is declared. Fields: - category: Type of declaration (Variable, Function, Class, etc.) - node: AST node pointing to the declaration location - name: Name of the declared symbol (undefined for anonymous/implicit declarations) Examples: ```python def my_function(x: int) -> str:  # Function declaration return str(x) class MyClass:  # Class declaration x: int      # Variable declaration T = TypeVar('T')  # TypeParam declaration ```

---

## ResolveImportOptions

`struct` · `tsp_types::protocol::ResolveImportOptions`

Also reachable as `tsp_types::ResolveImportOptions`

```rust
struct ResolveImportOptions
```

**Fields**: `allow_externally_hidden_access`, `resolve_local_names`, `skip_file_needed_check`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Options for customizing import resolution behavior. Controls how the type server resolves import statements and accesses imported symbols. Used for: - Fine-tuning import resolution during type checking - Controlling access to private/hidden module members - Optimizing resolution by skipping file checks TODO: See if we can remove this as these are pretty specific to Pyright at the moment. Examples: ```python # resolveLocalNames affects whether local assignments are resolved: from module import name name = something_else  # Does 'name' refer to import or local assignment? # allowExternallyHiddenAccess affects access to _private names: from module import _internal_function  # Normally hidden from external access ```

---

## ResolveImportParams

`struct` · `tsp_types::protocol::ResolveImportParams`

Also reachable as `tsp_types::ResolveImportParams`

```rust
struct ResolveImportParams
```

**Fields**: `module_descriptor`, `snapshot`, `source_uri`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Parameters for the ResolveImportRequest. Provides the context needed to resolve a Python import statement to its file location. Used when: - Resolving `import` or `from...import` statements - Finding the file that contains an imported module - Navigating to imported symbols Examples: ```python # In file.py: from os.path import join  # sourceUri = file.py, moduleDescriptor = os.path import mymodule          # sourceUri = file.py, moduleDescriptor = mymodule from . import utils      # sourceUri = file.py, moduleDescriptor = .utils (relative) ```

---

## ResolveImportRequest

`struct` · `tsp_types::protocol::ResolveImportRequest`

Also reachable as `tsp_types::ResolveImportRequest`

```rust
struct ResolveImportRequest
```

**Fields**: `method`, `id`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Request to resolve an import. This is used to resolve the import name to its location in the file system.

---

## SentinelLiteral

`struct` · `tsp_types::protocol::SentinelLiteral`

Also reachable as `tsp_types::SentinelLiteral`

```rust
struct SentinelLiteral
```

**Fields**: `class_name`, `class_node`, `module_name`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a sentinel value (a unique object used as a marker). Used for special singleton values that act as sentinels in APIs. Fields: - classNode: AST node where the sentinel class is defined - moduleName: Module containing the sentinel - className: Name of the sentinel class Examples: ```python # Common sentinel pattern class _Sentinel: pass MISSING = _Sentinel() def get_value(key: str, default: int | _Sentinel = MISSING) -> int: ... # MISSING is a SentinelLiteral pointing to the _Sentinel class instance # Used in standard library (e.g., dataclasses.MISSING) from dataclasses import field, MISSING # MISSING is tracked as a SentinelLiteral ```

---

## SnapshotChangedNotification

`struct` · `tsp_types::protocol::SnapshotChangedNotification`

Also reachable as `tsp_types::SnapshotChangedNotification`

```rust
struct SnapshotChangedNotification
```

**Fields**: `jsonrpc`, `method`, `params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Notification sent by the server to indicate any outstanding snapshots are invalid.

---

## SpecializedFunctionTypes

`struct` · `tsp_types::protocol::SpecializedFunctionTypes`

Also reachable as `tsp_types::SpecializedFunctionTypes`

```rust
struct SpecializedFunctionTypes
```

**Fields**: `parameter_default_types`, `parameter_types`, `return_type`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents specialized (concrete) types for a generic function's parameters and return type. Used when generic type parameters are substituted with actual types. Fields: - parameterTypes: Concrete types for each parameter after type variable substitution - parameterDefaultTypes: Specialized types for default values (if different from declared) - returnType: Specialized return type after type variable substitution Examples: ```python # Generic function def identity[T](x: T) -> T: return x # When called as identity[int](42): # - parameterTypes = [int] (T substituted with int) # - returnType = int (T substituted with int) # For list.append bound to list[str]: # - parameterTypes = [str] (specialized from generic T) ```

---

## SynthesizedDeclaration

`struct` · `tsp_types::protocol::SynthesizedDeclaration`

Also reachable as `tsp_types::SynthesizedDeclaration`

```rust
struct SynthesizedDeclaration
```

**Fields**: `kind`, `uri`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a synthesized declaration (not in source code). Used for implicitly created symbols like built-in types or decorator-generated members. Fields: - uri: The file URI where this is conceptually declared (often the module using it) Examples: ```python # Built-in functions have synthesized declarations len([1, 2, 3])  # len is synthesized, not from source # @dataclass generates __init__, __eq__, etc. - synthesized declarations @dataclass class Point: x: int y: int # Point.__init__ is synthesized ```

---

## SynthesizedType

`struct` · `tsp_types::protocol::SynthesizedType`

Also reachable as `tsp_types::SynthesizedType`

```rust
struct SynthesizedType
```

**Fields**: `flags`, `id`, `kind`, `metadata`, `stub_content`, `type_alias_info`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents synthesized/generated types. When the type server generates its own types that do not directly correspond to source code declarations, it uses this handle. The stub content should be a complete, valid Python stub (.pyi) that includes: 1. All necessary imports (typing module, collections.abc, etc.) 2. TypeVar and ParamSpec declarations used in the type 3. Type aliases or class definitions 4. Function signatures with full parameter and return type annotations This approach is particularly useful for: - Synthesized methods from decorators like @dataclass.__init__ - NewType declarations - Generic type specializations Examples: # Example 1: Synthesized dataclass __init__ from dataclasses import dataclass @dataclass class Point: x: int y: int # Stub content for Point.__init__: """ def __init__(self, x: int, y: int) -> None: '''Initialize Point''' """ # metadata: { primaryDefinitionOffset: 0 } # Example 2: Generic function with TypeVar from typing import TypeVar T = TypeVar('T') def identity(x: T) -> T: return x # Stub content: """ from typing import TypeVar T = TypeVar('T') def identity(x: T) -> T: ... """ # metadata: { primaryDefinitionOffset: 45 } (offset points to 'def') # Example 3: ParamSpec function from typing import ParamSpec, Callable P = ParamSpec('P') def wrapper(func: Callable[P, int]) -> Callable[P, str]: ... # Stub content: """ from typing import ParamSpec, Callable P = ParamSpec('P') def wrapper(func: Callable[P, int]) -> Callable[P, str]: ... """ # metadata: { primaryDefinitionOffset: 67 } # Example 4: NewType from typing import NewType UserId = NewType('UserId', int) # Stub content: """ from typing import NewType UserId = NewType('UserId', int) """ # metadata: { primaryDefinitionOffset: 25 } (offset points to 'UserId') # Example 5: Complex generic specialization with ParamSpec class Wrapper[P, R]: func: Callable[P, R] def example(x: int, y: str) -> bool: ... w: Wrapper[(x: int, y: str), bool] = Wrapper() # Stub content for Wrapper[(x: int, y: str), bool]: """ from typing import ParamSpec, TypeVar, Generic, Callable P = ParamSpec('P') R = TypeVar('R') class Wrapper(Generic[P, R]): func: Callable[P, R] """ # metadata: { primaryDefinitionOffset: 87 } (offset points to 'class Wrapper') ``` Important: The stub content is used to reconstruct the type on the client side by: 1. Parsing the stub as a Python type stub file 2. Evaluating the type expressions within the stub 3. Extracting the resulting type for use in type checking and IntelliSense

---

## SynthesizedTypeMetadata

`struct` · `tsp_types::protocol::SynthesizedTypeMetadata`

Also reachable as `tsp_types::SynthesizedTypeMetadata`

```rust
struct SynthesizedTypeMetadata
```

**Fields**: `module`, `primary_definition_offset`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metadata about a synthesized type that provides additional context. This information is used by the client to enhance IntelliSense and type checking.

---

## TypeAliasInfo

`struct` · `tsp_types::protocol::TypeAliasInfo`

Also reachable as `tsp_types::TypeAliasInfo`

```rust
struct TypeAliasInfo
```

**Fields**: `computed_variance`, `file_uri`, `full_name`, `is_type_alias_type`, `module_name`, `name`, `scope_id`, `type_args`, `type_params`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Contains metadata about a type alias. Used when a type is created through a type alias statement (PEP 613) or traditional assignment. Fields: - name: Short name of the alias - fullName: Fully qualified name including module path - moduleName: Module where the alias is defined - fileUri: File location of the alias definition - scopeId: Scope identifier for the alias (for scoped type variables) - isTypeAliasType: True if this uses the `type` keyword (PEP 695) - typeParams: Generic type parameters declared by the alias - typeArgs: Concrete type arguments when the alias is specialized - computedVariance: Inferred variance for type parameters Examples: ```python # PEP 695 style (isTypeAliasType=true) type IntList = list[int] # Traditional style (isTypeAliasType=false) IntList = list[int] # Generic alias with type parameters type Pair[T] = tuple[T, T] # typeParams=[T], can be specialized to Pair[int] # Using typing.TypeAlias from typing import TypeAlias UserId: TypeAlias = int ```

---

## TypeBase

`struct` · `tsp_types::protocol::TypeBase`

Also reachable as `tsp_types::TypeBase`

```rust
struct TypeBase
```

**Fields**: `flags`, `id`, `kind`, `type_alias_info`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Base interface for all Type variants. Provides common fields shared by all type representations in the protocol. This is the foundation interface extended by all Type types: - BuiltInType - RegularType (and its subclasses FunctionType, ClassType) - UnionType - ModuleType - TypeVarType - OverloadedType - SynthesizedType - TypeReference The type parameter T constrains the `kind` field to match the implementing type. Common fields: - id: Unique identifier for cycle detection and caching - kind: Discriminator for the Type union - flags: Characteristics of the type (Instantiable, Instance, Callable, etc.) - typeAliasInfo: Optional alias information if type comes from a type alias Used throughout the protocol to represent Python types in a serializable format.

---

## TypeFlags

`struct` · `tsp_types::protocol::TypeFlags`

Also reachable as `tsp_types::TypeFlags`

```rust
struct TypeFlags
```

**Implements**: `core::ops::bit::BitAnd`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn contains(self, other: Self) -> bool
fn new() -> Self
fn with_callable(self) -> Self
fn with_from_alias(self) -> Self
fn with_generic(self) -> Self
fn with_instance(self) -> Self
fn with_instantiable(self) -> Self
fn with_interface(self) -> Self
fn with_literal(self) -> Self
fn with_optional(self) -> Self
fn with_unbound(self) -> Self
fn with_unpacked(self) -> Self
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, rhs: TypeFlags) -> TypeFlags
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, rhs: TypeFlags) -> TypeFlags
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, rhs: TypeFlags)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(d: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

Flags that describe the characteristics of a type. These flags can be combined using bitwise operations.

---

## TypeReferenceType

`struct` · `tsp_types::protocol::TypeReferenceType`

Also reachable as `tsp_types::TypeReferenceType`

```rust
struct TypeReferenceType
```

**Fields**: `flags`, `id`, `kind`, `type_alias_info`, `type_reference_id`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a reference to another type by its ID. Used to avoid duplicating large type structures and to handle forward references. Used for: - Deduplication: When the same type appears multiple times, subsequent occurrences can reference the first occurrence instead of duplicating all fields - Cyclic references: Breaking cycles in recursive type definitions - Large types: Reducing payload size for complex types used repeatedly This is an optimization mechanism in the protocol to keep type handles compact when transmitting over the wire. Examples: ```python # Recursive type definition class Node: value: int next: Node | None  # 'Node' references back to itself # When serializing the type of 'next', the second occurrence of Node # uses TypeReferenceType pointing to the first Node's ID # Repeated complex type def process_lists( list1: list[dict[str, int]], list2: list[dict[str, int]],  # Can reference the type from list1 list3: list[dict[str, int]]   # Can reference the type from list1 ) -> None: pass ```

---

## TypeServerMultiConnectionCapability

`struct` · `tsp_types::protocol::TypeServerMultiConnectionCapability`

Also reachable as `tsp_types::TypeServerMultiConnectionCapability`

```rust
struct TypeServerMultiConnectionCapability
```

**Fields**: `supported_transports`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Capability shape exchanged via the LSP initialize request/response under `capabilities.experimental.typeServerMultiConnection`.

---

## UnionType

`struct` · `tsp_types::protocol::UnionType`

Also reachable as `tsp_types::UnionType`

```rust
struct UnionType
```

**Fields**: `flags`, `id`, `kind`, `sub_types`, `type_alias_info`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a union of multiple types (Type1 | Type2 | ...). Used when a value can be one of several different types. Used for: - Explicit union type annotations using `|` or `Union[...]` - Optional types (which are unions with None) - Type narrowing results (e.g., after isinstance checks) - Inferred types from multiple branches Examples: ```python # Explicit union annotation def process(value: int | str) -> None: pass # Optional (union with None) def find(key: str) -> str | None: return None # Inferred union from branches if condition: x = 42        # int else: x = "hello"  # str # x has type int | str ```

---

## ConnectionResponse

`type_alias` · `tsp_types::protocol::ConnectionResponse`

Also reachable as `tsp_types::ConnectionResponse`

```rust
type ConnectionResponse = ConnectionRequestResult
```

Response to the [ConnectionRequest].

---

## GetComputedTypeResponse

`type_alias` · `tsp_types::protocol::GetComputedTypeResponse`

Also reachable as `tsp_types::GetComputedTypeResponse`

```rust
type GetComputedTypeResponse = Type
```

Response to the [GetComputedTypeRequest].

---

## GetDeclaredTypeResponse

`type_alias` · `tsp_types::protocol::GetDeclaredTypeResponse`

Also reachable as `tsp_types::GetDeclaredTypeResponse`

```rust
type GetDeclaredTypeResponse = Type
```

Response to the [GetDeclaredTypeRequest].

---

## GetExpectedTypeResponse

`type_alias` · `tsp_types::protocol::GetExpectedTypeResponse`

Also reachable as `tsp_types::GetExpectedTypeResponse`

```rust
type GetExpectedTypeResponse = Type
```

Response to the [GetExpectedTypeRequest].

---

## GetPythonSearchPathsResponse

`type_alias` · `tsp_types::protocol::GetPythonSearchPathsResponse`

Also reachable as `tsp_types::GetPythonSearchPathsResponse`

```rust
type GetPythonSearchPathsResponse = Vec<String>
```

Response to the [GetPythonSearchPathsRequest].

---

## GetSnapshotResponse

`type_alias` · `tsp_types::protocol::GetSnapshotResponse`

Also reachable as `tsp_types::GetSnapshotResponse`

```rust
type GetSnapshotResponse = i32
```

Response to the [GetSnapshotRequest].

---

## GetSupportedProtocolVersionResponse

`type_alias` · `tsp_types::protocol::GetSupportedProtocolVersionResponse`

Also reachable as `tsp_types::GetSupportedProtocolVersionResponse`

```rust
type GetSupportedProtocolVersionResponse = String
```

Response to the [GetSupportedProtocolVersionRequest].

---

## ResolveImportResponse

`type_alias` · `tsp_types::protocol::ResolveImportResponse`

Also reachable as `tsp_types::ResolveImportResponse`

```rust
type ResolveImportResponse = String
```

Response to the [ResolveImportRequest].

---

## TypeVarType

`type_alias` · `tsp_types::protocol::TypeVarType`

Also reachable as `tsp_types::TypeVarType`

```rust
type TypeVarType = DeclaredType
```

---
