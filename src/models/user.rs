use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub features: Vec<String>,
    pub notifications: Option<bool>,
    pub tabcoins: i64,
    pub tabcash: i64,
    // TODO: Use `chrono` crate to serialize/deserialize Date format
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub id: String,
    pub token: String,
    pub expires_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditProfilePayload {
    #[serde(rename = "username")]
    pub new_username: Option<String>,
    #[serde(rename = "email")]
    pub new_email: Option<String>,
    #[serde(rename = "password")]
    pub new_password: Option<String>,
    pub notifications: Option<bool>,
}
//
// impl Serialize for EditProfilePayload {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // 3 is the number of fields in the struct.
//         let mut state = serializer.serialize_struct("EditProfilePayload", 4)?;
//
//         state.serialize_field("username", &self.username)?;
//         state.serialize_field("email", &self.email)?;
//         state.serialize_field("password", &self.password)?;
//         state.serialize_field("notifications", &self.notifications)?;
//
//         state.end()
//     }
// }
