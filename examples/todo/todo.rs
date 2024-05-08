pub struct Todo {
    pub id: UUID,
    pub title: String,
    pub created_at: Instant,
    pub checked_at: Option<Instant>,
}

pub struct CreateTodo {
    pub title: String,
}
