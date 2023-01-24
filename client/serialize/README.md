# tp_serialize

## Generating flatbuffers

1. Get flatc v23.1.21 (must be exact version) on your path
1. cd to `serialize/rust/src/generated` dir
1. run `flatc --rust ../../../flatbuffers/all.fbs --gen-all --rust-module-root-file`
