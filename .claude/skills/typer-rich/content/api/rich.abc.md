# `rich.abc`

Distribution: `rich`

## f

`rich.abc.f`

```python
f = Foo()
```

**Inferred type** (`ty`, not declared in the source): `Foo`

## t

`rich.abc.t`

```python
t = Text()
```

**Inferred type** (`ty`, not declared in the source): `Text`

## Foo

`rich.abc.Foo`

```python
class Foo
```

## RichRenderable

`rich.abc.RichRenderable`

```python
class RichRenderable(ABC)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`

An abstract base class for Rich renderables.

Note that there is no need to extend this class, the intended use is to check if an
object supports the Rich renderable protocol. For example::

    if isinstance(my_object, RichRenderable):
        console.print(my_object)


