# `datafusion_expr::registry`

Crate `datafusion-expr` · 9 public items · structured records in [`model/datafusion_expr.registry.json`](../model/datafusion_expr.registry.json)

## ExtensionTypeRegistration

`struct` · `datafusion_expr::registry::ExtensionTypeRegistration`

```rust
struct ExtensionTypeRegistration
```

**Derives**: Debug

**Methods** (3)

```rust
fn create_df_extension_type(&self, storage_type: &DataType, metadata: Option<&str>) -> Result<DFExtensionTypeRef>
fn new_arc(name: impl Into<String>, factory: impl Fn(&DataType, Option<&str>) -> Result<DFExtensionTypeRef> + Send + Sync + 'static) -> ExtensionTypeRegistrationRef
fn type_name(&self) -> &str
```

The registration of an extension type. Implementations of this trait are responsible for
*creating* instances of [`DFExtensionType`] that represent the entire semantics of an extension
type.

# Why do we need a Registration?

A good question is why this trait is even necessary. Why not directly register the
[`DFExtensionType`] in a registry?

While this works for extension types requiring no additional metadata (e.g., `arrow.uuid`), it
does not work for more complex extension types with metadata. For example, consider an extension
type `custom.shortened(n)` that aims to short the pretty-printing string to `n` characters.
Here, `n` is a parameter of the extension type and should be a field in the struct that
implements the [`DFExtensionType`]. The job of the registration is to read the metadata from the
field and create the corresponding [`DFExtensionType`] instance with the correct `n` set.

[`DFExtensionType`]: datafusion_common::types::DFExtensionType

---

## MemoryExtensionTypeRegistry

`struct` · `datafusion_expr::registry::MemoryExtensionTypeRegistry`

```rust
struct MemoryExtensionTypeRegistry
```

**Implements**: `core::convert::From`, `datafusion_expr::registry::ExtensionTypeRegistry`

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn all_extension_types(&self) -> Vec<ExtensionTypeRegistrationRef>
fn new_empty() -> Self
fn new_with_canonical_extension_types() -> Self
fn new_with_types(types: impl IntoIterator<Item = ExtensionTypeRegistrationRef>) -> Result<Self>
```

**via `core::convert::From`**

```rust
fn from(value: HashMap<String, ExtensionTypeRegistrationRef>) -> Self
```

**via `datafusion_expr::registry::ExtensionTypeRegistry`**

```rust
fn add_extension_type_registration(&self, extension_type: ExtensionTypeRegistrationRef) -> Result<Option<ExtensionTypeRegistrationRef>>
fn extension_type_registration(&self, name: &str) -> Result<ExtensionTypeRegistrationRef>
fn extension_type_registrations(&self) -> Vec<ExtensionTypeRegistrationRef>
fn remove_extension_type_registration(&self, name: &str) -> Result<Option<ExtensionTypeRegistrationRef>>
```

An [`ExtensionTypeRegistry`] that uses in memory [`HashMap`]s.

---

## MemoryFunctionRegistry

`struct` · `datafusion_expr::registry::MemoryFunctionRegistry`

Also reachable as `datafusion_execution::registry::MemoryFunctionRegistry`

```rust
struct MemoryFunctionRegistry
```

**Implements**: `datafusion_expr::registry::FunctionRegistry`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::registry::FunctionRegistry`**

```rust
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>>
fn higher_order_function_names(&self) -> HashSet<String>
fn register_higher_order_function(&mut self, function: Arc<HigherOrderUDF>) -> Result<Option<Arc<HigherOrderUDF>>>
fn register_udaf(&mut self, udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>>
fn register_udf(&mut self, udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>>
fn register_udwf(&mut self, udaf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>>
fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>>
fn udafs(&self) -> HashSet<String>
fn udf(&self, name: &str) -> Result<Arc<ScalarUDF>>
fn udfs(&self) -> HashSet<String>
fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>>
fn udwfs(&self) -> HashSet<String>
```

A  [`FunctionRegistry`] that uses in memory [`HashMap`]s

---

## ExtensionTypeRegistry

`trait` · `datafusion_expr::registry::ExtensionTypeRegistry`

```rust
trait ExtensionTypeRegistry: Debug + Send + Sync
```

**Implementors** (1)

- `datafusion_expr::registry::MemoryExtensionTypeRegistry`

**Methods** (6)

