# Protocol, features and constraints

The protocol version and feature set are the compatibility contract. A client that does not implement a required writer feature must refuse to write rather than write something another reader will misinterpret. Adding a feature is therefore a one-way door: it can lock out older clients, which is the point.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `buoyant_kernel::actions::Protocol` | struct | 12 | [prose](../api/buoyant_kernel.actions.md#protocol) | [records](../model/buoyant_kernel.actions.json) |
| `buoyant_kernel::table_features::TableFeature` | enum | 14 | [prose](../api/buoyant_kernel.table_features.md#tablefeature) | [records](../model/buoyant_kernel.table_features.json) |
| `deltalake_core::kernel::models::actions::TableFeatures` | enum | 10 | [prose](../api/deltalake_core.kernel.models.actions.md#tablefeatures) | [records](../model/deltalake_core.kernel.models.actions.json) |
| `deltalake_core::operations::constraints::ConstraintBuilder` | struct | 8 | [prose](../api/deltalake_core.operations.constraints.md#constraintbuilder) | [records](../model/deltalake_core.operations.constraints.json) |
| `deltalake_core::operations::add_feature::AddTableFeatureBuilder` | struct | 8 | [prose](../api/deltalake_core.operations.add_feature.md#addtablefeaturebuilder) | [records](../model/deltalake_core.operations.add_feature.json) |
| `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder` | struct | 6 | [prose](../api/deltalake_core.operations.drop_column_not_null.md#dropcolumnnotnullbuilder) | [records](../model/deltalake_core.operations.drop_column_not_null.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::kernel::schema::DataCheck` | 3 | 0 | 3 | [DataCheck](../traits/DataCheck.md) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Add a table feature deliberately: it raises the protocol version and can make the table unreadable to older clients.
- Constraints are enforced on write by this library; they are not enforced retroactively over existing data.

## Anti-patterns

- Enabling a feature to silence an error without checking which readers still need the table.
- Assuming a constraint validates the rows already in the table.

## Agent checklist

- Which clients read this table, and do they support the features it requires?
- Are constraints added before the data they are meant to guard?
