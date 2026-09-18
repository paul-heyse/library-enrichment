# `arrow_flight::sql::client::PreparedStatement`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.client.PreparedStatement.json).

<a id="op-2dab74e72a9fe695ae8db446"></a>
## PreparedStatement

`struct` · `arrow_flight::sql::client::PreparedStatement` · arrow-flight 59.3.0

```rust
struct PreparedStatement<T>
```

Source: `src/sql/client.rs:481`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

A PreparedStatement

<a id="op-de309c221ebb1b863749aede"></a>
## clone

`function` · `arrow_flight::sql::client::PreparedStatement::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> PreparedStatement<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::sql::client::PreparedStatement", "path": "PreparedStatement"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [480, 17], "end": [480, 22], "filename": "src/sql/client.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/client.rs:480`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6a5cbc4977171dfc6c80cf2"></a>
## close

`function` · `arrow_flight::sql::client::PreparedStatement::close` · arrow-flight 59.3.0

```rust
async fn close(self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::sql::client::PreparedStatement", "path": "PreparedStatement"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [489, 1], "end": [620, 2], "filename": "src/sql/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/client.rs:609`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Close the prepared statement, so that this PreparedStatement can not used
anymore and server can free up any resources.

<a id="op-a61e75ab047414e6c1c8782e"></a>
## dataset_schema

`function` · `arrow_flight::sql::client::PreparedStatement::dataset_schema` · arrow-flight 59.3.0

```rust
fn dataset_schema(&self) -> Result<&Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::sql::client::PreparedStatement", "path": "PreparedStatement"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [489, 1], "end": [620, 2], "filename": "src/sql/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/client.rs:552`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieve the ResultSet schema from the query.

<a id="op-ddddab5bbae9984c51720dab"></a>
## execute

`function` · `arrow_flight::sql::client::PreparedStatement::execute` · arrow-flight 59.3.0

```rust
async fn execute(&mut self) -> Result<FlightInfo>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::sql::client::PreparedStatement", "path": "PreparedStatement"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [489, 1], "end": [620, 2], "filename": "src/sql/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/client.rs:512`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Executes the prepared statement query on the server.

<a id="op-e529e1211b9e497624edfa8f"></a>
## execute_update

`function` · `arrow_flight::sql::client::PreparedStatement::execute_update` · arrow-flight 59.3.0

```rust
async fn execute_update(&mut self) -> Result<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::sql::client::PreparedStatement", "path": "PreparedStatement"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [489, 1], "end": [620, 2], "filename": "src/sql/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/client.rs:527`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Executes the prepared statement update query on the server.

<a id="op-56842990c856086bfce67852"></a>
## fmt

`function` · `arrow_flight::sql::client::PreparedStatement::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::sql::client::PreparedStatement", "path": "PreparedStatement"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [480, 10], "end": [480, 15], "filename": "src/sql/client.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/client.rs:480`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa68d618119791dcacc5b9b4"></a>
## parameter_schema

`function` · `arrow_flight::sql::client::PreparedStatement::parameter_schema` · arrow-flight 59.3.0

```rust
fn parameter_schema(&self) -> Result<&Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::sql::client::PreparedStatement", "path": "PreparedStatement"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [489, 1], "end": [620, 2], "filename": "src/sql/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/client.rs:547`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieve the parameter schema from the query.

<a id="op-e2f510348bb3a01e2acb3cf2"></a>
## set_parameters

`function` · `arrow_flight::sql::client::PreparedStatement::set_parameters` · arrow-flight 59.3.0

```rust
fn set_parameters(&mut self, parameter_binding: RecordBatch) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_flight::sql::client::PreparedStatement", "path": "PreparedStatement"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::body::Body", "path": "tonic::body::Body"}}}], "constraints": []}}, "id": "tonic::client::service::GrpcService", "path": "tonic::client::GrpcService"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "bytes::Bytes"}}}}, "name": "Data"}]}}, "id": "http_body::Body", "path": "Body"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "tonic::codegen::StdError", "path": "tonic::codegen::StdError"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"qualified_path": {"args": null, "name": "ResponseBody", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "tonic::client::service::GrpcService", "path": ""}}}, "trait": {"args": null, "id": "http_body::Body", "path": "Body"}}}}}]}, "is_negative": false, "span": {"begin": [489, 1], "end": [620, 2], "filename": "src/sql/client.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/client.rs:557`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set a RecordBatch that contains the parameters that will be bind.
