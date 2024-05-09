/* UNSUPPORTED */
/* UNSUPPORTED */
class Todo {
  UUID id;
  String title;
  Instant createdAt;
  Instant? checkedAt;

  Todo({
    required this.id,
    required this.title,
    required this.createdAt,
    this.checkedAt,
  });

  static void $hWrite(Writer writer,value:Todo) => {}
  factory Todo.$hRead(Reader reader) => {}
  static Schema $hSchema = {};
}

class CreateTodo {
  String title;

  CreateTodo({
    required this.title,
  });

  static void $hWrite(Writer writer,value:CreateTodo) => {}
  factory CreateTodo.$hRead(Reader reader) => {}
  static Schema $hSchema = {};
}
