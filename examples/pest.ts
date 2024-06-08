// AUTOGENERATED FILE - DO NOT EDIT

export type UUID = string;

import { Instant } from './pest.external';

export class Test {
  hello: (boolean | null);
}

export class Foo {
  world: Test;
  bar: (number[]);
  status: Status;
  tae: Map<string, number>;
}

export enum Status {
  Pending = 'Pending',
  Running = 'Running',
  Done = 'Done',
}

export class TestServiceConsumer {
  constructor(
    protected request: (method: string, inputs: any) => Promise<any>
  ) {}

  test(id: UUID): Promise<Foo> {
    return this.request("test", { id });
  }

  test2(id: UUID): Promise<Foo> {
    return this.request("test2", { id });
  }
}

export interface TestServiceProvider {
  test(id: UUID): Promise<Foo>;

  test2(id: UUID): Promise<Foo>;
}

// prettier-ignore
export const $schema = {models:{UUID:{type:'alias',inner:{type:'string',metadata:{}}},Instant:{type:'external',inner:{type:'string',metadata:{type:'test'}}},Test:{type:'struct',fields:{hello:{type:'nullable',inner:{type:'bool'},metadata:{}}}},Foo:{type:'struct',fields:{world:{type:'reference',name:'Test',metadata:{}},bar:{type:'list',inner:{type:'int32'},metadata:{}},status:{type:'reference',name:'Status',metadata:{}},tae:{type:'map',key:{type:'string'},value:{type:'int32'},metadata:{}}}},Status:{type:'enum',fields:{Pending:'',Running:'',Done:''}}},services:{TestService:{type:'service',methods:{test:{inputs:{id:{type:'reference',name:'UUID'}},output:{type:'reference',name:'Foo'},metadata:{hello:'test',precision:0.1,rest:{method:'GET',url:'/test/{id}'}}},test2:{inputs:{id:{type:'reference',name:'UUID'}},output:{type:'reference',name:'Foo'},metadata:{}}}}}} as const;