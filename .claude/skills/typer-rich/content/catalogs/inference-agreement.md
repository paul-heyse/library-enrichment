# Where the two engines disagree

`inferred.tsv` holds types the source does not state. They come from **ty**, asked by
hover at each definition site. The pinned checker answers the module-level ones too, in
the batch report it produces anyway, so every row can carry a second opinion.

137 agree · 75 where ty is narrower · 37 differ · 7 the checker did not answer.

**75 of these are precision, not conflict.** ty narrows a constant to its literal type
where the checker gives the widened base. Both are right. Code
written against the narrow answer will not type-check under the other engine, which is why they are
counted separately rather than folded into either bucket.

`unanswered` is not disagreement. ty was asked about class attributes and function
returns as well as module-level names; the checker's `global_variables` covers only the
last of those, so most rows have nothing to compare against.

Comparison strips each dotted name to its leaf and drops rendering noise, because the
two engines spell types differently by design -- `Logger` against `logging.Logger` is
agreement, not conflict. A row below is a real difference of opinion about the type.

| Item | ty | checker |
|---|---|---|
| `rich.align.AlignMethod` | ````xml <special-form 'Literal["left", "center", "right"]'> `` | `type[typing.Literal['center', 'left', 'right']]` |
| `rich.align.VerticalAlignMethod` | ````xml <special-form 'Literal["top", "middle", "bottom"]'> `` | `type[typing.Literal['bottom', 'middle', 'top']]` |
| `rich.cells.CellSpan` | ````xml <class 'tuple[int, int, int]'> ```` | `type[tuple[int, int, int]]` |
| `rich.console.HighlighterType` | ````xml <Callable special-form '(str | Text, /) -> Text'> ```` | `type[(rich.text.Text | str) -> rich.text.Text]` |
| `rich.console.JustifyMethod` | ````xml <special-form 'Literal["default", "left", "center", "` | `type[typing.Literal['center', 'default', 'full', 'left', 'ri` |
| `rich.console.OverflowMethod` | ````xml <special-form 'Literal["fold", "crop", "ellipsis", "i` | `type[typing.Literal['crop', 'ellipsis', 'fold', 'ignore']]` |
| `rich.console.RenderResult` | ````xml <class 'Iterable[ConsoleRenderable | RichCast | str |` | `type[typing.Iterable[rich.console.ConsoleRenderable | rich.c` |
| `rich.console.RenderableType` | ````xml <types.UnionType special-form 'ConsoleRenderable | Ri` | `type[rich.console.ConsoleRenderable | rich.console.RichCast ` |
| `rich.emoji.EmojiVariant` | ````xml <special-form 'Literal["emoji", "text"]'> ```` | `type[typing.Literal['emoji', 'text']]` |
| `rich.json.json_data` | `str | Any` | `str | Unknown` |
| `rich.layout.RegionMap` | ````xml <class 'dict[Layout, Region]'> ```` | `type[dict[rich.layout.Layout, rich.region.Region]]` |
| `rich.layout.RenderMap` | ````xml <class 'dict[Layout, LayoutRender]'> ```` | `type[dict[rich.layout.Layout, rich.layout.LayoutRender]]` |
| `rich.live.examples` | `cycle[str | Panel | Table | ... omitted 3 union elements]` | `itertools.cycle[rich.panel.Panel | rich.rule.Rule | rich.syn` |
| `rich.live.progress_renderables` | `list[str | Panel | Table | ... omitted 3 union elements]` | `list[rich.panel.Panel | rich.rule.Rule | rich.syntax.Syntax ` |
| `rich.live_render.VerticalOverflowMethod` | ````xml <special-form 'Literal["crop", "ellipsis", "visible"]` | `type[typing.Literal['crop', 'ellipsis', 'visible']]` |
| `rich.markdown.markdown_body` | `str` | `str | Unknown` |
| `rich.padding.PaddingDimensions` | ````xml <types.UnionType special-form 'int | tuple[int] | tup` | `type[int | tuple[int] | tuple[int, int] | tuple[int, int, in` |
| `rich.pretty.d` | `defaultdict[Unknown, int]` | `collections.defaultdict[str, int]` |
| `rich.pretty.data` | `dict[str, list[float | str | set[int | tuple[int, int, int, ` | `dict[str, rich.pretty.BrokenRepr | collections.Counter[str] ` |
| `rich.progress.GetTimeCallable` | ````xml <Callable special-form '() -> float'> ```` | `type[() -> float]` |
| `rich.progress.TaskID` | ````xml <NewType pseudo-class 'TaskID'> ```` | `type[rich.progress.TaskID]` |
| `rich.progress.examples` | `cycle[str | Panel | Table | ... omitted 3 union elements]` | `itertools.cycle[rich.panel.Panel | rich.rule.Rule | rich.syn` |
| `rich.progress.progress_renderables` | `list[str | Panel | Table | ... omitted 3 union elements]` | `list[rich.panel.Panel | rich.rule.Rule | rich.syntax.Syntax ` |
| `rich.repr.Result` | ````xml <class 'Iterable[Any | tuple[Any] | tuple[str, Any] |` | `type[typing.Iterable[tuple[str, typing.Any] | tuple[str, typ` |
| `rich.repr.RichReprResult` | ````xml <class 'Iterable[Any | tuple[Any] | tuple[str, Any] |` | `type[typing.Iterable[tuple[str, typing.Any] | tuple[str, typ` |
| `rich.segment.ControlCode` | ````xml <types.UnionType special-form 'tuple[ControlType] | t` | `type[tuple[rich.segment.ControlType] | tuple[rich.segment.Co` |
| `rich.style.StyleType` | ````xml <types.UnionType special-form 'str | Style'> ```` | `type[rich.style.Style | str]` |
| `rich.syntax.SyntaxPosition` | ````xml <class 'tuple[int, int]'> ```` | `type[tuple[int, int]]` |
| `rich.syntax.TokenType` | ````xml <class 'tuple[str, ...]'> ```` | `type[tuple[str, ...]]` |
| `rich.syntax.code` | `str | Any` | `str | Unknown` |
| `rich.text.GetStyleCallable` | ````xml <Callable special-form '(str, /) -> str | Style | Non` | `type[(str) -> rich.style.Style | str | None]` |
| `rich.text.TextType` | ````xml <types.UnionType special-form 'str | Text'> ``` --- A` | `type[rich.text.Text | str]` |
| `rich.tree.GuideType` | ````xml <class 'tuple[str, str, str, str]'> ```` | `type[tuple[str, str, str, str]]` |
| `typer.core.MarkupMode` | ````xml <special-form 'Literal["markdown", "rich"] | None'> `` | `type[typing.Literal['markdown', 'rich'] | None]` |
| `typer.models.AnyType` | ````xml <special-form 'type[Any]'> ```` | `type[type[Any]]` |
| `typer.models.NoneType` | ````xml <class 'NoneType'> ```` | `type[types.NoneType]` |
| `typer.rich_utils.MarkupModeStrict` | ````xml <special-form 'Literal["markdown", "rich"]'> ```` | `type[typing.Literal['markdown', 'rich']]` |

Neither answer is a promise upstream made. An inferred type is correct at this pin and
can change without a release note, which is why these rows are separated from the
declared ones rather than merged into them.
