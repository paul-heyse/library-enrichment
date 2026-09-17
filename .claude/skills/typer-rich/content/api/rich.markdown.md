# `rich.markdown`

Distribution: `rich`

## args

`rich.markdown.args`

```python
args = parser.parse_args()
```

**Inferred type** (`ty`, not declared in the source): `Namespace`

## console

`rich.markdown.console`

```python
console = Console(file=fileio, force_terminal=args.force_color, width=args.width)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## fileio

`rich.markdown.fileio`

```python
fileio = io.StringIO()
```

**Inferred type** (`ty`, not declared in the source): `StringIO`

## markdown

`rich.markdown.markdown`

```python
markdown = Markdown(markdown_body, justify='full' if args.justify else 'left', code_theme=args.code_theme, hyperlinks=args.hyperlinks, inline_code_lexer=args.inline_code_lexer)
```

**Inferred type** (`ty`, not declared in the source): `Markdown`

## markdown_body

`rich.markdown.markdown_body`

```python
markdown_body = markdown_file.read()
```

**Inferred type** (`ty`, not declared in the source): `str`

## parser

`rich.markdown.parser`

```python
parser = argparse.ArgumentParser(description='Render Markdown to the console with Rich')
```

**Inferred type** (`ty`, not declared in the source): `ArgumentParser`

## BlockQuote

`rich.markdown.BlockQuote`

```python
class BlockQuote(TextElement)
```

**Bases** `TextElement`

**Declared members (3)**

- `elements: Renderables = Renderables()`  _instance-attribute_
- `def on_child_close(self, context: MarkdownContext, child: MarkdownElement) -> bool`
- `style_name = 'markdown.block_quote'`  _class-attribute, instance-attribute_

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `create`, `new_line`
- from `rich.markdown.TextElement`: `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A block quote.


## CodeBlock

`rich.markdown.CodeBlock`

```python
class CodeBlock(TextElement)
```

**Bases** `TextElement`

**Declared members (4)**

- `def create(cls, markdown: Markdown, token: Token) -> CodeBlock`  _classmethod_
- `lexer_name = lexer_name`  _instance-attribute_
- `style_name = 'markdown.code_block'`  _class-attribute, instance-attribute_
- `theme = theme`  _instance-attribute_

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `new_line`, `on_child_close`
- from `rich.markdown.TextElement`: `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A code block with syntax highlighting.


## Heading

`rich.markdown.Heading`

```python
class Heading(TextElement)
```

**Bases** `TextElement`

**Declared members (5)**

- `LEVEL_ALIGN: dict[str, JustifyMethod] = {'h1': 'center', 'h2': 'left', 'h3': 'left', 'h4': 'left', 'h5': 'left', 'h6': 'left'}`  _class-attribute_
- `def create(cls, markdown: Markdown, token: Token) -> Heading`  _classmethod_
- `def on_enter(self, context: MarkdownContext) -> None`
- `style_name = f'markdown.{tag}'`  _instance-attribute_
- `tag = tag`  _instance-attribute_

**Inherited (4)**

- from `rich.markdown.MarkdownElement`: `new_line`, `on_child_close`
- from `rich.markdown.TextElement`: `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A heading.


## HeadingFormat

`rich.markdown.HeadingFormat`

```python
class HeadingFormat
```

**Declared members (2)**

- `justify: JustifyMethod = 'left'`  _class-attribute, instance-attribute_
- `style: str = ''`  _class-attribute, instance-attribute_

## HorizontalRule

`rich.markdown.HorizontalRule`

```python
class HorizontalRule(MarkdownElement)
```

**Bases** `MarkdownElement`

**Declared members (1)**

