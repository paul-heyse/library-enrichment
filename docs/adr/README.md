# Architecture decision records

`AGENT_HANDOFF.md` requires an ADR, with evidence and tests, for every necessary deviation from
the blueprint. ADRs are also the only legitimate route to changing a frozen contract or a
binding decision.

Create one with `/adr <slug>`, from [`0000-template.md`](0000-template.md).

| ADR | Title | Status | Boundary |
|---|---|---|---|
| [0001](0001-repository-location-and-name.md) | Repository location and name | accepted | none |
| [0002](0002-dual-mit-apache-license.md) | Dual MIT / Apache-2.0 license | accepted | none |
| [0003](0003-in-repo-development-state-sandbox.md) | In-repo development state sandbox | accepted | repository boundary |
| [0004](0004-python-314-pin.md) | Python 3.14 pin | accepted | none |
