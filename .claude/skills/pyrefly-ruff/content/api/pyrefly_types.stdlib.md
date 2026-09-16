# `pyrefly_types::stdlib`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.stdlib.json`](../model/pyrefly_types.stdlib.json)

## Stdlib

`struct` · `pyrefly_types::stdlib::Stdlib`

```rust
struct Stdlib
```

**Derives**: Clone, Debug

**Methods** (80)

```rust
fn async_generator(&self, yield_ty: Type, send_ty: Type) -> ClassType
fn async_iterable(&self, x: Type) -> ClassType
fn async_iterator(&self, x: Type) -> ClassType
fn awaitable(&self, x: Type) -> ClassType
fn awaitable_object(&self) -> &Class
fn base_exception(&self) -> &ClassType
fn base_exception_group(&self, x: Type) -> Option<ClassType>
fn bool(&self) -> &ClassType
fn builtins_type(&self) -> &ClassType
fn bytearray(&self) -> &ClassType
fn bytes(&self) -> &ClassType
fn complex(&self) -> &ClassType
fn coroutine(&self, yield_ty: Type, send_ty: Type, return_ty: Type) -> ClassType
fn coroutine_object(&self) -> &Class
fn date(&self) -> &ClassType
fn datetime(&self) -> &ClassType
fn decimal(&self) -> &ClassType
fn deque(&self, x: Type) -> ClassType
fn dict(&self, key: Type, value: Type) -> ClassType
fn dict_items(&self, key: Type, value: Type) -> ClassType
fn dict_keys(&self, key: Type, value: Type) -> ClassType
fn dict_object(&self) -> &Class
fn dict_values(&self, key: Type, value: Type) -> ClassType
fn ellipsis_type(&self) -> &ClassType
fn enum_class(&self) -> &ClassType
fn enum_flag(&self) -> &ClassType
fn enum_meta(&self) -> &ClassType
fn enumerate(&self, x: Type) -> ClassType
fn exception_group(&self, x: Type) -> Option<ClassType>
fn exception_group_object(&self) -> Option<&Class>
fn float(&self) -> &ClassType
fn for_bootstrapping() -> Stdlib
fn frozenset(&self, x: Type) -> ClassType
fn frozenset_object(&self) -> &Class
fn function_type(&self) -> &ClassType
fn generator(&self, yield_ty: Type, send_ty: Type, return_ty: Type) -> ClassType
fn generic_alias(&self) -> &ClassType
fn int(&self) -> &ClassType
fn iterable(&self, x: Type) -> ClassType
fn iterator(&self, x: Type) -> ClassType
fn list(&self, x: Type) -> ClassType
fn list_object(&self) -> &Class
fn mapping(&self, key: Type, value: Type) -> ClassType
fn mapping_object(&self) -> &Class
fn method_type(&self) -> &ClassType
fn module_type(&self) -> &ClassType
fn mutable_sequence(&self, x: Type) -> ClassType
fn named_tuple_fallback(&self) -> &ClassType
fn new(version: PythonVersion, lookup_class: &dyn Fn(ModuleName, &Name) -> Option<(Class, Option<Arc<TParams>>)>, lookup_export_location: &dyn Fn(ModuleName, &Name) -> Option<(Module, TextRange)>) -> Self
fn new_with_bootstrapping(bootstrapping: bool, version: PythonVersion, lookup_class: &dyn Fn(ModuleName, &Name) -> Option<(Class, Option<Arc<TParams>>)>, lookup_export_location: &dyn Fn(ModuleName, &Name) -> Option<(Module, TextRange)>) -> Self
fn none_type(&self) -> &ClassType
fn object(&self) -> &ClassType
fn param_spec(&self) -> &ClassType
fn param_spec_args(&self) -> &ClassType
fn param_spec_args_as_tuple(&self, heap: &TypeHeap) -> ClassType
fn param_spec_kwargs(&self) -> &ClassType
fn param_spec_kwargs_as_dict(&self, heap: &TypeHeap) -> ClassType
fn partial(&self, ret: Type) -> ClassType
fn path(&self) -> &ClassType
fn property(&self) -> &ClassType
fn protocol_meta(&self) -> &ClassType
fn sentinel(&self) -> &ClassType
fn sequence(&self, x: Type) -> ClassType
fn set(&self, x: Type) -> ClassType
fn set_object(&self) -> &Class
fn slice_class_object(&self) -> Class
fn special_form_qname(&self, name: &str) -> Option<&QName>
fn str(&self) -> &ClassType
fn template(&self) -> Option<&ClassType>
fn time(&self) -> &ClassType
fn timedelta(&self) -> &ClassType
fn traceback_type(&self) -> &ClassType
fn tuple(&self, x: Type) -> ClassType
fn tuple_object(&self) -> &Class
fn type_alias_type(&self) -> &ClassType
fn type_var(&self) -> &ClassType
fn type_var_tuple(&self) -> &ClassType
fn typed_dict_fallback(&self) -> &ClassType
fn union_type(&self) -> Option<&ClassType>
fn uuid(&self) -> &ClassType
```

---
