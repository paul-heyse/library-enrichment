# `rich.errors`

Distribution: `rich`

## ConsoleError

`rich.errors.ConsoleError`

```python
class ConsoleError(Exception)
```

**Bases** `Exception`

An error in console operation.


## LiveError

`rich.errors.LiveError`

```python
class LiveError(ConsoleError)
```

**Bases** `ConsoleError`

Error related to Live display.


## MarkupError

`rich.errors.MarkupError`

```python
class MarkupError(ConsoleError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ConsoleError`

Markup was badly formatted.


## MissingStyle

`rich.errors.MissingStyle`

```python
class MissingStyle(StyleError)
```

**Bases** `StyleError`

No such style.


## NoAltScreen

`rich.errors.NoAltScreen`

```python
class NoAltScreen(ConsoleError)
```

**Bases** `ConsoleError`

Alt screen mode was required.


## NotRenderableError

`rich.errors.NotRenderableError`

```python
class NotRenderableError(ConsoleError)
```

**Bases** `ConsoleError`

Object is not renderable.


## StyleError

`rich.errors.StyleError`

```python
class StyleError(Exception)
```

**Bases** `Exception`

An error in styles.


## StyleStackError

`rich.errors.StyleStackError`

```python
class StyleStackError(ConsoleError)
```

**Bases** `ConsoleError`

Style stack is invalid.


## StyleSyntaxError

`rich.errors.StyleSyntaxError`

```python
class StyleSyntaxError(ConsoleError)
```

**Bases** `ConsoleError`

Style was badly formatted.


