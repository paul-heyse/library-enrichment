# `buoyant_kernel::utils::CollectInto`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.utils.CollectInto.json).

<a id="op-429abe37a51a5437bffb0583"></a>
## CollectInto

`trait` · `buoyant_kernel::utils::CollectInto` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait CollectInto<T>: IntoIterator + Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/utils.rs#L47).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/utils.rs:47`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

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

<a id="op-86575046b773c02a84e8f624"></a>
## collect_into

`function` · `buoyant_kernel::utils::CollectInto::collect_into` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn collect_into(self) -> T
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/utils.rs#L49).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/utils.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Collects this iterable into a `T`
