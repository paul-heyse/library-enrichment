# `typer._click.shell_completion`

Distribution: `typer`

## ShellCompleteType

`typer._click.shell_completion.ShellCompleteType`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ShellCompleteType = TypeVar('ShellCompleteType', bound='type[ShellComplete]')
```

## _available_shells

`typer._click.shell_completion._available_shells`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_available_shells: dict[str, type[ShellComplete]] = {}
```

## CompletionItem

Import as `typer.core.CompletionItem`  ·  defined at `typer._click.shell_completion.CompletionItem`

```python
class CompletionItem
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `help: str | None = help`  _instance-attribute_
- `type: str = type`  _instance-attribute_
- `value: Any = value`  _instance-attribute_

Represents a completion value and metadata about the value. The
default metadata is ``type`` to indicate special shell handling,
and ``help`` if a shell supports showing a help string next to the
value.

Arbitrary parameters can be passed when creating the object, and
accessed using ``item.attr``. If an attribute wasn't passed,
accessing it returns ``None``.


## ShellComplete

`typer._click.shell_completion.ShellComplete`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ShellComplete(ABC)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`

**Declared members (13)**

- `cli = cli`  _instance-attribute_
- `def complete(self) -> str`
  Produce the completion data to send back to the shell.
- `complete_var = complete_var`  _instance-attribute_
- `ctx_args = ctx_args`  _instance-attribute_
- `def format_completion(self, item: CompletionItem) -> str`  _abstractmethod_
  Format a completion item into the form recognized by the shell script. This must be implemented by subclasses.
- `func_name: str`  _property_
  The name of the shell function defined by the completion script.
- `def get_completion_args(self) -> tuple[list[str], str]`  _abstractmethod_
  Use the env vars defined by the shell script to return a tuple of ``args, incomplete``. This must be implemented by subclasses.
- `def get_completions(self, args: list[str], incomplete: str) -> list[CompletionItem]`
  Determine the context and last complete command or parameter from the complete args. Call that object's ``shell_complete`` method to get the completions for the incomplete value.
- `name: str`  _class-attribute_
  Name to register the shell as with `add_completion_class`. This is used in completion instructions (``{name}_source`` and ``{name}_complete``).
- `prog_name = prog_name`  _instance-attribute_
- `def source(self) -> str`
  Produce the shell script that defines the completion function. By default this ``%``-style formats `source_template` with the dict returned by `source_vars`.
- `source_template: str`  _class-attribute_
  Completion script template formatted by `source`. This must be provided by subclasses.
- `def source_vars(self) -> dict[str, Any]`  _abstractmethod_
  Vars for formatting `source_template`.

Base class for providing shell completion support. A subclass for
a given shell will override attributes and methods to implement the
completion instructions (``source`` and ``complete``).


## _is_incomplete_argument

`typer._click.shell_completion._is_incomplete_argument`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_incomplete_argument(ctx: Context, param: Parameter) -> bool
```

Determine if the given parameter is an argument that can still
accept values.


## _is_incomplete_option

`typer._click.shell_completion._is_incomplete_option`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_incomplete_option(ctx: Context, args: list[str], param: Parameter) -> bool
```

Determine if the given parameter is an option that needs a value.


## _resolve_context

`typer._click.shell_completion._resolve_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolve_context(cli: Command, ctx_args: MutableMapping[str, Any], prog_name: str, args: list[str]) -> Context
```

Produce the context hierarchy starting with the command and
traversing the complete arguments. This only follows the commands,
it doesn't trigger input prompts or callbacks.


## _resolve_incomplete

`typer._click.shell_completion._resolve_incomplete`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolve_incomplete(ctx: Context, args: list[str], incomplete: str) -> tuple[Command | Parameter, str]
```

Find the Click object that will handle the completion of the
incomplete value. Return the object and the incomplete value.


## _start_of_option

`typer._click.shell_completion._start_of_option`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _start_of_option(ctx: Context, value: str) -> bool
```

Check if the value looks like the start of an option.


## add_completion_class

`typer._click.shell_completion.add_completion_class`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def add_completion_class(cls: ShellCompleteType, name: str) -> ShellCompleteType
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Register a `ShellComplete` subclass under the given name.
The name will be provided by the completion instruction environment
variable during completion.


## get_completion_class

`typer._click.shell_completion.get_completion_class`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_completion_class(shell: str) -> type[ShellComplete] | None
```

Look up a registered `ShellComplete` subclass by the name
provided by the completion instruction environment variable. If the
name isn't registered, returns ``None``.


## split_arg_string

`typer._click.shell_completion.split_arg_string`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def split_arg_string(string: str) -> list[str]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Split an argument string as with `shlex.split`, but don't
fail if the string is incomplete. Ignores a missing closing quote or
incomplete escape sequence and uses the partial token as-is.