- `new_line = False`  _class-attribute, instance-attribute_

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `create`, `on_child_close`, `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A horizontal rule to divide sections.


## ImageItem

`rich.markdown.ImageItem`

```python
class ImageItem(TextElement)
```

**Bases** `TextElement`

**Declared members (6)**

- `def create(cls, markdown: Markdown, token: Token) -> MarkdownElement`  _classmethod_
  Factory to create markdown element,
- `destination = destination`  _instance-attribute_
- `hyperlinks = hyperlinks`  _instance-attribute_
- `link: str | None = None`  _instance-attribute_
- `new_line = False`  _class-attribute, instance-attribute_
- `def on_enter(self, context: MarkdownContext) -> None`

**Inherited (4)**

- from `rich.markdown.MarkdownElement`: `on_child_close`
- from `rich.markdown.TextElement`: `on_leave`, `on_text`, `style_name`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Renders a placeholder for an image.


## Link

`rich.markdown.Link`

```python
class Link(TextElement)
```

**Bases** `TextElement`

**Declared members (3)**

- `def create(cls, markdown: Markdown, token: Token) -> MarkdownElement`  _classmethod_
- `href = href`  _instance-attribute_
- `text = Text(text)`  _instance-attribute_

**Inherited (6)**

- from `rich.markdown.MarkdownElement`: `new_line`, `on_child_close`
- from `rich.markdown.TextElement`: `on_enter`, `on_leave`, `on_text`, `style_name`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## ListElement

`rich.markdown.ListElement`

```python
class ListElement(MarkdownElement)
```

**Bases** `MarkdownElement`

**Declared members (5)**

- `def create(cls, markdown: Markdown, token: Token) -> ListElement`  _classmethod_
- `items: list[ListItem] = []`  _instance-attribute_
- `list_start = list_start`  _instance-attribute_
- `list_type = list_type`  _instance-attribute_
- `def on_child_close(self, context: MarkdownContext, child: MarkdownElement) -> bool`

**Inherited (4)**

- from `rich.markdown.MarkdownElement`: `new_line`, `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A list element.


## ListItem

`rich.markdown.ListItem`

```python
class ListItem(TextElement)
```

**Bases** `TextElement`

**Declared members (5)**

- `elements: Renderables = Renderables()`  _instance-attribute_
- `def on_child_close(self, context: MarkdownContext, child: MarkdownElement) -> bool`
- `def render_bullet(self, console: Console, options: ConsoleOptions) -> RenderResult`
- `def render_number(self, console: Console, options: ConsoleOptions, number: int, last_number: int) -> RenderResult`
- `style_name = 'markdown.item'`  _class-attribute, instance-attribute_

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `create`, `new_line`
- from `rich.markdown.TextElement`: `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An item in a list.


## Markdown

`rich.markdown.Markdown`

```python
class Markdown(JupyterMixin)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (10)**

- `code_theme = code_theme`  _instance-attribute_
- `elements: dict[str, type[MarkdownElement]] = {'paragraph_open': Paragraph, 'heading_open': Heading, 'fence': CodeBlock, 'code_block': CodeBlock, 'blockquote_open': BlockQuote, 'hr': HorizontalRule, 'bullet_list_open': ListElement, 'ordered_list_open': ListElement, 'list_item_open': ListItem, 'image': ImageItem, 'table_open': TableElement, 'tbody_open': TableBodyElement, 'thead_open': TableHeaderElement, 'tr_open': TableRowElement, 'td_open': TableDataElement, 'th_open': TableDataElement}`  _class-attribute_
- `hyperlinks = hyperlinks`  _instance-attribute_
- `inline_code_lexer = inline_code_lexer`  _instance-attribute_
- `inline_code_theme = inline_code_theme or code_theme`  _instance-attribute_
- `inlines = {'em', 'strong', 'code', 's'}`  _class-attribute, instance-attribute_
- `justify: JustifyMethod | None = justify`  _instance-attribute_
- `markup = markup`  _instance-attribute_
- `parsed = parser.parse(markup)`  _instance-attribute_
- `style = style`  _instance-attribute_

A Markdown renderable.

Args:
    markup (str): A string containing markdown.
    code_theme (str, optional): Pygments theme for code blocks. Defaults to "monokai". See https://pygments.org/styles/ for code themes.
    justify (JustifyMethod, optional): Justify value for paragraphs. Defaults to None.
    style (Union[str, Style], optional): Optional style to apply to markdown.
    hyperlinks (bool, optional): Enable hyperlinks. Defaults to ``True``.
    inline_code_lexer: (str, optional): Lexer to use if inline code highlighting is
        enabled. Defaults to None.
    inline_code_theme: (Optional[str], optional): Pygments theme for inline code
        highlighting, or None for no highlighting. Defaults to None.


## MarkdownContext

`rich.markdown.MarkdownContext`

```python
class MarkdownContext
```

**Declared members (8)**

- `console = console`  _instance-attribute_
- `current_style: Style`  _property_
  Current style which is the product of all styles on the stack.
- `def enter_style(self, style_name: str | Style) -> Style`
  Enter a style context.
- `def leave_style(self) -> Style`
  Leave a style context.
- `def on_text(self, text: str, node_type: str) -> None`
  Called when the parser visits text.
- `options = options`  _instance-attribute_
- `stack: Stack[MarkdownElement] = Stack()`  _instance-attribute_
- `style_stack: StyleStack = StyleStack(style)`  _instance-attribute_

Manages the console render state.


## MarkdownElement

`rich.markdown.MarkdownElement`

```python
class MarkdownElement
```

**Declared members (6)**

