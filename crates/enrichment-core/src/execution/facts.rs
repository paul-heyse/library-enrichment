//! Bounded physical qualification receipts. Their eligibility is a native relation.
use crate::{config::ExecutionResources, execution::ProcessObservation};
use arrow::datatypes::Fields;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Fixed production qualification command; callers cannot supply a command or host path.
pub const RESOURCE_PROBE: &str = "set -eu; cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max /sys/fs/cgroup/memory.swap.max /sys/fs/cgroup/pids.max; stat -f -c '%T %b %S' /capsule";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceProbe {
    pub requested: ExecutionResources,
    pub observed: ExecutionResources,
    pub scratch_filesystem: String,
    pub process: ProcessObservation,
}

/// The operator's physical run receipt, decoded without deciding qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub containment_identity: String,
    pub qualified_at: crate::native_time::ObservationTime,
    pub execution_root: String,
    pub images: BTreeMap<String, String>,
    pub tools: BTreeMap<String, BTreeMap<String, String>>,
    pub resources: BTreeMap<String, ResourceProbe>,
}

pub fn resource_fields() -> Fields {
    <ExecutionResources as crate::native_union::NativeStruct>::fields()
}
