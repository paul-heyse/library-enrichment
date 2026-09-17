# `typer.models`

Distribution: `typer`

## AnyType

`typer.models.AnyType`

```python
AnyType = type[Any]
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'type[Any]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## CommandFunctionType

`typer.models.CommandFunctionType`

```python
CommandFunctionType = TypeVar('CommandFunctionType', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DefaultType

`typer.models.DefaultType`

```python
DefaultType = TypeVar('DefaultType')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## NoneType

`typer.models.NoneType`

```python
NoneType = type(None)
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'NoneType'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## Required

`typer.models.Required`

```python
Required = ...
```

**Inferred type** (`ty`, not declared in the source): `EllipsisType`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ArgumentInfo

`typer.models.ArgumentInfo`

```python
class ArgumentInfo(ParameterInfo)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ParameterInfo`

**Inherited (36)**

- from `typer.models.ParameterInfo`: `allow_dash`, `atomic`, `autocompletion`, `callback`, `case_sensitive`, `clamp`, `click_type`, `default`, `default_factory`, `dir_okay`, `encoding`, `envvar`, `errors`, `exists`, `expose_value`, `file_okay`, `formats`, `help`, `hidden`, `is_eager`, `lazy`, `max`, `metavar`, `min`, `mode`, `param_decls`, `parser`, `path_type`, `readable`, `resolve_path`, `rich_help_panel`, `shell_complete`, `show_choices`, `show_default`, `show_envvar`, `writable`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## CallbackParam

Import as `typer.CallbackParam`  ·  defined at `typer.models.CallbackParam`

```python
class CallbackParam(_click.Parameter)
```

**Also exported as** `typer.CallbackParam`

**Bases** `_click.Parameter`

**Inherited (29)**

- from `typer._click.core.Parameter`: `add_to_parser`, `callback`, `consume_value`, `default`, `envvar`, `expose_value`, `get_default`, `get_error_hint`, `get_help_record`, `get_usage_pieces`, `handle_parse_result`, `human_readable_name`, `is_eager`, `make_metavar`, `metavar`, `multiple`, `name`, `nargs`, `opts`, `param_type_name`, `process_value`, `required`, `resolve_envvar_value`, `secondary_opts`, `shell_complete`, `type`, `type_cast_value`, `value_from_envvar`, `value_is_missing`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

In a callback function, you can declare a function parameter with type `CallbackParam`
to access the specific Click [`Parameter`](https://click.palletsprojects.com/en/stable/api/#click.Parameter) object.


## CommandInfo

`typer.models.CommandInfo`

```python
class CommandInfo
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (13)**

- `add_help_option = add_help_option`  _instance-attribute_
- `callback = callback`  _instance-attribute_
- `cls = cls`  _instance-attribute_
- `context_settings = context_settings`  _instance-attribute_
- `deprecated = deprecated`  _instance-attribute_
- `epilog = epilog`  _instance-attribute_
- `help = help`  _instance-attribute_
- `hidden = hidden`  _instance-attribute_
- `name = name`  _instance-attribute_
- `no_args_is_help = no_args_is_help`  _instance-attribute_
- `options_metavar = options_metavar`  _instance-attribute_
- `rich_help_panel = rich_help_panel`  _instance-attribute_
- `short_help = short_help`  _instance-attribute_

## Context

Import as `typer.Context`  ·  defined at `typer.models.Context`

```python
class Context(_click.Context)
```

**Also exported as** `typer.Context`

**Bases** `_click.Context`

**Inherited (39)**

- from `typer._click.core.Context`: `abort`, `allow_extra_args`, `allow_interspersed_args`, `args`, `auto_envvar_prefix`, `call_on_close`, `close`, `color`, `command`, `command_path`, `default_map`, `ensure_object`, `exit`, `fail`, `find_object`, `find_root`, `formatter_class`, `get_help`, `get_parameter_source`, `get_usage`, `help_option_names`, `ignore_unknown_options`, `info_name`, `invoke`, `invoked_subcommand`, `lookup_default`, `make_formatter`, `max_content_width`, `meta`, `obj`, `params`, `parent`, `resilient_parsing`, `scope`, `set_parameter_source`, `show_default`, `terminal_width`, `token_normalize_func`, `with_resource`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The [`Context`](https://click.palletsprojects.com/en/stable/api/#click.Context) has some additional data about the current execution of your program.
When declaring it in a [callback](https://typer.tiangolo.com/tutorial/options/callback-and-context/) function,
you can access this additional information.


## DefaultPlaceholder

`typer.models.DefaultPlaceholder`

```python
class DefaultPlaceholder
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `value = value`  _instance-attribute_

You shouldn't use this class directly.

It's used internally to recognize when a default value has been overwritten, even
if the new value is `None`.


## DeveloperExceptionConfig

`typer.models.DeveloperExceptionConfig`

```python
class DeveloperExceptionConfig
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `pretty_exceptions_enable = pretty_exceptions_enable`  _instance-attribute_
- `pretty_exceptions_short = pretty_exceptions_short`  _instance-attribute_
- `pretty_exceptions_show_locals = pretty_exceptions_show_locals`  _instance-attribute_

## FileBinaryRead

Import as `typer.FileBinaryRead`  ·  defined at `typer.models.FileBinaryRead`

```python
class FileBinaryRead(io.BufferedReader)
```

**Also exported as** `typer.FileBinaryRead`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `io.BufferedReader`

You can use this class to read binary data, receiving `bytes`.
The default mode of this class is `mode="rb"`.
It is useful for reading binary files like images:

**Example**

```python
from typing import Annotated

import typer

app = typer.Typer()

@app.command()
def main(file: Annotated[typer.FileBinaryRead, typer.Option()]):
    processed_total = 0
    for bytes_chunk in file:
        # Process the bytes in bytes_chunk
        processed_total += len(bytes_chunk)
        print(f"Processed bytes total: {processed_total}")

if __name__ == "__main__":
    app()
```


## FileBinaryWrite

Import as `typer.FileBinaryWrite`  ·  defined at `typer.models.FileBinaryWrite`

```python
class FileBinaryWrite(io.BufferedWriter)
```

**Also exported as** `typer.FileBinaryWrite`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `io.BufferedWriter`

You can use this class to write binary data: you pass `bytes` to it instead of strings.
The default mode of this class is `mode="wb"`.
It is useful for writing binary files like images:

**Example**

```python
from typing import Annotated

import typer

app = typer.Typer()

@app.command()
def main(file: Annotated[typer.FileBinaryWrite, typer.Option()]):
    first_line_str = "some settings\n"
    # You cannot write str directly to a binary file; encode it first
    first_line_bytes = first_line_str.encode("utf-8")
    # Then you can write the bytes
    file.write(first_line_bytes)
    # This is already bytes, it starts with b"
    second_line = b"la cigÃ¼eÃ±a trae al niÃ±o"
    file.write(second_line)
    print("Binary file written")

if __name__ == "__main__":
    app()
```


## FileText

Import as `typer.FileText`  ·  defined at `typer.models.FileText`

```python
class FileText(io.TextIOWrapper)
```

**Also exported as** `typer.FileText`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `io.TextIOWrapper`

Gives you a file-like object for reading text, and you will get a `str` data from it.
The default mode of this class is `mode="r"`.

**Example**

```python
from typing import Annotated

import typer

app = typer.Typer()

@app.command()
def main(config: Annotated[typer.FileText, typer.Option()]):
    for line in config:
        print(f"Config line: {line}")

if __name__ == "__main__":
    app()
```


## FileTextWrite

Import as `typer.FileTextWrite`  ·  defined at `typer.models.FileTextWrite`

```python
class FileTextWrite(FileText)
```

**Also exported as** `typer.FileTextWrite`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FileText`

You can use this class for writing text. Alternatively, you can use `FileText` with `mode="w"`.
The default mode of this class is `mode="w"`.

**Example**

```python
from typing import Annotated

import typer

app = typer.Typer()

@app.command()
def main(config: Annotated[typer.FileTextWrite, typer.Option()]):
    config.write("Some config written by the app")
    print("Config written")

if __name__ == "__main__":
    app()
```


## OptionInfo

`typer.models.OptionInfo`

```python
class OptionInfo(ParameterInfo)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ParameterInfo`

**Declared members (6)**

- `allow_from_autoenv = allow_from_autoenv`  _instance-attribute_
- `confirmation_prompt = confirmation_prompt`  _instance-attribute_
- `count = count`  _instance-attribute_
- `hide_input = hide_input`  _instance-attribute_
- `prompt = prompt`  _instance-attribute_
- `prompt_required = prompt_required`  _instance-attribute_

**Inherited (36)**

- from `typer.models.ParameterInfo`: `allow_dash`, `atomic`, `autocompletion`, `callback`, `case_sensitive`, `clamp`, `click_type`, `default`, `default_factory`, `dir_okay`, `encoding`, `envvar`, `errors`, `exists`, `expose_value`, `file_okay`, `formats`, `help`, `hidden`, `is_eager`, `lazy`, `max`, `metavar`, `min`, `mode`, `param_decls`, `parser`, `path_type`, `readable`, `resolve_path`, `rich_help_panel`, `shell_complete`, `show_choices`, `show_default`, `show_envvar`, `writable`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## ParamMeta

`typer.models.ParamMeta`

```python
class ParamMeta
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `annotation = annotation`  _instance-attribute_
- `default = default`  _instance-attribute_
- `empty = inspect.Parameter.empty`  _class-attribute, instance-attribute_
- `name = name`  _instance-attribute_

## ParameterInfo

`typer.models.ParameterInfo`

```python
class ParameterInfo
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (36)**

- `allow_dash = allow_dash`  _instance-attribute_
- `atomic = atomic`  _instance-attribute_
- `autocompletion = autocompletion`  _instance-attribute_
- `callback = callback`  _instance-attribute_
- `case_sensitive = case_sensitive`  _instance-attribute_
- `clamp = clamp`  _instance-attribute_
- `click_type = click_type`  _instance-attribute_
- `default = default`  _instance-attribute_
- `default_factory = default_factory`  _instance-attribute_
- `dir_okay = dir_okay`  _instance-attribute_
- `encoding = encoding`  _instance-attribute_
- `envvar = envvar`  _instance-attribute_
- `errors = errors`  _instance-attribute_
- `exists = exists`  _instance-attribute_
- `expose_value = expose_value`  _instance-attribute_
- `file_okay = file_okay`  _instance-attribute_
- `formats = formats`  _instance-attribute_
- `help = help`  _instance-attribute_
- `hidden = hidden`  _instance-attribute_
- `is_eager = is_eager`  _instance-attribute_
- `lazy = lazy`  _instance-attribute_
- `max = max`  _instance-attribute_
- `metavar = metavar`  _instance-attribute_
- `min = min`  _instance-attribute_
- `mode = mode`  _instance-attribute_
- `param_decls = param_decls`  _instance-attribute_
- `parser = parser`  _instance-attribute_
- `path_type = path_type`  _instance-attribute_
- `readable = readable`  _instance-attribute_
- `resolve_path = resolve_path`  _instance-attribute_
- `rich_help_panel = rich_help_panel`  _instance-attribute_
- `shell_complete = shell_complete`  _instance-attribute_
- `show_choices = show_choices`  _instance-attribute_
- `show_default = show_default`  _instance-attribute_
- `show_envvar = show_envvar`  _instance-attribute_
- `writable = writable`  _instance-attribute_

## TyperInfo

`typer.models.TyperInfo`

```python
class TyperInfo
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (18)**

- `add_help_option = add_help_option`  _instance-attribute_
- `callback = callback`  _instance-attribute_
- `chain = chain`  _instance-attribute_
- `cls = cls`  _instance-attribute_
- `context_settings = context_settings`  _instance-attribute_
- `deprecated = deprecated`  _instance-attribute_
- `epilog = epilog`  _instance-attribute_
- `help = help`  _instance-attribute_
- `hidden = hidden`  _instance-attribute_
- `invoke_without_command = invoke_without_command`  _instance-attribute_
- `name = name`  _instance-attribute_
- `no_args_is_help = no_args_is_help`  _instance-attribute_
- `options_metavar = options_metavar`  _instance-attribute_
- `result_callback = result_callback`  _instance-attribute_
- `rich_help_panel = rich_help_panel`  _instance-attribute_
- `short_help = short_help`  _instance-attribute_
- `subcommand_metavar = subcommand_metavar`  _instance-attribute_
- `typer_instance = typer_instance`  _instance-attribute_

## TyperPath

`typer.models.TyperPath`

```python
class TyperPath(types.ParamType)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `types.ParamType`

**Declared members (13)**

- `allow_dash = allow_dash`  _instance-attribute_
- `def coerce_path_result(self, value: str | os.PathLike[str]) -> str | bytes | os.PathLike[str]`
- `def convert(self, value: str | os.PathLike[str], param: _click.Parameter | None, ctx: Context | None) -> str | bytes | os.PathLike[str]`
- `dir_okay = dir_okay`  _instance-attribute_
- `envvar_list_splitter: str = os.path.pathsep`  _class-attribute_
- `exists = exists`  _instance-attribute_
- `file_okay = file_okay`  _instance-attribute_
- `name = 'file'`  _instance-attribute_
- `readable = readable`  _instance-attribute_
- `resolve_path = resolve_path`  _instance-attribute_
- `def shell_complete(self, ctx: _click.Context, param: _click.Parameter, incomplete: str) -> list[CompletionItem]`
  Return an empty list so that the autocompletion functionality will work properly from the commandline.
- `type = path_type`  _instance-attribute_
- `writable = writable`  _instance-attribute_

**Inherited (6)**

- from `typer._click.types.ParamType`: `arity`, `fail`, `get_metavar`, `get_missing_message`, `is_composite`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## Default

`typer.models.Default`

```python
def Default(value: DefaultType) -> DefaultType
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

You shouldn't use this function directly.

It's used internally to recognize when a default value has been overwritten, even
if the new value is `None`.


