# `object_store::aws::credential::AwsCredential`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.credential.AwsCredential.json).

<a id="op-320bb29fbf19dc07e494390f"></a>
## AwsCredential

`struct` · `object_store::aws::credential::AwsCredential` · object_store 0.13.2

```rust
struct AwsCredential
```

Source: `src/aws/credential.rs:71`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A set of AWS security credentials

<a id="op-0d56b61b0d1e9ffa2d26cbdf"></a>
## eq

`function` · `object_store::aws::credential::AwsCredential::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &AwsCredential) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::credential::AwsCredential", "path": "AwsCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 14], "end": [70, 23], "filename": "src/aws/credential.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aws/credential.rs:70`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55d79bd6201cf115adf42c26"></a>
## fmt

`function` · `object_store::aws::credential::AwsCredential::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::credential::AwsCredential", "path": "AwsCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [88, 2], "filename": "src/aws/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aws/credential.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c209b4dd038e777e450985de"></a>
## key_id

`struct_field` · `object_store::aws::credential::AwsCredential::key_id` · object_store 0.13.2

```rust
key_id: String
```

Source: `src/aws/credential.rs:73`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

AWS_ACCESS_KEY_ID

<a id="op-70b9c681e52e8c3a6c2ca42a"></a>
## secret_key

`struct_field` · `object_store::aws::credential::AwsCredential::secret_key` · object_store 0.13.2

```rust
secret_key: String
```

Source: `src/aws/credential.rs:75`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

AWS_SECRET_ACCESS_KEY

<a id="op-3e69ed27db0b8b764ef73179"></a>
## token

`struct_field` · `object_store::aws::credential::AwsCredential::token` · object_store 0.13.2

```rust
token: Option<String>
```

Source: `src/aws/credential.rs:77`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

AWS_SESSION_TOKEN
