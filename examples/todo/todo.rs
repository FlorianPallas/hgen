use serde::{Serialize, Deserialize};
use external::Instant;

pub type UUID = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: UUID,
    pub title: String,
    pub created_at: Instant,
    pub checked_at: Option<Instant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoParams {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoParams {
    pub title: Option<String>,
}
