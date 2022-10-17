// use dotenv::dotenv;
use sea_orm_migration::prelude::*;
#[async_std::main]
async fn main() {
    // dotenv().ok();
    // let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    // println!("{:#?}", db_url);
    cli::run_cli(migration::Migrator).await;
}
