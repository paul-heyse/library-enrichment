# `rich.pretty`

Distribution: `rich`

## _BRACES

`rich.pretty._BRACES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_BRACES: Dict[type, Callable[[Any], Tuple[str, str, str]]] = {os._Environ: lambda _object: ('environ({', '})', 'environ({})'), array: _get_braces_for_array, defaultdict: _get_braces_for_defaultdict, Counter: lambda _object: ('Counter({', '})', 'Counter()'), deque: _get_braces_for_deque, dict: lambda _object: ('{', '}', '{}'), UserDict: lambda _object: ('{', '}', '{}'), frozenset: lambda _object: ('frozenset({', '})', 'frozenset()'), list: lambda _object: ('[', ']', '[]'), UserList: lambda _object: ('[', ']', '[]'), set: lambda _object: ('{', '}', 'set()'), tuple: lambda _object: ('(', ')', '()'), MappingProxyType: lambda _object: ('mappingproxy({', '})', 'mappingproxy({})')}
```

## _CONTAINERS

`rich.pretty._CONTAINERS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CONTAINERS = tuple(_BRACES.keys())
```

## _MAPPING_CONTAINERS

`rich.pretty._MAPPING_CONTAINERS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MAPPING_CONTAINERS = (dict, os._Environ, MappingProxyType, UserDict)
```

## _dummy_namedtuple

`rich.pretty._dummy_namedtuple`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_dummy_namedtuple = collections.namedtuple('_dummy_namedtuple', [])
```

## _has_attrs

`rich.pretty._has_attrs`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_has_attrs = hasattr(_attr_module, 'ib')
```

## d

`rich.pretty.d`

```python
d = defaultdict(int)
```

**Inferred type** (`ty`, not declared in the source): `defaultdict[Unknown, int]`

## data

`rich.pretty.data`

```python
data = {'foo': [1, 'Hello World!', 100.123, 323.232, 432324.0, {5, 6, 7, (1, 2, 3, 4), 8}], 'bar': frozenset({1, 2, 3}), 'defaultdict': defaultdict(list, {'crumble': ['apple', 'rhubarb', 'butter', 'sugar', 'flour']}), 'counter': Counter(['apple', 'orange', 'pear', 'kumquat', 'kumquat', 'durian' * 100]), 'atomic': (False, True, None), 'namedtuple': StockKeepingUnit('Sparkling British Spring Water', 'Carbonated spring water', 0.9, 'water', ['its amazing!', 'its terrible!']), 'Broken': BrokenRepr()}
```

**Inferred type** (`ty`, not declared in the source): `dict[str, list[float | str | set[int | tuple[int, int, int, int]]] | frozenset[int] | defaultdict[str, list[str]] | ... omitted 4 union elements]`

## BrokenRepr

`rich.pretty.BrokenRepr`

```python
class BrokenRepr
```

## Node

`rich.pretty.Node`

```python
class Node
```

**Declared members (14)**

- `def check_length(self, start_length: int, max_length: int) -> bool`
  Check the length fits within a limit.
- `children: Optional[List[Node]] = None`  _class-attribute, instance-attribute_
- `close_brace: str = ''`  _class-attribute, instance-attribute_
- `empty: str = ''`  _class-attribute, instance-attribute_
- `is_namedtuple: bool = False`  _class-attribute, instance-attribute_
- `is_tuple: bool = False`  _class-attribute, instance-attribute_
- `def iter_tokens(self) -> Iterable[str]`
  Generate tokens for this node.
- `key_repr: str = ''`  _class-attribute, instance-attribute_
- `key_separator: str = ': '`  _class-attribute, instance-attribute_
- `last: bool = False`  _class-attribute, instance-attribute_
- `open_brace: str = ''`  _class-attribute, instance-attribute_
- `def render(self, max_width: int = 80, indent_size: int = 4, expand_all: bool = False) -> str`
  Render the node to a pretty repr.
- `separator: str = ', '`  _class-attribute, instance-attribute_
- `value_repr: str = ''`  _class-attribute, instance-attribute_

A node in a repr tree. May be atomic or a container.


## Pretty

`rich.pretty.Pretty`

