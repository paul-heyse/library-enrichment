//! `codesearch-model` -- the canonical schema and the DDL that creates it.
//!
//! Two rules govern everything here, and both come from a probe rather than from taste:
//!
//! 1. **Every column type is a fixed point of the Delta conversion** (PB03). Delta narrows types
//!    silently rather than rejecting them, so the schema is authored in the narrowed vocabulary
//!    and [`ddl::assert_fixed_point`] proves it on a round trip.
//! 2. **Constraints are added at table creation, never afterwards** (PB13). A write followed by a
//!    failing constraint-add leaves the violating row committed *and* no constraint recorded; the
//!    reverse order rejects the write and leaves the table consistent with its own metadata.

pub mod ddl;
pub mod index_spec;
pub mod schema;
