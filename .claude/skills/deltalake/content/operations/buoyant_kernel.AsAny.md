# `buoyant_kernel::AsAny`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.AsAny.json).

<a id="op-ad519b2589ab7e04a99cd231"></a>
## AsAny

`trait` · `buoyant_kernel::AsAny` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait AsAny: Any + Send + Sync
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L333).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:333`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extension trait that makes it easier to work with traits objects that implement [`Any`],
implemented automatically for any type that satisfies `Any`, `Send`, and `Sync`. In particular,
given some `trait T: Any + Send + Sync`, it allows upcasting `T` to `dyn Any + Send + Sync`,
which in turn allows downcasting the result to a concrete type.

For example, the following code will compile:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::AsAny;
# use std::any::Any;
# use std::sync::Arc;
trait Foo : AsAny {}
struct Bar;
impl Foo for Bar {}

let f: Arc<dyn Foo> = Arc::new(Bar);
let a: Arc<dyn Any + Send + Sync> = f.as_any();
let b: Arc<Bar> = a.downcast().unwrap();
```

In contrast, very similar code that relies only on `Any` would fail to compile:

```fail_compile
# use std::any::Any;
# use std::sync::Arc;
trait Foo: Any + Send + Sync {}

struct Bar;
impl Foo for Bar {}

let f: Arc<dyn Foo> = Arc::new(Bar);
let b: Arc<Bar> = f.downcast().unwrap(); // `Arc::downcast` method not found
```

As would this:

```fail_compile
# use std::any::Any;
# use std::sync::Arc;
trait Foo: Any + Send + Sync {}

struct Bar;
impl Foo for Bar {}

let f: Arc<dyn Foo> = Arc::new(Bar);
let a: Arc<dyn Any + Send + Sync> = f; // trait upcasting coercion is not stable rust
let f: Arc<Bar> = a.downcast().unwrap();
```

NOTE: `AsAny` inherits the `Send + Sync` constraint from [`Arc::downcast`].

Unresolved upstream links (retained, not inferred): ``Arc::downcast``, ``Any``.

<a id="op-fb66e463267db4280f115c1e"></a>
## any_ref

`function` · `buoyant_kernel::AsAny::any_ref` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn any_ref(&self) -> &dyn Any + Send + Sync
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L349).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:349`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Obtains a `dyn Any` reference to the object:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::AsAny;
# use std::any::Any;
# use std::sync::Arc;
trait Foo : AsAny {}
struct Bar;
impl Foo for Bar {}

let f: &dyn Foo = &Bar;
let a: &dyn Any = f.any_ref();
let b: &Bar = a.downcast_ref().unwrap();
```

<a id="op-4141b90238c64e47c9bbb477"></a>
## as_any

`function` · `buoyant_kernel::AsAny::as_any` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_any(Arc<self>) -> Arc<dyn Any + Send + Sync>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L366).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:366`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Obtains an `Arc<dyn Any>` reference to the object:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::AsAny;
# use std::any::Any;
# use std::sync::Arc;
trait Foo : AsAny {}
struct Bar;
impl Foo for Bar {}

let f: Arc<dyn Foo> = Arc::new(Bar);
let a: Arc<dyn Any + Send + Sync> = f.as_any();
let b: Arc<Bar> = a.downcast().unwrap();
```

<a id="op-dbbe123a04d02ce53438fa55"></a>
## into_any

`function` · `buoyant_kernel::AsAny::into_any` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_any(Box<self>) -> Box<dyn Any + Send + Sync>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L383).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:383`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts the object to `Box<dyn Any>`:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::AsAny;
# use std::any::Any;
# use std::sync::Arc;
trait Foo : AsAny {}
struct Bar;
impl Foo for Bar {}

let f: Box<dyn Foo> = Box::new(Bar);
let a: Box<dyn Any> = f.into_any();
let b: Box<Bar> = a.downcast().unwrap();
```

<a id="op-2f58db39e866401d174a7651"></a>
## type_name

`function` · `buoyant_kernel::AsAny::type_name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn type_name(&self) -> &'static str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L387).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:387`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Convenient wrapper for [`std::any::type_name`], since [`Any`] does not provide it and
[`Any::type_id`] is useless as a debugging aid (its `Debug` is just a mess of hex digits).

Unresolved upstream links (retained, not inferred): ``std::any::type_name``, ``Any::type_id``, ``Any``.
