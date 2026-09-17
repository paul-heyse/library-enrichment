# `ra_ap_ide::annotations`

Crate `ra_ap_ide` · 4 public items · structured records in [`model/ra_ap_ide.annotations.json`](../model/ra_ap_ide.annotations.json)

## AnnotationKind

`enum` · `ra_ap_ide::annotations::AnnotationKind`

Also reachable as `ra_ap_ide::AnnotationKind`

```rust
enum AnnotationKind
```

**Variants**: `Runnable`, `HasImpls`, `HasReferences`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## AnnotationLocation

`enum` · `ra_ap_ide::annotations::AnnotationLocation`

Also reachable as `ra_ap_ide::AnnotationLocation`

```rust
enum AnnotationLocation
```

**Variants**: `AboveName`, `AboveWholeItem`

---

## Annotation

`struct` · `ra_ap_ide::annotations::Annotation`

Also reachable as `ra_ap_ide::Annotation`

```rust
struct Annotation
```

**Fields**: `range`, `kind`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## AnnotationConfig

`struct` · `ra_ap_ide::annotations::AnnotationConfig`

Also reachable as `ra_ap_ide::AnnotationConfig`

```rust
struct AnnotationConfig<'a>
```

**Fields**: `binary_target`, `annotate_runnables`, `annotate_impls`, `annotate_references`, `annotate_method_references`, `annotate_enum_variant_references`, `references_exclude_imports`, `references_exclude_tests`, `location`, `filter_adjacent_derive_implementations`, `ra_fixture`

---
