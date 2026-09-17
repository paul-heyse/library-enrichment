//! PB04 — is `DeltaOps`, the conventional delta-rs entry point, reachable at the pinned rev?
//!
//! It is absent from all 432 `deltalake::` re-exports the `deltalake` skill indexed, and absent
//! from that skill's `symbols.tsv`, `aliases.tsv`, `unresolved.tsv` and `unnameable.tsv`
//! (evidence 04 §5). Two explanations were possible: it does not exist at this rev, or the skill
//! did not index it. Only a compile distinguishes them.
//!
//! This is an `example`, not a `bin`, so `cargo build` never builds it and a failure here cannot
//! break the main probe.
//!
//!     cargo build --example pb04_deltaops
//!
//! Compiles  → `DeltaOps` exists; the skill's index has a gap, and evidence 04 §5 must be
//!             corrected to say so.
//! Fails     → `DeltaOps` genuinely absent at this rev; the individual operation builders are
//!             the only entry point, and the plan's call sites are already correct.

use deltalake::DeltaOps;

fn main() {
    // Referencing the type is the whole test; nothing needs to run.
    let _ = std::any::type_name::<DeltaOps>();
    println!("PB04: deltalake::DeltaOps resolved — it EXISTS at this rev");
}
