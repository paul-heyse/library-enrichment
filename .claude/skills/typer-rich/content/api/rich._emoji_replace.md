# `rich._emoji_replace`

Distribution: `rich`

## _EmojiSubMethod

`rich._emoji_replace._EmojiSubMethod`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_EmojiSubMethod = Callable[[_ReSubCallable, str], str]
```

## _ReStringMatch

`rich._emoji_replace._ReStringMatch`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ReStringMatch = Match[str]
```

## _ReSubCallable

`rich._emoji_replace._ReSubCallable`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ReSubCallable = Callable[[_ReStringMatch], str]
```

## _emoji_replace

`rich._emoji_replace._emoji_replace`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _emoji_replace(text: str, default_variant: Optional[str] = None, _emoji_sub: _EmojiSubMethod = re.compile('(:(\\S*?)(?:(?:\\-)(emoji|text))?:)').sub) -> str
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Replace emoji code in text.


