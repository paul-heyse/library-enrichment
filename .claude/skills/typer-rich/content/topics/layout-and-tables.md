# Tables, panels, columns and trees

A renderable describes structure; the Console decides size. Width flows downward from the Console, never upward from the Table -- so a table that wraps is almost always a Console question, not a Table one.

## Entry points

| Type | Kind | Import as | Prose | Records |
|---|---|---|---|---|
| `rich.table.Table` | class | `rich.table.Table` | [prose](../api/rich.table.md) | [records](../model/rich.table.json) |
| `rich.panel.Panel` | class | `rich.panel.Panel` | [prose](../api/rich.panel.md) | [records](../model/rich.panel.json) |
| `rich.tree.Tree` | class | `rich.tree.Tree` | [prose](../api/rich.tree.md) | [records](../model/rich.tree.json) |
| `rich.box.Box` | class | `rich.box.Box` | [prose](../api/rich.box.md) | [records](../model/rich.box.json) |

## Upstream guides

- [`corpus/rich/docs/tables.rst`](../corpus/rich/docs/tables.rst)
- [`corpus/rich/docs/panel.rst`](../corpus/rich/docs/panel.rst)

## Decision rules

- Width belongs to the Console, not the renderable. A `Table.width` is a request and the Console still crops.
- Output going into a file someone will diff: pin the width and use an ASCII box.
- A custom renderable with a natural width: implement `__rich_measure__` as well.

## Anti-patterns

- Assuming the default box renders identically everywhere; it is substituted on legacy Windows consoles, which this index never executes.
