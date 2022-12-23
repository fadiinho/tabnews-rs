use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersCreatedStatus {
    /// Date in the format dd/mm
    pub date: String,
    pub cadastros: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostsPublishedStatus {
    /// Date in the format dd/mm
    pub date: String,
    pub conteudos: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentsPublishedStatus {
    /// Date in the format dd/mm
    pub date: String,
    pub respostas: u64,
}
