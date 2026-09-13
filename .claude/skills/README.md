# Development skills

Workflows for agents *building* this repository. Currently empty: a skill should be written
when a workflow has actually repeated, not in anticipation of one.

`.claude/skills/` is the single source. `.codex/skills` and `.agents/skills` are symlinks to it,
so Claude Code and Codex discover the same files.

**Not to be confused with `skills/library-research/`**, which is the skill this service *ships*.
That one is a product deliverable governed by the blueprint, it stays synchronized with the
implemented tool contract, and it is installed to user scope only by an explicit, operator-run
`just install-skill --apply`.