```rust
fn add_extension_type_registration(&self, extension_type: ExtensionTypeRegistrationRef) -> Result<Option<ExtensionTypeRegistrationRef>>
fn create_extension_type_for_field(&self, field: &Field) -> Result<Option<DFExtensionTypeRef>>
fn extend(&self, extension_types: &[ExtensionTypeRegistrationRef]) -> Result<()>
fn extension_type_registration(&self, name: &str) -> Result<ExtensionTypeRegistrationRef>
fn extension_type_registrations(&self) -> Vec<ExtensionTypeRegistrationRef>
fn remove_extension_type_registration(&self, name: &str) -> Result<Option<ExtensionTypeRegistrationRef>>
```

Manages [`ExtensionTypeRegistration`]s, which allow users to register custom behavior for
extension types.

Each registration is connected to the extension type name, which can also be looked up to get
the registration.

---

## FunctionRegistry

`trait` · `datafusion_expr::registry::FunctionRegistry`

Also reachable as `datafusion::execution::FunctionRegistry`, `datafusion_execution::FunctionRegistry`, `datafusion_execution::registry::FunctionRegistry`

```rust
trait FunctionRegistry
```

**Implementors** (4)

- `datafusion::execution::context::SessionContext`
- `datafusion::execution::session_state::SessionState`
- `datafusion_execution::task::TaskContext`
- `datafusion_expr::registry::MemoryFunctionRegistry`

**Methods** (19)

```rust
fn deregister_higher_order_function(&mut self, _name: &str) -> Result<Option<Arc<HigherOrderUDF>>>
fn deregister_udaf(&mut self, _name: &str) -> Result<Option<Arc<AggregateUDF>>>
fn deregister_udf(&mut self, _name: &str) -> Result<Option<Arc<ScalarUDF>>>
fn deregister_udwf(&mut self, _name: &str) -> Result<Option<Arc<WindowUDF>>>
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>>
fn higher_order_function_names(&self) -> HashSet<String>
fn register_expr_planner(&mut self, _expr_planner: Arc<dyn ExprPlanner>) -> Result<()>
fn register_function_rewrite(&mut self, _rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> Result<()>
fn register_higher_order_function(&mut self, _function: Arc<HigherOrderUDF>) -> Result<Option<Arc<HigherOrderUDF>>>
fn register_udaf(&mut self, _udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>>
fn register_udf(&mut self, _udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>>
fn register_udwf(&mut self, _udaf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>>
fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>>
fn udafs(&self) -> HashSet<String>
fn udf(&self, name: &str) -> Result<Arc<ScalarUDF>>
fn udfs(&self) -> HashSet<String>
fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>>
fn udwfs(&self) -> HashSet<String>
```

A registry knows how to build logical expressions out of user-defined function' names

---

## SerializerRegistry

`trait` · `datafusion_expr::registry::SerializerRegistry`

Also reachable as `datafusion_execution::registry::SerializerRegistry`

```rust
trait SerializerRegistry: Debug + Send + Sync
```

**Implementors** (1)

- `datafusion::execution::context::EmptySerializerRegistry`

**Methods** (2)

```rust
fn deserialize_logical_plan(&self, name: &str, bytes: &[u8]) -> Result<Arc<dyn UserDefinedLogicalNode>>
fn serialize_logical_plan(&self, node: &dyn UserDefinedLogicalNode) -> Result<Vec<u8>>
```

Serializer and deserializer registry for extensions like [UserDefinedLogicalNode].

---

## ExtensionTypeFactory

`type_alias` · `datafusion_expr::registry::ExtensionTypeFactory`

```rust
type ExtensionTypeFactory = dyn Fn(&arrow_schema::DataType, Option<&str>) -> datafusion_common::Result<datafusion_common::types::DFExtensionTypeRef> + Send + Sync
```

A factory that creates instances of extension types from a storage [`DataType`] and the
metadata.

---

## ExtensionTypeRegistrationRef

`type_alias` · `datafusion_expr::registry::ExtensionTypeRegistrationRef`

```rust
type ExtensionTypeRegistrationRef = std::sync::Arc<ExtensionTypeRegistration>
```

A cheaply cloneable pointer to an [ExtensionTypeRegistration].

---

## ExtensionTypeRegistryRef

`type_alias` · `datafusion_expr::registry::ExtensionTypeRegistryRef`

```rust
type ExtensionTypeRegistryRef = std::sync::Arc<dyn ExtensionTypeRegistry>
```

A cheaply cloneable pointer to an [ExtensionTypeRegistry].

---
