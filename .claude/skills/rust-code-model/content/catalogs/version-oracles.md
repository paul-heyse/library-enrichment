# Version oracles

For each subject: how to ask what is actually running, what that answer covers, and the
reading that looks authoritative and is not. Three of these answer a question next to
the one you asked.

| Subject | Ask | Answers for | Trap |
|---|---|---|---|
| rustc | `rustc +TOOLCHAIN --version` | the compiler that will produce every MIR and dataflow observation | a bare +nightly floats; use a dated toolchain |
| rustdoc | `rustdoc +TOOLCHAIN -Z unstable-options --output-format json` | locally produced documents only | says nothing about what docs.rs will serve for a crate |
| rustdoc JSON | `read the format_version field of the payload` | that one document | the only reliable reading. rustdoc-types 0.61.0, which defines FORMAT_VERSION = 61, is served by docs.rs at format 60 |
| rustdoc-types | `rustdoc_types::FORMAT_VERSION` | the crate you compiled against | the minor version equals the format version, so 0.61.0 means 61 |
| rust-analyzer | `rust-analyzer --version` | the rustc release the binary was built in | NOT an ra_ap_* crate version, and there is no published mapping between them. The rust-analyzer repository carries no version number at all |
| ra_ap_* crates | `read the version from Cargo.lock` | the crates you linked | 0.0.N is bumped weekly and every release is semver-breaking; a caret requirement will float across breaking changes |
| cargo | `cargo metadata --format-version 1` | the envelope shape | version has read 1 for years while new keys were added; tolerate unknown fields |
| cargo_metadata | `read the version from Cargo.lock` | the crate | pre-1.0, so a minor bump is breaking; it may also lag fields Cargo already emits |
