# Typer's vendored Click

Typer 0.27.2 carries its own adapted copy of Click at `typer/_click/`. Fifteen
modules and 5,548 lines -- `Context`, `Parameter`, `ParamType`, shell completion and the
whole exception hierarchy -- now sit behind a private module path.

**`typer._click` is not `click`.** It is Typer's adapted copy. Every row under this
subject is a fact about that copy at this pin. Never cite one as a fact about the `click`
package, and never write the import.

## Why the index does not stop at "not importable"

It is true that you cannot import any of this, and on its own that is a useless answer.
You never import `typer._click.core.Context` -- you are *handed* one as `ctx`, and
`typer.Context` subclasses it, so most of what you can call on `typer.Context` is
declared in a module you cannot name. So nameability is recorded honestly and
**reachability** is recorded beside it.

| Reachability | Count | What it means |
|---|---|---|
| `importable` | 15 | Typer re-exports it; write that spelling |
| `subclassed-via` | 5 | a class you *can* name inherits from it |
| `subtype-of` | 6 | you handle it through an ancestor you can name |
| `received` | 6 | it arrives in a parameter of something you can name |
| `declared` | 14 | what your annotation becomes; see the `param-type` seam |
| `registered-via` | 1 | reached only through a registrar function |
| `internal` | 154 | genuinely never met; the manifest allowlist is the claim |

## Re-exported by Typer (15)

These you write as ordinary Typer API. The definition path is private; the import path
is not.

| Write | Defined at |
|---|---|
| `typer.cli.Command` | `typer._click.core.Command` |
| `typer.BadParameter` | `typer._click.exceptions.BadParameter` |
| `typer.main.get_current_context` | `typer._click.globals.get_current_context` |
| `typer.core.CompletionItem` | `typer._click.shell_completion.CompletionItem` |
| `typer.confirm` | `typer._click.termui.confirm` |
| `typer.getchar` | `typer._click.termui.getchar` |
| `typer.progressbar` | `typer._click.termui.progressbar` |
| `typer.prompt` | `typer._click.termui.prompt` |
| `typer.secho` | `typer._click.termui.secho` |
| `typer.style` | `typer._click.termui.style` |
| `typer.echo` | `typer._click.utils.echo` |
| `typer.format_filename` | `typer._click.utils.format_filename` |
| `typer.get_app_dir` | `typer._click.utils.get_app_dir` |
| `typer.get_binary_stream` | `typer._click.utils.get_binary_stream` |
| `typer.get_text_stream` | `typer._click.utils.get_text_stream` |

## Reached without being importable (32)

| Class | Reached | Via |
|---|---|---|
| `typer._click._termui_impl.ProgressBar` | received | `typer.progressbar (parameter)` |
| `typer._click.core.Context` | subclassed-via | `typer.Context (subclass)` |
| `typer._click.core.Parameter` | subclassed-via | `typer.CallbackParam (subclass)`, `typer.core.TyperArgument (subclass)` |
| `typer._click.core.ParameterSource` | received | `typer.CallbackParam.consume_value (parameter)`, `typer.Context._parameter_source (parameter)` |
| `typer._click.core.V` | received | `typer.Context.ensure_object (parameter)`, `typer.Context.find_object (parameter)` |
| `typer._click.exceptions.BadArgumentUsage` | subtype-of | `typer.TyperException (ancestor)` |
| `typer._click.exceptions.BadOptionUsage` | subtype-of | `typer.TyperException (ancestor)` |
| `typer._click.exceptions.ClickException` | subclassed-via | `typer.BadParameter (subclass)` |
| `typer._click.exceptions.FileError` | subtype-of | `typer.TyperException (ancestor)` |
| `typer._click.exceptions.MissingParameter` | subtype-of | `typer.BadParameter (ancestor)`, `typer.TyperException (ancestor)` |
| `typer._click.exceptions.NoArgsIsHelpError` | subtype-of | `typer.TyperException (ancestor)` |
| `typer._click.exceptions.NoSuchOption` | subtype-of | `typer.TyperException (ancestor)` |
| `typer._click.exceptions.UsageError` | subclassed-via | `typer.BadParameter (subclass)` |
| `typer._click.formatting.HelpFormatter` | received | `typer.Context.formatter_class (parameter)`, `typer.Context.make_formatter (parameter)` |
| `typer._click.parser._OptionParser` | received | `typer.cli.Command.make_parser (parameter)`, `typer.cli.TyperCLIGroup.make_parser (parameter)` |
| `typer._click.shell_completion.ShellComplete` | registered-via | `typer._click.shell_completion.add_completion_class (registry)` |
| `typer._click.termui.V` | received | `typer.progressbar (parameter)` |
| `typer._click.types.BoolParamType` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.CompositeParamType` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.DateTime` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.File` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.FloatParamType` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.FloatRange` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.FuncParamType` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.IntParamType` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.IntRange` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.ParamType` | subclassed-via | `typer.main.TyperChoice (subclass)`, `typer.models.TyperPath (subclass)` |
| `typer._click.types.StringParamType` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.Tuple` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types.UUIDParameterType` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types._NumberParamTypeBase` | declared | `typer.Option / typer.Argument (annotation resolution)` |
| `typer._click.types._NumberRangeBase` | declared | `typer.Option / typer.Argument (annotation resolution)` |

## Modules

| Module | Items |
|---|---|
| `typer._click._compat` | 42 |
| `typer._click._termui_impl` | 8 |
| `typer._click._textwrap` | 1 |
| `typer._click._winconsole` | 37 |
| `typer._click.core` | 9 |
| `typer._click.decorators` | 8 |
| `typer._click.exceptions` | 10 |
| `typer._click.formatting` | 6 |
| `typer._click.globals` | 5 |
| `typer._click.parser` | 8 |
| `typer._click.shell_completion` | 12 |
| `typer._click.termui` | 17 |
| `typer._click.types` | 24 |
| `typer._click.utils` | 14 |
