# `rich.terminal_theme`

Distribution: `rich`

## DEFAULT_TERMINAL_THEME

`rich.terminal_theme.DEFAULT_TERMINAL_THEME`

```python
DEFAULT_TERMINAL_THEME = TerminalTheme((255, 255, 255), (0, 0, 0), [(0, 0, 0), (128, 0, 0), (0, 128, 0), (128, 128, 0), (0, 0, 128), (128, 0, 128), (0, 128, 128), (192, 192, 192)], [(128, 128, 128), (255, 0, 0), (0, 255, 0), (255, 255, 0), (0, 0, 255), (255, 0, 255), (0, 255, 255), (255, 255, 255)])
```

**Inferred type** (`ty`, not declared in the source): `TerminalTheme`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DIMMED_MONOKAI

`rich.terminal_theme.DIMMED_MONOKAI`

```python
DIMMED_MONOKAI = TerminalTheme((25, 25, 25), (185, 188, 186), [(58, 61, 67), (190, 63, 72), (135, 154, 59), (197, 166, 53), (79, 118, 161), (133, 92, 141), (87, 143, 164), (185, 188, 186), (136, 137, 135)], [(251, 0, 31), (15, 114, 47), (196, 112, 51), (24, 109, 227), (251, 0, 103), (46, 112, 109), (253, 255, 185)])
```

**Inferred type** (`ty`, not declared in the source): `TerminalTheme`

## MONOKAI

`rich.terminal_theme.MONOKAI`

```python
MONOKAI = TerminalTheme((12, 12, 12), (217, 217, 217), [(26, 26, 26), (244, 0, 95), (152, 224, 36), (253, 151, 31), (157, 101, 255), (244, 0, 95), (88, 209, 235), (196, 197, 181), (98, 94, 76)], [(244, 0, 95), (152, 224, 36), (224, 213, 97), (157, 101, 255), (244, 0, 95), (88, 209, 235), (246, 246, 239)])
```

**Inferred type** (`ty`, not declared in the source): `TerminalTheme`

## NIGHT_OWLISH

`rich.terminal_theme.NIGHT_OWLISH`

```python
NIGHT_OWLISH = TerminalTheme((255, 255, 255), (64, 63, 83), [(1, 22, 39), (211, 66, 62), (42, 162, 152), (218, 170, 1), (72, 118, 214), (64, 63, 83), (8, 145, 106), (122, 129, 129), (122, 129, 129)], [(247, 110, 110), (73, 208, 197), (218, 194, 107), (92, 167, 228), (105, 112, 152), (0, 201, 144), (152, 159, 177)])
```

**Inferred type** (`ty`, not declared in the source): `TerminalTheme`

## SVG_EXPORT_THEME

`rich.terminal_theme.SVG_EXPORT_THEME`

```python
SVG_EXPORT_THEME = TerminalTheme((41, 41, 41), (197, 200, 198), [(75, 78, 85), (204, 85, 90), (152, 168, 75), (208, 179, 68), (96, 138, 177), (152, 114, 159), (104, 160, 179), (197, 200, 198), (154, 155, 153)], [(255, 38, 39), (0, 130, 61), (208, 132, 66), (25, 132, 233), (255, 44, 122), (57, 130, 128), (253, 253, 197)])
```

**Inferred type** (`ty`, not declared in the source): `TerminalTheme`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _ColorTuple

`rich.terminal_theme._ColorTuple`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ColorTuple = Tuple[int, int, int]
```

## TerminalTheme

`rich.terminal_theme.TerminalTheme`

```python
class TerminalTheme
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `ansi_colors = Palette(normal + (bright or normal))`  _instance-attribute_
- `background_color = ColorTriplet(*background)`  _instance-attribute_
- `foreground_color = ColorTriplet(*foreground)`  _instance-attribute_

A color theme used when exporting console content.

Args:
    background (Tuple[int, int, int]): The background color.
    foreground (Tuple[int, int, int]): The foreground (text) color.
    normal (List[Tuple[int, int, int]]): A list of 8 normal intensity colors.
    bright (List[Tuple[int, int, int]], optional): A list of 8 bright colors, or None
        to repeat normal intensity. Defaults to None.


