#!/bin/bash
# Automates build process for Platform codegen on macOS.

set -ex

# Defaults
BUILD_PROFILE="dev"
BUILD_PROFILE_DIRNAME="debug"
LIB_NAME="unity_states"

# Parse CLI options
while getopts ":ailrw" options; do
    case $options in
        a) FLAG_ANDROID=true ;;
        i) FLAG_IOS=true ;;
        l) FLAG_LINUX=true ;;
        r) BUILD_PROFILE="release"; BUILD_PROFILE_DIRNAME="release" ;;
        w) WORKAROUND_ROSETTA_ISSUE=true ;;
    esac
done

## BUILD FOR CURRENT HOST ##

CLIENT_DIR=$(readlink -f $(dirname $0))
PLATFORM_DIR=$(readlink -f $CLIENT_DIR/..)
RSHARP_DIR="$PLATFORM_DIR/crates/rsharp"
HOST_TARGET_TRIPLE=$(rustc -vV | sed -n 's|host: ||p')
CARGO_IOS_BUILDTOOL="cross"

# Override for aarch64 Macs which can compile to iOS without `cross`
if [ "$HOST_TARGET_TRIPLE" == "aarch64-apple-darwin" ]; then CARGO_IOS_BUILDTOOL="cargo"; fi

# Build Platform core and demo libraries.
cd $PLATFORM_DIR
cargo build --profile $BUILD_PROFILE --target $HOST_TARGET_TRIPLE -p unity_states

# Generate C bindings.
cargo test -p tp_client -p rsharp -p unity_states
cargo run -p cs_codegen -- -f
cargo run -p rsharp_codegen -- -f

# Copy native library to Unity project
rsync $PLATFORM_DIR/target/$HOST_TARGET_TRIPLE/$BUILD_PROFILE_DIRNAME/lib$LIB_NAME.dylib $PLATFORM_DIR/target/$HOST_TARGET_TRIPLE/lib$LIB_NAME.dylib

## CREATE EMPTY FILE TARGETS FOR SYMLINKS #

# Create empty files to symlink against in the event that
# we don't cross-compile to all of these platforms.
mkdir -p target/aarch64-apple-darwin && touch $_/lib$LIB_NAME.dylib
mkdir -p target/aarch64-apple-ios && touch $_/lib$LIB_NAME.a
mkdir -p target/aarch64-linux-android && touch $_/lib$LIB_NAME.so
mkdir -p target/armv7-linux-androideabi && touch $_/lib$LIB_NAME.so
mkdir -p target/x86_64-unknown-linux-gnu && touch $_/lib$LIB_NAME.so

MANAGED_LIB_DIR_IOS=$PLATFORM_DIR/demos/unity_states/cs/bin/ios
mkdir -p $MANAGED_LIB_DIR_IOS
touch $MANAGED_LIB_DIR_IOS/client.dll
touch $MANAGED_LIB_DIR_IOS/rsharp.dll
touch $MANAGED_LIB_DIR_IOS/unity_states.dll

## PREPARE FOR CROSS-COMPILATION ##

mkdir -p $PLATFORM_DIR/target/$BUILD_PROFILE_DIRNAME

## CROSS-COMPILE TO LINUX

if ! [ -z "$FLAG_LINUX" ]
then
    cross build --profile $BUILD_PROFILE --target x86_64-unknown-linux-gnu -p unity_states

    # Move library into symlinked location (if needed)
    rsync $PLATFORM_DIR/target/x86_64-unknown-linux-gnu/$BUILD_PROFILE_DIRNAME/lib$LIB_NAME.so $PLATFORM_DIR/target/x86_64-unknown-linux-gnu/lib$LIB_NAME.so
fi

## CROSS-COMPILE TO ANDROID

if ! [ -z "$FLAG_ANDROID" ]
then
    cross build --profile $BUILD_PROFILE --target aarch64-linux-android -p unity_states
    cross build --profile $BUILD_PROFILE --target armv7-linux-androideabi -p unity_states

    # Move library into symlinked locations (if needed)
    rsync $PLATFORM_DIR/target/aarch64-linux-android/$BUILD_PROFILE_DIRNAME/lib$LIB_NAME.so $PLATFORM_DIR/target/aarch64-linux-android/lib$LIB_NAME.so
    rsync $PLATFORM_DIR/target/armv7-linux-androideabi/$BUILD_PROFILE_DIRNAME/lib$LIB_NAME.so $PLATFORM_DIR/target/armv7-linux-androideabi/lib$LIB_NAME.so
fi

## CROSS-COMPILE TO IOS

if ! [ -z "$FLAG_IOS" ]
then
    $CARGO_IOS_BUILDTOOL build --profile $BUILD_PROFILE --target aarch64-apple-ios -p unity_states

    # Move library into symlinked locations (if needed)
    rsync $PLATFORM_DIR/target/aarch64-apple-ios/$BUILD_PROFILE_DIRNAME/lib$LIB_NAME.a $PLATFORM_DIR/target/aarch64-apple-ios/lib$LIB_NAME.a
fi

## BINDINGS FOR DYNAMICALLY-LINKED TARGETS ##

# This works around a Rosetta 2 issue. See this thread for details:
# https://github.com/0xTELEPORTAL/constellation/pull/118#discussion_r981944921
if ! [ -z "$WORKAROUND_ROSETTA_ISSUE" ]
then
    TMP_ID_0=56
    TMP_ID_1=57
    mv $CLIENT_DIR/codegen_pinvoke/codegen.csproj $CLIENT_DIR/codegen_pinvoke/codegen$TMP_ID_0.csproj
fi

# Generate C# bindings.
rsync $PLATFORM_DIR/target/$HOST_TARGET_TRIPLE/lib$LIB_NAME.dylib $PLATFORM_DIR/target/
cd $CLIENT_DIR/codegen_pinvoke
dotnet restore
dotnet run -a x64 # CppSharp is compiled for x64.

cd $CLIENT_DIR/cs/tests
dotnet test

# Package DLLs for use in Unity demo.
cd $PLATFORM_DIR/demos/unity_states/cs
CONSTELLATION_SKIP_CODEGEN=true dotnet build -o bin/main

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
    CONSTELLATION_SKIP_CODEGEN=true dotnet build -o bin/ios -p:DefineConstants=UNITY_IOS
fi
