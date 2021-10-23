use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
struct Ticket {
	id: i64,
	name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let pool = PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(30))
        .min_connections(0)
		.max_connections(5)
        .connect_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(10*60))
		.connect("postgres://postgres:welcome@localhost/postgres")
		.await?;

	sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS ticket (
  id bigserial,
  name text
);"#,
	)
	.execute(&pool)
	.await?;
	let row: (i64,) = sqlx::query_as("insert into ticket (name) values ($1) returning id")
		.bind("a new ticket")
		.fetch_one(&pool)
		.await?;

	Ok(())
}