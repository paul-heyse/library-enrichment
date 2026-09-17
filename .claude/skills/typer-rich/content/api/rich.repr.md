# `rich.repr`

Distribution: `rich`

## Result

`rich.repr.Result`

```python
Result = Iterable[Union[Any, Tuple[Any], Tuple[str, Any], Tuple[str, Any, Any]]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'Iterable[Any | tuple[Any] | tuple[str, Any] | tuple[str, Any, Any]]'> ````

**Also exported as** `rich.repr.RichReprResult`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## RichReprResult

`rich.repr.RichReprResult`

```python
RichReprResult = Result
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'Iterable[Any | tuple[Any] | tuple[str, Any] | tuple[str, Any, Any]]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## T

`rich.repr.T`

```python
T = TypeVar('T')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## console

`rich.repr.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## foo

`rich.repr.foo`

```python
foo = Foo()
```

**Inferred type** (`ty`, not declared in the source): `Foo`

## Foo

`rich.repr.Foo`

```python
class Foo
```

## ReprError

`rich.repr.ReprError`

```python
class ReprError(Exception)
```

**Bases** `Exception`

An error occurred when attempting to build a repr.


## auto

`rich.repr.auto`

```python
def auto(cls: Optional[Type[T]] = None, angular: Optional[bool] = None) -> Union[Type[T], Callable[[Type[T]], Type[T]]]
```

**Overloads** (the signature above is the runtime dispatcher):

- `def auto(cls: Optional[Type[T]]) -> Type[T]`
- `def auto(angular: bool = False) -> Callable[[Type[T]], Type[T]]`

Class decorator to create __repr__ from __rich_repr__


## rich_repr

`rich.repr.rich_repr`

```python
def rich_repr(cls: Optional[Type[T]] = None, angular: bool = False) -> Union[Type[T], Callable[[Type[T]], Type[T]]]
```

**Overloads** (the signature above is the runtime dispatcher):

- `def rich_repr(cls: Optional[Type[T]]) -> Type[T]`
- `def rich_repr(angular: bool = False) -> Callable[[Type[T]], Type[T]]`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

