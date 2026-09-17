# `rich.json`

Distribution: `rich`

## args

`rich.json.args`

```python
args = parser.parse_args()
```

**Inferred type** (`ty`, not declared in the source): `Namespace`

## console

`rich.json.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## error_console

`rich.json.error_console`

```python
error_console = Console(stderr=True)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## json_data

`rich.json.json_data`

```python
json_data = sys.stdin.read()
```

**Inferred type** (`ty`, not declared in the source): `str | Any`

## parser

`rich.json.parser`

```python
parser = argparse.ArgumentParser(description='Pretty print json')
```

**Inferred type** (`ty`, not declared in the source): `ArgumentParser`

## JSON

`rich.json.JSON`

```python
class JSON
```

**Declared members (2)**

- `def from_data(cls, data: Any, indent: Union[None, int, str] = 2, highlight: bool = True, skip_keys: bool = False, ensure_ascii: bool = False, check_circular: bool = True, allow_nan: bool = True, default: Optional[Callable[[Any], Any]] = None, sort_keys: bool = False) -> JSON`  _classmethod_
  Encodes a JSON object from arbitrary data.
- `text = highlighter(json)`  _instance-attribute_

A renderable which pretty prints JSON.

Args:
    json (str): JSON encoded data.
    indent (Union[None, int, str], optional): Number of characters to indent by. Defaults to 2.
    highlight (bool, optional): Enable highlighting. Defaults to True.
    skip_keys (bool, optional): Skip keys not of a basic type. Defaults to False.
    ensure_ascii (bool, optional): Escape all non-ascii characters. Defaults to False.
    check_circular (bool, optional): Check for circular references. Defaults to True.
    allow_nan (bool, optional): Allow NaN and Infinity values. Defaults to True.
    default (Callable, optional): A callable that converts values that can not be encoded
        in to something that can be JSON encoded. Defaults to None.
    sort_keys (bool, optional): Sort dictionary keys. Defaults to False.


