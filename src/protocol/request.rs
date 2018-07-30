//! Request messages

use protocol::model::Session;

#[derive(Deserialize, Serialize)]
/// The credentials based login request
pub struct LoginCredentials {
    /// The username
    pub username: String,

    /// The password
    pub password: String,
}

#[derive(Deserialize, Serialize)]
/// The session based login request
pub struct LoginSession(pub Session);

#[derive(Deserialize, Serialize)]
/// The logout request
pub struct Logout(pub Session);
