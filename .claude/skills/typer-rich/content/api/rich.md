# `rich`

Distribution: `rich`

## _IMPORT_CWD

`rich._IMPORT_CWD`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_IMPORT_CWD = os.path.abspath(os.getcwd())
```

## __all__

`rich.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['get_console', 'reconfigure', 'print', 'inspect', 'print_json']
```

## _console

`rich._console`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_console: Optional[Console] = None
```

## get_console

`rich.get_console`

```python
def get_console() -> Console
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get a global :class:`~rich.console.Console` instance. This function is used when Rich requires a Console,
and hasn't been explicitly given one.

Returns:
    Console: A console instance.


## inspect

`rich.inspect`

```python
def inspect(obj: Any, console: Optional[Console] = None, title: Optional[str] = None, help: bool = False, methods: bool = False, docs: bool = True, private: bool = False, dunder: bool = False, sort: bool = True, all: bool = False, value: bool = True) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Inspect any Python object.

* inspect(<OBJECT>) to see summarized info.
* inspect(<OBJECT>, methods=True) to see methods.
* inspect(<OBJECT>, help=True) to see full (non-abbreviated) help.
* inspect(<OBJECT>, private=True) to see private attributes (single underscore).
* inspect(<OBJECT>, dunder=True) to see attributes beginning with double underscore.
* inspect(<OBJECT>, all=True) to see all attributes.

Args:
    obj (Any): An object to inspect.
    title (str, optional): Title to display over inspect result, or None use type. Defaults to None.
    help (bool, optional): Show full help text rather than just first paragraph. Defaults to False.
    methods (bool, optional): Enable inspection of callables. Defaults to False.
    docs (bool, optional): Also render doc strings. Defaults to True.
    private (bool, optional): Show private attributes (beginning with underscore). Defaults to False.
    dunder (bool, optional): Show attributes starting with double underscore. Defaults to False.
    sort (bool, optional):  Sort attributes alphabetically, callables at the top, leading and trailing underscores ignored. Defaults to True.
    all (bool, optional): Show all attributes. Defaults to False.
    value (bool, optional): Pretty print value. Defaults to True.


## print

`rich.print`

```python
def print(objects: Any = (), sep: str = ' ', end: str = '\n', file: Optional[IO[str]] = None, flush: bool = False) -> None
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Print object(s) supplied via positional arguments.
This function has an identical signature to the built-in print.
For more advanced features, see the :class:`~rich.console.Console` class.

Args:
    sep (str, optional): Separator between printed objects. Defaults to " ".
    end (str, optional): Character to write at end of output. Defaults to "\\n".
    file (IO[str], optional): File to write to, or None for stdout. Defaults to None.
    flush (bool, optional): Has no effect as Rich always flushes output. Defaults to False.


## print_json

`rich.print_json`

```python
def print_json(json: Optional[str] = None, data: Any = None, indent: Union[None, int, str] = 2, highlight: bool = True, skip_keys: bool = False, ensure_ascii: bool = False, check_circular: bool = True, allow_nan: bool = True, default: Optional[Callable[[Any], Any]] = None, sort_keys: bool = False) -> None
```

Pretty prints JSON. Output will be valid JSON.

Args:
    json (str): A string containing JSON.
    data (Any): If json is not supplied, then encode this data.
    indent (int, optional): Number of spaces to indent. Defaults to 2.
    highlight (bool, optional): Enable highlighting of output: Defaults to True.
    skip_keys (bool, optional): Skip keys not of a basic type. Defaults to False.
    ensure_ascii (bool, optional): Escape all non-ascii characters. Defaults to False.
    check_circular (bool, optional): Check for circular references. Defaults to True.
    allow_nan (bool, optional): Allow NaN and Infinity values. Defaults to True.
    default (Callable, optional): A callable that converts values that can not be encoded
        in to something that can be JSON encoded. Defaults to None.
    sort_keys (bool, optional): Sort dictionary keys. Defaults to False.


## reconfigure

`rich.reconfigure`

```python
def reconfigure(args: Any = (), kwargs: Any = {}) -> None
```

Reconfigures the global console by replacing it with another.

Args:
    *args (Any): Positional arguments for the replacement :class:`~rich.console.Console`.
    **kwargs (Any): Keyword arguments for the replacement :class:`~rich.console.Console`.


