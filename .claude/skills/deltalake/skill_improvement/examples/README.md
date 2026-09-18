# Executed implementation examples

The standalone [probe manifest](../evidence/implementation/probes/Cargo.toml) and its retained lock
are the executable dependency/example profile. Run `python3 scripts/run_probes.py` from the skill
root; tables use in-memory storage or disposable local directories. Exact fixture origins/licenses
are retained alongside the tests.

| Need | Working example and assertions |
|---|---|
| Snapshot lifetime, inferred LoadBuilder, replay markers, errors after publication | [contracts.rs](../evidence/implementation/probes/tests/contracts.rs) |
| Write modes, logical plans, staging, merge, casts, CDF, conflict, retention and optimize | [behavior.rs](../evidence/implementation/probes/tests/behavior.rs) |
| Custom Session wrapper and UDF fallback policy | [session.rs](../evidence/implementation/probes/tests/session.rs) |
| Deletion vectors, column mapping and feature admission | [fixtures.rs](../evidence/implementation/probes/tests/fixtures.rs) |
| Nested schema conversion, pool limits and store replacement | [boundaries.rs](../evidence/implementation/probes/tests/boundaries.rs) |

Some assertions deliberately record pin-specific defects or failure behavior. Read their comments
and the associated decision brief before copying code. A success assertion for an expected error
means that error was reproduced, not that the upstream defect was fixed.

The other files in this directory are retained planning examples. The implemented reviewed model
lives in `authoring/capabilities/` and `content/capabilities/`; those are the current schemas/content.
