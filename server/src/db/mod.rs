use lazy_static::lazy_static;
use sqlx::PgPool;
use tokio::sync::OnceCell;

pub mod account;

lazy_static! {
    static ref DB_CONNECTION: OnceCell<PgPool> = OnceCell::new();
}

pub async fn init(url: &str) -> sqlx::Result<()> {
    let connection = PgPool::connect(url).await?;

    DB_CONNECTION
        .set(connection)
        .expect("DB_CONNECTION initialised before connection to database");
    println!("* Connected to database");

    Ok(())
}
