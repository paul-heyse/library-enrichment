# Same job, different answer

Typer depends on Rich, and `typer/rich_utils.py` is the seam where help, errors and
`rich_markup_mode` render. That makes them one subject -- and it also means they present
near-duplicate surfaces that look interchangeable and are not.

Each row names the spelling to write, the one an agent reaches for instead, and what
actually differs. Rows carrying a probe id were measured, not reasoned about.

| I want to… | Write | Not | Because | Probe |
|---|---|---|---|---|
| print a line of plain text from a CLI | `typer.echo` | `rich.print`, `rich.console.Console.print` | Both Rich surfaces parse markup, so any bracketed word in the string may be silently deleted. `typer.echo` is literal. | `O001` |
| print text I did not write myself | `rich.markup.escape` | `rich.console.Console.print` | Untrusted text reaching a Console is parsed as markup. Escape it, or pass `markup=False`. | `M003` |
| print a table, a panel or anything laid out | `rich.console.Console.print` | `typer.echo` | `typer.echo` has no concept of a renderable or a width; it writes the string it is given. | — |
| colour one line of output | `typer.secho` | `typer.style`, `rich.style.Style` | `typer.style` returns a string you still have to echo. A rich `Style` object is not accepted by either Typer function -- they take colour strings. | `O002` |
| reuse a colour in several places | `rich.style.Style` | `typer.colors.RED` | `typer.colors.RED` is the plain string "red". It is legal in Rich markup only because the spellings coincide, and it carries no other attributes. | `O002` |
| show a progress bar | `rich.progress.Progress` | `typer.progressbar` | `typer.progressbar` is the vendored Click one: no columns, no transient mode, and it writes to a stream Rich does not own, so it fights a live Console. | — |
| ask the user a question | `rich.prompt.Prompt.ask` | `typer.prompt`, `typer.confirm` | Both Typer prompts bypass the Console, so under a `Live` or `Progress` the prompt is overwritten. Fine standalone. | — |
| exit with a code I choose | `typer.Exit` | `typer.Abort` | `Abort` always exits 1 and prints "Aborted."; `Exit` takes the code. | — |
| catch any error Typer raises | `typer.TyperException` | `typer.BadParameter` | `BadParameter` is the only vendored Click exception re-exported at top level; the other six are reached through the `TyperException` ancestor. | — |
| assert on CLI output in a test | `typer.testing.CliRunner` | `rich.console.Console` | A recording Console sees only what you printed through it, not what the vendored Click wrote. `CliRunner` also pins the plain formatter's width, though not the Rich one. | — |
| control how wide the output is | `rich.console.Console` | `typer.get_terminal_size` | Width is a property of the Console, not of the thing you print -- and even an explicitly passed width loses to `TERM=dumb`. | `E003` |
| get the installed version at runtime | `importlib.metadata.version` | `rich.__version__` | Does not exist at rich 15.0.0. `typer.__version__` does, which makes the asymmetry easy to trip over. | `V002` |

## The rule underneath

`typer.echo` is literal: no markup, no wrapping, no width. Everything on the Rich side
parses markup and wraps to a Console width. That single difference explains most of the
table, and it is why a help string containing `[path]` loses the word while the same
string through `typer.echo` does not.
