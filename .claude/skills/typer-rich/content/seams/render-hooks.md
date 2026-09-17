# Intercepting everything a Console renders

**kind** `abstract-base` · **supported** `documented` · **subject** `rich`

An abstract base. The required members are the obligations; everything else is provided, and provided is where the capability hides.

Upstream documents this.

## Entry points

- `rich.console.RenderHook` · defined at `rich.console.RenderHook` · `api/rich.console.md`

## Mental model

`process_renderables` sees every batch on its way out and may rewrite it. This is the mechanism `Live` is built on, which is also the warning: two things pushing hooks onto one Console will fight.

## Required

- `process_renderables`

## Implementors (1)

- `rich.live.Live`

## Decision rules

- You want to wrap or annotate all output: push a render hook.
- You want to capture output instead: `Console(record=True)` and `export_text`, or a StringIO file.

## Anti-patterns

- Pushing a hook while a `Live` or `Progress` owns the same Console.
