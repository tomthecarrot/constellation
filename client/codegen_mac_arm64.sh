# Automates build process for Platform codegen on Apple Silicon Macs.
# This script should be run from the `client` directory.

set -e

CLIENT_DIR=$(realpath $(dirname $0))
PLATFORM_DIR=$(realpath $CLIENT_DIR/..)
RSHARP_DIR="$PLATFORM_DIR/crates/rsharp"

rm -rf $CLIENT_DIR/cs/src/generated
rm -rf $RSHARP_DIR/cs/src/generated

# Build Platform core and demo libraries.
cd $PLATFORM_DIR
cargo build -p tp_client
cargo build -p rsharp
cargo build -p unity_states

# Strip .dylib extension for symlink compatibility across OS targets.
mv $PLATFORM_DIR/target/debug/deps/libtp_client.dylib $PLATFORM_DIR/target/debug/deps/libtp_client
mv $PLATFORM_DIR/target/debug/deps/librsharp.dylib $PLATFORM_DIR/target/debug/deps/librsharp
mv $PLATFORM_DIR/target/debug/deps/libunity_states.dylib $PLATFORM_DIR/target/debug/deps/libunity_states

# Copy Platform libraries for C# tests.
cp $PLATFORM_DIR/target/debug/deps/libtp_client $CLIENT_DIR/cs/tests/bin/Debug/net6.0/
cp $PLATFORM_DIR/target/debug/deps/librsharp $CLIENT_DIR/cs/tests/bin/Debug/net6.0/
cp $PLATFORM_DIR/target/debug/deps/libunity_states $CLIENT_DIR/cs/tests/bin/Debug/net6.0/

# Generate C bindings.
cargo test -p tp_client
cargo run -p rsharp
cargo test -p unity_states

# Generate C# bindings.
cd $CLIENT_DIR/codegen_pinvoke
dotnet run -a x64 # CppSharp is compiled for x64.

cd $CLIENT_DIR/codegen_wrapped
cargo run

cd $CLIENT_DIR/cs/tests
dotnet test

# Package DLLs for use in Unity demo.
cd $PLATFORM_DIR/demos/unity_states/cs
dotnet build
