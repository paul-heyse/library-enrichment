# `sqlparser::ast::comments::Comments`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.comments.Comments.json).

<a id="op-0538140f8659f8173fe0287c"></a>
## Comments

`struct` · `sqlparser::ast::comments::Comments` · sqlparser 0.62.0

```rust
struct Comments
```

Source: `src/ast/comments.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An opaque container for comments from a parse SQL source code.

<a id="op-f1abbb2d3390dde8faaeec31"></a>
## clone

`function` · `sqlparser::ast::comments::Comments::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Comments
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comments", "path": "Comments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 26], "end": [28, 31], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/comments.rs:28`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e928c50037451fc1ee693db5"></a>
## default

`function` · `sqlparser::ast::comments::Comments::default` · sqlparser 0.62.0

```rust
fn default() -> Comments
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comments", "path": "Comments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 10], "end": [28, 17], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ast/comments.rs:28`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2e9e65fb072221f78752d7f"></a>
## find

`function` · `sqlparser::ast::comments::Comments::find` · sqlparser 0.62.0

```rust
fn find<R: RangeBounds<Location>>(&self, range: R) -> Iter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comments", "path": "Comments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 1], "end": [145, 2], "filename": "src/ast/comments.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/comments.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Finds comments starting within the given location range. The order of
iterator reflects the order of the comments as encountered in the parsed
source code.

# Example
```rust
use sqlparser::{dialect::GenericDialect, parser::Parser, tokenizer::Location};

let sql = r#"/*
 header comment ...
 ... spanning multiple lines
*/

 -- first statement
 SELECT 'hello' /* world */ FROM DUAL;

 -- second statement
 SELECT 123 FROM DUAL;

 -- trailing comment
"#;

let (ast, comments) = Parser::parse_sql_with_comments(&GenericDialect, sql).unwrap();

// all comments appearing before line seven, i.e. before the first statement itself
assert_eq!(
   &comments.find(..Location::new(7, 1)).map(|c| c.as_str()).collect::<Vec<_>>(),
   &["\n header comment ...\n ... spanning multiple lines\n", " first statement\n"]);

// all comments appearing within the first statement
assert_eq!(
   &comments.find(Location::new(7, 1)..Location::new(8,1)).map(|c| c.as_str()).collect::<Vec<_>>(),
   &[" world "]);

// all comments appearing within or after the first statement
assert_eq!(
   &comments.find(Location::new(7, 1)..).map(|c| c.as_str()).collect::<Vec<_>>(),
   &[" world ", " second statement\n", " trailing comment\n"]);
```

The [Spanned](crate::ast::Spanned) trait allows you to access location
information for certain AST nodes.

<a id="op-f2a8da6f762ebaf5d1822a8c"></a>
## fmt

`function` · `sqlparser::ast::comments::Comments::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::comments::Comments", "path": "Comments"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 19], "end": [28, 24], "filename": "src/ast/comments.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/comments.rs:28`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
