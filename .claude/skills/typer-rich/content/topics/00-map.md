# Capability map

Each page maps one capability to the types that implement it, the upstream guide that
explains it, and the decisions worth making deliberately. Start here when the question
is *which construct do I want*, rather than *how is this spelled*.

| Topic | Covers |
|---|---|
| [Building a command-line interface](building-a-command.md) | Typer apps, commands, arguments, options, subcommands |
| [Getting text onto the terminal](printing-and-styling.md) | echo, print, Console, styles, colour constants |
| [Square brackets, and where text goes missing](markup-and-escaping.md) | rich markup, escape, rich_markup_mode, help strings |
| [Tables, panels, columns and trees](layout-and-tables.md) | Table, Panel, Columns, Tree, Rule, Align, box styles |
| [Why the output is the width it is](width-and-wrapping.md) | Console width, COLUMNS, TERM, cell width, soft_wrap |
| [Why there is no colour](colour-and-terminal-detection.md) | NO_COLOR, FORCE_COLOR, TTY_COMPATIBLE, color_system, force_terminal |
| [Asking the user something](input-and-prompts.md) | prompt, confirm, Prompt, Confirm, password input |
| [Progress bars and live displays](progress-and-liveness.md) | Progress, Live, Status, Spinner, track |
| [Failing usefully](errors-and-tracebacks.md) | Exit, Abort, TyperException, MarkupError, rich tracebacks |
| [Shaping the help output](help-and-docs.md) | docstrings, rich_markup_mode, rich_help_panel, epilog, rich_utils |
| [Turning command-line strings into your types](custom-types-and-parsing.md) | annotations, ParamType, parser, callbacks, enums, paths |
| [Asserting on what your CLI printed](testing-and-capture.md) | CliRunner, Console(record=True), export_text, StringIO |
| [Packaging and completion](shipping-a-cli.md) | entry points, shell completion, app directories, launching |