```python
class Pretty(JupyterMixin)
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (12)**

- `expand_all = expand_all`  _instance-attribute_
- `highlighter = highlighter or ReprHighlighter()`  _instance-attribute_
- `indent_guides = indent_guides`  _instance-attribute_
- `indent_size = indent_size`  _instance-attribute_
- `insert_line = insert_line`  _instance-attribute_
- `justify: Optional[JustifyMethod] = justify`  _instance-attribute_
- `margin = margin`  _instance-attribute_
- `max_depth = max_depth`  _instance-attribute_
- `max_length = max_length`  _instance-attribute_
- `max_string = max_string`  _instance-attribute_
- `no_wrap = no_wrap`  _instance-attribute_
- `overflow: Optional[OverflowMethod] = overflow`  _instance-attribute_

A rich renderable that pretty prints an object.

Args:
    _object (Any): An object to pretty print.
    highlighter (HighlighterType, optional): Highlighter object to apply to result, or None for ReprHighlighter. Defaults to None.
    indent_size (int, optional): Number of spaces in indent. Defaults to 4.
    justify (JustifyMethod, optional): Justify method, or None for default. Defaults to None.
    overflow (OverflowMethod, optional): Overflow method, or None for default. Defaults to None.
    no_wrap (Optional[bool], optional): Disable word wrapping. Defaults to False.
    indent_guides (bool, optional): Enable indentation guides. Defaults to False.
    max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to None.
    max_string (int, optional): Maximum length of string before truncating, or None to disable. Defaults to None.
    max_depth (int, optional): Maximum depth of nested data structures, or None for no maximum. Defaults to None.
    expand_all (bool, optional): Expand all containers. Defaults to False.
    margin (int, optional): Subtrace a margin from width to force containers to expand earlier. Defaults to 0.
    insert_line (bool, optional): Insert a new line if the output has multiple new lines. Defaults to False.


## StockKeepingUnit

`rich.pretty.StockKeepingUnit`

```python
class StockKeepingUnit(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (5)**

- `category: str`  _instance-attribute_
- `description: str`  _instance-attribute_
- `name: str`  _instance-attribute_
- `price: float`  _instance-attribute_
- `reviews: List[str]`  _instance-attribute_

## Thing

`rich.pretty.Thing`

```python
class Thing
```

## _Line

`rich.pretty._Line`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Line
```

**Declared members (11)**

- `def check_length(self, max_length: int) -> bool`
  Check this line fits within a given number of cells.
- `def expand(self, indent_size: int) -> Iterable[_Line]`
  Expand this line by adding children on their own line.
- `expandable: bool`  _property_
  Check if the line may be expanded.
- `expanded: bool = False`  _class-attribute, instance-attribute_
- `is_root: bool = False`  _class-attribute, instance-attribute_
- `last: bool = False`  _class-attribute, instance-attribute_
- `node: Optional[Node] = None`  _class-attribute, instance-attribute_
- `parent: Optional[_Line] = None`  _class-attribute, instance-attribute_
- `suffix: str = ''`  _class-attribute, instance-attribute_
- `text: str = ''`  _class-attribute, instance-attribute_
- `whitespace: str = ''`  _class-attribute, instance-attribute_

A line in repr output.


## _get_attr_fields

`rich.pretty._get_attr_fields`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_attr_fields(obj: Any) -> Sequence[_attr_module.Attribute[Any]]
```

Get fields for an attrs object.


## _get_braces_for_array

`rich.pretty._get_braces_for_array`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_braces_for_array(_object: array[Any]) -> Tuple[str, str, str]
```

## _get_braces_for_defaultdict

`rich.pretty._get_braces_for_defaultdict`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_braces_for_defaultdict(_object: DefaultDict[Any, Any]) -> Tuple[str, str, str]
```

## _get_braces_for_deque

`rich.pretty._get_braces_for_deque`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_braces_for_deque(_object: Deque[Any]) -> Tuple[str, str, str]
```

## _has_default_namedtuple_repr

`rich.pretty._has_default_namedtuple_repr`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_default_namedtuple_repr(obj: object) -> bool
```

Check if an instance of namedtuple contains the default repr

Args:
    obj (object): A namedtuple

Returns:
    bool: True if the default repr is used, False if there's a custom repr.


## _ipy_display_hook

`rich.pretty._ipy_display_hook`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _ipy_display_hook(value: Any, console: Optional[Console] = None, overflow: OverflowMethod = 'ignore', crop: bool = False, indent_guides: bool = False, max_length: Optional[int] = None, max_string: Optional[int] = None, max_depth: Optional[int] = None, expand_all: bool = False) -> Union[str, None]
```

## _is_attr_object

`rich.pretty._is_attr_object`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_attr_object(obj: Any) -> bool
```

Check if an object was created with attrs module.


## _is_dataclass_repr

`rich.pretty._is_dataclass_repr`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_dataclass_repr(obj: object) -> bool
```

Check if an instance of a dataclass contains the default repr.

Args:
    obj (object): A dataclass instance.

Returns:
    bool: True if the default repr is used, False if there is a custom repr.


## _is_namedtuple

`rich.pretty._is_namedtuple`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_namedtuple(obj: Any) -> bool
```

Checks if an object is most likely a namedtuple. It is possible
to craft an object that passes this check and isn't a namedtuple, but
there is only a minuscule chance of this happening unintentionally.

Args:
    obj (Any): The object to test

Returns:
    bool: True if the object is a namedtuple. False otherwise.


## _safe_isinstance

`rich.pretty._safe_isinstance`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _safe_isinstance(obj: object, class_or_tuple: Union[type, Tuple[type, ...]]) -> bool
```

isinstance can fail in rare cases, for example types with no __class__


## install

`rich.pretty.install`

```python
def install(console: Optional[Console] = None, overflow: OverflowMethod = 'ignore', crop: bool = False, indent_guides: bool = False, max_length: Optional[int] = None, max_string: Optional[int] = None, max_depth: Optional[int] = None, expand_all: bool = False) -> None
```

Install automatic pretty printing in the Python REPL.

Args:
    console (Console, optional): Console instance or ``None`` to use global console. Defaults to None.
    overflow (Optional[OverflowMethod], optional): Overflow method. Defaults to "ignore".
    crop (Optional[bool], optional): Enable cropping of long lines. Defaults to False.
    indent_guides (bool, optional): Enable indentation guides. Defaults to False.
    max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to None.
    max_string (int, optional): Maximum length of string before truncating, or None to disable. Defaults to None.
    max_depth (int, optional): Maximum depth of nested data structures, or None for no maximum. Defaults to None.
    expand_all (bool, optional): Expand all containers. Defaults to False.
    max_frames (int): Maximum number of frames to show in a traceback, 0 for no maximum. Defaults to 100.


## is_expandable

`rich.pretty.is_expandable`

```python
def is_expandable(obj: Any) -> bool
```

Check if an object may be expanded by pretty print.


## pprint

`rich.pretty.pprint`

```python
def pprint(_object: Any, console: Optional[Console] = None, indent_guides: bool = True, max_length: Optional[int] = None, max_string: Optional[int] = None, max_depth: Optional[int] = None, expand_all: bool = False) -> None
```

A convenience function for pretty printing.

Args:
    _object (Any): Object to pretty print.
    console (Console, optional): Console instance, or None to use default. Defaults to None.
    max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to None.
    max_string (int, optional): Maximum length of strings before truncating, or None to disable. Defaults to None.
    max_depth (int, optional): Maximum depth for nested data structures, or None for unlimited depth. Defaults to None.
    indent_guides (bool, optional): Enable indentation guides. Defaults to True.
    expand_all (bool, optional): Expand all containers. Defaults to False.


## pretty_repr

`rich.pretty.pretty_repr`

```python
def pretty_repr(_object: Any, max_width: int = 80, indent_size: int = 4, max_length: Optional[int] = None, max_string: Optional[int] = None, max_depth: Optional[int] = None, expand_all: bool = False) -> str
```

Prettify repr string by expanding on to new lines to fit within a given width.

Args:
    _object (Any): Object to repr.
    max_width (int, optional): Desired maximum width of repr string. Defaults to 80.
    indent_size (int, optional): Number of spaces to indent. Defaults to 4.
    max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to None.
    max_string (int, optional): Maximum length of string before truncating, or None to disable truncating.
        Defaults to None.
    max_depth (int, optional): Maximum depth of nested data structure, or None for no depth.
        Defaults to None.
    expand_all (bool, optional): Expand all containers regardless of available width. Defaults to False.

Returns:
    str: A possibly multi-line representation of the object.


## traverse

`rich.pretty.traverse`

```python
def traverse(_object: Any, max_length: Optional[int] = None, max_string: Optional[int] = None, max_depth: Optional[int] = None) -> Node
```

Traverse object and generate a tree.

Args:
    _object (Any): Object to be traversed.
    max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to None.
    max_string (int, optional): Maximum length of string before truncating, or None to disable truncating.
        Defaults to None.
    max_depth (int, optional): Maximum depth of data structures, or None for no maximum.
        Defaults to None.

Returns:
    Node: The root of a tree structure which can be used to render a pretty repr.


