# TypeOutput

`pyrefly_types::type_output::TypeOutput`

```rust
trait TypeOutput
```

Prose: [`api/pyrefly_types.type_output.md`](../api/pyrefly_types.type_output.md#typeoutput) · records: [`model/pyrefly_types.type_output.json`](../model/pyrefly_types.type_output.json)

## Required

Every implementation must supply these.

```rust
fn write_builtin(&mut self, name: &str, qname: Option<&QName>) -> fmt::Result
fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result
fn write_lit(&mut self, lit: &Lit) -> fmt::Result
fn write_qname(&mut self, qname: &QName) -> fmt::Result
fn write_reference(&mut self, module: ModuleName, name: &str) -> fmt::Result
fn write_special_form(&mut self, name: &str, qname: Option<&QName>) -> fmt::Result
fn write_str(&mut self, s: &str) -> fmt::Result
fn write_targs(&mut self, targs: &TArgs) -> fmt::Result
fn write_type(&mut self, ty: &Type) -> fmt::Result
```

## Implementors (3)

Read one before writing your own.

- `pyrefly_types::type_output::AnnotationOutput`
- `pyrefly_types::type_output::DisplayOutput`
- `pyrefly_types::type_output::OutputWithLocations`

## Documentation

A trait that will be will be used for formatting types, but also allow
additional functionality. The major difference between implementations
of this trait will be the location that type information is written to
For example, this will be used to write type information to a formatter as we do now,
and also allow us to collect the same types along with their location into a vector.
