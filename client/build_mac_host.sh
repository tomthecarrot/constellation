# Automates build process for Platform codegen on macOS.

set -e

CLIENT_DIR=$(realpath $(dirname $0))
PLATFORM_DIR=$(realpath $CLIENT_DIR/..)
RSHARP_DIR="$PLATFORM_DIR/crates/rsharp"

mkdir -p $CLIENT_DIR/serialize/rust/src/generated
cd $CLIENT_DIR/serialize/rust/src/generated
flatc --rust ../../../flatbuffers/all.fbs --gen-all --rust-module-root-file

# Build Platform core and demo libraries.
cd $PLATFORM_DIR
cargo build --all

# Generate C bindings.
cargo test --all
cargo run -p cs_codegen -- -f
cargo run -p rsharp_codegen -- -f

# Rename library from specific -> generic for symbol compatibility.
mv $PLATFORM_DIR/target/debug/libunity_states.dylib $PLATFORM_DIR/target/debug/libconstellation.dylib

# Generate C# bindings.
cd $CLIENT_DIR/codegen_pinvoke
dotnet restore
dotnet run -a x64 # CppSharp is compiled for x64.

cd $CLIENT_DIR/cs/tests
dotnet test

# Package DLLs for use in Unity demo.
cd $PLATFORM_DIR/demos/unity_states/cs
dotnet build
