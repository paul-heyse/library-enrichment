# `typer._click.core`

Distribution: `typer`

## F

`typer._click.core.F`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
F = TypeVar('F', bound='Callable[..., Any]')
```

## V

`typer._click.core.V`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
V = TypeVar('V')
```

## Command

Import as `typer.cli.Command`  ·  defined at `typer._click.core.Command`

```python
class Command(ABC)
```

**Also exported as** `typer._click.Command`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`

**Declared members (34)**

- `add_help_option = add_help_option`  _instance-attribute_
- `allow_extra_args = False`  _class-attribute, instance-attribute_
- `allow_interspersed_args = True`  _class-attribute, instance-attribute_
- `callback = callback`  _instance-attribute_
- `def collect_usage_pieces(self, ctx: Context) -> list[str]`
  Returns all the pieces that go into the usage line and returns it as a list of strings.
- `context_class: type[Context] = Context`  _class-attribute, instance-attribute_
- `context_settings: MutableMapping[str, Any] = context_settings`  _instance-attribute_
- `deprecated = deprecated`  _instance-attribute_
- `epilog = epilog`  _instance-attribute_
- `def format_epilog(self, ctx: Context, formatter: HelpFormatter) -> None`
  Writes the epilog into the formatter if it exists.
- `def format_help(self, ctx: Context, formatter: HelpFormatter) -> None`
  Writes the help into the formatter if it exists.
- `def format_help_text(self, ctx: Context, formatter: HelpFormatter) -> None`
  Writes the help text to the formatter if it exists.
- `def format_options(self, ctx: Context, formatter: HelpFormatter) -> None`  _abstractmethod_
- `def format_usage(self, ctx: Context, formatter: HelpFormatter) -> None`
  Writes the usage line into the formatter.
- `def get_help(self, ctx: Context) -> str`
  Formats the help into a string and returns it.
- `def get_help_option(self, ctx: Context) -> Union[TyperOption, None]`
  Returns the help option object.
- `def get_help_option_names(self, ctx: Context) -> list[str]`
  Returns the names for the help option.
- `def get_params(self, ctx: Context) -> list[Parameter]`
- `def get_short_help_str(self, limit: int = 45) -> str`
  Gets short help for the command or makes it by shortening the long help string.
- `def get_usage(self, ctx: Context) -> str`
  Formats the usage line into a string and returns it.
- `help = help`  _instance-attribute_
- `hidden = hidden`  _instance-attribute_
- `ignore_unknown_options = False`  _class-attribute, instance-attribute_
- `def invoke(self, ctx: Context) -> Any`
  Given a context, this invokes the attached callback (if it exists) in the right way.
- `def main(self, args: Sequence[str] | None = None, prog_name: str | None = None, complete_var: str | None = None, standalone_mode: bool = True, windows_expand_args: bool = True, extra: Any = {}) -> Any`  _abstractmethod_
- `def make_context(self, info_name: str | None, args: list[str], parent: Context | None = None, extra: Any = {}) -> Context`
  This function when given an info name and arguments will kick off the parsing and create a new `Context`.  It does not invoke the actual command callback though.
- `def make_parser(self, ctx: Context) -> _OptionParser`
  Creates the underlying option parser for this command.
- `name = name`  _instance-attribute_
- `no_args_is_help = no_args_is_help`  _instance-attribute_
- `options_metavar = options_metavar`  _instance-attribute_
- `params: list[Parameter] = params or []`  _instance-attribute_
- `def parse_args(self, ctx: Context, args: list[str]) -> list[str]`
- `def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]`
  Return a list of completions for the incomplete value. Looks at the names of options and chained multi-commands.
- `short_help = short_help`  _instance-attribute_

Commands are the basic building block of command line interfaces in
Click.  A basic command handles command line parsing and might dispatch
more parsing to commands nested below it.


## Context

`typer._click.core.Context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Context
```

**Also exported as** `typer._click.Context`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (39)**

- `def abort(self) -> NoReturn`
  Aborts the script.
- `allow_extra_args = allow_extra_args`  _instance-attribute_
- `allow_interspersed_args: bool = allow_interspersed_args`  _instance-attribute_
- `args: list[str] = []`  _instance-attribute_
- `auto_envvar_prefix: str | None = auto_envvar_prefix`  _instance-attribute_
- `def call_on_close(self, f: Callable[..., Any]) -> Callable[..., Any]`
  Register a function to be called when the context tears down.
- `def close(self) -> None`
  Invoke all close callbacks registered with `call_on_close`, and exit all context managers entered with `with_resource`.
- `color: bool | None = color`  _instance-attribute_
- `command = command`  _instance-attribute_
- `command_path: str`  _property_
  The computed command path.  This is used for the ``usage`` information on the help page.  It's automatically created by combining the info names of the chain of contexts to the root.
- `default_map: MutableMapping[str, Any] | None = default_map`  _instance-attribute_
- `def ensure_object(self, object_type: type[V]) -> V`
  Like `find_object` but sets the innermost object to a new instance of `object_type` if it does not exist.
- `def exit(self, code: int = 0) -> NoReturn`
  Exits the application with a given exit code.
- `def fail(self, message: str) -> NoReturn`
  Aborts the execution of the program with a specific error message.
- `def find_object(self, object_type: type[V]) -> V | None`
  Finds the closest object of a given type.
- `def find_root(self) -> Context`
  Finds the outermost context.
- `formatter_class: type[HelpFormatter] = HelpFormatter`  _class-attribute, instance-attribute_
- `def get_help(self) -> str`
  Helper method to get formatted help page for the current context and command.
- `def get_parameter_source(self, name: str) -> ParameterSource | None`
  Get the source of a parameter. This indicates the location from which the value of the parameter was obtained.
- `def get_usage(self) -> str`
  Helper method to get formatted usage string for the current context and command.
- `help_option_names: list[str] = help_option_names`  _instance-attribute_
- `ignore_unknown_options: bool = ignore_unknown_options`  _instance-attribute_
- `info_name = info_name`  _instance-attribute_
- `def invoke(self, callback: Callable[..., V], args: Any = (), kwargs: Any = {}) -> V`
  Invokes a command callback in exactly the way it expects.  There are two ways to invoke this method:
- `invoked_subcommand: str | None = None`  _instance-attribute_
- `def lookup_default(self, name: str, call: bool = True) -> Any | None`
  Get the default for a parameter from `default_map`.
- `def make_formatter(self) -> HelpFormatter`
  Creates the HelpFormatter for the help and usage output.
- `max_content_width: int | None = max_content_width`  _instance-attribute_
- `meta: dict[str, Any]`  _property_
  This is a dictionary which is shared with all the contexts that are nested.  It exists so that click utilities can store some state here if they need to.  It is however the responsibility of that code to manage this dictionary well.
- `obj: Any = obj`  _instance-attribute_
- `params: dict[str, Any] = {}`  _instance-attribute_
- `parent = parent`  _instance-attribute_
- `resilient_parsing: bool = resilient_parsing`  _instance-attribute_
- `def scope(self, cleanup: bool = True) -> Iterator[Context]`
  This helper method can be used with the context object to promote it to the current thread local (see `get_current_context`). The default behavior of this is to invoke the cleanup functions which can be disabled by setting `cleanup` to `Fa…
