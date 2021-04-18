use serde::Deserialize;

// Secrets defines the list of required secrets
#[derive(Deserialize)]
pub struct Secrets {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub base_url: String,
}
