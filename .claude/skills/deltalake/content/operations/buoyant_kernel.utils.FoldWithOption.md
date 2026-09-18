# `buoyant_kernel::utils::FoldWithOption`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.utils.FoldWithOption.json).

<a id="op-513b8966e98df66f8fef4759"></a>
## FoldWithOption

`trait` · `buoyant_kernel::utils::FoldWithOption` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait FoldWithOption: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/utils.rs#L150).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/utils.rs:150`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extension trait for folding zero or one value from an [`Option`] into a base value.

Unresolved upstream links (retained, not inferred): ``Option``.

<a id="op-4cfb63e876a7e9dbdf6ae38a"></a>
## fold_with

`function` · `buoyant_kernel::utils::FoldWithOption::fold_with` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fold_with<U>(self, opt: Option<U>, f: impl FnOnce(Self, U) -> Self) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/utils.rs#L156).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/utils.rs:156`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Applies an optional fold operation `f` to `self` if `opt` is [`Some`]; otherwise returns
`self`.

Similar to `opt.iter().fold(self, |acc, value| f(acc, value))`, but accepting `FnOnce`
instead of requiring `FnMut`, and with the base value as receiver instead of the option.

Unresolved upstream links (retained, not inferred): ``Some``.
