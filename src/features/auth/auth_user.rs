use diesel::prelude::*;
use axum_login::{
    AuthUser,
    secrecy::SecretVec,
};

impl AuthUser<i32> for AuthUserModel {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::auth_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUserModel {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub password_hash: String,
}