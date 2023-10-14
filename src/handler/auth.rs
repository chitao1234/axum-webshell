use axum::{extract::Query, response::IntoResponse, Extension};
use axum_login::{
    secrecy::SecretVec,
    AuthUser, PostgresStore,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::db::get_user;
pub use crate::db::User;

impl AuthUser<i64> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    id: i64,
    password_hash: String,
}

type AuthContext = axum_login::extractors::AuthContext<i64, User, PostgresStore<User>>;

pub async fn login_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    mut auth: AuthContext,
    Query(login_user): Query<LoginUser>,
) {
    let user = get_user(pool, login_user.id).await;
    if login_user.password_hash == user.password_hash {
        println!("Welcome, {}", user.name);
        auth.login(&user).await.unwrap();
    }
}

pub async fn logout_handler(mut auth: AuthContext) {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;
}

pub async fn protected_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Logged in as: {}", user.name)
}