- `def set_parameter_source(self, name: str, source: ParameterSource) -> None`
  Set the source of a parameter. This indicates the location from which the value of the parameter was obtained.
- `show_default: bool | None = show_default`  _instance-attribute_
- `terminal_width: int | None = terminal_width`  _instance-attribute_
- `token_normalize_func: Callable[[str], str] | None = token_normalize_func`  _instance-attribute_
- `def with_resource(self, context_manager: AbstractContextManager[V]) -> V`
  Register a resource as if it were used in a ``with`` statement. The resource will be cleaned up when the context is popped.

The context is a special internal object that holds state relevant
for the script execution at every single level.  It's normally invisible
to commands unless they opt-in to getting access to it.

The context is useful as it can pass internal objects around and can
control special execution features such as reading data from
environment variables.

A context can be used as context manager in which case it will call
`close` on teardown.


## Parameter

`typer._click.core.Parameter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Parameter(ABC)
```

**Also exported as** `typer._click.Parameter`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`

**Declared members (29)**

- `def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None`  _abstractmethod_
- `callback = callback`  _instance-attribute_
- `def consume_value(self, ctx: Context, opts: Mapping[str, Any]) -> tuple[Any, ParameterSource]`
- `default: Any | Callable[[], Any] | None = default`  _instance-attribute_
- `envvar = envvar`  _instance-attribute_
- `expose_value = expose_value`  _instance-attribute_
- `def get_default(self, ctx: Context, call: bool = True) -> Any | Callable[[], Any] | None`
  Get the default for the parameter
- `def get_error_hint(self, ctx: Context) -> str`
  Get a stringified version of the param for use in error messages to indicate which param caused the error.
- `def get_help_record(self, ctx: Context) -> tuple[str, str] | None`  _abstractmethod_
- `def get_usage_pieces(self, ctx: Context) -> list[str]`
- `def handle_parse_result(self, ctx: Context, opts: Mapping[str, Any], args: list[str]) -> tuple[Any, list[str]]`
  Process the value produced by the parser from user input.
- `human_readable_name: str`  _property_
  Returns the human readable name of this parameter.  This is the same as the name for options, but the metavar for arguments.
- `is_eager = is_eager`  _instance-attribute_
- `def make_metavar(self, ctx: Context) -> str`
- `metavar = metavar`  _instance-attribute_
- `multiple = multiple`  _instance-attribute_
- `name: str | None`  _instance-attribute_
- `nargs = nargs`  _instance-attribute_
- `opts: list[str]`  _instance-attribute_
- `param_type_name = 'parameter'`  _class-attribute, instance-attribute_
- `def process_value(self, ctx: Context, value: Any) -> Any`
  Process the value of this parameter
- `required = required`  _instance-attribute_
- `def resolve_envvar_value(self, ctx: Context) -> str | None`
  Returns the value found in the environment variable(s) attached to this parameter.
- `secondary_opts: list[str]`  _instance-attribute_
- `def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]`
  Return a list of completions for the incomplete value. If a ``shell_complete`` function was given during init, it is used. Otherwise, the `type` `ParamType.shell_complete` function is used.
- `type: types.ParamType = types.convert_type(type, default)`  _instance-attribute_
- `def type_cast_value(self, ctx: Context, value: Any) -> Any`
  Convert and validate a value against the parameter's `type`, `multiple`, and `nargs`.
- `def value_from_envvar(self, ctx: Context) -> str | Sequence[str] | None`
  Process the raw environment variable string for this parameter.
- `def value_is_missing(self, value: Any) -> bool`  _abstractmethod_

A parameter to a command comes in two versions: they are either
`Option`\s or `Argument`\s.

Some settings are supported by both options and arguments.


## ParameterSource

`typer._click.core.ParameterSource`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ParameterSource(enum.Enum)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `enum.Enum`

**Declared members (5)**

- `COMMANDLINE = enum.auto()`  _class-attribute, instance-attribute_
  The value was provided by the command line args.
- `DEFAULT = enum.auto()`  _class-attribute, instance-attribute_
  Used the default specified by the parameter.
- `DEFAULT_MAP = enum.auto()`  _class-attribute, instance-attribute_
  Used a default provided by `Context.default_map`.
- `ENVIRONMENT = enum.auto()`  _class-attribute, instance-attribute_
  The value was provided with an environment variable.
- `PROMPT = enum.auto()`  _class-attribute, instance-attribute_
  Used a prompt to confirm a default or provide a value.

This is an `Enum` that indicates the source of a
parameter's value.


## _complete_visible_commands

`typer._click.core._complete_visible_commands`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _complete_visible_commands(ctx: Context, incomplete: str) -> Iterator[tuple[str, Command]]
```

List all the subcommands of a group that start with the
incomplete value and aren't hidden.


## augment_usage_errors

`typer._click.core.augment_usage_errors`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def augment_usage_errors(ctx: Context, param: Union[Parameter, None] = None) -> Iterator[None]
```

Context manager that attaches extra information to exceptions.


## iter_params_for_processing

`typer._click.core.iter_params_for_processing`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def iter_params_for_processing(invocation_order: Sequence[Parameter], declaration_order: Sequence[Parameter]) -> list[Parameter]
```

Returns all declared parameters in the order they should be processed.

The declared parameters are re-shuffled depending on the order in which
they were invoked, as well as the eagerness of each parameters.

The invocation order takes precedence over the declaration order. I.e. the
order in which the user provided them to the CLI is respected.

This behavior and its effect on callback evaluation is detailed at:
https://click.palletsprojects.com/en/stable/advanced/#callback-evaluation-order


