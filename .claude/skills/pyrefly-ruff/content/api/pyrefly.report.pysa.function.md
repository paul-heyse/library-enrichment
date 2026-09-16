# `pyrefly::report::pysa::function`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.report.pysa.function.json`](../model/pyrefly.report.pysa.function.json)

## FunctionId

`enum` · `pyrefly::report::pysa::function::FunctionId`

```rust
enum FunctionId
```

**Variants**: `Function`, `ModuleTopLevel`, `ClassTopLevel`, `ClassField`, `FunctionDecoratedTarget`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn serialize_to_string(&self) -> String
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

Represents a unique identifier for a function **within a module**.

---

## DecoratedFunction

`struct` · `pyrefly::report::pysa::function::DecoratedFunction`

```rust
struct DecoratedFunction<'a>
```

**Fields**: `idx`, `undecorated`

**Derives**: Clone, Copy, Debug

---

## FunctionBaseDefinition

`struct` · `pyrefly::report::pysa::function::FunctionBaseDefinition`

```rust
struct FunctionBaseDefinition
```

**Fields**: `name`, `name_location`, `parent`, `is_overload`, `is_staticmethod`, `is_classmethod`, `is_property_getter`, `is_property_setter`, `is_stub`, `is_def_statement`, `defining_class`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_method(&self) -> bool
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents information about a function definition, collected in a pre-analysis step.
See `FunctionDefinition` for the type exported to Pysa. This only store memory-efficient information.

---

## FunctionRef

`struct` · `pyrefly::report::pysa::function::FunctionRef`

```rust
struct FunctionRef
```

**Fields**: `module_id`, `module_name`, `function_id`, `function_name`

**Implements**: `pyrefly::report::pysa::call_graph::FunctionTrait`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn from_decorated_function(function: &DecoratedFunction<'_>, context: &ModuleAnswersContext) -> Self
fn get_decorated_target(self) -> Option<Self>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ModuleFunctionDefinitions

`struct` · `pyrefly::report::pysa::function::ModuleFunctionDefinitions`

```rust
struct ModuleFunctionDefinitions<GenericFunctionDefinition>
```

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn as_map(&self) -> &HashMap<FunctionId, GenericFunctionDefinition>
fn get(&self, function_id: &FunctionId) -> Option<&GenericFunctionDefinition>
fn new() -> Self
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
