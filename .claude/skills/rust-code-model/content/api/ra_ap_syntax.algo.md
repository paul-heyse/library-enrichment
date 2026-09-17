# `ra_ap_syntax::algo`

Crate `ra_ap_syntax` · 12 public items · structured records in [`model/ra_ap_syntax.algo.json`](../model/ra_ap_syntax.algo.json)

## ancestors_at_offset

`function` · `ra_ap_syntax::algo::ancestors_at_offset`

```rust
fn ancestors_at_offset(node: &SyntaxNode, offset: TextSize) -> impl Iterator<Item = SyntaxNode>
```

Returns ancestors of the node at the offset, sorted by length. This should
do the right thing at an edge, e.g. when searching for expressions at `{
$0foo }` we will get the name reference instead of the whole block, which
we would get if we just did `find_token_at_offset(...).flat_map(|t|
t.parent().ancestors())`.

---

## find_node_at_offset

`function` · `ra_ap_syntax::algo::find_node_at_offset`

```rust
fn find_node_at_offset<N: AstNode>(syntax: &SyntaxNode, offset: TextSize) -> Option<N>
```

Finds a node of specific Ast type at offset. Note that this is slightly
imprecise: if the cursor is strictly between two nodes of the desired type,
as in

```ignore
struct Foo {}|struct Bar;
```

then the shorter node will be silently preferred.

---

## find_node_at_range

`function` · `ra_ap_syntax::algo::find_node_at_range`

```rust
fn find_node_at_range<N: AstNode>(syntax: &SyntaxNode, range: TextRange) -> Option<N>
```

---

## has_errors

`function` · `ra_ap_syntax::algo::has_errors`

```rust
fn has_errors(node: &SyntaxNode) -> bool
```

---

## least_common_ancestor

`function` · `ra_ap_syntax::algo::least_common_ancestor`

```rust
fn least_common_ancestor(u: &SyntaxNode, v: &SyntaxNode) -> Option<SyntaxNode>
```

---

## least_common_ancestor_element

`function` · `ra_ap_syntax::algo::least_common_ancestor_element`

```rust
fn least_common_ancestor_element(u: impl Element, v: impl Element) -> Option<SyntaxNode>
```

---

## neighbor

`function` · `ra_ap_syntax::algo::neighbor`

```rust
fn neighbor<T: AstNode>(me: &T, direction: Direction) -> Option<T>
```

---

## next_non_trivia_token

`function` · `ra_ap_syntax::algo::next_non_trivia_token`

```rust
fn next_non_trivia_token(e: impl Into<SyntaxElement>) -> Option<SyntaxToken>
```

---

## non_trivia_sibling

`function` · `ra_ap_syntax::algo::non_trivia_sibling`

```rust
fn non_trivia_sibling(element: SyntaxElement, direction: Direction) -> Option<SyntaxElement>
```

Finds the first sibling in the given direction which is not `trivia`

---

## previous_non_trivia_token

`function` · `ra_ap_syntax::algo::previous_non_trivia_token`

```rust
fn previous_non_trivia_token(e: impl Into<SyntaxElement>) -> Option<SyntaxToken>
```

---

## skip_trivia_token

`function` · `ra_ap_syntax::algo::skip_trivia_token`

```rust
fn skip_trivia_token(token: SyntaxToken, direction: Direction) -> Option<SyntaxToken>
```

Skip to next non `trivia` token

---

## skip_whitespace_token

`function` · `ra_ap_syntax::algo::skip_whitespace_token`

```rust
fn skip_whitespace_token(token: SyntaxToken, direction: Direction) -> Option<SyntaxToken>
```

Skip to next non `whitespace` token

---
