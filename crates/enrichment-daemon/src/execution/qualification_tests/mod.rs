//! Real operator qualification fixtures, not unit tests. Ignored until CP11 is closed and
//! the operator explicitly invokes the containment tier. No fixture API exists in production.
mod boundary;
mod cleanup;

use super::*;
impl Runner {
    /// Test-only construction of one exact operator contract from compiled harness inputs.
    /// Production still exposes only finite Invocation and private fixed tool/resource probes.
    async fn fixture_run(
        &self,
        image: &str,
        capsule: &Path,
        argv: &[String],
        cancel: Arc<AtomicBool>,
    ) -> io::Result<ProcessObservation> {
        self.fixture_outputs(image, capsule, argv, cancel, Default::default())
            .await
    }
    async fn fixture_outputs(
        &self,
        image: &str,
        capsule: &Path,
        argv: &[String],
        cancel: Arc<AtomicBool>,
        outputs: std::collections::BTreeMap<String, protocol::OutputKind>,
    ) -> io::Result<ProcessObservation> {
        let (runner, operation) = self.fixture_request(image, capsule, argv, outputs).await?;
        runner.execute(image, capsule, operation, cancel).await
    }
    async fn fixture_request(
        &self,
        image: &str,
        capsule: &Path,
        argv: &[String],
        outputs: std::collections::BTreeMap<String, protocol::OutputKind>,
    ) -> io::Result<(Self, Operation)> {
        let mut runner = self.clone().admitted().await?;
        let mut operation = runner.operation_from_command(
            image,
            capsule,
            None,
            ProducerCommand {
                mode: Mode::Command,
                network: Network::Offline,
                argv: argv.to_vec(),
                files: vec![],
                directories: vec![],
                required_files: vec![],
            },
        )?;
        operation.outputs = outputs;
        operation.validate()?;
        runner.authority = Authority::Qualification(Arc::new(description::Qualification::fixture(
            self.ownership.runtime().clone(),
            &operation,
        )));
        Ok((runner, operation))
    }
}
