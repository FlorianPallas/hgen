import { Instant } from './todo.extern';

export type UUID = string & {
  type: 'uuid'
};

export class Todo {
  id: UUID;
  title: string;
  createdAt: Instant;
  checkedAt: (Instant | null);
}

export class CreateTodo {
  title: string;
}
