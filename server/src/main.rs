use server::mocks::mock_main_axum;

#[tokio::main]
async fn main() {
    mock_main_axum::start_app().await;
}