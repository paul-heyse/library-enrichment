# Development skills

Workflows for agents *building* this repository. A skill is written when a workflow has
actually repeated, not in anticipation of one.

| Skill | Written because |
|---|---|
| [`design-review/`](design-review/SKILL.md) | Design review had already run twice — commits `e74d012` and `0cd7330` — finding a gate C20 repository-boundary breach, a gate passing on an unearned assertion, and a skipped test reported as `failed`. Neither round left a durable artifact and neither produced a `rules/` entry. ADR-0009. |

A slash command in `.claude/commands/` is the right home for a short procedure that names a few
`just` recipes. A skill is the right home when the instruction needs progressive disclosure —
`design-review` is one page plus a reference the reviewer loads only when a lens is needed, and
its normative standard lives in `docs/design_review/design_principles/` where ADRs and reviews
cite it too.

`.claude/skills/` is the single source. `.codex/skills` and `.agents/skills` are symlinks to it,
so Claude Code and Codex discover the same files.

**Not to be confused with `skills/library-research/`**, which is the skill this service *ships*.
That one is a product deliverable governed by the blueprint, it stays synchronized with the
implemented tool contract, and it is installed to user scope only by an explicit, operator-run
`just install-skill --apply`.
