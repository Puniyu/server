#[tokio::main]
async fn main() {
    use puniyu_server::run_server;
    run_server(None, None).await;
}