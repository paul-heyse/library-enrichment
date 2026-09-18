# `buoyant_kernel::utils`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.utils.json`](../model/buoyant_kernel.utils.json)

## try_parse_uri

`function` · `buoyant_kernel::utils::try_parse_uri`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.utils.try_parse_uri.md)

Also reachable as `buoyant_kernel::try_parse_uri`, `delta_kernel::utils::try_parse_uri`

```rust
fn try_parse_uri(uri: impl AsRef<str>) -> DeltaResult<url::Url>
```

Try to parse string uri into a URL for a table path. This will do it's best to handle things
like `/local/paths`, and even `../relative/paths`.

---

## CollectInto

`trait` · `buoyant_kernel::utils::CollectInto`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.utils.CollectInto.md)

Also reachable as `buoyant_kernel::CollectInto`, `delta_kernel::utils::CollectInto`

```rust
trait CollectInto<T>: IntoIterator + Sized
```

**Methods** (1)

```rust
fn collect_into(self) -> T
```

Dual of the `FromIterator` trait, similar to how `Into` is the dual of `From`. It is
automatically implemented for any iterable whose items collect into `T`, and can drastically
simplify type bounds. For example, `CollectInto` allows to write this:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::CollectInto;
# struct Foo;
fn foo(arg: impl CollectInto<Foo>) -> Foo {
    arg.collect_into()
}
```

instead of the much more verbose:

```
# struct Foo;
fn foo<T>(arg: impl IntoIterator<Item = T>) -> Foo
where
    Foo: FromIterator<T>,
{
    Foo::from_iter(arg)
}
```

---

## FoldWithOption

`trait` · `buoyant_kernel::utils::FoldWithOption`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.utils.FoldWithOption.md)

Also reachable as `delta_kernel::utils::FoldWithOption`

```rust
trait FoldWithOption: Sized
```

**Methods** (1)

```rust
fn fold_with<U>(self, opt: Option<U>, f: impl FnOnce(Self, U) -> Self) -> Self
```

Extension trait for folding zero or one value from an [`Option`] into a base value.

---
