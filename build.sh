cargo build --release

target/release/hgen -i examples/todo/todo.hgen -o examples/todo/out/todo.ts
target/release/hgen -i examples/todo/todo.hgen -o examples/todo/out/todo.rs
target/release/hgen -i examples/todo/todo.hgen -o examples/todo/out/todo.dart
target/release/hgen -i examples/todo/todo.hgen -o examples/todo/out/todo.json

target/release/hgen -i examples/multi-file/public.hgen -o examples/multi-file/out/public.ts
target/release/hgen -i examples/multi-file/public.hgen -o examples/multi-file/out/public.rs
target/release/hgen -i examples/multi-file/public.hgen -o examples/multi-file/out/public.dart
target/release/hgen -i examples/multi-file/public.hgen -o examples/multi-file/out/public.json

target/release/hgen -i examples/multi-file/private.hgen -o examples/multi-file/out/private.ts
target/release/hgen -i examples/multi-file/private.hgen -o examples/multi-file/out/private.rs
target/release/hgen -i examples/multi-file/private.hgen -o examples/multi-file/out/private.dart
target/release/hgen -i examples/multi-file/private.hgen -o examples/multi-file/out/private.json
