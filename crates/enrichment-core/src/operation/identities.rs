//! Exact native inputs for request, acquisition and normalization identities.
use crate::native_union::Rule;
crate::native_struct! {
    pub struct ProducerImplementation {
        producer: String => Rule::NonEmpty,
        components: std::collections::BTreeMap<String, String> => Rule::Map,
    }
}
crate::native_struct! {
    pub struct VerificationConfiguration {
        release_id: crate::identity::ReleaseId => Rule::Text,
        environment: crate::identity::Environment => Rule::Text,
        mode: crate::execution::ProbeMode => Rule::Text,
        image: String => Rule::NonEmpty,
        containment: String => Rule::NonEmpty,
    }
}
crate::native_struct! {
    pub struct ContainmentIdentity {
        contract: String => Rule::NonEmpty,
        protocol: u32 => Rule::Text,
        components: std::collections::BTreeMap<String, String> => Rule::Map,
        resources: crate::config::ExecutionResources => Rule::Text,
        output_bytes: usize => Rule::Text,
        deadline_seconds: u64 => Rule::Text,
        cleanup_deadline_seconds: u64 => Rule::Text,
    }
}
crate::native_struct! {
    pub struct ProcessInventory {
        entries: crate::capsule_protocol::inventory::Inventory => Rule::Map,
    }
}
crate::native_struct! {
    pub struct PhysicalFile {
        path: std::path::PathBuf => Rule::Text,
        device: u64 => Rule::Text,
        inode: u64 => Rule::Text,
        mode: u32 => Rule::Text,
        bytes: u64 => Rule::Text,
        modified_seconds: i64 => Rule::Text,
        modified_nanoseconds: i64 => Rule::Text,
        changed_seconds: i64 => Rule::Text,
        changed_nanoseconds: i64 => Rule::Text,
    }
}
crate::native_struct! {
    pub struct PhysicalInventory {
        files: Vec<PhysicalFile> => Rule::Set,
    }
}
crate::native_struct! {
    pub struct ResearchInvocation {
        request: crate::request::ResearchRequest => Rule::Text,
    }
}
crate::native_struct! {
    pub struct AcquisitionConfiguration {
        producer: String => Rule::NonEmpty,
        configuration: crate::config::Producers => Rule::Text,
    }
}
crate::native_struct! {
    pub struct NormalizationConfiguration {
        configuration_digest: String => Rule::Sha256,
        components: std::collections::BTreeMap<String, String> => Rule::Map,
    }
}
crate::native_struct! {
    pub struct InspectionConfiguration {
        release_id: crate::identity::ReleaseId => Rule::Text,
        environment: crate::identity::Environment => Rule::Text,
        image: String => Rule::NonEmpty,
        containment: String => Rule::NonEmpty,
    }
}
crate::native_struct! {
    pub struct RustdocBuildConfiguration {
        environment: crate::identity::Environment => Rule::Text,
        rustc_identity: String => Rule::NonEmpty,
        producer_version: String => Rule::NonEmpty,
        format_version: u32 => Rule::Text,
        image_id: String => Rule::NonEmpty,
        containment_identity: String => Rule::Sha256,
    }
}
impl RustdocBuildConfiguration {
    /// Retained configuration bytes use the same native value encoder as other producer
    /// inputs. JSON rendering and member order cannot become configuration authority.
    pub fn canonical_bytes(&self) -> datafusion::common::Result<Vec<u8>> {
        crate::native_identity::record_bytes(
            "enrichment/rustdoc-build-configuration/1",
            <Self as crate::native_union::NativeStruct>::encode(&[Some(self)])?,
        )
    }
}
crate::native_struct! {
    pub struct CapsuleIdentity {
        release: crate::identity::Release => Rule::Text,
        environment: crate::identity::Environment => Rule::Text,
        context: crate::identity::Context => Rule::Text,
        image: String => Rule::NonEmpty,
        containment: String => Rule::NonEmpty,
        profile: crate::policy::ExecutionProfile => Rule::Text,
    }
}
