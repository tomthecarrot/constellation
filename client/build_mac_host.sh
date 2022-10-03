# Automates build process for Platform codegen on macOS.

set -ex

# Defaults
BUILD_PROFILE="dev"

# Parse CLI options
while getopts ":ailrw" options; do
    case $options in
        a) FLAG_ANDROID=true ;;
        i) FLAG_IOS=true ;;
        l) FLAG_LINUX=true ;;
        r) BUILD_PROFILE="release" ;;
        w) WORKAROUND_ROSETTA_ISSUE=true ;;
    esac
done

## BUILD FOR CURRENT HOST ##

CLIENT_DIR=$(realpath $(dirname $0))
PLATFORM_DIR=$(realpath $CLIENT_DIR/..)
RSHARP_DIR="$PLATFORM_DIR/crates/rsharp"
HOST_TARGET_TRIPLE=$(rustc -vV | sed -n 's|host: ||p')

# Build Platform core and demo libraries.
cd $PLATFORM_DIR
cross build --target $HOST_TARGET_TRIPLE -p unity_states

# Generate C bindings.
cargo test --all
cargo run -p cs_codegen -- -f
cargo run -p rsharp_codegen -- -f

# Copy native library to Unity project
rsync $PLATFORM_DIR/target/$HOST_TARGET_TRIPLE/$BUILD_PROFILE/libunity_states.dylib $PLATFORM_DIR/target/$HOST_TARGET_TRIPLE/libunity_states.dylib

## CROSS-COMPILE TO LINUX

if ! [ -z "$FLAG_LINUX" ]
then
    cross build --target x86_64-unknown-linux-gnu -p unity_states

    # Move library into symlinked location (if needed)
    rsync $PLATFORM_DIR/target/x86_64-unknown-linux-gnu/$BUILD_PROFILE/libunity_states.so $PLATFORM_DIR/target/x86_64-unknown-linux-gnu/libunity_states.so
fi

## CROSS-COMPILE TO ANDROID

if ! [ -z "$FLAG_ANDROID" ]
then
    cross build --target aarch64-linux-android -p unity_states
    cross build --target armv7-linux-androideabi -p unity_states

    # Move library into symlinked locations (if needed)
    rsync $PLATFORM_DIR/target/aarch64-linux-android/$BUILD_PROFILE/libunity_states.so $PLATFORM_DIR/target/aarch64-linux-android/libunity_states.so
    rsync $PLATFORM_DIR/target/armv7-linux-androideabi/$BUILD_PROFILE/libunity_states.so $PLATFORM_DIR/target/armv7-linux-androideabi/libunity_states.so
fi

## CROSS-COMPILE TO IOS

if ! [ -z "$FLAG_IOS" ]
then
    cross build --target aarch64-apple-ios -p unity_states

    # Move library into symlinked locations (if needed)
    rsync $PLATFORM_DIR/target/aarch64-apple-ios/$BUILD_PROFILE/libunity_states.so $PLATFORM_DIR/target/aarch64-apple-ios/libunity_states.so
fi

## BINDINGS FOR DYNAMICALLY-LINKED TARGETS ##

# This works around a Rosetta 2 issue. See this thread for details:
# https://github.com/0xTELEPORTAL/constellation/pull/118#discussion_r981944921
if ! [ -z "$WORKAROUND_ROSETTA_ISSUE" ]
then
    TMP_ID_0=13
    TMP_ID_1=14
    mv $CLIENT_DIR/codegen_pinvoke/codegen.csproj $CLIENT_DIR/codegen_pinvoke/codegen$TMP_ID_0.csproj
fi

# Generate C# bindings.
rsync $PLATFORM_DIR/target/$HOST_TARGET_TRIPLE/libunity_states.dylib $PLATFORM_DIR/target/
cd $CLIENT_DIR/codegen_pinvoke
dotnet restore
dotnet run -a x64 # CppSharp is compiled for x64.

cd $CLIENT_DIR/cs/tests
dotnet test

# Package DLLs for use in Unity demo.
cd $PLATFORM_DIR/demos/unity_states/cs
dotnet build -o bin/main

## BINDINGS FOR STATICALLY-LINKED TARGETS ##

if ! [ -z "$FLAG_IOS" ]
then
    # Generate C# bindings.
    cd $CLIENT_DIR/codegen_pinvoke

    if ! [ -z "$WORKAROUND_ROSETTA_ISSUE" ]
    then
        mv $CLIENT_DIR/codegen_pinvoke/codegen$TMP_ID_0.csproj $CLIENT_DIR/codegen_pinvoke/codegen$TMP_ID_1.csproj
    fi

    # CppSharp is compiled for x64.
    # The -a flag is necessary when running on an Apple Silicon (aarch64) Mac.
    dotnet run -a x64 -p:DefineConstants=UNITY_IOS

    if ! [ -z "$WORKAROUND_ROSETTA_ISSUE" ]
    then
        mv $CLIENT_DIR/codegen_pinvoke/codegen$TMP_ID_1.csproj $CLIENT_DIR/codegen_pinvoke/codegen.csproj
    fi

    # Package DLLs for use in Unity demo.
    cd $PLATFORM_DIR/demos/unity_states/cs
    dotnet build -o bin/ios -p:DefineConstants=UNITY_IOS
fi
