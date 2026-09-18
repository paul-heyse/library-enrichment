// A syntactic negative control for DataFusion attribution, not a compiled program.
struct TelemetryClient;
impl TelemetryClient {
    async fn collect(&self) -> Vec<u8> { vec![] }
}
async fn telemetry(client: TelemetryClient) {
    let _events = client.collect().await;
}
