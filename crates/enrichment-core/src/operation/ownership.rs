//! Durable physical ownership shares the native control predecessor with jobs and publication.
use crate::native_union::Rule;

crate::native_struct! { pub struct PreparedCapsule {
    inputs: crate::operation::identities::CapsuleIdentity => Rule::Text,
    environment: crate::identity::Environment => Rule::Text,
    lock: String => Rule::Text,
    inventory: crate::capsule_protocol::inventory::Inventory => Rule::Map,
} }
crate::native_struct! { pub struct RetainedCapsule {
    key: String => Rule::NonEmpty,
    cache: String => Rule::NonEmpty,
    generation: String => Rule::NonEmpty,
    prepared: PreparedCapsule => Rule::Text,
    sequence: u64 => Rule::Text,
} }

crate::native_struct! { pub struct ExecutionRoot {
    root: String => Rule::NonEmpty,
    cache: String => Rule::NonEmpty,
    broker: String => Rule::NonEmpty,
} }
crate::native_vocabulary! { pub enum PhysicalState {
    Reserved = "reserved", Creating = "creating", Settled = "settled", Absent = "absent",
} }
crate::native_struct! { pub struct PhysicalOwner {
    owner_id: crate::identity::PhysicalOwnerId => Rule::Text,
    root: String => Rule::NonEmpty,
    cache: String => Rule::NonEmpty,
    capsule: String => Rule::NonEmpty,
    image: String => Rule::NonEmpty,
    operation_id: crate::identity::ProcessOperationId => Rule::Text,
    authority: crate::execution::ProcessAuthority => Rule::Text,
    created_at: crate::native_time::ObservationTime => Rule::Text,
    state: PhysicalState => Rule::Text,
    creator_boot_id: Option<String> => Rule::Text,
    creator_pid: Option<u32> => Rule::Text,
    sequence: u64 => Rule::Text,
} }
crate::native_union! { pub enum OwnershipObservation {
    Creating = "creating" { boot_id: String => Rule::NonEmpty },
    Creator = "creator" { pid: u32 => Rule::UnsignedRange {min:1,max:u32::MAX as u64} },
    Settled = "settled" { boot_id: String => Rule::NonEmpty },
    Absent = "absent" { boot_id: String => Rule::NonEmpty },
} }
crate::native_struct! { pub struct StorageReservation {
    reservation_id: crate::identity::StorageReservationId => Rule::Text,
    cache: String => Rule::NonEmpty,
    bytes: u64 => Rule::Text,
    released: bool => Rule::Text,
    sequence: u64 => Rule::Text,
} }
