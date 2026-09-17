# `typer._completion_classes`

Distribution: `typer`

## BashComplete

`typer._completion_classes.BashComplete`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class BashComplete(ShellComplete)
```

**Bases** `ShellComplete`

**Declared members (7)**

- `def complete(self) -> str`
- `def format_completion(self, item: CompletionItem) -> str`
- `def get_completion_args(self) -> tuple[list[str], str]`
- `name = Shells.bash.value`  _class-attribute, instance-attribute_
- `def source(self) -> str`
- `source_template = COMPLETION_SCRIPT_BASH`  _class-attribute, instance-attribute_
- `def source_vars(self) -> dict[str, Any]`

**Inherited (6)**

- from `typer._click.shell_completion.ShellComplete`: `cli`, `complete_var`, `ctx_args`, `func_name`, `get_completions`, `prog_name`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## FishComplete

`typer._completion_classes.FishComplete`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class FishComplete(ShellComplete)
```

**Bases** `ShellComplete`

**Declared members (6)**

- `def complete(self) -> str`
- `def format_completion(self, item: CompletionItem) -> str`
- `def get_completion_args(self) -> tuple[list[str], str]`
- `name = Shells.fish.value`  _class-attribute, instance-attribute_
- `source_template = COMPLETION_SCRIPT_FISH`  _class-attribute, instance-attribute_
- `def source_vars(self) -> dict[str, Any]`

**Inherited (7)**

- from `typer._click.shell_completion.ShellComplete`: `cli`, `complete_var`, `ctx_args`, `func_name`, `get_completions`, `prog_name`, `source`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## PowerShellComplete

`typer._completion_classes.PowerShellComplete`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PowerShellComplete(ShellComplete)
```

**Bases** `ShellComplete`

**Declared members (5)**

- `def format_completion(self, item: CompletionItem) -> str`
- `def get_completion_args(self) -> tuple[list[str], str]`
- `name = Shells.powershell.value`  _class-attribute, instance-attribute_
- `source_template = COMPLETION_SCRIPT_POWER_SHELL`  _class-attribute, instance-attribute_
- `def source_vars(self) -> dict[str, Any]`

**Inherited (8)**

- from `typer._click.shell_completion.ShellComplete`: `cli`, `complete`, `complete_var`, `ctx_args`, `func_name`, `get_completions`, `prog_name`, `source`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## ZshComplete

`typer._completion_classes.ZshComplete`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ZshComplete(ShellComplete)
```

**Bases** `ShellComplete`

**Declared members (6)**

- `def complete(self) -> str`
- `def format_completion(self, item: CompletionItem) -> str`
- `def get_completion_args(self) -> tuple[list[str], str]`
- `name = Shells.zsh.value`  _class-attribute, instance-attribute_
- `source_template = COMPLETION_SCRIPT_ZSH`  _class-attribute, instance-attribute_
- `def source_vars(self) -> dict[str, Any]`

**Inherited (7)**

- from `typer._click.shell_completion.ShellComplete`: `cli`, `complete_var`, `ctx_args`, `func_name`, `get_completions`, `prog_name`, `source`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## _sanitize_help_text

`typer._completion_classes._sanitize_help_text`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _sanitize_help_text(text: str) -> str
```

Sanitizes the help text by removing rich tags


## completion_init

Import as `typer.completion.completion_init`  ·  defined at `typer._completion_classes.completion_init`

```python
def completion_init() -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

