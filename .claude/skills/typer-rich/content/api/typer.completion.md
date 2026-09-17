# `typer.completion`

Distribution: `typer`

## _click_patched

`typer.completion._click_patched`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_click_patched = False
```

## _install_completion_no_auto_placeholder_function

`typer.completion._install_completion_no_auto_placeholder_function`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _install_completion_no_auto_placeholder_function(install_completion: Shells = Option(None, callback=install_callback, expose_value=False, help='Install completion for the specified shell.'), show_completion: Shells = Option(None, callback=show_callback, expose_value=False, help='Show completion for the specified shell, to copy it or customize the installation.')) -> Any
```

## _install_completion_placeholder_function

`typer.completion._install_completion_placeholder_function`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _install_completion_placeholder_function(install_completion: bool = Option(None, '--install-completion', callback=install_callback, expose_value=False, help='Install completion for the current shell.'), show_completion: bool = Option(None, '--show-completion', callback=show_callback, expose_value=False, help='Show completion for the current shell, to copy it or customize the installation.')) -> Any
```

## get_completion_inspect_parameters

`typer.completion.get_completion_inspect_parameters`

```python
def get_completion_inspect_parameters() -> tuple[ParamMeta, ParamMeta]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## install_callback

`typer.completion.install_callback`

```python
def install_callback(ctx: _click.Context, param: _click.Parameter, value: Any) -> Any
```

## shell_complete

`typer.completion.shell_complete`

```python
def shell_complete(cli: _click.Command, ctx_args: MutableMapping[str, Any], prog_name: str, complete_var: str, instruction: str) -> int
```

## show_callback

`typer.completion.show_callback`

```python
def show_callback(ctx: _click.Context, param: _click.Parameter, value: Any) -> Any
```

