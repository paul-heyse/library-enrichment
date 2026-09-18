# `sqlparser::ast::AlterUser`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AlterUser.json).

<a id="op-a2b8fd64d32cb00843b4c024"></a>
## AlterUser

`struct` · `sqlparser::ast::AlterUser` · sqlparser 0.62.0

```rust
struct AlterUser
```

Source: `src/ast/mod.rs:11525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modifies the properties of a user

[Snowflake Syntax:](https://docs.snowflake.com/en/sql-reference/sql/alter-user)
```sql
ALTER USER [ IF EXISTS ] [ <name> ] [ OPTIONS ]
```

[PostgreSQL Syntax:](https://www.postgresql.org/docs/current/sql-alteruser.html)
```sql
ALTER USER <role_specification> [ WITH ] option [ ... ]
```

<a id="op-288aff13cae06ecab1834daf"></a>
## abort_all_queries

`struct_field` · `sqlparser::ast::AlterUser::abort_all_queries` · sqlparser 0.62.0

```rust
abort_all_queries: bool
```

Source: `src/ast/mod.rs:11536`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Abort all running queries for the user.

<a id="op-5a1ce6100cca799cdd60d971"></a>
## add_mfa_method_otp

`struct_field` · `sqlparser::ast::AlterUser::add_mfa_method_otp` · sqlparser 0.62.0

```rust
add_mfa_method_otp: Option<AlterUserAddMfaMethodOtp>
```

Source: `src/ast/mod.rs:11550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Add an MFA OTP method with optional count.

<a id="op-c34c498da7c17b500ba44264"></a>
## add_role_delegation

`struct_field` · `sqlparser::ast::AlterUser::add_role_delegation` · sqlparser 0.62.0

```rust
add_role_delegation: Option<AlterUserAddRoleDelegation>
```

Source: `src/ast/mod.rs:11538`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optionally add a delegated role authorization.

<a id="op-346ca3b498c87aa2d01b5c48"></a>
## clone

