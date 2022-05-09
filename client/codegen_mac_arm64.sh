# Automates build process for Platform codegen on Apple Silicon Macs.
# This script should be run from the `client` directory.

CLIENT_DIR=$(pwd)
PLATFORM_DIR=$CLIENT_DIR/..

rm -rf $CLIENT_DIR/cs/src/generated

cd $PLATFORM_DIR
cargo build
cargo build --target x86_64-apple-darwin

if [ ! -f $CLIENT_DIR/cs/tests/bin/Debug/net6.0/libtp_client.dylib ]; then
    ln -s $PLATFORM_DIR/target/debug/libtp_client.dylib $CLIENT_DIR/cs/tests/bin/Debug/net6.0/libtp_client.dylib
fi

if [ ! -f $CLIENT_DIR/cs/tests/bin/Debug/net6.0/librsharp.dylib ]; then
    ln -s $PLATFORM_DIR/target/debug/librsharp.dylib $CLIENT_DIR/cs/tests/bin/Debug/net6.0/librsharp.dylib
fi

mv $PLATFORM_DIR/target/x86_64-apple-darwin/debug/libtp_client.dylib $PLATFORM_DIR/target/x86_64-apple-darwin/debug/libtp_client
mv $PLATFORM_DIR/target/x86_64-apple-darwin/debug/librsharp.dylib $PLATFORM_DIR/target/x86_64-apple-darwin/debug/librsharp

cd $CLIENT_DIR/codegen_pinvoke
dotnet run -a x64

cd $CLIENT_DIR/codegen_wrapped
cargo run

cd $CLIENT_DIR/cs/tests
dotnet test
