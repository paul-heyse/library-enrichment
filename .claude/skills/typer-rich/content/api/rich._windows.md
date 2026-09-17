# `rich._windows`

Distribution: `rich`

## features

`rich._windows.features`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
features = get_windows_console_features()
```

## windll

`rich._windows.windll`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
windll = LibraryLoader(ctypes.WinDLL)
```

## WindowsConsoleFeatures

`rich._windows.WindowsConsoleFeatures`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class WindowsConsoleFeatures
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `truecolor: bool = False`  _class-attribute, instance-attribute_
  The console supports truecolor.
- `vt: bool = False`  _class-attribute, instance-attribute_
  The console supports VT codes.

Windows features available.


## get_windows_console_features

`rich._windows.get_windows_console_features`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_windows_console_features() -> WindowsConsoleFeatures
```

Get windows console features.

Returns:
    WindowsConsoleFeatures: An instance of WindowsConsoleFeatures.


