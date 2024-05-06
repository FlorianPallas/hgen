pub struct Todo {
    pub id: String,
    pub title: String,
    pub created_at: Instant,
    pub checked_at: Option<Instant>,
}

pub struct CreateTodo {
    pub title: String,
}
