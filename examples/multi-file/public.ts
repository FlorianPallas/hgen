// AUTOGENERATED FILE - DO NOT EDIT

export class Post {
  slug: string;
  title: string;
  body: string;
  author: string;
}

export class PostServiceConsumer {
  constructor(
    protected request: (method: string, inputs: any) => Promise<any>
  ) {}

  find(): Promise<(Post[])> {
    return this.request("find", {  });
  }

  findOne(slug: string): Promise<Post | null> {
    return this.request("findOne", { slug });
  }
}

export interface PostServiceProvider {
  find(): Promise<(Post[])>;

  findOne(slug: string): Promise<Post | null>;
}

// prettier-ignore
export const $schema = {models:{Post:{type:'struct',fields:{slug:{type:'string',data:{}},title:{type:'string',data:{}},body:{type:'string',data:{}},author:{type:'string',data:{}}}}},services:{PostService:{type:'service',methods:{find:{inputs:{},output:{type:'list',inner:{type:'reference',name:'Post'},data:{}}},findOne:{inputs:{slug:{type:'string',data:{}}},output:{type:'nullable',inner:{type:'reference',name:'Post'},data:{}}}}}}} as const;