# hGEN

> API Schema Language for Humans

> [!WARNING]  
> This project is in early development and not ready for production use.

## Philosophy

- **Single Source of Truth**: Making sure that your API is consistent across all your services and clients is hard. With hGEN, you define and maintain your API in a single place, while fast code generation keeps overhead low.

- **Strict by Design**: What is the point of a schema language if it doesn't enforce strict rules? hGEN is heavily inspired by Rust's type system, making sure that your API is as safe as possible.

- **Compile Time Reflection**: Since code generated files are not meant to be edited directly, hGEN gives you that power back in form of an easy to use and strongly typed reflection metadata format, allowing you to build upon the generated code to implement you own types, validation, mapping logic and more.

- **Scalable Metadata**: Other schema or reflection methods require lots of ugly annotations and lack type safety. This is why one goal of hGEN is to provide a scalable way of defining metadata directly in the schema, allowing you to define validation, serialization and more in a type-safe way without taking away from the readability of the schema.

## Schema Language

hGEN defines its own schema language to describe APIs. The language is heavily inspired by languages like TypeScript, Kotlin and Dart, making it easy to learn and use. hGEN primitive types are based on Rust however, pushing for a more strict and safe API design.

```ts
interface User {
  id: Int64 & {
    format: "uuid";
  };
  firstName: String;
  middleName?: String;
  lastName: String;
  age: Int32;
}

interface CreateUserRequest {
  user: User;
}

service UserService {
  createUser(firstName: String, lastName: String): User;
}
```

## Beyond Types

### Reflection

Since hGEN knows the structure of your API better than every other tool in your stack, it can provide powerful reflection capabilities. Optionally hGEN can bake in reflection metadata into the generated code, allowing you to inspect and manipulate your API at runtime.

```ts
export class User {
  id: string;
  name: string;

  static $fields = {
    id: {
      type: "string",
      nullable: false,
      data: { type: "uuid" },
    },
    name: {
      type: "string",
      nullable: false,
      data: {},
    },
  } as const;
}
```

### Validation

With hGENs scalable metadata format, validation can be defined directly in the schema. This allows you to share validation logic between your backend and frontend, making it easy to give real-time feedback to your users without spamming your API with invalid requests.

```ts
export class User {
  id: string & {
    format: "uuid";
  };
  name: string & {
    min: 1;
    max: 100;
  };
}
```

## Reference

### Targets

- TypeScript
  - [x] Types
  - [x] Reflection
  - [ ] Client
  - [ ] Server
- Dart
  - [x] Types
  - [ ] Reflection
  - [ ] Client
  - [ ] Server
- Rust
  - [x] Types
  - [ ] Reflection
  - [ ] Client
  - [ ] Server

### Types

- Primitive Types:

  - [x] `Bool`
  - [x] `Int32`, `Int64`
  - [ ] `UInt32`, `UInt64`
  - [x] `Float32`, `Float64`
  - [x] `String`
  - [x] `?` Nullable

- Object Types:

  - [x] `struct`
  - [x] `enum`
  - [ ] `bitflags`

- Collection Types:

  - [x] `List<T>`
  - [x] `Set<T>`
  - [x] `Map<K, V>`

- Complex Types:

  - [x] `Instant`
  - [ ] `Duration`

- Other

  - [ ] `A | B` Unions / Rust Enums
  - [ ] `(T1, T2, ..., Tn)` Tuples
  - [ ] Custom Types
  - [ ] Strong Error Types / Rust Results
