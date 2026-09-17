# `rich.rule`

Distribution: `rich`

## console

`rich.rule.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## text

`rich.rule.text`

```python
text = sys.argv[1]
```

**Inferred type** (`ty`, not declared in the source): `str`

## Rule

`rich.rule.Rule`

```python
class Rule(JupyterMixin)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (5)**

- `align = align`  _instance-attribute_
- `characters = characters`  _instance-attribute_
- `end = end`  _instance-attribute_
- `style = style`  _instance-attribute_
- `title = title`  _instance-attribute_

A console renderable to draw a horizontal rule (line).

Args:
    title (Union[str, Text], optional): Text to render in the rule. Defaults to "".
    characters (str, optional): Character(s) used to draw the line. Defaults to "─".
    style (StyleType, optional): Style of Rule. Defaults to "rule.line".
    end (str, optional): Character at end of Rule. defaults to "\\n"
    align (str, optional): How to align the title, one of "left", "center", or "right". Defaults to "center".


