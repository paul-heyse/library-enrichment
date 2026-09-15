//! Actual kernel limits, read through the production capsule during image qualification.
use enrichment_core::{
    config::ExecutionResources,
    execution::{ProcessEnd, ProcessObservation},
};
use serde::{Deserialize, Serialize};
use std::io;

/// Fixed qualification code only; no request can supply this script or a host path.
pub const PROBE: &str = "set -eu; cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max /sys/fs/cgroup/memory.swap.max /sys/fs/cgroup/pids.max; stat -f -c '%T %b %S' /capsule";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceProbe {
    pub requested: ExecutionResources,
    pub observed: ExecutionResources,
    pub scratch_filesystem: String,
    pub process: ProcessObservation,
}

impl ResourceProbe {
    pub fn new(requested: ExecutionResources, process: ProcessObservation) -> io::Result<Self> {
        if process.end != ProcessEnd::Exited
            || process.exit_code != Some(0)
            || !process.cleanup_confirmed
        {
            return Err(io::Error::other(
                "resource probe did not exit successfully with confirmed cleanup",
            ));
        }
        let invalid = || io::Error::other("invalid kernel resource observation");
        let lines: Vec<_> = process.stdout.lines().collect();
        if lines.len() != 5 {
            return Err(invalid());
        }
        let cpu: Vec<_> = lines[0].split_whitespace().collect();
        let filesystem: Vec<_> = lines[4].split_whitespace().collect();
        if cpu.len() != 2 || filesystem.len() != 3 {
            return Err(invalid());
        }
        let integer = |s: &str| s.parse::<u64>().map_err(|_| invalid());
        let observed = ExecutionResources {
            cpu_quota_micros: integer(cpu[0])?,
            cpu_period_micros: integer(cpu[1])?,
            memory_bytes: integer(lines[1])?,
            swap_bytes: integer(lines[2])?,
            pids: integer(lines[3])?.try_into().map_err(|_| invalid())?,
            scratch_bytes: integer(filesystem[1])?
                .checked_mul(integer(filesystem[2])?)
                .ok_or_else(invalid)?,
        };
        if observed != requested || filesystem[0] != "tmpfs" {
            return Err(io::Error::other(format!(
                "kernel resources differ from request: requested={requested:?}, observed={observed:?}, filesystem={}",
                filesystem[0]
            )));
        }
        Ok(Self {
            requested,
            observed,
            scratch_filesystem: filesystem[0].into(),
            process,
        })
    }

    /// Revalidate retained typed values against raw observations and the current request.
    pub fn validate(&self, requested: &ExecutionResources, image: &str) -> io::Result<()> {
        if &self.requested != requested || self.process.image_id != image {
            return Err(io::Error::other(
                "resource qualification describes another image or request",
            ));
        }
        let checked = Self::new(requested.clone(), self.process.clone())?;
        if &checked != self {
            return Err(io::Error::other(
                "typed resource receipt disagrees with actual probe output",
            ));
        }
        Ok(())
    }
}
