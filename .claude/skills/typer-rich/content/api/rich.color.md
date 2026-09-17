# `rich.color`

Distribution: `rich`

## ANSI_COLOR_NAMES

`rich.color.ANSI_COLOR_NAMES`

```python
ANSI_COLOR_NAMES = {'black': 0, 'red': 1, 'green': 2, 'yellow': 3, 'blue': 4, 'magenta': 5, 'cyan': 6, 'white': 7, 'bright_black': 8, 'bright_red': 9, 'bright_green': 10, 'bright_yellow': 11, 'bright_blue': 12, 'bright_magenta': 13, 'bright_cyan': 14, 'bright_white': 15, 'grey0': 16, 'gray0': 16, 'navy_blue': 17, 'dark_blue': 18, 'blue3': 20, 'blue1': 21, 'dark_green': 22, 'deep_sky_blue4': 25, 'dodger_blue3': 26, 'dodger_blue2': 27, 'green4': 28, 'spring_green4': 29, 'turquoise4': 30, 'deep_sky_blue3': 32, 'dodger_blue1': 33, 'green3': 40, 'spring_green3': 41, 'dark_cyan': 36, 'light_sea_green': 37, 'deep_sky_blue2': 38, 'deep_sky_blue1': 39, 'spring_green2': 47, 'cyan3': 43, 'dark_turquoise': 44, 'turquoise2': 45, 'green1': 46, 'spring_green1': 48, 'medium_spring_green': 49, 'cyan2': 50, 'cyan1': 51, 'dark_red': 88, 'deep_pink4': 125, 'purple4': 55, 'purple3': 56, 'blue_violet': 57, 'orange4': 94, 'grey37': 59, 'gray37': 59, 'medium_purple4': 60, 'slate_blue3': 62, 'royal_blue1': 63, 'chartreuse4': 64, 'dark_sea_green4': 71, 'pale_turquoise4': 66, 'steel_blue': 67, 'steel_blue3': 68, 'cornflower_blue': 69, 'chartreuse3': 76, 'cadet_blue': 73, 'sky_blue3': 74, 'steel_blue1': 81, 'pale_green3': 114, 'sea_green3': 78, 'aquamarine3': 79, 'medium_turquoise': 80, 'chartreuse2': 112, 'sea_green2': 83, 'sea_green1': 85, 'aquamarine1': 122, 'dark_slate_gray2': 87, 'dark_magenta': 91, 'dark_violet': 128, 'purple': 129, 'light_pink4': 95, 'plum4': 96, 'medium_purple3': 98, 'slate_blue1': 99, 'yellow4': 106, 'wheat4': 101, 'grey53': 102, 'gray53': 102, 'light_slate_grey': 103, 'light_slate_gray': 103, 'medium_purple': 104, 'light_slate_blue': 105, 'dark_olive_green3': 149, 'dark_sea_green': 108, 'light_sky_blue3': 110, 'sky_blue2': 111, 'dark_sea_green3': 150, 'dark_slate_gray3': 116, 'sky_blue1': 117, 'chartreuse1': 118, 'light_green': 120, 'pale_green1': 156, 'dark_slate_gray1': 123, 'red3': 160, 'medium_violet_red': 126, 'magenta3': 164, 'dark_orange3': 166, 'indian_red': 167, 'hot_pink3': 168, 'medium_orchid3': 133, 'medium_orchid': 134, 'medium_purple2': 140, 'dark_goldenrod': 136, 'light_salmon3': 173, 'rosy_brown': 138, 'grey63': 139, 'gray63': 139, 'medium_purple1': 141, 'gold3': 178, 'dark_khaki': 143, 'navajo_white3': 144, 'grey69': 145, 'gray69': 145, 'light_steel_blue3': 146, 'light_steel_blue': 147, 'yellow3': 184, 'dark_sea_green2': 157, 'light_cyan3': 152, 'light_sky_blue1': 153, 'green_yellow': 154, 'dark_olive_green2': 155, 'dark_sea_green1': 193, 'pale_turquoise1': 159, 'deep_pink3': 162, 'magenta2': 200, 'hot_pink2': 169, 'orchid': 170, 'medium_orchid1': 207, 'orange3': 172, 'light_pink3': 174, 'pink3': 175, 'plum3': 176, 'violet': 177, 'light_goldenrod3': 179, 'tan': 180, 'misty_rose3': 181, 'thistle3': 182, 'plum2': 183, 'khaki3': 185, 'light_goldenrod2': 222, 'light_yellow3': 187, 'grey84': 188, 'gray84': 188, 'light_steel_blue1': 189, 'yellow2': 190, 'dark_olive_green1': 192, 'honeydew2': 194, 'light_cyan1': 195, 'red1': 196, 'deep_pink2': 197, 'deep_pink1': 199, 'magenta1': 201, 'orange_red1': 202, 'indian_red1': 204, 'hot_pink': 206, 'dark_orange': 208, 'salmon1': 209, 'light_coral': 210, 'pale_violet_red1': 211, 'orchid2': 212, 'orchid1': 213, 'orange1': 214, 'sandy_brown': 215, 'light_salmon1': 216, 'light_pink1': 217, 'pink1': 218, 'plum1': 219, 'gold1': 220, 'navajo_white1': 223, 'misty_rose1': 224, 'thistle1': 225, 'yellow1': 226, 'light_goldenrod1': 227, 'khaki1': 228, 'wheat1': 229, 'cornsilk1': 230, 'grey100': 231, 'gray100': 231, 'grey3': 232, 'gray3': 232, 'grey7': 233, 'gray7': 233, 'grey11': 234, 'gray11': 234, 'grey15': 235, 'gray15': 235, 'grey19': 236, 'gray19': 236, 'grey23': 237, 'gray23': 237, 'grey27': 238, 'gray27': 238, 'grey30': 239, 'gray30': 239, 'grey35': 240, 'gray35': 240, 'grey39': 241, 'gray39': 241, 'grey42': 242, 'gray42': 242, 'grey46': 243, 'gray46': 243, 'grey50': 244, 'gray50': 244, 'grey54': 245, 'gray54': 245, 'grey58': 246, 'gray58': 246, 'grey62': 247, 'gray62': 247, 'grey66': 248, 'gray66': 248, 'grey70': 249, 'gray70': 249, 'grey74': 250, 'gray74': 250, 'grey78': 251, 'gray78': 251, 'grey82': 252, 'gray82': 252, 'grey85': 253, 'gray85': 253, 'grey89': 254, 'gray89': 254, 'grey93': 255, 'gray93': 255}
```

