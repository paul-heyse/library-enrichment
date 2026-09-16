# `pyrefly::alt::class::django`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.class.django.json`](../model/pyrefly.alt.class.django.json)

## is_django_choices_subclass

`function` · `pyrefly::alt::class::django::is_django_choices_subclass`

```rust
fn is_django_choices_subclass(bases_with_metadata: &[(pyrefly_types::class::Class, &alt::types::class_metadata::ClassMetadata)]) -> bool
```

---

## transform_django_enum_value

`function` · `pyrefly::alt::class::django::transform_django_enum_value`

```rust
fn transform_django_enum_value(ty: pyrefly_types::types::Type, heap: &pyrefly_types::heap::TypeHeap) -> pyrefly_types::types::Type
```

Strip the label element from a Django enum tuple value.
Django `Choices` enums use `(value, label)` tuples; this strips the last
element (the label) and returns the remaining value portion.

---
