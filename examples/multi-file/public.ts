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
export const $schema = {models:{Post:{type:'Struct',fields:{slug:{type:'String'},title:{type:'String'},body:{type:'String'},author:{type:'String'}}}},services:{PostService:{type:'Service',methods:{find:{inputs:{},output:{type:'List',inner:{type:'Reference',name:'Post'}}},findOne:{inputs:{slug:{type:'String'}},output:{type:'Nullable',inner:{type:'Reference',name:'Post'}}}}}}} as const;