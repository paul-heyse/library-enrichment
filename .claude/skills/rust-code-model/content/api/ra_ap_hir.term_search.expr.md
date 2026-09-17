# `ra_ap_hir::term_search::expr`

Crate `ra_ap_hir` · 1 public items · structured records in [`model/ra_ap_hir.term_search.expr.json`](../model/ra_ap_hir.term_search.expr.json)

## Expr

`enum` · `ra_ap_hir::term_search::expr::Expr`

Also reachable as `ra_ap_hir::term_search::Expr`

```rust
enum Expr<'db>
```

**Variants**: `Const`, `Static`, `Local`, `ConstParam`, `FamousType`, `Function`, `Method`, `Variant`, `Struct`, `Tuple`, `Field`, `Reference`, `Many`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn gen_source_code(&self, sema_scope: &SemanticsScope<'db>, many_formatter: &mut dyn FnMut(&Type<'db>) -> String, cfg: FindPathConfig, display_target: DisplayTarget) -> Result<String, DisplaySourceCodeError>
fn is_many(&self) -> bool
fn traits_used(&self, db: &dyn HirDatabase) -> Vec<Trait>
fn ty(&self, db: &'db dyn HirDatabase) -> Type<'db>
```

Type tree shows how can we get from set of types to some type.

Consider the following code as an example
```ignore
fn foo(x: i32, y: bool) -> Option<i32> { None }
fn bar() {
   let a = 1;
   let b = true;
   let c: Option<i32> = _;
}
```
If we generate type tree in the place of `_` we get
```txt
      Option<i32>
          |
    foo(i32, bool)
     /        \
 a: i32      b: bool
```
So in short it pretty much gives us a way to get type `Option<i32>` using the items we have in
scope.

---
