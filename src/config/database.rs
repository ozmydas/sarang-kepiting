use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use tokio::sync::OnceCell;
use tracing::info;

static DB: Lazy<OnceCell<PgPool>> = Lazy::new(OnceCell::new);

pub async fn connect_db() -> Result<(), Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/db_sarangkepiting")
        .await?;

    // sqlx::migrate!().run(&pool).await?;

    info!("🐘 postgres successfully connected 💡");

    DB.set(pool).expect("❌ Database already initialized");

    Ok(())
} // end func

pub fn get_db() -> &'static PgPool {
    DB.get().expect("❌ Database is not initialized")
} // end func
