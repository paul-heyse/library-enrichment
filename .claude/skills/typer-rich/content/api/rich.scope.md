# `rich.scope`

Distribution: `rich`

## render_scope

`rich.scope.render_scope`

```python
def render_scope(scope: Mapping[str, Any], title: Optional[TextType] = None, sort_keys: bool = True, indent_guides: bool = False, max_length: Optional[int] = None, max_string: Optional[int] = None, max_depth: Optional[int] = None, overflow: Optional[OverflowMethod] = None) -> ConsoleRenderable
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Render python variables in a given scope.

Args:
    scope (Mapping): A mapping containing variable names and values.
    title (str, optional): Optional title. Defaults to None.
    sort_keys (bool, optional): Enable sorting of items. Defaults to True.
    indent_guides (bool, optional): Enable indentation guides. Defaults to False.
    max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to None.
    max_string (int, optional): Maximum length of string before truncating, or None to disable. Defaults to None.
    max_depth (int, optional): Maximum depths of locals before truncating, or None to disable. Defaults to None.
    overflow (OverflowMethod, optional): How to handle overflowing locals, or None to disable. Defaults to None.

Returns:
    ConsoleRenderable: A renderable object.


## test

`rich.scope.test`

```python
def test(foo: float, bar: float) -> None
```

