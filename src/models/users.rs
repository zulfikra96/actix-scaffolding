use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    SUPERADMIN,
    ADMIN,
}


#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub email: String,
    pub role: Role,
    pub exp: isize
}
