---
name: deltalake
description: Choose built-in Delta Lake operations and integrations from task routes, reviewed input/output/effect contracts, commit-pinned APIs and executable evidence. Use for snapshots, DataFusion reads, writes and merge, schema/protocol choices, change feeds, transactions, storage and maintenance.
---

# Delta Lake built-ins for implementation decisions

Proposed entrypoint, not installed. Link labels below describe future generated resources.

Find the route matching the task and representation already available. Read the capability's
decision brief before designing custom storage, transaction or DataFusion machinery. It identifies
the useful built-in, alternatives, prerequisites, inputs, outputs, effects and remaining unknowns.

| You need to… | Start with… |
|---|---|
| Query rows with a freshness or version requirement | Read/snapshot routes; table handle → snapshot → provider/stream |
| Append, replace, update or merge | Write/DML routes; batches/logical plan → operation → updated table/metrics |
| Preserve schema and application constraints | Schema and per-operation protocol support matrix |
| Retry or recover a write | Replay/commit routes, including publication and post-commit errors |
| Consume changes | CDF interval, schema, filtering and retention contract |
| Reduce files, remove expired objects or restore history | Maintenance effects comparison |
| Select storage, catalogs or kernel integration | Crate roles and public integration entrypoints |

Use `find` for task vocabulary, `show` for a reviewed brief or exact symbol, and `compare` for
conditional alternatives. Default results are bounded; expand full API/source/evidence as needed.
Canonical paths and facade/kernel aliases resolve to the same capability records.

The reference uses a delta-rs commit and its exact kernel/dependency profile, rather than a
published crate version. The short profile summary and provenance record identify that capture;
match the consumer's actual dependency graph before copying a consequential API composition.

Keep these distinctions visible:

- Snapshot/provider reuse preserves the chosen version; freshness requires an explicit policy.
- A transaction marker records progress; it is not automatic replay suppression for every writer.
- An operation error may follow log publication; inspect its phase and effects.
- Recognized protocol features, read/write admission and operation support are separate facts.
- Public returned types, public import paths and internal trait machinery are different access levels.

Use full contracts for exact member details, and retained source/probes for behavior not established
by documentation. Syntax matches nominate questions; they do not identify a receiver type or prove
an incorrect implementation. Unknown coverage is not proof a capability is absent.
