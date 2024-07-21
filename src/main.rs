// use sqlx::postgres::PgPoolOptions;

mod lhvconnect;
use lhvconnect::Client;

#[tokio::main]
async fn main() {
    let _client = Client::from_env(lhvconnect::Endpoint::LiveEU).await.unwrap();
}
