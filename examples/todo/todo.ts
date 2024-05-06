export class Todo {
  id: string;
  title: string;
  createdAt: Date;
  checkedAt?: Date;

  // prettier-ignore
  static $fields = {id:{type:'string',nullable:false,data:{format:'"uuid"'}},title:{type:'string',nullable:false,data:{}},createdAt:{type:'instant',nullable:false,data:{}},checkedAt:{type:'instant',nullable:true,data:{}}} as const
}

export class CreateTodo {
  title: string;

  // prettier-ignore
  static $fields = {title:{type:'string',nullable:false,data:{}}} as const
}

