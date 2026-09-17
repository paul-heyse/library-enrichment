# `rich.jupyter`

Distribution: `rich`

## JUPYTER_HTML_FORMAT

`rich.jupyter.JUPYTER_HTML_FORMAT`

```python
JUPYTER_HTML_FORMAT = '<pre style="white-space:pre;overflow-x:auto;line-height:normal;font-family:Menlo,\'DejaVu Sans Mono\',consolas,\'Courier New\',monospace">{code}</pre>\n'
```

**Inferred type** (`ty`, not declared in the source): `Literal["<pre style=\"white-space:pre;overflow-x:auto;line-height:normal;font-family:Menlo,'DejaVu Sans Mono',consolas,'Courier New',monospace\">{code}</pre>\n"]`

## JupyterMixin

`rich.jupyter.JupyterMixin`

```python
class JupyterMixin
```

_19 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Add to an Rich renderable to make it render in Jupyter notebook.


## JupyterRenderable

`rich.jupyter.JupyterRenderable`

```python
class JupyterRenderable
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `html = html`  _instance-attribute_
- `text = text`  _instance-attribute_

A shim to write html to Jupyter notebook.


## _render_segments

`rich.jupyter._render_segments`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _render_segments(segments: Iterable[Segment]) -> str
```

## display

`rich.jupyter.display`

```python
def display(segments: Iterable[Segment], text: str) -> None
```

Render segments to Jupyter.


## print

`rich.jupyter.print`

```python
def print(args: Any = (), kwargs: Any = {}) -> None
```

Proxy for Console print.


