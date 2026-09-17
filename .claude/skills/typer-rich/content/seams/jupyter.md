# Rendering inside a notebook

**kind** `subclass` · **supported** `observed-only` · **subject** `rich`

A concrete class you are meant to extend.

Indexed from source and not executed here. Real, and not demonstrated by any probe in this repository -- see the known limits in `reference.md`.

## Entry points

- `rich.jupyter.JupyterMixin` · defined at `rich.jupyter.JupyterMixin` · `api/rich.jupyter.md`

## Mental model

`JupyterMixin` gives a renderable a `_repr_mimebundle_`, so a notebook displays it without a Console. Indexed from source and never executed: no probe here imports IPython, and `is_jupyter()` is False in all of them.

## Implementors (20)

- `rich._inspect.Inspect`
- `rich.align.Align`
- `rich.align.VerticalCenter`
- `rich.bar.Bar`
- `rich.columns.Columns`
- `rich.constrain.Constrain`
- `rich.emoji.Emoji`
- `rich.live.Live`
- `rich.markdown.Markdown`
- `rich.padding.Padding`
- `rich.panel.Panel`
- `rich.pretty.Pretty`
- `rich.progress.Progress`
- `rich.progress_bar.ProgressBar`
- `rich.rule.Rule`
- `rich.status.Status`
- `rich.syntax.Syntax`
- `rich.table.Table`
- `rich.text.Text`
- `rich.tree.Tree`

## Decision rules

- Your renderable should display in a notebook cell: mix in `JupyterMixin`.
- Rich is not detecting the notebook: `Console(force_jupyter=True)`.

## Anti-patterns

- Trusting this page as verified behaviour. It is marked observed-only for a reason -- see known limit 6 in reference.md.
