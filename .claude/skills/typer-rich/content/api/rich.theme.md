# `rich.theme`

Distribution: `rich`

## theme

`rich.theme.theme`

```python
theme = Theme()
```

**Inferred type** (`ty`, not declared in the source): `Theme`

## Theme

`rich.theme.Theme`

```python
class Theme
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `config: str`  _property_
  Get contents of a config file for this theme.
- `def from_file(cls, config_file: IO[str], source: Optional[str] = None, inherit: bool = True) -> Theme`  _classmethod_
  Load a theme from a text mode file.
- `def read(cls, path: str, inherit: bool = True, encoding: Optional[str] = None) -> Theme`  _classmethod_
  Read a theme from a path.
- `styles: Dict[str, Style] = DEFAULT_STYLES.copy() if inherit else {}`  _instance-attribute_

A container for style information, used by :class:`~rich.console.Console`.

Args:
    styles (Dict[str, Style], optional): A mapping of style names on to styles. Defaults to None for a theme with no styles.
    inherit (bool, optional): Inherit default styles. Defaults to True.


## ThemeStack

`rich.theme.ThemeStack`

```python
class ThemeStack
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `get = self._entries[-1].get`  _instance-attribute_
- `def pop_theme(self) -> None`
  Pop (and discard) the top-most theme.
- `def push_theme(self, theme: Theme, inherit: bool = True) -> None`
  Push a theme on the top of the stack.

A stack of themes.

Args:
    theme (Theme): A theme instance


## ThemeStackError

`rich.theme.ThemeStackError`

```python
class ThemeStackError(Exception)
```

**Bases** `Exception`

Base exception for errors related to the theme stack.


