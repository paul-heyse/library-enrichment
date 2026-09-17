# Not reachable from here

Things that are real -- in an API, in a rustup component, or in a reader's memory --
and cannot be obtained in this setting. Silence in an index is not evidence of absence,
so the absences worth knowing about are written down.

| Capability | Exists in | Reachable | Why | Use instead |
|---|---|---|---|---|
| rustdoc JSON for any rustc-internal crate | nowhere | no | the rust-docs-json rustup component ships std, core, alloc, proc_macro, test and std_detect only | the rustc-docs component, or the pinned source in content/corpus/rustc |
| a stable API for MIR | rustc_public (formerly stable_mir) | partially | still requires a nightly compiler and rustc_private | rustc -Zunpretty=stable-mir to see the projection, or the pinned rustc_public source |
| rust-analyzer search | the CLI | no | panics on this build with 'Try to use attached db, but not db is attached' | ssr with a rule, or ast-grep for a purely structural match |
| a JSON Schema for cargo metadata output | nowhere | no | rust-lang/cargo publishes no schema file for the metadata envelope | the Cargo book page in content/corpus/cargo, and cargo_metadata's own types |
| a mapping from an ra_ap version to a rust-analyzer commit | nowhere published | no | versions are minted at publish time by xtask; the repository has no version field | match by publication date, and record that it is an inference |
| feature gates in rustdoc JSON | the compiler | no | an item gated off is absent from the document with no trace | the [features] table of Cargo.toml, indexed as features.tsv |
| function bodies in rustdoc JSON | the compiler | no | the format models the documented surface, not code | MIR, THIR or the HIR layer |
| search-index.js on nightly-rustc docs | older rustdoc | no | rustdoc moved to a search.index/ stringdex backend; the old path is a 404 | search.index/root.js, crates.js, or each crate's all.html |
