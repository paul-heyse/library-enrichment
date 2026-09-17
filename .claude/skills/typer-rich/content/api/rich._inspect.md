# `rich._inspect`

Distribution: `rich`

## Inspect

`rich._inspect.Inspect`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Inspect(JupyterMixin)
```

**Bases** `JupyterMixin`

**Declared members (10)**

- `docs = docs or help`  _instance-attribute_
- `dunder = dunder`  _instance-attribute_
- `help = help`  _instance-attribute_
- `highlighter = ReprHighlighter()`  _instance-attribute_
- `methods = methods`  _instance-attribute_
- `obj = obj`  _instance-attribute_
- `private = private or dunder`  _instance-attribute_
- `sort = sort`  _instance-attribute_
- `title = title or self._make_title(obj)`  _instance-attribute_
- `value = value`  _instance-attribute_

A renderable to inspect any Python Object.

Args:
    obj (Any): An object to inspect.
    title (str, optional): Title to display over inspect result, or None use type. Defaults to None.
    help (bool, optional): Show full help text rather than just first paragraph. Defaults to False.
    methods (bool, optional): Enable inspection of callables. Defaults to False.
    docs (bool, optional): Also render doc strings. Defaults to True.
    private (bool, optional): Show private attributes (beginning with underscore). Defaults to False.
    dunder (bool, optional): Show attributes starting with double underscore. Defaults to False.
    sort (bool, optional): Sort attributes alphabetically, callables at the top, leading and trailing underscores ignored. Defaults to True.
    all (bool, optional): Show all attributes. Defaults to False.
    value (bool, optional): Pretty print value of object. Defaults to True.


## _first_paragraph

`rich._inspect._first_paragraph`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _first_paragraph(doc: str) -> str
```

Get the first paragraph from a docstring.


## get_object_types_mro

`rich._inspect.get_object_types_mro`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_object_types_mro(obj: Union[object, Type[Any]]) -> Tuple[type, ...]
```

Returns the MRO of an object's class, or of the object itself if it's a class.


## get_object_types_mro_as_strings

`rich._inspect.get_object_types_mro_as_strings`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_object_types_mro_as_strings(obj: object) -> Collection[str]
```

Returns the MRO of an object's class as full qualified names, or of the object itself if it's a class.

Examples:
    `object_types_mro_as_strings(JSONDecoder)` will return `['json.decoder.JSONDecoder', 'builtins.object']`


## is_object_one_of_types

`rich._inspect.is_object_one_of_types`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def is_object_one_of_types(obj: object, fully_qualified_types_names: Collection[str]) -> bool
```

Returns `True` if the given object's class (or the object itself, if it's a class) has one of the
fully qualified names in its MRO.


