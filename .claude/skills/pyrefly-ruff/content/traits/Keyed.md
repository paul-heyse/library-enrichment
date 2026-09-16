# Keyed

`pyrefly::binding::binding::Keyed`

```rust
trait Keyed: Hash + Eq + Clone + DisplayWith<pyrefly_python::module::Module> + Debug + 'static
```

Prose: [`api/pyrefly.binding.binding.md`](../api/pyrefly.binding.binding.md#keyed) · records: [`model/pyrefly.binding.binding.json`](../model/pyrefly.binding.binding.json)

## Required

Every implementation must supply these.

```rust
fn range_with(idx: Idx<Self>, bindings: &Bindings) -> TextRange where BindingTable: TableKeyed<Self, Value = BindingEntry<Self>>
fn to_anyidx(idx: Idx<Self>) -> AnyIdx
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn try_to_anykey(&self) -> Option<AnyExportedKey>
```

## Implementors (25)

Read one before writing your own.

- `pyrefly::binding::binding::Key`
- `pyrefly::binding::binding::KeyAbstractClassCheck`
- `pyrefly::binding::binding::KeyAnnotation`
- `pyrefly::binding::binding::KeyClass`
- `pyrefly::binding::binding::KeyClassBaseType`
- `pyrefly::binding::binding::KeyClassChecks`
- `pyrefly::binding::binding::KeyClassDisjointBase`
- `pyrefly::binding::binding::KeyClassField`
- `pyrefly::binding::binding::KeyClassMetadata`
- `pyrefly::binding::binding::KeyClassMro`
- `pyrefly::binding::binding::KeyClassSubscriptSymmetry`
- `pyrefly::binding::binding::KeyClassSynthesizedFields`
- `pyrefly::binding::binding::KeyDecoratedFunction`
- `pyrefly::binding::binding::KeyDecorator`
- `pyrefly::binding::binding::KeyDjangoRelations`
- `pyrefly::binding::binding::KeyExpect`
- `pyrefly::binding::binding::KeyExport`
- `pyrefly::binding::binding::KeyLegacyTypeParam`
- `pyrefly::binding::binding::KeyTParams`
- `pyrefly::binding::binding::KeyTypeAlias`
- `pyrefly::binding::binding::KeyUndecoratedFunction`
- `pyrefly::binding::binding::KeyUndecoratedFunctionRange`
- `pyrefly::binding::binding::KeyVariance`
- `pyrefly::binding::binding::KeyYield`
- `pyrefly::binding::binding::KeyYieldFrom`

## Documentation

Any key that sets `EXPORTED` to `true` should not include positions
Incremental updates depend on knowing when a file's exports changed, which uses equality between exported keys
Moving code around should not cause all dependencies to be re-checked
