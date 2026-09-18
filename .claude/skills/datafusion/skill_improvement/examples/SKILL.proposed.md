---
name: datafusion
description: Identify and compare built-in DataFusion and Arrow capabilities for a programming task, then inspect their input/output contracts, implementation conditions, and evidence. Use when choosing query-engine or Arrow APIs, composing them, or checking whether custom code can reuse an existing capability.
---

# DataFusion and Arrow: choose the capability, then inspect its contract

This is a proposed entry-point demonstration. The links below use the assessment bundle's real
files; a deployed replacement would point to the generated task, capability, and operation pages
described in the target design. The existing skill has not been replaced.

## Start with the question you have

| Question | First reference |
|---|---|
| What could solve this task? | [Task families and distinguishing questions](../TARGET_DESIGN.md#task-map) |
| Which crate owns the relevant functionality? | [Crate roles](crate-roles.tsv), then canonical paths in the [symbol index](../../content/index/symbols.tsv) |
| Which similar APIs should I compare? | [Worked capability references](../CAPABILITY_EXAMPLES.md) |
| What does a known API accept, produce, and require? | Its operation contract and full method documentation; the current inventory is under [API pages](../../content/api/) |
| What evidence supports a consequential behavior? | The capability's source/probe references; [retained examples](../evidence/README.md) |

Read only the relevant branch. Known-symbol lookup is a shortcut, not a reason to ignore a nearby
built-in that better matches the task. A typical decision needs one comparison and one contract.

Before selecting an API, establish the conditions that distinguish the plausible candidates:
representation and execution phase, required null/order/schema behavior, and material resource or
lifecycle constraints. Consult deeper evidence when those conditions are consequential or unclear.

## Interpret the reference precisely

- A capability's presence does not establish suitability. Explain which conditions favor it and
  which neighboring built-in you considered.
- Keep logical construction, analysis, optimization, execution, and output collection distinct.
- Read semantic inputs and outputs as well as Rust types: cardinality, order, nulls, metadata,
  allocation/sharing, errors, and state/lifetime obligations can decide the choice.
- Canonical definition paths and usable re-export paths serve different purposes. Check
  [aliases](../../content/index/aliases.tsv); a defining crate is not an automatic new dependency.
- Documented, source-inspected, compiled, and executed claims have different scopes. Empty search
  results and syntax matches alone do not establish absence or semantic identity.
- Match consequential claims to the application's actual dependency/features profile. The current
  reference records DataFusion 55.1.0, Arrow/Parquet 59.3.0, object_store 0.13.2, and sqlparser 0.62.0.

Prefer the smallest composition of existing built-ins that meets the task's contract. Custom
providers/functions/operators remain appropriate where that contract requires them; inspect the
relevant extension obligations rather than implementing optional hooks indiscriminately.

When reporting a choice, give the selected capability, decisive conditions, important output and
implementation consequences, a meaningful alternative, and any assumption still unverified.
