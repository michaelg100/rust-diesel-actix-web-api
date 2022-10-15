use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonPost {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct JsonId {
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct JsonUpdatePost {
    pub id: i32,
    pub title: String,
    pub body: String,
}