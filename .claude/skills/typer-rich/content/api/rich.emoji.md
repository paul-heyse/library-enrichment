# `rich.emoji`

Distribution: `rich`

## EmojiVariant

`rich.emoji.EmojiVariant`

```python
EmojiVariant = Literal['emoji', 'text']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["emoji", "text"]'> ````

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## columns

`rich.emoji.columns`

```python
columns = Columns((f':{name}: {name}' for name in sorted(EMOJI.keys()) if '\u200d' not in name), column_first=True)
```

**Inferred type** (`ty`, not declared in the source): `Columns`

## console

`rich.emoji.console`

```python
console = Console(record=True)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## Emoji

`rich.emoji.Emoji`

```python
class Emoji(JupyterMixin)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (5)**

- `VARIANTS = {'text': '︎', 'emoji': '️'}`  _class-attribute, instance-attribute_
- `name = name`  _instance-attribute_
- `def replace(cls, text: str) -> str`  _classmethod_
  Replace emoji markup with corresponding unicode characters.
- `style = style`  _instance-attribute_
- `variant = variant`  _instance-attribute_

## NoEmoji

`rich.emoji.NoEmoji`

```python
class NoEmoji(Exception)
```

**Bases** `Exception`

No emoji by that name.


