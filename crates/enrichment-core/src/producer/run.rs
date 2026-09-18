//! Native producer-run provenance and canonical input/coverage identities.
use crate::{evidence::Gap, policy::ExecutionProfile};
use std::collections::BTreeMap;

crate::native_vocabulary! {
/// How one producer run ended.
///
/// The values are, in order: `succeeded`, `partial`, `failed`, `skipped`.
    #[schemars(inline)]
    pub enum RunOutcome { Succeeded = "succeeded", Partial = "partial", Failed = "failed", Skipped = "skipped" }
}
// Physical observations and the composed run share one field declaration. Only the
// composed run adds the native-selected semantic input map.
macro_rules! producer_record {
    ($name:ident $(, $field:ident : $kind:ty => $rule:expr)*) => {
    crate::native_struct! {
    pub struct $name {
        attempt_id: crate::identity::AttemptId => crate::native_union::Rule::Text,
        producer: String => crate::native_union::Rule::NonEmpty,
        producer_version: String => crate::native_union::Rule::NonEmpty,
        config_digest: String => crate::native_union::Rule::Sha256,
        $($field: $kind => $rule,)*
        profile: ExecutionProfile => crate::native_union::Rule::Text,
        started_at: crate::native_time::ObservationTime => crate::native_union::Rule::Text,
        finished_at: crate::native_time::ObservationTime => crate::native_union::Rule::Text,
        outcome: RunOutcome => crate::native_union::Rule::Text,
        gaps: Vec<Gap> => crate::native_union::Rule::Set,
        log: Option<String> => crate::native_union::Rule::Text,
    }
    }
    };
}
producer_record!(ProducerAttempt);
producer_record!(ProducerRun, inputs: BTreeMap<String, String> => crate::native_union::Rule::Map);

impl ProducerRun {
    /// Semantic producer identity includes outcome/coverage but excludes attempt provenance.
    #[must_use]
    pub fn semantic_binding_id(&self) -> String {
        crate::native_key::Key::ProducerBinding
            .batch_value(
                &crate::evidence::arrow_model::provenance::producer_fields(std::slice::from_ref(
                    self,
                ))
                .expect("native producer fields"),
            )
            .expect("native producer identity")
    }
    /// The single-flight key (§8.2): producer version, normalized options, input digests and
    /// profile. Deliberately excludes timestamps and the request that triggered the run.
    #[must_use]
    pub fn dedupe_key(&self) -> String {
        crate::native_key::Key::ProducerPlan
            .batch_value(
                &crate::evidence::arrow_model::provenance::producer_fields(std::slice::from_ref(
                    self,
                ))
                .expect("native producer fields"),
            )
            .expect("native producer identity")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dedupe_key_ignores_time() {
        let mut run = ProducerRun {
            attempt_id: crate::identity::AttemptId::new(),
            producer: "p".to_owned(),
            producer_version: "1".to_owned(),
            config_digest: "ab".repeat(32),
            inputs: BTreeMap::from([("x".to_owned(), "ab".repeat(32))]),
            profile: ExecutionProfile::Static,
            started_at: crate::native_time::ObservationTime::from_micros(1).unwrap(),
            finished_at: crate::native_time::ObservationTime::from_micros(2).unwrap(),
            outcome: RunOutcome::Succeeded,
            gaps: Vec::new(),
            log: None,
        };
        let key = run.dedupe_key();
        run.started_at = crate::native_time::ObservationTime::from_micros(3).unwrap();
        run.finished_at = crate::native_time::ObservationTime::from_micros(4).unwrap();
        assert_eq!(run.dedupe_key(), key);
        run.inputs.insert("y".to_owned(), "cd".repeat(32));
        assert_ne!(run.dedupe_key(), key);
    }
}
