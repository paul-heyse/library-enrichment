# `typer.main`

Distribution: `typer`

## _original_except_hook

`typer.main._original_except_hook`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_original_except_hook = sys.excepthook
```

## _typer_developer_exception_attr_name

`typer.main._typer_developer_exception_attr_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_typer_developer_exception_attr_name = '__typer_developer_exception__'
```

## Typer

Import as `typer.Typer`  ·  defined at `typer.main.Typer`

```python
class Typer
```

**Also exported as** `typer.Typer`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (13)**

- `def add_typer(self, typer_instance: Typer, name: Annotated[str | None, Doc("\n                The name of this subcommand.\n                See [the tutorial about name and help](https://typer.tiangolo.com/tutorial/subcommands/name-and-help) for different ways of setting a command's name,\n                and which one takes priority.\n                ")] = Default(None), cls: Annotated[type[TyperGroup] | None, Doc('\n                The class of this subcommand. Mainly used when [using the Click library underneath](https://typer.tiangolo.com/tutorial/using-click/). Can usually be left at the default value `None`.\n                Otherwise, should be a subtype of `TyperGroup`.\n                ')] = Default(None), invoke_without_command: Annotated[bool, Doc('\n                By setting this to `True`, you can make sure a callback is executed even when no subcommand is provided.\n                ')] = Default(False), no_args_is_help: Annotated[bool, Doc('\n                If this is set to `True`, running a command without any arguments will automatically show the help page.\n                ')] = Default(False), subcommand_metavar: Annotated[str | None, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                How to represent the subcommand argument in help.\n                ")] = Default(None), chain: Annotated[bool, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                Allow passing more than one subcommand argument.\n                ")] = Default(False), result_callback: Annotated[Callable[..., Any] | None, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                A function to call after the group's and subcommand's callbacks.\n                ")] = Default(None), context_settings: Annotated[dict[Any, Any] | None, Doc("\n                Pass configurations for the [context](https://typer.tiangolo.com/tutorial/commands/context/).\n                Available configurations can be found in the docs for Click's `Context` [here](https://click.palletsprojects.com/en/stable/api/#context).\n                ")] = Default(None), callback: Annotated[Callable[..., Any] | None, Doc('\n                Add a callback to this app.\n                See [the tutorial about callbacks](https://typer.tiangolo.com/tutorial/commands/callback/) for more details.\n                ')] = Default(None), help: Annotated[str | None, Doc("\n                Help text for the subcommand.\n                See [the tutorial about name and help](https://typer.tiangolo.com/tutorial/subcommands/name-and-help) for different ways of setting a command's help,\n                and which one takes priority.\n                ")] = Default(None), epilog: Annotated[str | None, Doc('\n                Text that will be printed right after the help text.\n                ')] = Default(None), short_help: Annotated[str | None, Doc('\n                A shortened version of the help text that can be used e.g. in the help table listing subcommands.\n                When not defined, the normal `help` text will be used instead.\n                ')] = Default(None), options_metavar: Annotated[str | None, Doc('\n                In the example usage string of the help text for a command, the default placeholder for various arguments is `[OPTIONS]`.\n                Set `options_metavar` to change this into a different string. When `None`, the default value will be used.\n                ')] = Default(None), add_help_option: Annotated[bool, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                By default each command registers a `--help` option. This can be disabled by this parameter.\n                ")] = Default(True), hidden: Annotated[bool, Doc('\n                Hide this command from help outputs. `False` by default.\n                ')] = Default(False), deprecated: Annotated[bool, Doc('\n                Mark this command as deprecated in the help outputs. `False` by default.\n                ')] = False, rich_help_panel: Annotated[str | None, Doc('\n                Set the panel name of the command when the help is printed with Rich.\n                ')] = Default(None)) -> None`
  Add subcommands to the main app using `app.add_typer()`. Subcommands may be defined in separate modules, ensuring clean separation of code by functionality.
- `def callback(self, cls: Annotated[type[TyperGroup] | None, Doc('\n                The class of this app. Mainly used when [using the Click library underneath](https://typer.tiangolo.com/tutorial/using-click/). Can usually be left at the default value `None`.\n                Otherwise, should be a subtype of `TyperGroup`.\n                ')] = Default(None), invoke_without_command: Annotated[bool, Doc('\n                By setting this to `True`, you can make sure a callback is executed even when no subcommand is provided.\n                ')] = Default(False), no_args_is_help: Annotated[bool, Doc('\n                If this is set to `True`, running a command without any arguments will automatically show the help page.\n                ')] = Default(False), subcommand_metavar: Annotated[str | None, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                How to represent the subcommand argument in help.\n                ")] = Default(None), chain: Annotated[bool, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                Allow passing more than one subcommand argument.\n                ")] = Default(False), result_callback: Annotated[Callable[..., Any] | None, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                A function to call after the group's and subcommand's callbacks.\n                ")] = Default(None), context_settings: Annotated[dict[Any, Any] | None, Doc("\n                Pass configurations for the [context](https://typer.tiangolo.com/tutorial/commands/context/).\n                Available configurations can be found in the docs for Click's `Context` [here](https://click.palletsprojects.com/en/stable/api/#context).\n                ")] = Default(None), help: Annotated[str | None, Doc("\n                Help text for the command.\n                See [the tutorial about name and help](https://typer.tiangolo.com/tutorial/subcommands/name-and-help) for different ways of setting a command's help,\n                and which one takes priority.\n                ")] = Default(None), epilog: Annotated[str | None, Doc('\n                Text that will be printed right after the help text.\n                ')] = Default(None), short_help: Annotated[str | None, Doc('\n                A shortened version of the help text that can be used e.g. in the help table listing subcommands.\n                When not defined, the normal `help` text will be used instead.\n                ')] = Default(None), options_metavar: Annotated[str | None, Doc('\n                In the example usage string of the help text for a command, the default placeholder for various arguments is `[OPTIONS]`.\n                Set `options_metavar` to change this into a different string. When `None`, the default value will be used.\n                ')] = Default(None), add_help_option: Annotated[bool, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                By default each command registers a `--help` option. This can be disabled by this parameter.\n                ")] = Default(True), hidden: Annotated[bool, Doc('\n                Hide this command from help outputs. `False` by default.\n                ')] = Default(False), deprecated: Annotated[bool, Doc('\n                Mark this command as deprecated in the help text. `False` by default.\n                ')] = Default(False), rich_help_panel: Annotated[str | None, Doc('\n                Set the panel name of the command when the help is printed with Rich.\n                ')] = Default(None)) -> Callable[[CommandFunctionType], CommandFunctionType]`
  Using the decorator `@app.callback`, you can declare the CLI parameters for the main CLI application.
- `def command(self, name: Annotated[str | None, Doc('\n                The name of this command.\n                ')] = None, cls: Annotated[type[TyperCommand] | None, Doc('\n                The class of this command. Mainly used when [using the Click library underneath](https://typer.tiangolo.com/tutorial/using-click/). Can usually be left at the default value `None`.\n                Otherwise, should be a subtype of `TyperCommand`.\n                ')] = None, context_settings: Annotated[dict[Any, Any] | None, Doc("\n                Pass configurations for the [context](https://typer.tiangolo.com/tutorial/commands/context/).\n                Available configurations can be found in the docs for Click's `Context` [here](https://click.palletsprojects.com/en/stable/api/#context).\n                ")] = None, help: Annotated[str | None, Doc("\n                Help text for the command.\n                See [the tutorial about name and help](https://typer.tiangolo.com/tutorial/subcommands/name-and-help) for different ways of setting a command's help,\n                and which one takes priority.\n                ")] = None, epilog: Annotated[str | None, Doc('\n                Text that will be printed right after the help text.\n                ')] = None, short_help: Annotated[str | None, Doc('\n                A shortened version of the help text that can be used e.g. in the help table listing subcommands.\n                When not defined, the normal `help` text will be used instead.\n                ')] = None, options_metavar: Annotated[str | None, Doc('\n                In the example usage string of the help text for a command, the default placeholder for various arguments is `[OPTIONS]`.\n                Set `options_metavar` to change this into a different string. When `None`, the default value will be used.\n                ')] = Default(None), add_help_option: Annotated[bool, Doc("\n                **Note**: you probably shouldn't use this parameter, it is inherited\n                from Click and supported for compatibility.\n\n                ---\n\n                By default each command registers a `--help` option. This can be disabled by this parameter.\n                ")] = True, no_args_is_help: Annotated[bool, Doc('\n                If this is set to `True`, running a command without any arguments will automatically show the help page.\n                ')] = False, hidden: Annotated[bool, Doc('\n                Hide this command from help outputs. `False` by default.\n                ')] = False, deprecated: Annotated[bool, Doc('\n                Mark this command as deprecated in the help outputs. `False` by default.\n                ')] = False, rich_help_panel: Annotated[str | None, Doc('\n                Set the panel name of the command when the help is printed with Rich.\n                ')] = Default(None)) -> Callable[[CommandFunctionType], CommandFunctionType]`
  Using the decorator `@app.command`, you can define a subcommand of the previously defined Typer app.
- `info = TyperInfo(name=name, cls=cls, invoke_without_command=invoke_without_command, no_args_is_help=no_args_is_help, subcommand_metavar=subcommand_metavar, chain=chain, result_callback=result_callback, context_settings=context_settings, callback=callback, help=help, epilog=epilog, short_help=short_help, options_metavar=options_metavar, add_help_option=add_help_option, hidden=hidden, deprecated=deprecated)`  _instance-attribute_
- `pretty_exceptions_enable = pretty_exceptions_enable`  _instance-attribute_
- `pretty_exceptions_short = pretty_exceptions_short`  _instance-attribute_
- `pretty_exceptions_show_locals = pretty_exceptions_show_locals`  _instance-attribute_
- `registered_callback: TyperInfo | None = None`  _instance-attribute_
- `registered_commands: list[CommandInfo] = []`  _instance-attribute_
- `registered_groups: list[TyperInfo] = []`  _instance-attribute_
- `rich_help_panel = rich_help_panel`  _instance-attribute_
- `rich_markup_mode: MarkupMode = rich_markup_mode`  _instance-attribute_
- `suggest_commands = suggest_commands`  _instance-attribute_

`Typer` main class, the main entrypoint to use Typer.

Read more in the
[Typer docs for First Steps](https://typer.tiangolo.com/tutorial/typer-app/).

## Example

```python
import typer

app = typer.Typer()
```


## _is_linux_or_bsd

`typer.main._is_linux_or_bsd`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_linux_or_bsd() -> bool
```

## _is_macos

`typer.main._is_macos`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_macos() -> bool
```

## determine_type_convertor

`typer.main.determine_type_convertor`

```python
def determine_type_convertor(type_: Any) -> Callable[[Any], Any] | None
```

## except_hook

`typer.main.except_hook`

```python
def except_hook(exc_type: type[BaseException], exc_value: BaseException, tb: TracebackType | None) -> None
```

## generate_enum_convertor

`typer.main.generate_enum_convertor`

```python
def generate_enum_convertor(enum: type[Enum]) -> Callable[[Any], Any]
```

## generate_list_convertor

`typer.main.generate_list_convertor`

```python
def generate_list_convertor(convertor: Callable[[Any], Any] | None, default_value: Any | None) -> Callable[[Sequence[Any] | None], list[Any] | None]
```

## generate_tuple_convertor

`typer.main.generate_tuple_convertor`

```python
def generate_tuple_convertor(types: Sequence[Any]) -> Callable[[tuple[Any, ...] | None], tuple[Any, ...] | None]
```

## get_callback

`typer.main.get_callback`

```python
def get_callback(callback: Callable[..., Any] | None = None, params: Sequence[_click.Parameter] = [], convertors: dict[str, Callable[[str], Any]] | None = None, context_param_name: str | None = None, pretty_exceptions_short: bool) -> Callable[..., Any] | None
```

## get_click_param

`typer.main.get_click_param`

```python
def get_click_param(param: ParamMeta) -> tuple[TyperArgument | TyperOption, Any]
```

## get_click_type

`typer.main.get_click_type`

```python
def get_click_type(annotation: Any, parameter_info: ParameterInfo) -> types.ParamType
```

## get_command

`typer.main.get_command`

```python
def get_command(typer_instance: Typer) -> _click.Command
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## get_command_from_info

`typer.main.get_command_from_info`

```python
def get_command_from_info(command_info: CommandInfo, pretty_exceptions_short: bool, rich_markup_mode: MarkupMode) -> _click.Command
```

## get_command_name

`typer.main.get_command_name`

```python
def get_command_name(name: str) -> str
```

## get_default_option_flag_name

`typer.main.get_default_option_flag_name`

```python
def get_default_option_flag_name(name: str, metavar: str | None) -> str
```

## get_group

`typer.main.get_group`

```python
def get_group(typer_instance: Typer) -> TyperGroup
```

## get_group_from_info

`typer.main.get_group_from_info`

```python
def get_group_from_info(group_info: TyperInfo, pretty_exceptions_short: bool, suggest_commands: bool, rich_markup_mode: MarkupMode) -> TyperGroup
```

## get_install_completion_arguments

`typer.main.get_install_completion_arguments`

```python
def get_install_completion_arguments() -> tuple[_click.Parameter, _click.Parameter]
```

## get_param_callback

`typer.main.get_param_callback`

```python
def get_param_callback(callback: Callable[..., Any] | None = None, convertor: Callable[..., Any] | None = None) -> Callable[..., Any] | None
```

## get_param_completion

`typer.main.get_param_completion`

```python
def get_param_completion(callback: Callable[..., Any] | None = None) -> Callable[..., Any] | None
```

## get_params_convertors_ctx_param_name_from_function

`typer.main.get_params_convertors_ctx_param_name_from_function`

```python
def get_params_convertors_ctx_param_name_from_function(callback: Callable[..., Any] | None) -> tuple[list[TyperArgument | TyperOption], dict[str, Any], str | None]
```

## launch

Import as `typer.launch`  ·  defined at `typer.main.launch`

```python
def launch(url: Annotated[str, Doc('\n            URL or filename of the thing to launch.\n            ')], wait: Annotated[bool, Doc('\n            Wait for the program to exit before returning. This only works if the launched program blocks.\n            ')] = False, locate: Annotated[bool, Doc('\n            If this is set to `True`, then instead of launching the application associated with the URL, it will attempt to\n            launch a file manager with the file located. This might have weird effects if the URL does not point to the filesystem.\n            ')] = False) -> int
```

**Also exported as** `typer.launch`

This function launches the given URL (or filename) in the default
viewer application for this file type.  If this is an executable, it
might launch the executable in a new session.  The return value is
the exit code of the launched application.  Usually, `0` indicates
success.

This function handles url in different operating systems separately:
 - On macOS (Darwin), it uses the `open` command.
 - On Linux and BSD, it uses `xdg-open` if available.
 - On Windows (and other OSes), it uses the standard webbrowser module.

The function avoids, when possible, using the webbrowser module on Linux and macOS
to prevent spammy terminal messages from some browsers (e.g., Chrome).

## Examples
```python
    import typer

    typer.launch("https://typer.tiangolo.com/")
```

```python
    import typer

    typer.launch("/my/downloaded/file", locate=True)
```


## lenient_issubclass

`typer.main.lenient_issubclass`

```python
def lenient_issubclass(cls: Any, class_or_tuple: AnyType | tuple[AnyType, ...]) -> bool
```

## param_path_convertor

`typer.main.param_path_convertor`

```python
def param_path_convertor(value: str | None = None) -> Path | None
```

## run

Import as `typer.run`  ·  defined at `typer.main.run`

```python
def run(function: Annotated[Callable[..., Any], Doc('\n            The function that should power this CLI application.\n            ')]) -> None
```

**Also exported as** `typer.run`

This function converts a given function to a CLI application with `Typer()` and executes it.

## Example

```python
import typer

def main(name: str):
    print(f"Hello {name}")

if __name__ == "__main__":
    typer.run(main)
```


## solve_typer_info_defaults

`typer.main.solve_typer_info_defaults`

```python
def solve_typer_info_defaults(typer_info: TyperInfo) -> TyperInfo
```

## solve_typer_info_help

`typer.main.solve_typer_info_help`

```python
def solve_typer_info_help(typer_info: TyperInfo) -> str
```

