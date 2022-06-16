# Automates build process for Platform codegen on Apple Silicon Macs.
# This script should be run from the `client` directory.

set -e

CLIENT_DIR=$(realpath $(dirname $0))
PLATFORM_DIR=$(realpath $CLIENT_DIR/..)
RSHARP_DIR="$PLATFORM_DIR/crates/rsharp"

rm -rf $CLIENT_DIR/cs/src/generated
rm -rf $RSHARP_DIR/cs/src/generated

cd $PLATFORM_DIR
cargo build
cargo build --target x86_64-apple-darwin

if [ ! -f $CLIENT_DIR/cs/tests/bin/Debug/net6.0/libtp_client.dylib ]; then
    ln -s $PLATFORM_DIR/target/debug/deps/libtp_client.dylib $CLIENT_DIR/cs/tests/bin/Debug/net6.0/libtp_client.dylib
fi

if [ ! -f $CLIENT_DIR/cs/tests/bin/Debug/net6.0/librsharp.dylib ]; then
    ln -s $PLATFORM_DIR/target/debug/deps/librsharp.dylib $CLIENT_DIR/cs/tests/bin/Debug/net6.0/librsharp.dylib
fi

mv $PLATFORM_DIR/target/x86_64-apple-darwin/debug/deps/libtp_client.dylib $PLATFORM_DIR/target/x86_64-apple-darwin/debug/deps/libtp_client
mv $PLATFORM_DIR/target/x86_64-apple-darwin/debug/deps/librsharp.dylib $PLATFORM_DIR/target/x86_64-apple-darwin/debug/deps/librsharp

cd $CLIENT_DIR/rust/
cargo test

cd $RSHARP_DIR/rust
cargo test

cd $CLIENT_DIR/codegen_pinvoke
dotnet run -a x64

cd $CLIENT_DIR/codegen_wrapped
cargo run

cd $CLIENT_DIR/cs/tests
dotnet test
