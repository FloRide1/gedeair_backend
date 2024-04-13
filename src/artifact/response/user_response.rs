use serde::Serialize;

use crate::oidc::User;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        UserResponse {
            id: value.id,
            username: value.username,
            name: value.name,
            email: value.email,
        }
    }
}
