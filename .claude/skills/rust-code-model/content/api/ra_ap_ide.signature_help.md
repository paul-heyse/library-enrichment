# `ra_ap_ide::signature_help`

Crate `ra_ap_ide` · 1 public items · structured records in [`model/ra_ap_ide.signature_help.json`](../model/ra_ap_ide.signature_help.json)

## SignatureHelp

`struct` · `ra_ap_ide::signature_help::SignatureHelp`

Also reachable as `ra_ap_ide::SignatureHelp`

```rust
struct SignatureHelp
```

**Fields**: `doc`, `signature`, `active_parameter`

**Derives**: Debug

**Methods** (2)

```rust
fn parameter_labels(&self) -> impl Iterator<Item = &str> + '_
fn parameter_ranges(&self) -> &[TextRange]
```

Contains information about an item signature as seen from a use site.

This includes the "active parameter", which is the parameter whose value is currently being
edited.

---