**Inferred type** (`ty`, not declared in the source): `dict[str, int]`

## RE_COLOR

`rich.color.RE_COLOR`

```python
RE_COLOR = re.compile('^\n\\#([0-9a-f]{6})$|\ncolor\\(([0-9]{1,3})\\)$|\nrgb\\(([\\d\\s,]+)\\)$\n', re.VERBOSE)
```

**Inferred type** (`ty`, not declared in the source): `Pattern[str]`

## WINDOWS

`rich.color.WINDOWS`

```python
WINDOWS = sys.platform == 'win32'
```

**Inferred type** (`ty`, not declared in the source): `Literal[False]`

## color

`rich.color.color`

```python
color = EIGHT_BIT_PALETTE[color_number]
```

**Inferred type** (`ty`, not declared in the source): `ColorTriplet`

## color_cell

`rich.color.color_cell`

```python
color_cell = Text(' ' * 10, style=f'on {name}')
```

**Inferred type** (`ty`, not declared in the source): `Text`

## colors

`rich.color.colors`

```python
colors = sorted((v, k) for k, v in ANSI_COLOR_NAMES.items())
```

**Inferred type** (`ty`, not declared in the source): `list[tuple[int, str]]`

## console

`rich.color.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## table

`rich.color.table`

```python
table = Table(show_footer=False, show_edge=True)
```

**Inferred type** (`ty`, not declared in the source): `Table`

## Color

`rich.color.Color`

```python
class Color(NamedTuple)
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NamedTuple`

**Declared members (15)**

- `def default(cls) -> Color`  _classmethod_
  Get a Color instance representing the default color.
- `def downgrade(self, system: ColorSystem) -> Color`  _cached_
  Downgrade a color system to a system with fewer colors.
- `def from_ansi(cls, number: int) -> Color`  _classmethod_
  Create a Color number from it's 8-bit ansi number.
- `def from_rgb(cls, red: float, green: float, blue: float) -> Color`  _classmethod_
  Create a truecolor from three color components in the range(0->255).
- `def from_triplet(cls, triplet: ColorTriplet) -> Color`  _classmethod_
  Create a truecolor RGB color from a triplet of values.
- `def get_ansi_codes(self, foreground: bool = True) -> Tuple[str, ...]`  _cached_
  Get the ANSI escape codes for this color.
- `def get_truecolor(self, theme: Optional[TerminalTheme] = None, foreground: bool = True) -> ColorTriplet`
  Get an equivalent color triplet for this color.
- `is_default: bool`  _property_
  Check if the color is a default color.
- `is_system_defined: bool`  _property_
  Check if the color is ultimately defined by the system.
- `name: str`  _instance-attribute_
  The name of the color (typically the input to Color.parse).
- `number: Optional[int] = None`  _class-attribute, instance-attribute_
  The color number, if a standard color, or None.
- `def parse(cls, color: str) -> Color`  _cached, classmethod_
  Parse a color definition.
- `system: ColorSystem`  _property_
  Get the native color system for this color.
- `triplet: Optional[ColorTriplet] = None`  _class-attribute, instance-attribute_
  A triplet of color components, if an RGB color.
- `type: ColorType`  _instance-attribute_
  The type of the color.

Terminal color definition.


## ColorParseError

`rich.color.ColorParseError`

```python
class ColorParseError(Exception)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

The color could not be parsed.


## ColorSystem

`rich.color.ColorSystem`

```python
class ColorSystem(IntEnum)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `IntEnum`

**Declared members (4)**

- `EIGHT_BIT = 2`  _class-attribute, instance-attribute_
- `STANDARD = 1`  _class-attribute, instance-attribute_
- `TRUECOLOR = 3`  _class-attribute, instance-attribute_
- `WINDOWS = 4`  _class-attribute, instance-attribute_

One of the 3 color system supported by terminals.


## ColorType

`rich.color.ColorType`

```python
class ColorType(IntEnum)
```

**Bases** `IntEnum`

**Declared members (5)**

- `DEFAULT = 0`  _class-attribute, instance-attribute_
- `EIGHT_BIT = 2`  _class-attribute, instance-attribute_
- `STANDARD = 1`  _class-attribute, instance-attribute_
- `TRUECOLOR = 3`  _class-attribute, instance-attribute_
- `WINDOWS = 4`  _class-attribute, instance-attribute_

Type of color stored in Color class.


## blend_rgb

`rich.color.blend_rgb`

```python
def blend_rgb(color1: ColorTriplet, color2: ColorTriplet, cross_fade: float = 0.5) -> ColorTriplet
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Blend one RGB color in to another.


## parse_rgb_hex

`rich.color.parse_rgb_hex`

```python
def parse_rgb_hex(hex_color: str) -> ColorTriplet
```

Parse six hex characters in to RGB triplet.


