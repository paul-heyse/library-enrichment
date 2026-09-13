# Library Enrichment — Blueprint Bundle

An implementation handoff for a separate Rust/Python library-evidence service, exposed through FastMCP 4 and used alongside Context7.

**This bundle is a specification, not a working MCP implementation. No service has been installed, repository created remotely, or client configuration changed.**

## Start here

1. Give a programming agent `AGENT_HANDOFF.md` and `IMPLEMENTATION_BLUEPRINT.md`.
2. Use `contracts/` and `tests/ACCEPTANCE_PLAN.md` as executable contract/test starting points.
3. Retain `skills/library-research/` as the companion skill, updating its contracts with the implementation.

## Contents

- `IMPLEMENTATION_BLUEPRINT.md`: architecture, language-specific producers, evidence schemas, tools, isolation, workflows, installation, phases, and definition of done.
- `AGENT_HANDOFF.md`: concise execution instruction for the implementing agent.
- `skills/library-research/`: portable skill and progressively loaded references.
- `contracts/research-envelope.schema.json`: baseline machine-readable response envelope; specialize `data` by tool in the implementation.
- `contracts/examples/`: validated illustrative results; not results of real library analysis.
- `config/service.example.toml`: proposed configuration contract and explicit defaults.
- `tests/ACCEPTANCE_PLAN.md`: required behavioral cases.
- `SOURCES.md`: primary documentation used to verify the blueprint's external-tool assumptions.

Prepared 2026-09-13. Exact dependency pins must be chosen and tested during implementation.
