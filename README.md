# hGEN

> API Schema Language for Humans

> [!WARNING]  
> This project is in early development and not ready for production use.

## Usage

The hGEN CLI can be used to generate code for a given schema file.

```bash
$ cargo install hgen
$ hgen -i schema.hgen -o ts
```

## Philosophy

- **Single Source of Truth**: Making sure that your API is consistent across all your services and clients is hard. With hGEN, you define and maintain your API in a single place, while fast code generation keeps overhead low.

- **Strict by Design**: What is the point of a schema language if it doesn't enforce strict rules? hGEN is heavily inspired by Rust's type system, making sure that your API is as safe as possible.

- **Compile Time Reflection**: Since code generated files are not meant to be edited directly, hGEN gives you that power back in form of an easy to use and strongly typed reflection metadata format, allowing you to build upon the generated code to implement you own types, validation, mapping logic and more.

- **Scalable Metadata**: Other schema or reflection methods require lots of ugly annotations and lack type safety. This is why one goal of hGEN is to provide a scalable way of defining metadata directly in the schema, allowing you to define validation, serialization and more in a type-safe way without taking away from the readability of the schema.

## Schema Language

hGEN defines its own schema language to describe APIs. The language is heavily inspired by languages like TypeScript, Kotlin and Dart, making it easy to learn and use. hGEN primitive types are based on Rust however, pushing for a more strict and safe API design.

```
extern alias Instant = String;

alias UUID = String & {
  type: uuid,
};

struct Todo {
  id: UUID,
  title: String,
  createdAt: Instant,
  checkedAt: Instant?,
}

struct CreateTodoParams {
  title: String,
}

service TodoService {
  create(params: CreateTodoParams) -> Todo,
  find() -> List<Todo>,
  check(id: UUID) -> Unit,
  uncheck(id: UUID) -> Unit,
}
```

## Reference

### Targets

- TypeScript

  > https://www.npmjs.com/package/hgen

- Dart

  > https://pub.dev/packages/hgen

- Rust

  > https://crates.io/crates/hgen

### Types

- Primitive Types:

  - [x] `Bool`
  - [x] `Int32`, `Int64`
  - [ ] `UInt32`, `UInt64`
  - [x] `Float32`, `Float64`
  - [x] `String`
  - [x] `Optional`

- Object Types:

  - [x] `struct`
  - [x] `enum`
  - [ ] Data Enums / Unions
  - [ ] Bitfield Enums

- Collection Types:

  - [ ] `[T, size]`
  - [ ] `(T1, T2, ..., Tn)` Tuples
  - [x] `List<T>`
  - [x] `Set<T>`
  - [x] `Map<K, V>`

- Other

  - [x] Type Aliases
  - [x] External Types
  - [ ] Result Types
