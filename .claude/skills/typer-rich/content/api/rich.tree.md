# `rich.tree`

Distribution: `rich`

## GuideType

`rich.tree.GuideType`

```python
GuideType = Tuple[str, str, str, str]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'tuple[str, str, str, str]'> ````

## code

`rich.tree.code`

```python
code = 'class Segment(NamedTuple):\n    text: str = ""\n    style: Optional[Style] = None\n    is_control: bool = False\n'
```

**Inferred type** (`ty`, not declared in the source): `Literal["class Segment(NamedTuple):\n text: str = \"\"\n style: Optional[Style] = None\n is_control: bool = False\n"]`

## console

`rich.tree.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## containers_node

`rich.tree.containers_node`

```python
containers_node = node.add(':file_folder: [bold magenta]Containers', guide_style='bold magenta')
```

**Inferred type** (`ty`, not declared in the source): `Tree`

## markdown

`rich.tree.markdown`

```python
markdown = Markdown('### example.md\n> Hello, World!\n>\n> Markdown _all_ the things\n')
```

**Inferred type** (`ty`, not declared in the source): `Markdown`

## node

`rich.tree.node`

```python
node = root.add(':file_folder: Renderables', guide_style='red')
```

**Inferred type** (`ty`, not declared in the source): `Tree`

## panel

`rich.tree.panel`

```python
panel = Panel.fit('Just a panel', border_style='red')
```

**Inferred type** (`ty`, not declared in the source): `Panel`

## root

`rich.tree.root`

```python
root = Tree('🌲 [b green]Rich Tree', highlight=True, hide_root=True)
```

**Inferred type** (`ty`, not declared in the source): `Tree`

## simple_node

`rich.tree.simple_node`

```python
simple_node = node.add(':file_folder: [bold yellow]Atomic', guide_style='uu green')
```

**Inferred type** (`ty`, not declared in the source): `Tree`

## syntax

`rich.tree.syntax`

```python
syntax = Syntax(code, 'python', theme='monokai', line_numbers=True)
```

**Inferred type** (`ty`, not declared in the source): `Syntax`

## table

`rich.tree.table`

```python
table = Table(row_styles=['', 'dim'])
```

**Inferred type** (`ty`, not declared in the source): `Table`

## Tree

`rich.tree.Tree`

```python
class Tree(JupyterMixin)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (10)**

- `ASCII_GUIDES = ('    ', '|   ', '+-- ', '`-- ')`  _class-attribute, instance-attribute_
- `TREE_GUIDES = [('    ', '│   ', '├── ', '└── '), ('    ', '┃   ', '┣━━ ', '┗━━ '), ('    ', '║   ', '╠══ ', '╚══ ')]`  _class-attribute, instance-attribute_
- `def add(self, label: RenderableType, style: Optional[StyleType] = None, guide_style: Optional[StyleType] = None, expanded: bool = True, highlight: Optional[bool] = False) -> Tree`
  Add a child tree.
- `children: List[Tree] = []`  _instance-attribute_
- `expanded = expanded`  _instance-attribute_
- `guide_style = guide_style`  _instance-attribute_
- `hide_root = hide_root`  _instance-attribute_
- `highlight = highlight`  _instance-attribute_
- `label = label`  _instance-attribute_
- `style = style`  _instance-attribute_

A renderable for a tree structure.

Attributes:
    ASCII_GUIDES (GuideType): Guide lines used when Console.ascii_only is True.
    TREE_GUIDES (List[GuideType, GuideType, GuideType]): Default guide lines.

Args:
    label (RenderableType): The renderable or str for the tree label.
    style (StyleType, optional): Style of this tree. Defaults to "tree".
    guide_style (StyleType, optional): Style of the guide lines. Defaults to "tree.line".
    expanded (bool, optional): Also display children. Defaults to True.
    highlight (bool, optional): Highlight renderable (if str). Defaults to False.
    hide_root (bool, optional): Hide the root node. Defaults to False.


