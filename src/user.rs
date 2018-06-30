#[derive(Debug, Clone, PartialEq, FromForm)]
pub struct UserWithPlaintextPassword {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
}
