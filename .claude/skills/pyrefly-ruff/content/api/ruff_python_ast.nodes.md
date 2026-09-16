# `ruff_python_ast::nodes`

Crate `ruff_python_ast` · 63 public items · structured records in [`model/ruff_python_ast.nodes.json`](../model/ruff_python_ast.nodes.json)

## AnyParameterRef

`enum` · `ruff_python_ast::nodes::AnyParameterRef`

Also reachable as `ruff_python_ast::AnyParameterRef`

```rust
enum AnyParameterRef<'a>
```

**Variants**: `Variadic`, `NonVariadic`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn annotation(self) -> Option<&'a Expr>
const fn as_parameter(self) -> &'a Parameter
fn default(self) -> Option<&'a Expr>
const fn is_variadic(self) -> bool
const fn name(self) -> &'a Identifier
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Enumeration of the two kinds of parameter

---

## ArgOrKeyword

`enum` · `ruff_python_ast::nodes::ArgOrKeyword`

Also reachable as `ruff_python_ast::ArgOrKeyword`

```rust
enum ArgOrKeyword<'a>
```

**Variants**: `Arg`, `Keyword`

**Implements**: `core::convert::From`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
const fn as_keyword(self) -> Option<&'a Keyword>
const fn as_variadic(self) -> Option<&'a Keyword>
const fn is_variadic(self) -> bool
const fn value(self) -> &'a Expr
```

**via `core::convert::From`**

```rust
fn from(keyword: &'a Keyword) -> Self
fn from(arg: &'a Expr) -> Self
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

An entry in the argument list of a function call.

---

## BoolOp

`enum` · `ruff_python_ast::nodes::BoolOp`

Also reachable as `ruff_python_ast::BoolOp`

```rust
enum BoolOp
```

**Variants**: `And`, `Or`

**Implements**: `core::fmt::Display`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn as_str(&self) -> &'static str
const fn is_and(&self) -> bool
const fn is_or(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

See also [boolop](https://docs.python.org/3/library/ast.html#ast.BoolOp)

---

## CmpOp

`enum` · `ruff_python_ast::nodes::CmpOp`

Also reachable as `ruff_python_ast::CmpOp`

```rust
enum CmpOp
```

**Variants**: `Eq`, `NotEq`, `Lt`, `LtE`, `Gt`, `GtE`, `Is`, `IsNot`, `In`, `NotIn`

**Implements**: `core::fmt::Display`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
const fn as_str(&self) -> &'static str
const fn is_eq(&self) -> bool
const fn is_gt(&self) -> bool
const fn is_gt_e(&self) -> bool
const fn is_in(&self) -> bool
const fn is_is(&self) -> bool
const fn is_is_not(&self) -> bool
const fn is_lt(&self) -> bool
const fn is_lt_e(&self) -> bool
const fn is_not_eq(&self) -> bool
const fn is_not_in(&self) -> bool
const fn negate(&self) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

See also [cmpop](https://docs.python.org/3/library/ast.html#ast.cmpop)

---

## ConversionFlag

`enum` · `ruff_python_ast::nodes::ConversionFlag`

Also reachable as `ruff_python_ast::ConversionFlag`

```rust
enum ConversionFlag
```

**Variants**: `None`, `Str`, `Ascii`, `Repr`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
const fn is_ascii(&self) -> bool
const fn is_none(&self) -> bool
const fn is_repr(&self) -> bool
const fn is_str(&self) -> bool
fn to_byte(&self) -> Option<u8>
fn to_char(&self) -> Option<char>
```

Transforms a value prior to formatting it.

---

## ExprContext

`enum` · `ruff_python_ast::nodes::ExprContext`

Also reachable as `ruff_python_ast::ExprContext`

```rust
enum ExprContext
```

**Variants**: `Load`, `Store`, `Del`, `Invalid`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
const fn is_del(&self) -> bool
const fn is_invalid(&self) -> bool
const fn is_load(&self) -> bool
const fn is_store(&self) -> bool
```

See also [expr_context](https://docs.python.org/3/library/ast.html#ast.expr_context)

---

## FStringPart

`enum` · `ruff_python_ast::nodes::FStringPart`

Also reachable as `ruff_python_ast::FStringPart`

```rust
enum FStringPart
```

**Variants**: `Literal`, `FString`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn as_f_string(&self) -> Option<&FString>
fn as_literal(&self) -> Option<&StringLiteral>
fn as_mut_f_string(&mut self) -> Option<&mut FString>
fn as_mut_literal(&mut self) -> Option<&mut StringLiteral>
fn expect_f_string(self) -> FString where Self: ::std::fmt::Debug
fn expect_literal(self) -> StringLiteral where Self: ::std::fmt::Debug
fn f_string(self) -> Option<FString>
fn is_empty_literal(&self) -> bool
const fn is_f_string(&self) -> bool
const fn is_literal(&self) -> bool
fn literal(self) -> Option<StringLiteral>
fn quote_style(&self) -> Quote
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

An f-string part which is either a string literal or an f-string.

---

## IpyEscapeKind

`enum` · `ruff_python_ast::nodes::IpyEscapeKind`

Also reachable as `ruff_python_ast::IpyEscapeKind`

```rust
enum IpyEscapeKind
```

**Variants**: `Shell`, `ShCap`, `Help`, `Help2`, `Magic`, `Magic2`, `Quote`, `Quote2`, `Paren`

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn as_str(self) -> &'static str
const fn is_help(self) -> bool
const fn is_magic(self) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(ch: char) -> Result<Self, Self::Error>
fn try_from(ch: [char; 2]) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

The kind of escape command as defined in [IPython Syntax] in the IPython codebase.

[IPython Syntax]: https://github.com/ipython/ipython/blob/635815e8f1ded5b764d66cacc80bbe25e9e2587f/IPython/core/inputtransformer2.py#L335-L343

---

## IrrefutablePatternKind

`enum` · `ruff_python_ast::nodes::IrrefutablePatternKind`

Also reachable as `ruff_python_ast::IrrefutablePatternKind`

```rust
enum IrrefutablePatternKind
```

**Variants**: `Name`, `Wildcard`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## Number

`enum` · `ruff_python_ast::nodes::Number`

Also reachable as `ruff_python_ast::Number`

```rust
enum Number
```

**Variants**: `Int`, `Float`, `Complex`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn as_float(&self) -> Option<&f64>
fn as_int(&self) -> Option<&int::Int>
fn as_mut_float(&mut self) -> Option<&mut f64>
fn as_mut_int(&mut self) -> Option<&mut int::Int>
fn expect_float(self) -> f64 where Self: ::std::fmt::Debug
fn expect_int(self) -> int::Int where Self: ::std::fmt::Debug
fn float(self) -> Option<f64>
fn int(self) -> Option<int::Int>
const fn is_complex(&self) -> bool
const fn is_float(&self) -> bool
const fn is_int(&self) -> bool
```

---

## Operator

`enum` · `ruff_python_ast::nodes::Operator`

Also reachable as `ruff_python_ast::Operator`

```rust
enum Operator
```

**Variants**: `Add`, `Sub`, `Mult`, `MatMult`, `Div`, `Mod`, `Pow`, `LShift`, `RShift`, `BitOr`, `BitXor`, `BitAnd`, `FloorDiv`

**Implements**: `core::fmt::Display`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (17)

```rust
const fn as_str(&self) -> &'static str
const fn dunder(self) -> &'static str
const fn in_place_dunder(self) -> &'static str
const fn is_add(&self) -> bool
const fn is_bit_and(&self) -> bool
const fn is_bit_or(&self) -> bool
const fn is_bit_xor(&self) -> bool
const fn is_div(&self) -> bool
const fn is_floor_div(&self) -> bool
const fn is_l_shift(&self) -> bool
const fn is_mat_mult(&self) -> bool
const fn is_mod(&self) -> bool
const fn is_mult(&self) -> bool
const fn is_pow(&self) -> bool
const fn is_r_shift(&self) -> bool
const fn is_sub(&self) -> bool
const fn reflected_dunder(self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

See also [operator](https://docs.python.org/3/library/ast.html#ast.operator)

---

## PatternOrKeyword

`enum` · `ruff_python_ast::nodes::PatternOrKeyword`

Also reachable as `ruff_python_ast::PatternOrKeyword`

```rust
enum PatternOrKeyword<'a>
```

**Variants**: `Pattern`, `Keyword`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

An entry in the argument list of a class pattern.

---

## Singleton

`enum` · `ruff_python_ast::nodes::Singleton`

Also reachable as `ruff_python_ast::Singleton`

```rust
enum Singleton
```

**Variants**: `None`, `True`, `False`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: bool) -> Self
```

---

## UnaryOp

`enum` · `ruff_python_ast::nodes::UnaryOp`

Also reachable as `ruff_python_ast::UnaryOp`

```rust
enum UnaryOp
```

**Variants**: `Invert`, `Not`, `UAdd`, `USub`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
const fn as_str(&self) -> &'static str
const fn is_invert(&self) -> bool
const fn is_not(&self) -> bool
const fn is_u_add(&self) -> bool
const fn is_u_sub(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

See also [unaryop](https://docs.python.org/3/library/ast.html#ast.unaryop)

---

## Alias

`struct` · `ruff_python_ast::nodes::Alias`

Also reachable as `ruff_python_ast::Alias`

```rust
struct Alias
```

**Fields**: `range`, `node_index`, `name`, `asname`

**Implements**: `ruff_python_ast::identifier::Identifier`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::identifier::Identifier`**

```rust
fn identifier(&self) -> TextRange
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [alias](https://docs.python.org/3/library/ast.html#ast.alias)

---

## AnyStringFlags

`struct` · `ruff_python_ast::nodes::AnyStringFlags`

Also reachable as `ruff_python_ast::AnyStringFlags`

```rust
struct AnyStringFlags
```

**Implements**: `core::convert::From`, `ruff_python_ast::nodes::StringFlags`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
const fn is_byte_string(self) -> bool
const fn is_interpolated_string(self) -> bool
const fn is_raw_string(self) -> bool
const fn is_u_string(self) -> bool
fn new(prefix: AnyStringPrefix, quotes: Quote, triple_quotes: TripleQuotes) -> Self
fn with_prefix(self, prefix: AnyStringPrefix) -> Self
fn with_quote_style(self, quotes: Quote) -> Self
fn with_triple_quotes(self, triple_quotes: TripleQuotes) -> Self
fn with_unclosed(self, unclosed: bool) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: BytesLiteralFlags) -> Self
fn from(value: StringLiteralFlags) -> Self
fn from(value: FStringFlags) -> Self
fn from(value: TStringFlags) -> Self
```

**via `ruff_python_ast::nodes::StringFlags`**

```rust
fn is_unclosed(self) -> bool
fn prefix(self) -> AnyStringPrefix
fn quote_style(self) -> Quote
fn triple_quotes(self) -> TripleQuotes
```

---

## Arguments

`struct` · `ruff_python_ast::nodes::Arguments`

Also reachable as `ruff_python_ast::Arguments`

```rust
struct Arguments
```

**Fields**: `range`, `node_index`, `args`, `keywords`

**Implements**: `pyrefly_util::display::DisplayWith`, `pyrefly_util::visit::Visit`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn find_argument(&self, name: &str, position: usize) -> Option<ArgOrKeyword<'_>>
fn find_argument_value(&self, name: &str, position: usize) -> Option<&Expr>
fn find_keyword(&self, keyword_name: &str) -> Option<&Keyword>
fn find_positional(&self, position: usize) -> Option<&Expr>
fn inner_range(&self) -> TextRange
fn is_empty(&self) -> bool
fn iter_source_order(&self) -> ArgumentsSourceOrder<'_>
fn l_paren_range(&self) -> TextRange
fn len(&self) -> usize
fn r_paren_range(&self) -> TextRange
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node used to represent the arguments passed to a function call or class definition.

For example, given:
```python
foo(1, 2, 3, bar=4, baz=5)
```
The `Arguments` node would span from the left to right parentheses (inclusive), and contain
the arguments and keyword arguments in the order they appear in the source code.

Similarly, given:
```python
class Foo(Bar, baz=1, qux=2):
    pass
```
The `Arguments` node would again span from the left to right parentheses (inclusive), and
contain the `Bar` argument and the `baz` and `qux` keyword arguments in the order they
appear in the source code.

In the context of a class definition, the Python-style AST refers to the arguments as `bases`,
as they represent the "explicitly specified base classes", while the keyword arguments are
typically used for `metaclass`, with any additional arguments being passed to the `metaclass`.

---

## ArgumentsSourceOrder

`struct` · `ruff_python_ast::nodes::ArgumentsSourceOrder`

Also reachable as `ruff_python_ast::ArgumentsSourceOrder`

```rust
struct ArgumentsSourceOrder<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

The iterator returned by [`Arguments::iter_source_order`].

---

## BytesLiteral

`struct` · `ruff_python_ast::nodes::BytesLiteral`

Also reachable as `ruff_python_ast::BytesLiteral`

```rust
struct BytesLiteral
```

**Fields**: `range`, `node_index`, `value`, `flags`

**Implements**: `core::ops::deref::Deref`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn as_slice(&self) -> &[u8]
fn content_range(&self) -> TextRange
fn invalid(range: TextRange) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node that represents a single bytes literal which is part of an
[`ExprBytesLiteral`].

---

## BytesLiteralFlags

`struct` · `ruff_python_ast::nodes::BytesLiteralFlags`

Also reachable as `ruff_python_ast::BytesLiteralFlags`

```rust
struct BytesLiteralFlags
```

**Implements**: `core::convert::From`, `ruff_python_ast::nodes::StringFlags`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn empty() -> Self
const fn is_invalid(self) -> bool
const fn prefix(self) -> ByteStringPrefix
fn with_invalid(self) -> Self
fn with_prefix(self, prefix: ByteStringPrefix) -> Self
fn with_quote_style(self, quote_style: Quote) -> Self
fn with_triple_quotes(self, triple_quotes: TripleQuotes) -> Self
fn with_unclosed(self, unclosed: bool) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: AnyStringFlags) -> BytesLiteralFlags
```

**via `ruff_python_ast::nodes::StringFlags`**

```rust
fn is_unclosed(self) -> bool
fn prefix(self) -> AnyStringPrefix
fn quote_style(self) -> Quote
fn triple_quotes(self) -> TripleQuotes
```

Flags that can be queried to obtain information
regarding the prefixes and quotes used for a bytes literal.

## Notes on usage

If you're using a `Generator` from the `ruff_python_codegen` crate to generate a lint-rule fix
from an existing bytes literal, consider passing along the [`BytesLiteral::flags`] field. If
you don't have an existing literal but have a `Checker` from the `ruff_linter` crate available,
consider using `Checker::default_bytes_flags` to create instances of this struct; this method
will properly handle surrounding f-strings. For usage that doesn't fit into one of these
categories, the public constructor [`BytesLiteralFlags::empty`] can be used.

---

## BytesLiteralValue

`struct` · `ruff_python_ast::nodes::BytesLiteralValue`

Also reachable as `ruff_python_ast::BytesLiteralValue`

```rust
struct BytesLiteralValue
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn as_slice(&self) -> &[BytesLiteral]
fn bytes(&self) -> impl Iterator<Item = u8> + '_
fn concatenated(values: Vec<BytesLiteral>) -> Self
fn is_empty(&self) -> bool
const fn is_implicit_concatenated(&self) -> bool
fn iter(&self) -> Iter<'_, BytesLiteral>
fn iter_mut(&mut self) -> IterMut<'_, BytesLiteral>
fn len(&self) -> usize
fn single(value: BytesLiteral) -> Self
```

The value representing a [`ExprBytesLiteral`].

---

## Comprehension

`struct` · `ruff_python_ast::nodes::Comprehension`

Also reachable as `ruff_python_ast::Comprehension`

```rust
struct Comprehension
```

**Fields**: `range`, `node_index`, `target`, `iter`, `ifs`, `is_async`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [comprehension](https://docs.python.org/3/library/ast.html#ast.comprehension)

---

## DebugText

`struct` · `ruff_python_ast::nodes::DebugText`

Also reachable as `ruff_python_ast::DebugText`

```rust
struct DebugText
```

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn as_str(&self) -> &str
fn expression(&self) -> &str
fn leading(&self) -> &str
fn new(leading: &str, expression: &str, trailing: &str) -> Self
fn trailing(&self) -> &str
```

The debug text of a self-documenting f-string expression (e.g., `f"{x=}"`).

Stores the concatenation of leading text, expression source, and trailing text as a single
[`CompactString`], with byte offsets to split them. The offsets are needed because the leading
and trailing portions can contain non-whitespace characters (grouping parentheses, comments in
triple-quoted f-strings) that cannot be distinguished from expression content by scanning.

[`CompactString`]: compact_str::CompactString

---

## Decorator

`struct` · `ruff_python_ast::nodes::Decorator`

Also reachable as `pyrefly::query::TypeQueryDecorator`, `ruff_python_ast::Decorator`

```rust
struct Decorator
```

**Fields**: `range`, `node_index`, `expression`

**Implements**: `pyrefly_util::display::DisplayWith`, `pyrefly_util::visit::Visit`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [decorator](https://docs.python.org/3/library/ast.html#ast.decorator)

---

## DictItem

`struct` · `ruff_python_ast::nodes::DictItem`

Also reachable as `ruff_python_ast::DictItem`

```rust
struct DictItem
```

**Fields**: `key`, `value`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Represents an item in a [dictionary literal display][1].

Consider the following Python dictionary literal:
```python
{key1: value1, **other_dictionary}
```

In our AST, this would be represented using an `ExprDict` node containing
two `DictItem` nodes inside it:
```ignore
[
    DictItem {
        key: Some(Expr::Name(ExprName { id: "key1" })),
        value: Expr::Name(ExprName { id: "value1" }),
    },
    DictItem {
        key: None,
        value: Expr::Name(ExprName { id: "other_dictionary" }),
    }
]
```

[1]: https://docs.python.org/3/reference/expressions.html#displays-for-lists-sets-and-dictionaries

---

## DictKeyIterator

`struct` · `ruff_python_ast::nodes::DictKeyIterator`

Also reachable as `ruff_python_ast::DictKeyIterator`

```rust
struct DictKeyIterator<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn is_empty(&self) -> bool
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn last(self) -> Option<Self::Item>
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

---

## DictValueIterator

`struct` · `ruff_python_ast::nodes::DictValueIterator`

Also reachable as `ruff_python_ast::DictValueIterator`

```rust
struct DictValueIterator<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn is_empty(&self) -> bool
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn last(self) -> Option<Self::Item>
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

---

## DisplayFlags

`struct` · `ruff_python_ast::nodes::DisplayFlags`

Also reachable as `ruff_python_ast::DisplayFlags`

```rust
struct DisplayFlags<'a>
```

**Implements**: `core::fmt::Display`

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## ElifElseClause

`struct` · `ruff_python_ast::nodes::ElifElseClause`

Also reachable as `ruff_python_ast::ElifElseClause`

```rust
struct ElifElseClause
```

**Fields**: `range`, `node_index`, `test`, `body`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

---

## ExceptHandlerExceptHandler

`struct` · `ruff_python_ast::nodes::ExceptHandlerExceptHandler`

Also reachable as `ruff_python_ast::ExceptHandlerExceptHandler`

```rust
struct ExceptHandlerExceptHandler
```

**Fields**: `range`, `node_index`, `type_`, `name`, `body`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [ExceptHandler](https://docs.python.org/3/library/ast.html#ast.ExceptHandler)

---

## FString

`struct` · `ruff_python_ast::nodes::FString`

Also reachable as `ruff_python_ast::FString`

```rust
struct FString
```

**Fields**: `range`, `node_index`, `elements`, `flags`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node that represents a single f-string which is part of an [`ExprFString`].

---

## FStringFlags

`struct` · `ruff_python_ast::nodes::FStringFlags`

Also reachable as `ruff_python_ast::FStringFlags`

```rust
struct FStringFlags
```

**Implements**: `core::convert::From`, `ruff_python_ast::nodes::StringFlags`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn empty() -> Self
const fn prefix(self) -> FStringPrefix
fn with_prefix(self, prefix: FStringPrefix) -> Self
fn with_quote_style(self, quote_style: Quote) -> Self
fn with_triple_quotes(self, triple_quotes: TripleQuotes) -> Self
fn with_unclosed(self, unclosed: bool) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: AnyStringFlags) -> FStringFlags
```

**via `ruff_python_ast::nodes::StringFlags`**

```rust
fn is_unclosed(self) -> bool
fn prefix(self) -> AnyStringPrefix
fn quote_style(self) -> Quote
fn triple_quotes(self) -> TripleQuotes
```

Flags that can be queried to obtain information
regarding the prefixes and quotes used for an f-string.

Note: This is identical to [`TStringFlags`] except that
the implementation of the `prefix` method of the
[`StringFlags`] trait returns a variant of
`AnyStringPrefix::Format`.

## Notes on usage

If you're using a `Generator` from the `ruff_python_codegen` crate to generate a lint-rule fix
from an existing f-string literal, consider passing along the [`FString::flags`] field. If you
don't have an existing literal but have a `Checker` from the `ruff_linter` crate available,
consider using `Checker::default_fstring_flags` to create instances of this struct; this method
will properly handle nested f-strings. For usage that doesn't fit into one of these categories,
the public constructor [`FStringFlags::empty`] can be used.

---

## FStringValue

`struct` · `ruff_python_ast::nodes::FStringValue`

Also reachable as `ruff_python_ast::FStringValue`

```rust
struct FStringValue
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn as_slice(&self) -> &[FStringPart]
fn concatenated(values: Vec<FStringPart>) -> Self
fn elements(&self) -> impl Iterator<Item = &InterpolatedStringElement>
fn f_strings(&self) -> impl Iterator<Item = &FString>
fn is_empty_literal(&self) -> bool
fn is_implicit_concatenated(&self) -> bool
fn iter(&self) -> Iter<'_, FStringPart>
fn iter_mut(&mut self) -> IterMut<'_, FStringPart>
fn literals(&self) -> impl Iterator<Item = &StringLiteral>
fn single(value: FString) -> Self
```

The value representing an [`ExprFString`].

---

## Identifier

`struct` · `ruff_python_ast::nodes::Identifier`

Also reachable as `ruff_python_ast::Identifier`

```rust
struct Identifier
```

**Fields**: `id`, `range`, `node_index`

**Implements**: `core::convert::AsRef`, `core::fmt::Display`, `core::ops::deref::Deref`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn as_str(&self) -> &str
fn id(&self) -> &Name
fn is_valid(&self) -> bool
fn new(id: impl Into<Name>, range: TextRange) -> Self
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An `Identifier` with an empty `id` is invalid.

For example, in the following code `id` will be empty.
```python
def 1():
    ...
```

---

## InterpolatedElement

`struct` · `ruff_python_ast::nodes::InterpolatedElement`

Also reachable as `ruff_python_ast::InterpolatedElement`

```rust
struct InterpolatedElement
```

**Fields**: `range`, `node_index`, `expression`, `debug_text`, `conversion`, `format_spec`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [FormattedValue](https://docs.python.org/3/library/ast.html#ast.FormattedValue)

---

## InterpolatedStringElements

`struct` · `ruff_python_ast::nodes::InterpolatedStringElements`

Also reachable as `ruff_python_ast::InterpolatedStringElements`

```rust
struct InterpolatedStringElements
```

**Implements**: `core::convert::From`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn interpolations(&self) -> impl Iterator<Item = &InterpolatedElement>
fn literals(&self) -> impl Iterator<Item = &InterpolatedStringLiteralElement>
```

**via `core::convert::From`**

```rust
fn from(elements: Vec<InterpolatedStringElement>) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

A newtype wrapper around a list of [`InterpolatedStringElement`].

---

## InterpolatedStringFormatSpec

`struct` · `ruff_python_ast::nodes::InterpolatedStringFormatSpec`

Also reachable as `ruff_python_ast::InterpolatedStringFormatSpec`

```rust
struct InterpolatedStringFormatSpec
```

**Fields**: `range`, `node_index`, `elements`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

---

## InterpolatedStringLiteralElement

`struct` · `ruff_python_ast::nodes::InterpolatedStringLiteralElement`

Also reachable as `ruff_python_ast::InterpolatedStringLiteralElement`

```rust
struct InterpolatedStringLiteralElement
```

**Fields**: `range`, `node_index`, `value`

**Implements**: `core::ops::deref::Deref`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_valid(&self) -> bool
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An `FStringLiteralElement` with an empty `value` is an invalid f-string element.

---

## IrrefutablePattern

`struct` · `ruff_python_ast::nodes::IrrefutablePattern`

Also reachable as `ruff_python_ast::IrrefutablePattern`

```rust
struct IrrefutablePattern
```

**Fields**: `kind`, `range`, `node_index`

---

## Keyword

`struct` · `ruff_python_ast::nodes::Keyword`

Also reachable as `ruff_python_ast::Keyword`

```rust
struct Keyword
```

**Fields**: `range`, `node_index`, `arg`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [keyword](https://docs.python.org/3/library/ast.html#ast.keyword)

---

## MatchCase

`struct` · `ruff_python_ast::nodes::MatchCase`

Also reachable as `ruff_python_ast::MatchCase`

```rust
struct MatchCase
```

**Fields**: `range`, `node_index`, `pattern`, `guard`, `body`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [match_case](https://docs.python.org/3/library/ast.html#ast.match_case)

---

## Parameter

`struct` · `ruff_python_ast::nodes::Parameter`

Also reachable as `ruff_python_ast::Parameter`

```rust
struct Parameter
```

**Fields**: `range`, `node_index`, `name`, `annotation`

**Implements**: `ruff_python_ast::identifier::Identifier`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn annotation(&self) -> Option<&Expr>
const fn name(&self) -> &Identifier
```

**via `ruff_python_ast::identifier::Identifier`**

```rust
fn identifier(&self) -> TextRange
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [arg](https://docs.python.org/3/library/ast.html#ast.arg)

---

## ParameterWithDefault

`struct` · `ruff_python_ast::nodes::ParameterWithDefault`

Also reachable as `ruff_python_ast::ParameterWithDefault`

```rust
struct ParameterWithDefault
```

**Fields**: `range`, `node_index`, `parameter`, `default`

**Implements**: `ruff_python_ast::identifier::Identifier`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn annotation(&self) -> Option<&Expr>
fn default(&self) -> Option<&Expr>
const fn name(&self) -> &Identifier
fn uses_pep_484_positional_only_convention(&self) -> bool
```

**via `ruff_python_ast::identifier::Identifier`**

```rust
fn identifier(&self) -> TextRange
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An alternative type of AST `arg`. This is used for each function argument that might have a default value.
Used by `Arguments` original type.

NOTE: This type is different from original Python AST.

---

## Parameters

`struct` · `ruff_python_ast::nodes::Parameters`

Also reachable as `ruff_python_ast::Parameters`

```rust
struct Parameters
```

**Fields**: `range`, `node_index`, `posonlyargs`, `args`, `vararg`, `kwonlyargs`, `kwarg`

**Implements**: `pyrefly_util::visit::Visit`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn find(&self, name: &str) -> Option<&ParameterWithDefault>
fn includes(&self, name: &str) -> bool
fn index(&self, name: &str) -> Option<usize>
fn is_empty(&self) -> bool
fn iter(&self) -> ParametersIterator<'_>
fn iter_non_variadic_params(&self) -> impl Iterator<Item = &ParameterWithDefault>
fn iter_source_order(&self) -> ParametersSourceOrderIterator<'_>
fn len(&self) -> usize
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An alternative type of AST `arguments`. This is ruff_python_parser-friendly and human-friendly definition of function arguments.
This form also has advantage to implement pre-order traverse.

`defaults` and `kw_defaults` fields are removed and the default values are placed under each [`ParameterWithDefault`] typed argument.
`vararg` and `kwarg` are still typed as `arg` because they never can have a default value.

The original Python-style AST type orders `kwonlyargs` fields by default existence; [Parameters] has location-ordered `kwonlyargs` fields.

NOTE: This type differs from the original Python AST. See: [arguments](https://docs.python.org/3/library/ast.html#ast.arguments).

---

## ParametersIterator

`struct` · `ruff_python_ast::nodes::ParametersIterator`

Also reachable as `ruff_python_ast::ParametersIterator`

```rust
struct ParametersIterator<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn last(self) -> Option<Self::Item>
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

---

## ParametersSourceOrderIterator

`struct` · `ruff_python_ast::nodes::ParametersSourceOrderIterator`

Also reachable as `ruff_python_ast::ParametersSourceOrderIterator`

```rust
struct ParametersSourceOrderIterator<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

The iterator returned by [`Parameters::iter_source_order`].

---

## PatternArguments

`struct` · `ruff_python_ast::nodes::PatternArguments`

Also reachable as `ruff_python_ast::PatternArguments`

```rust
struct PatternArguments
```

**Fields**: `range`, `node_index`, `patterns`, `keywords`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn iter_source_order(&self) -> PatternArgumentsSourceOrder<'_>
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node to represent the arguments to a [`crate::PatternMatchClass`], i.e., the
parenthesized contents in `case Point(1, x=0, y=0)`.

Like [`Arguments`], but for [`crate::PatternMatchClass`].

---

## PatternArgumentsSourceOrder

`struct` · `ruff_python_ast::nodes::PatternArgumentsSourceOrder`

Also reachable as `ruff_python_ast::PatternArgumentsSourceOrder`

```rust
struct PatternArgumentsSourceOrder<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

The iterator returned by [`PatternArguments::iter_source_order`].

---

## PatternKeyword

`struct` · `ruff_python_ast::nodes::PatternKeyword`

Also reachable as `ruff_python_ast::PatternKeyword`

```rust
struct PatternKeyword
```

**Fields**: `range`, `node_index`, `attr`, `pattern`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node to represent the keyword arguments to a [`crate::PatternMatchClass`], i.e., the
`x=0` and `y=0` in `case Point(x=0, y=0)`.

Like [`Keyword`], but for [`crate::PatternMatchClass`].

---

## StringLiteral

`struct` · `ruff_python_ast::nodes::StringLiteral`

Also reachable as `ruff_python_ast::StringLiteral`

```rust
struct StringLiteral
```

**Fields**: `range`, `node_index`, `value`, `flags`

**Implements**: `core::ops::deref::Deref`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn as_str(&self) -> &str
fn content_range(&self) -> TextRange
fn invalid(range: TextRange) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node that represents a single string literal which is part of an
[`ExprStringLiteral`].

---

## StringLiteralFlags

`struct` · `ruff_python_ast::nodes::StringLiteralFlags`

Also reachable as `ruff_python_ast::StringLiteralFlags`

```rust
struct StringLiteralFlags
```

**Implements**: `core::convert::From`, `ruff_python_ast::nodes::StringFlags`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn empty() -> Self
const fn is_invalid(self) -> bool
const fn prefix(self) -> StringLiteralPrefix
fn with_invalid(self) -> Self
fn with_prefix(self, prefix: StringLiteralPrefix) -> Self
fn with_quote_style(self, quote_style: Quote) -> Self
fn with_triple_quotes(self, triple_quotes: TripleQuotes) -> Self
fn with_unclosed(self, unclosed: bool) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: AnyStringFlags) -> StringLiteralFlags
```

**via `ruff_python_ast::nodes::StringFlags`**

```rust
fn is_unclosed(self) -> bool
fn prefix(self) -> AnyStringPrefix
fn quote_style(self) -> Quote
fn triple_quotes(self) -> TripleQuotes
```

Flags that can be queried to obtain information
regarding the prefixes and quotes used for a string literal.

## Notes on usage

If you're using a `Generator` from the `ruff_python_codegen` crate to generate a lint-rule fix
from an existing string literal, consider passing along the [`StringLiteral::flags`] field or
the result of the [`StringLiteralValue::first_literal_flags`] method. If you don't have an
existing string but have a `Checker` from the `ruff_linter` crate available, consider using
`Checker::default_string_flags` to create instances of this struct; this method will properly
handle surrounding f-strings. For usage that doesn't fit into one of these categories, the
public constructor [`StringLiteralFlags::empty`] can be used.

---

## StringLiteralValue

`struct` · `ruff_python_ast::nodes::StringLiteralValue`

Also reachable as `ruff_python_ast::StringLiteralValue`

```rust
struct StringLiteralValue
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn as_slice(&self) -> &[StringLiteral]
fn chars(&self) -> impl Iterator<Item = char> + Clone + '_
fn concatenated(strings: Vec<StringLiteral>) -> Self
fn first_literal_flags(&self) -> StringLiteralFlags
fn is_empty(&self) -> bool
const fn is_implicit_concatenated(&self) -> bool
fn is_unicode(&self) -> bool
fn iter(&self) -> Iter<'_, StringLiteral>
fn iter_mut(&mut self) -> IterMut<'_, StringLiteral>
fn len(&self) -> usize
fn single(string: StringLiteral) -> Self
fn to_str(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

The value representing a [`ExprStringLiteral`].

---

## TString

`struct` · `ruff_python_ast::nodes::TString`

Also reachable as `ruff_python_ast::TString`

```rust
struct TString
```

**Fields**: `range`, `node_index`, `elements`, `flags`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn is_empty(&self) -> bool
fn quote_style(&self) -> Quote
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node that represents a single t-string which is part of an [`ExprTString`].

---

## TStringFlags

`struct` · `ruff_python_ast::nodes::TStringFlags`

Also reachable as `ruff_python_ast::TStringFlags`

```rust
struct TStringFlags
```

**Implements**: `core::convert::From`, `ruff_python_ast::nodes::StringFlags`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn empty() -> Self
const fn prefix(self) -> TStringPrefix
fn with_prefix(self, prefix: TStringPrefix) -> Self
fn with_quote_style(self, quote_style: Quote) -> Self
fn with_triple_quotes(self, triple_quotes: TripleQuotes) -> Self
fn with_unclosed(self, unclosed: bool) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: AnyStringFlags) -> TStringFlags
```

**via `ruff_python_ast::nodes::StringFlags`**

```rust
fn is_unclosed(self) -> bool
fn prefix(self) -> AnyStringPrefix
fn quote_style(self) -> Quote
fn triple_quotes(self) -> TripleQuotes
```

Flags that can be queried to obtain information
regarding the prefixes and quotes used for an f-string.

Note: This is identical to [`FStringFlags`] except that
the implementation of the `prefix` method of the
[`StringFlags`] trait returns a variant of
`AnyStringPrefix::Template`.

## Notes on usage

If you're using a `Generator` from the `ruff_python_codegen` crate to generate a lint-rule fix
from an existing t-string literal, consider passing along the [`FString::flags`] field. If you
don't have an existing literal but have a `Checker` from the `ruff_linter` crate available,
consider using `Checker::default_tstring_flags` to create instances of this struct; this method
will properly handle nested t-strings. For usage that doesn't fit into one of these categories,
the public constructor [`TStringFlags::empty`] can be used.

---

## TStringValue

`struct` · `ruff_python_ast::nodes::TStringValue`

Also reachable as `ruff_python_ast::TStringValue`

```rust
struct TStringValue
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn as_slice(&self) -> &[TString]
fn concatenated(values: Vec<TString>) -> Self
fn elements(&self) -> impl Iterator<Item = &InterpolatedStringElement>
fn is_empty_iterable(&self) -> bool
fn is_implicit_concatenated(&self) -> bool
fn iter(&self) -> Iter<'_, TString>
fn iter_mut(&mut self) -> IterMut<'_, TString>
fn single(value: TString) -> Self
```

The value representing an [`ExprTString`].

---

## TypeParams

`struct` · `ruff_python_ast::nodes::TypeParams`

Also reachable as `ruff_python_ast::TypeParams`

```rust
struct TypeParams
```

**Fields**: `range`, `node_index`, `type_params`

**Implements**: `core::ops::deref::Deref`, `pyrefly_util::visit::Visit`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node used to represent a sequence of type parameters.

For example, given:
```python
class C[T, U, V]: ...
```
The `TypeParams` node would span from the left to right brackets (inclusive), and contain
the `T`, `U`, and `V` type parameters in the order they appear in the source code.

---

## WithItem

`struct` · `ruff_python_ast::nodes::WithItem`

Also reachable as `ruff_python_ast::WithItem`

```rust
struct WithItem
```

**Fields**: `range`, `node_index`, `context_expr`, `optional_vars`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [withitem](https://docs.python.org/3/library/ast.html#ast.withitem)

---

## StringFlags

`trait` · `ruff_python_ast::nodes::StringFlags`

Also reachable as `ruff_python_ast::StringFlags`

```rust
trait StringFlags: Copy
```

**Implementors** (6)

- `ruff_python_ast::nodes::AnyStringFlags`
- `ruff_python_ast::nodes::BytesLiteralFlags`
- `ruff_python_ast::nodes::FStringFlags`
- `ruff_python_ast::nodes::StringLiteralFlags`
- `ruff_python_ast::nodes::TStringFlags`
- `ruff_python_ast::token::TokenFlags`

**Methods** (11)

```rust
fn as_any_string_flags(self) -> AnyStringFlags
fn closer_len(self) -> TextSize
fn display_contents(self, contents: &str) -> DisplayFlags<'_>
fn is_triple_quoted(self) -> bool
fn is_unclosed(self) -> bool
fn opener_len(self) -> TextSize
fn prefix(self) -> AnyStringPrefix
fn quote_len(self) -> TextSize
fn quote_str(self) -> &'static str
fn quote_style(self) -> Quote
fn triple_quotes(self) -> TripleQuotes
```

---

## DecoratorList

`type_alias` · `ruff_python_ast::nodes::DecoratorList`

Also reachable as `ruff_python_ast::DecoratorList`

```rust
type DecoratorList = thin_vec::ThinVec<Decorator>
```

---

## ParameterWithDefaults

`type_alias` · `ruff_python_ast::nodes::ParameterWithDefaults`

Also reachable as `ruff_python_ast::ParameterWithDefaults`

```rust
type ParameterWithDefaults = thin_vec::ThinVec<ParameterWithDefault>
```

---

## PatternKeys

`type_alias` · `ruff_python_ast::nodes::PatternKeys`

Also reachable as `ruff_python_ast::PatternKeys`

```rust
type PatternKeys = thin_vec::ThinVec<Expr>
```

---

## Patterns

`type_alias` · `ruff_python_ast::nodes::Patterns`

Also reachable as `ruff_python_ast::Patterns`

```rust
type Patterns = thin_vec::ThinVec<Pattern>
```

---

## Suite

`type_alias` · `ruff_python_ast::nodes::Suite`

Also reachable as `ruff_python_ast::Suite`

```rust
type Suite = thin_vec::ThinVec<Stmt>
```

**Implements**: `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`

A suite represents a sequence of [`Stmt`].

See: <https://docs.python.org/3/reference/compound_stmts.html#grammar-token-python-grammar-suite>

---
