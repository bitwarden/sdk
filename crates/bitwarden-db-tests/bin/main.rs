use bitwarden_db_tests::run_tests;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    run_tests().await;
}