`function` · `sqlparser::ast::AlterUser::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterUser
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11522, 17], "end": [11522, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b424ed6bb699428875ec99c"></a>
## cmp

`function` · `sqlparser::ast::AlterUser::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterUser) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11522, 51], "end": [11522, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c84955f2bb4a26063799f7e"></a>
## deserialize

`function` · `sqlparser::ast::AlterUser::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11523, 49], "end": [11523, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5267180ce4657025b5539b9"></a>
## enroll_mfa

`struct_field` · `sqlparser::ast::AlterUser::enroll_mfa` · sqlparser 0.62.0

```rust
enroll_mfa: bool
```

Source: `src/ast/mod.rs:11542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Enroll the user in MFA.

<a id="op-6e78abeefa85216c64c67662"></a>
## eq

`function` · `sqlparser::ast::AlterUser::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterUser) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11522, 24], "end": [11522, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8635cfb6c0e680a400690088"></a>
## fmt

`function` · `sqlparser::ast::AlterUser::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11522, 10], "end": [11522, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7d5acb940aacdb14d9a500a"></a>
## fmt

`function` · `sqlparser::ast::AlterUser::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11676, 1], "end": [11761, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d93c1f99f8b4b9fc11a135a2"></a>
## hash

`function` · `sqlparser::ast::AlterUser::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11522, 56], "end": [11522, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1a80a6eba2b0052db2cc715"></a>
## if_exists

`struct_field` · `sqlparser::ast::AlterUser::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:11527`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to only alter the user if it exists.

<a id="op-4e31da3a91326010ee5298ff"></a>
## modify_mfa_method

`struct_field` · `sqlparser::ast::AlterUser::modify_mfa_method` · sqlparser 0.62.0

```rust
modify_mfa_method: Option<AlterUserModifyMfaMethod>
```

Source: `src/ast/mod.rs:11548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modify an MFA method for the user.

<a id="op-6b3367ea82211d803e2b8cc3"></a>
## name

`struct_field` · `sqlparser::ast::AlterUser::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:11529`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the user to alter.

<a id="op-f989072c3e8c8cee5b766373"></a>
## partial_cmp

`function` · `sqlparser::ast::AlterUser::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterUser) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11522, 35], "end": [11522, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbed7f0c780134df38ccb780"></a>
## password

`struct_field` · `sqlparser::ast::AlterUser::password` · sqlparser 0.62.0

```rust
password: Option<AlterUserPassword>
```

Source: `src/ast/mod.rs:11564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The following options are PostgreSQL-specific: <https://www.postgresql.org/docs/current/sql-alteruser.html>

<a id="op-f681b0df032625c16d3332c0"></a>
## remove_mfa_method

`struct_field` · `sqlparser::ast::AlterUser::remove_mfa_method` · sqlparser 0.62.0

```rust
remove_mfa_method: Option<MfaMethodKind>
```

Source: `src/ast/mod.rs:11546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Remove the user's default MFA method.

<a id="op-d4a41c14adb72a7a77f7126b"></a>
## remove_role_delegation

`struct_field` · `sqlparser::ast::AlterUser::remove_role_delegation` · sqlparser 0.62.0

```rust
remove_role_delegation: Option<AlterUserRemoveRoleDelegation>
```

Source: `src/ast/mod.rs:11540`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optionally remove a delegated role authorization.

<a id="op-747ffe6dcbc1c9c35479a31a"></a>
## rename_to

`struct_field` · `sqlparser::ast::AlterUser::rename_to` · sqlparser 0.62.0

```rust
rename_to: Option<Ident>
```

Source: `src/ast/mod.rs:11532`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional new name for the user (Snowflake-specific).
See: <https://docs.snowflake.com/en/sql-reference/sql/alter-user#syntax>

<a id="op-c65ce8e5d7b8a7229eeec638"></a>
## reset_password

`struct_field` · `sqlparser::ast::AlterUser::reset_password` · sqlparser 0.62.0

```rust
reset_password: bool
```

Source: `src/ast/mod.rs:11534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Reset the user's password.

<a id="op-74993572104d9cc1329eba31"></a>
## serialize

`function` · `sqlparser::ast::AlterUser::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11523, 38], "end": [11523, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7762454cbec8de560664ea91"></a>
## set_default_mfa_method

`struct_field` · `sqlparser::ast::AlterUser::set_default_mfa_method` · sqlparser 0.62.0

```rust
set_default_mfa_method: Option<MfaMethodKind>
```

Source: `src/ast/mod.rs:11544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the default MFA method for the user.

<a id="op-9103f3895eba38a251ab00b9"></a>
## set_policy

`struct_field` · `sqlparser::ast::AlterUser::set_policy` · sqlparser 0.62.0

```rust
set_policy: Option<AlterUserSetPolicy>
```

Source: `src/ast/mod.rs:11552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set a user policy.

<a id="op-f9664278ef553eeb0bc99575"></a>
## set_props

`struct_field` · `sqlparser::ast::AlterUser::set_props` · sqlparser 0.62.0

```rust
set_props: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:11560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Key/value properties to set on the user.

<a id="op-0eea072e924420aca981e30e"></a>
## set_tag

`struct_field` · `sqlparser::ast::AlterUser::set_tag` · sqlparser 0.62.0

```rust
set_tag: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:11556`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Key/value tag options to set on the user.

<a id="op-687002dd7d18e7df8657a488"></a>
## unset_policy

`struct_field` · `sqlparser::ast::AlterUser::unset_policy` · sqlparser 0.62.0

```rust
unset_policy: Option<UserPolicyKind>
```

Source: `src/ast/mod.rs:11554`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unset a user policy.

<a id="op-714d6b3b70d420d61335c2a5"></a>
## unset_props

`struct_field` · `sqlparser::ast::AlterUser::unset_props` · sqlparser 0.62.0

```rust
unset_props: Vec<String>
```

Source: `src/ast/mod.rs:11562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Properties to unset on the user.

<a id="op-ac1a386758f9a12ffd36b1a5"></a>
## unset_tag

`struct_field` · `sqlparser::ast::AlterUser::unset_tag` · sqlparser 0.62.0

```rust
unset_tag: Vec<String>
```

Source: `src/ast/mod.rs:11558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tags to unset on the user.

<a id="op-063326b39b1afb081fb33afc"></a>
## visit

`function` · `sqlparser::ast::AlterUser::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11524, 47], "end": [11524, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b9034b86042fdcf174fc96c"></a>
## visit

`function` · `sqlparser::ast::AlterUser::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11524, 40], "end": [11524, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
