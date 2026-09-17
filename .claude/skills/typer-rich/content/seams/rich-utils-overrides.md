# Restyling Typer's help output

**kind** `module-constant` · **supported** `private` · **subject** `typer`

Extension by rebinding a module-level name. It works; it is not a promise.

**Not a public interface.** It works at this pinned release and upstream has promised nothing. Prefer the supported alternative named below, and if you use this anyway, write down that you did.

## Entry points

- `typer.rich_utils` — a module, 84 indexed names · `api/typer.rich_utils.md`

## Mental model

`typer.rich_utils` holds module-level constants -- the `STYLE_*` names, `MAX_WIDTH`, `COLOR_SYSTEM`, `FORCE_TERMINAL` -- that the help renderer reads. Rebinding them works. Nothing promises it will keep working, and several are captured at first import rather than read per call, so the order of your imports becomes load-bearing. This page exists so that an agent who finds the mechanism does not ship it while describing it as an API.

## Required

- `ABORTED_TEXT`
- `ALIGN_COMMANDS_PANEL`
- `ALIGN_ERRORS_PANEL`
- `ALIGN_OPTIONS_PANEL`
- `ANSI_PREFIX`
- `ARGUMENTS_PANEL_TITLE`
- `COLOR_SYSTEM`
- `COMMANDS_PANEL_TITLE`
- `DEFAULT_STRING`
- `DEPRECATED_STRING`
- `ENVVAR_STRING`
- `ERRORS_PANEL_TITLE`
- `FORCE_TERMINAL`
- `MARKUP_MODE_MARKDOWN`
- `MARKUP_MODE_RICH`
- `MAX_WIDTH`
- `MarkupModeStrict`
- `OPTIONS_PANEL_TITLE`
- `RANGE_STRING`
- `REQUIRED_LONG_STRING`
- `REQUIRED_SHORT_STRING`
- `RICH_HELP`
- `STYLE_ABORTED`
- `STYLE_COMMANDS_PANEL_BORDER`
- `STYLE_COMMANDS_TABLE_BORDER_STYLE`
- `STYLE_COMMANDS_TABLE_BOX`
- `STYLE_COMMANDS_TABLE_FIRST_COLUMN`
- `STYLE_COMMANDS_TABLE_LEADING`
- `STYLE_COMMANDS_TABLE_PADDING`
- `STYLE_COMMANDS_TABLE_PAD_EDGE`
- `STYLE_COMMANDS_TABLE_ROW_STYLES`
- `STYLE_COMMANDS_TABLE_SHOW_LINES`
- `STYLE_DEPRECATED`
- `STYLE_DEPRECATED_COMMAND`
- `STYLE_ERRORS_PANEL_BORDER`
- `STYLE_ERRORS_SUGGESTION`
- `STYLE_HELPTEXT`
- `STYLE_HELPTEXT_FIRST_LINE`
- `STYLE_NEGATIVE_OPTION`
- `STYLE_NEGATIVE_SWITCH`
- `STYLE_OPTION`
- `STYLE_OPTIONS_PANEL_BORDER`
- `STYLE_OPTIONS_TABLE_BORDER_STYLE`
- `STYLE_OPTIONS_TABLE_BOX`
- `STYLE_OPTIONS_TABLE_LEADING`
- `STYLE_OPTIONS_TABLE_PADDING`
- `STYLE_OPTIONS_TABLE_PAD_EDGE`
- `STYLE_OPTIONS_TABLE_ROW_STYLES`
- `STYLE_OPTIONS_TABLE_SHOW_LINES`
- `STYLE_OPTION_DEFAULT`
- `STYLE_OPTION_ENVVAR`
- `STYLE_OPTION_HELP`
- `STYLE_REQUIRED_LONG`
- `STYLE_REQUIRED_SHORT`
- `STYLE_SWITCH`
- `STYLE_TYPES`
- `STYLE_TYPES_SEPARATOR`
- `STYLE_USAGE`
- `STYLE_USAGE_COMMAND`
- `_RICH_HELP_PANEL_NAME`
- `_TERMINAL_WIDTH`
- `_TYPER_FORCE_DISABLE_TERMINAL`
- `highlighter`
- `negative_highlighter`
- `types_highlighter`

## Decision rules

- Grouping options under headings: `rich_help_panel=`, which is supported.
- Markdown instead of rich markup in docstrings: `rich_markup_mode=`, which is supported.
- Restructuring the help layout: a custom `TyperGroup`, which is supported.
- Changing a colour with nothing else available: rebind the constant, and write down that you did.

## Anti-patterns

- Passing a `Theme` to your own Console and expecting help to pick it up: `rich_utils` builds its own Console and never consults yours.
- Setting `TERMINAL_WIDTH` after `typer.rich_utils` has been imported: `MAX_WIDTH` was already frozen.

## Observed

Probe `T002` — see `content/probes/00-index.md`.
