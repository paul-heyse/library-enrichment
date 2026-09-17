# `typer._click.exceptions`

Distribution: `typer`

## BadArgumentUsage

`typer._click.exceptions.BadArgumentUsage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class BadArgumentUsage(UsageError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `UsageError`

**Inherited (7)**

- from `typer._click.exceptions.ClickException`: `show_color`
- from `typer._click.exceptions.UsageError`: `cmd`, `ctx`, `exit_code`, `show`
- from `typer.exceptions.TyperException`: `format_message`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Raised if an argument is generally supplied but the use of the argument
was incorrect.  This is for instance raised if the number of values
for an argument is not correct.


## BadOptionUsage

`typer._click.exceptions.BadOptionUsage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class BadOptionUsage(UsageError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `UsageError`

**Declared members (1)**

- `option_name = option_name`  _instance-attribute_

**Inherited (7)**

- from `typer._click.exceptions.ClickException`: `show_color`
- from `typer._click.exceptions.UsageError`: `cmd`, `ctx`, `exit_code`, `show`
- from `typer.exceptions.TyperException`: `format_message`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Raised if an option is generally supplied but the use of the option
was incorrect.  This is for instance raised if the number of arguments
for an option is not correct.


## BadParameter

Import as `typer.BadParameter`  ·  defined at `typer._click.exceptions.BadParameter`

```python
class BadParameter(UsageError)
```

**Also exported as** `typer.BadParameter`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `UsageError`

**Declared members (3)**

- `def format_message(self) -> str`
- `param = param`  _instance-attribute_
- `param_hint = param_hint`  _instance-attribute_

**Inherited (6)**

- from `typer._click.exceptions.ClickException`: `show_color`
- from `typer._click.exceptions.UsageError`: `cmd`, `ctx`, `exit_code`, `show`
- from `typer.exceptions.TyperException`: `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An exception that formats out a standardized error message for a
bad parameter.  This is useful when thrown from a callback or type as
Click will attach contextual information to it (for instance, which
parameter it is).


## ClickException

`typer._click.exceptions.ClickException`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ClickException(TyperException)
```

**Also exported as** `typer._click.ClickException`

**Bases** `TyperException`

**Declared members (2)**

- `def show(self, file: IO[Any] | None = None) -> None`
- `show_color: bool | None = resolve_color_default()`  _instance-attribute_

**Inherited (3)**

- from `typer.exceptions.TyperException`: `exit_code`, `format_message`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An exception that Click can handle and show to the user.


## FileError

`typer._click.exceptions.FileError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class FileError(ClickException)
```

**Bases** `ClickException`

**Declared members (3)**

- `filename = filename`  _instance-attribute_
- `def format_message(self) -> str`
- `ui_filename: str = format_filename(filename)`  _instance-attribute_

**Inherited (4)**

- from `typer._click.exceptions.ClickException`: `show`, `show_color`
- from `typer.exceptions.TyperException`: `exit_code`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Raised if a file cannot be opened.


## MissingParameter

`typer._click.exceptions.MissingParameter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class MissingParameter(BadParameter)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BadParameter`

**Declared members (2)**

- `def format_message(self) -> str`
- `param_type = param_type`  _instance-attribute_

**Inherited (8)**

- from `typer._click.exceptions.BadParameter`: `param`, `param_hint`
- from `typer._click.exceptions.ClickException`: `show_color`
- from `typer._click.exceptions.UsageError`: `cmd`, `ctx`, `exit_code`, `show`
- from `typer.exceptions.TyperException`: `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Raised if click required an option or argument but it was not
provided when invoking the script.


## NoArgsIsHelpError

`typer._click.exceptions.NoArgsIsHelpError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class NoArgsIsHelpError(UsageError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `UsageError`

**Declared members (2)**

- `ctx: Context`  _instance-attribute_
- `def show(self, file: IO[Any] | None = None) -> None`

**Inherited (5)**

- from `typer._click.exceptions.ClickException`: `show_color`
- from `typer._click.exceptions.UsageError`: `cmd`, `exit_code`
- from `typer.exceptions.TyperException`: `format_message`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## NoSuchOption

`typer._click.exceptions.NoSuchOption`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class NoSuchOption(UsageError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `UsageError`

**Declared members (3)**

- `def format_message(self) -> str`
- `option_name = option_name`  _instance-attribute_
- `possibilities = possibilities`  _instance-attribute_

**Inherited (6)**

- from `typer._click.exceptions.ClickException`: `show_color`
- from `typer._click.exceptions.UsageError`: `cmd`, `ctx`, `exit_code`, `show`
- from `typer.exceptions.TyperException`: `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Raised if click attempted to handle an option that does not
exist.


## UsageError

`typer._click.exceptions.UsageError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class UsageError(ClickException)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ClickException`

**Declared members (4)**

- `cmd: Command | None = self.ctx.command if self.ctx else None`  _instance-attribute_
- `ctx = ctx`  _instance-attribute_
- `exit_code = 2`  _class-attribute, instance-attribute_
- `def show(self, file: IO[Any] | None = None) -> None`

**Inherited (3)**

- from `typer._click.exceptions.ClickException`: `show_color`
- from `typer.exceptions.TyperException`: `format_message`, `message`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An internal exception that signals a usage error.  This typically
aborts any further handling.


## _join_param_hints

`typer._click.exceptions._join_param_hints`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _join_param_hints(param_hint: Sequence[str] | str | None) -> str | None
```

