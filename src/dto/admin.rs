use serde::Deserialize;

#[derive(Deserialize)]
pub struct Admin {
    pub username: String,
    pub password: String,
}