- `def create(cls, markdown: Markdown, token: Token) -> MarkdownElement`  _classmethod_
  Factory to create markdown element,
- `new_line: bool = True`  _class-attribute_
- `def on_child_close(self, context: MarkdownContext, child: MarkdownElement) -> bool`
  Called when a child element is closed.
- `def on_enter(self, context: MarkdownContext) -> None`
  Called when the node is entered.
- `def on_leave(self, context: MarkdownContext) -> None`
  Called when the parser leaves the element.
- `def on_text(self, context: MarkdownContext, text: TextType) -> None`
  Called when text is parsed.

## Paragraph

`rich.markdown.Paragraph`

```python
class Paragraph(TextElement)
```

**Bases** `TextElement`

**Declared members (3)**

- `def create(cls, markdown: Markdown, token: Token) -> Paragraph`  _classmethod_
- `justify: JustifyMethod = justify`  _instance-attribute_
- `style_name = 'markdown.paragraph'`  _class-attribute, instance-attribute_

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `new_line`, `on_child_close`
- from `rich.markdown.TextElement`: `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Paragraph.


## TableBodyElement

`rich.markdown.TableBodyElement`

```python
class TableBodyElement(MarkdownElement)
```

**Bases** `MarkdownElement`

**Declared members (2)**

- `def on_child_close(self, context: MarkdownContext, child: MarkdownElement) -> bool`
- `rows: list[TableRowElement] = []`  _instance-attribute_

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `create`, `new_line`, `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

MarkdownElement corresponding to `tbody_open` and `tbody_close`.


## TableDataElement

`rich.markdown.TableDataElement`

```python
class TableDataElement(MarkdownElement)
```

**Bases** `MarkdownElement`

**Declared members (4)**

- `content: Text = Text('', justify=justify)`  _instance-attribute_
- `def create(cls, markdown: Markdown, token: Token) -> MarkdownElement`  _classmethod_
- `justify = justify`  _instance-attribute_
- `def on_text(self, context: MarkdownContext, text: TextType) -> None`

**Inherited (4)**

- from `rich.markdown.MarkdownElement`: `new_line`, `on_child_close`, `on_enter`, `on_leave`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

MarkdownElement corresponding to `td_open` and `td_close`
and `th_open` and `th_close`.


## TableElement

`rich.markdown.TableElement`

```python
class TableElement(MarkdownElement)
```

**Bases** `MarkdownElement`

**Declared members (3)**

- `body: TableBodyElement | None = None`  _instance-attribute_
- `header: TableHeaderElement | None = None`  _instance-attribute_
- `def on_child_close(self, context: MarkdownContext, child: MarkdownElement) -> bool`

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `create`, `new_line`, `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

MarkdownElement corresponding to `table_open`.


## TableHeaderElement

`rich.markdown.TableHeaderElement`

```python
class TableHeaderElement(MarkdownElement)
```

**Bases** `MarkdownElement`

**Declared members (2)**

- `def on_child_close(self, context: MarkdownContext, child: MarkdownElement) -> bool`
- `row: TableRowElement | None = None`  _instance-attribute_

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `create`, `new_line`, `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

MarkdownElement corresponding to `thead_open` and `thead_close`.


## TableRowElement

`rich.markdown.TableRowElement`

```python
class TableRowElement(MarkdownElement)
```

**Bases** `MarkdownElement`

**Declared members (2)**

- `cells: list[TableDataElement] = []`  _instance-attribute_
- `def on_child_close(self, context: MarkdownContext, child: MarkdownElement) -> bool`

**Inherited (5)**

- from `rich.markdown.MarkdownElement`: `create`, `new_line`, `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

MarkdownElement corresponding to `tr_open` and `tr_close`.


## TextElement

`rich.markdown.TextElement`

```python
class TextElement(MarkdownElement)
```

**Bases** `MarkdownElement`

**Declared members (4)**

- `def on_enter(self, context: MarkdownContext) -> None`
- `def on_leave(self, context: MarkdownContext) -> None`
- `def on_text(self, context: MarkdownContext, text: TextType) -> None`
- `style_name = 'none'`  _class-attribute, instance-attribute_

**Inherited (3)**

- from `rich.markdown.MarkdownElement`: `create`, `new_line`, `on_child_close`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Base class for elements that render text.


## UnknownElement

`rich.markdown.UnknownElement`

```python
class UnknownElement(MarkdownElement)
```

**Bases** `MarkdownElement`

**Inherited (6)**

- from `rich.markdown.MarkdownElement`: `create`, `new_line`, `on_child_close`, `on_enter`, `on_leave`, `on_text`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An unknown element.

Hopefully there will be no unknown elements, and we will have a MarkdownElement for
everything in the document.


