# Seams

Where your code meets theirs. Not all of these are extension points, and the
`supported` column is the one to read first: `private` means it works and upstream has
promised nothing, which is a different thing from an API even though it is reached the
same way.

| Seam | Kind | Supported | Subject | Implementors | What it is |
|---|---|---|---|---:|---|
| [`renderable-protocol`](renderable-protocol.md) | runtime-protocol | `documented` | rich | 49 | Making your own object printable |
| [`measurement`](measurement.md) | duck-protocol | `documented` | rich | 18 | Telling rich how wide your renderable wants to be |
| [`highlighter`](highlighter.md) | abstract-base | `documented` | rich | 9 | Automatic styling of text by pattern |
| [`theme-and-style`](theme-and-style.md) | parameter | `documented` | rich | 0 | Naming your own styles |
| [`box`](box.md) | parameter | `documented` | rich | 0 | Choosing the characters a border is drawn with |
| [`render-hooks`](render-hooks.md) | abstract-base | `documented` | rich | 1 | Intercepting everything a Console renders |
| [`jupyter`](jupyter.md) | subclass | `observed-only` | rich | 20 | Rendering inside a notebook |
| [`param-type`](param-type.md) | abstract-base | `documented` | click | 16 | Turning a command-line string into your own type |
| [`callbacks-and-context`](callbacks-and-context.md) | parameter | `documented` | typer | 0 | Running code around a command, and sharing state |
| [`command-classes`](command-classes.md) | subclass | `documented` | typer | 1 | Changing how a command or group behaves |
| [`rich-utils-overrides`](rich-utils-overrides.md) | module-constant | `private` | typer | 0 | Restyling Typer's help output |
