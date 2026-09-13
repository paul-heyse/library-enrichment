# Development skills

Workflows for agents *building* this repository. A skill is written when a workflow has
actually repeated, not in anticipation of one.

| Skill | Written because |
|---|---|
| [`design-review/`](design-review/SKILL.md) | Design review had already run twice — commits `e74d012` and `0cd7330` — finding a gate C20 repository-boundary breach, a gate passing on an unearned assertion, and a skipped test reported as `failed`. Neither round left a durable artifact and neither produced a `rules/` entry. ADR-0009. |
| [`adr/`](adr/SKILL.md) | Recording a decision is the most repeated workflow here, and the one a deviation is required to use. |
| [`phase-gate/`](phase-gate/SKILL.md) | Run at every phase boundary, and the place truthful reporting is easiest to get wrong. |
| [`verify-upstream/`](verify-upstream/SKILL.md) | Run before every pin; ADR-0005 exists because it contradicted a documented claim. |
| [`acceptance-report/`](acceptance-report/SKILL.md) | Run before any delivery report, and audited by a subagent rather than trusted. |
| [`handoff/`](handoff/SKILL.md) | Run at the end of any session that changed what is true. |

**Everything an agent workflow needs lives here, not in a runtime-specific directory.** A Claude
Code slash-command directory is visible to Claude Code alone; Codex never sees it. The five
workflows that used to live in one — `adr`, `phase-gate`, `verify-upstream`,
`acceptance-report`, `handoff` — are skills for that reason, and both runtimes surface a skill
as `/name`. `scripts/check_agent_config.py` fails if a runtime-specific instruction surface
reappears.

Progressive disclosure is the second reason: `design-review` is one page plus a reference the
reviewer loads only when a lens is needed, and its normative standard lives in
`docs/design_review/design_principles/` where ADRs and reviews cite it too.

`.claude/skills/` and `.claude/agents/` are the single source. `.codex/` and `.agents/` expose
both through symlinks, so Claude Code and Codex discover the same files. `just lint-agents`
checks that they resolve, that every skill's `name` matches its directory, and that every path
and `just` recipe the instructions name actually exists.

**Not to be confused with `skills/library-research/`**, which is the skill this service *ships*.
That one is a product deliverable governed by the blueprint, it stays synchronized with the
implemented tool contract, and it is installed to user scope only by an explicit, operator-run
`just install-skill --apply`.
