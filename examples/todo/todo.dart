import 'package:json_annotation/json_annotation.dart';

part 'todo.g.dart';

@JsonSerializable()
class Todo {
  String id;
  String title;
  DateTime createdAt;
  DateTime? checkedAt;

  Todo({
    required this.id,
    required this.title,
    required this.createdAt,
    this.checkedAt,
  });

  factory Todo.fromJson(Map<String, dynamic> json) => _$TodoFromJson(json);
  Map<String, dynamic> toJson() => _$TodoToJson(this);
}

@JsonSerializable()
class CreateTodo {
  String title;

  CreateTodo({
    required this.title,
  });

  factory CreateTodo.fromJson(Map<String, dynamic> json) => _$CreateTodoFromJson(json);
  Map<String, dynamic> toJson() => _$CreateTodoToJson(this);
}

