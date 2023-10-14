use sqlx::{postgres::PgPoolOptions, Pool};

#[derive(Debug, Default, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub password_hash: String,
    pub name: String,
}

pub async fn create_pool(database_url: &str) -> Pool<sqlx::Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .unwrap();
    return pool;
}

pub async fn get_user(pool: Pool<sqlx::Postgres>, id: i64) -> User {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap()
}

pub async fn insert_user(pool: Pool<sqlx::Postgres>, user: User) {
    sqlx::query(
        "INSERT INTO users (id, password_hash, name)
        VALUES ($1, $2, $3)")
        .bind(user.id)
        .bind(user.password_hash)
        .bind(user.name)
        .execute(&pool)
        .await
        .unwrap();
}
