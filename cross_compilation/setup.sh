set -e

PLATFORM_DIR=$(realpath $(dirname $0)/..)

mkdir -p $PLATFORM_DIR/cross_compilation/build
cd $PLATFORM_DIR/cross_compilation/build

# Cloning cross is necessary in order to get access to the `build-docker-image` command.
git clone https://github.com/cross-rs/cross
cd cross
git submodule update --init --remote

# For compliance with Apple's EULA, this file is only accessible from inside the Teleportal VPN.
# The Mac OS X SDK can be downloaded from Apple's developer site.
export IOS_SDK_URL="http://fs.in.teleportal.dev/constellation/build/deps/MacOSX12.3.sdk.tar.xz"

cargo build-docker-image aarch64-apple-ios-cross \
  --build-arg "IOS_SDK_URL=$IOS_SDK_URL"
