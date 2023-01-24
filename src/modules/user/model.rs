#[derive(Debug)]
pub struct User {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
}
