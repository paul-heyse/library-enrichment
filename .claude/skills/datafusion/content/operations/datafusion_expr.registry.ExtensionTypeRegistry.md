# `datafusion_expr::registry::ExtensionTypeRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.registry.ExtensionTypeRegistry.json).

<a id="op-b25230d4d8f3bc59da15fb85"></a>
## ExtensionTypeRegistry

`trait` · `datafusion_expr::registry::ExtensionTypeRegistry` · datafusion-expr 55.1.0

```rust
trait ExtensionTypeRegistry: Debug + Send + Sync
```

Source: `src/registry.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Manages [`ExtensionTypeRegistration`](../operations/datafusion_expr.registry.ExtensionTypeRegistration.md#op-7a56e431478a615d9d67405c)s, which allow users to register custom behavior for
extension types.

Each registration is connected to the extension type name, which can also be looked up to get
the registration.

<a id="op-78205d1dc0ba82d74ae86260"></a>
## add_extension_type_registration

`function` · `datafusion_expr::registry::ExtensionTypeRegistry::add_extension_type_registration` · datafusion-expr 55.1.0

```rust
fn add_extension_type_registration(&self, extension_type: ExtensionTypeRegistrationRef) -> Result<Option<ExtensionTypeRegistrationRef>>
```

Source: `src/registry.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Registers a new [ExtensionTypeRegistrationRef](../operations/datafusion_expr.registry.ExtensionTypeRegistrationRef.md#op-182f94f1e8665da855947adb), returning any previously registered
implementation.

Returns an error if the type cannot be registered, for example, if the registry is
read-only.

<a id="op-ffcb2c06d6d4512792a17d95"></a>
## create_extension_type_for_field

`function` · `datafusion_expr::registry::ExtensionTypeRegistry::create_extension_type_for_field` · datafusion-expr 55.1.0

```rust
fn create_extension_type_for_field(&self, field: &Field) -> Result<Option<DFExtensionTypeRef>>
```

Source: `src/registry.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a [`DFExtensionTypeRef`](../operations/datafusion_common.types.extension.DFExtensionTypeRef.md#op-24969730b079964af558647f) from the type information in the `field`.

The result `Ok(None)` indicates that there is no extension type metadata. Returns an error
if the extension type in the metadata is not found.

<a id="op-3dfbb8fb98f7bea7a0b6aa96"></a>
## extend

`function` · `datafusion_expr::registry::ExtensionTypeRegistry::extend` · datafusion-expr 55.1.0

```rust
fn extend(&self, extension_types: &[ExtensionTypeRegistrationRef]) -> Result<()>
```

Source: `src/registry.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Extends the registry with the provided extension types.

Returns an error if the type cannot be registered, for example, if the registry is
read-only.

<a id="op-2bc0e127588a9d3a7bc6804a"></a>
## extension_type_registration

`function` · `datafusion_expr::registry::ExtensionTypeRegistry::extension_type_registration` · datafusion-expr 55.1.0

```rust
fn extension_type_registration(&self, name: &str) -> Result<ExtensionTypeRegistrationRef>
```

Source: `src/registry.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a reference to registration of an extension type named `name`.

Returns an error if there is no extension type with that name.

<a id="op-3f72ea2ea2b918f41df96a23"></a>
## extension_type_registrations

`function` · `datafusion_expr::registry::ExtensionTypeRegistry::extension_type_registrations` · datafusion-expr 55.1.0

```rust
fn extension_type_registrations(&self) -> Vec<ExtensionTypeRegistrationRef>
```

Source: `src/registry.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns all registered [ExtensionTypeRegistration](../operations/datafusion_expr.registry.ExtensionTypeRegistration.md#op-7a56e431478a615d9d67405c).

<a id="op-4563620c80c6baa3b1d9d401"></a>
## remove_extension_type_registration

`function` · `datafusion_expr::registry::ExtensionTypeRegistry::remove_extension_type_registration` · datafusion-expr 55.1.0

```rust
fn remove_extension_type_registration(&self, name: &str) -> Result<Option<ExtensionTypeRegistrationRef>>
```

Source: `src/registry.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Deregisters an extension type registration with the name `name`, returning the
implementation that was deregistered.

Returns an error if the type cannot be deregistered, for example, if the registry is
read-only.
