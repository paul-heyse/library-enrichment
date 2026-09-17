# `rich.filesize`

Distribution: `rich`

## __all__

`rich.filesize.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['decimal']
```

## _to_str

`rich.filesize._to_str`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _to_str(size: int, suffixes: Iterable[str], base: int, precision: Optional[int] = 1, separator: Optional[str] = ' ') -> str
```

## decimal

`rich.filesize.decimal`

```python
def decimal(size: int, precision: Optional[int] = 1, separator: Optional[str] = ' ') -> str
```

Convert a filesize in to a string (powers of 1000, SI prefixes).

In this convention, ``1000 B = 1 kB``.

This is typically the format used to advertise the storage
capacity of USB flash drives and the like (*256 MB* meaning
actually a storage capacity of more than *256 000 000 B*),
or used by **Mac OS X** since v10.6 to report file sizes.

Arguments:
    int (size): A file size.
    int (precision): The number of decimal places to include (default = 1).
    str (separator): The string to separate the value from the units (default = " ").

Returns:
    `str`: A string containing a abbreviated file size and units.

Example:
    >>> filesize.decimal(30000)
    '30.0 kB'
    >>> filesize.decimal(30000, precision=2, separator="")
    '30.00kB'


## pick_unit_and_suffix

`rich.filesize.pick_unit_and_suffix`

```python
def pick_unit_and_suffix(size: int, suffixes: List[str], base: int) -> Tuple[int, str]
```

Pick a suffix and base for the given size.


