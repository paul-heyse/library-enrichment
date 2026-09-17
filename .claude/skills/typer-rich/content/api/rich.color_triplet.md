# `rich.color_triplet`

Distribution: `rich`

## ColorTriplet

`rich.color_triplet.ColorTriplet`

```python
class ColorTriplet(NamedTuple)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NamedTuple`

**Declared members (6)**

- `blue: int`  _instance-attribute_
  Blue component in 0 to 255 range.
- `green: int`  _instance-attribute_
  Green component in 0 to 255 range.
- `hex: str`  _property_
  get the color triplet in CSS style.
- `normalized: Tuple[float, float, float]`  _property_
  Convert components into floats between 0 and 1.
- `red: int`  _instance-attribute_
  Red component in 0 to 255 range.
- `rgb: str`  _property_
  The color in RGB format.

The red, green, and blue components of a color.


