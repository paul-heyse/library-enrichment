---
paths:
  - "crates/**"
  - "Cargo.toml"
  - "Cargo.lock"
  - "deny.toml"
  - "clippy.toml"
  - "rust-toolchain.toml"
---

# Rust core boundary

Rust owns identities, package resolution, fetching, normalization, evidence storage, querying,
job state, policy, and publication. If a capability is being added in Python that belongs in
this list, it is in the wrong place.

**Policy is enforced here, not in tool annotations or the skill.** An MCP annotation describes
behavior to a client; it does not constrain anything. Network limits, archive extraction
safety, execution profiles, and environment allowlisting are core responsibilities.

**Only the daemon publishes.** Workers write staged outputs under job-specific paths. Validate
schema, references, digest integrity, and expected coverage before publication; publish an
immutable snapshot directory, then atomically update the context's current pointer. Readers
must never observe a partially written table, including after a crash mid-publication (C06).

Producers are typed `ProducerSpec`/`ProducerPlan` records over a small explicit dependency
graph. Do not build a generic orchestration platform. A producer derives its command from
typed options; it never concatenates a shell command supplied by a caller.

Identity is four distinct things — `release_id`, `environment_id`, `context_id`, `snapshot_id`
— and they do not collapse. A context records whether its environment is `unspecified`,
`declared`, `resolved`, or `verified`. When later evidence resolves an unknown field, return a
**new derived context**; never mutate the meaning of an existing ID (C16).

One Arrow/DataFusion type universe. `deny.toml` sets `multiple-versions = "deny"`; pin
DataFusion first and derive `arrow`, `parquet`, and `object_store` from it. The reverse order
fails. Keep analysis dependencies for studied libraries out of our own dependency graph.

Banned dependency classes: graph databases, embeddings, vector stores, gRPC, Redis, FastAPI.
`deny.toml` and `just deps-policy` are the oracles.

Stable toolchain for our code. The dated nightly is a recorded producer identity in
`config/toolchains.toml`, never a build domain — see `AGENTS.md`.

Note that `cargo nextest` does not run doctests. Schema round-trip examples must be real
`#[test]` functions, or `cargo test --doc` must join the gate, or §6.3's round trips are
silently untested.
