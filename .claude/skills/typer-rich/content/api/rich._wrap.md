# `rich._wrap`

Distribution: `rich`

## console

`rich._wrap.console`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
console = Console(width=10)
```

## re_word

`rich._wrap.re_word`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
re_word = re.compile('\\s*\\S+\\s*')
```

## divide_line

Import as `rich.text.divide_line`  ·  defined at `rich._wrap.divide_line`

```python
def divide_line(text: str, width: int, fold: bool = True) -> list[int]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Given a string of text, and a width (measured in cells), return a list
of cell offsets which the string should be split at in order for it to fit
within the given width.

Args:
    text: The text to examine.
    width: The available cell width.
    fold: If True, words longer than `width` will be folded onto a new line.

Returns:
    A list of indices to break the line at.


## words

`rich._wrap.words`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def words(text: str) -> Iterable[tuple[int, int, str]]
```

Yields each word from the text as a tuple
containing (start_index, end_index, word). A "word" in this context may
include the actual word and any whitespace to the right.


