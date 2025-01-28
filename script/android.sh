#!/bin/bash

# Refer: https://developer.android.com/ndk/downloads
# Refer: https://docs.rs/openssl/latest/openssl/
# Refer: https://github.com/openssl/openssl/blob/master/NOTES-ANDROID.md

# Config openssl
ANDROID_API=35
ANDROID_NDK=$1
OPENSSL_TAG=3.4.0
OPENSSL_SRC=openssl-openssl-${OPENSSL_TAG}

# Fetch openssl
curl -L https://github.com/openssl/openssl/archive/refs/tags/openssl-${OPENSSL_TAG}.zip -o openssl.zip
unzip openssl.zip

# Build openssl
pushd ${OPENSSL_SRC} || exit
export ANDROID_NDK_ROOT=$ANDROID_NDK
export PATH=$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH
./Configure android-arm64 -D__ANDROID_API__=${ANDROID_API} --prefix="$PWD"/output
make
make install
popd || exit

# Clean openssl
rm -f openssl.zip

# Build promptx
export ANDROID_NDK_HOME=$ANDROID_NDK
export AARCH64_LINUX_ANDROID_OPENSSL_DIR=$OPENSSL_SRC/output
export AARCH64_LINUX_ANDROID_OPENSSL_STATIC=1
cargo ndk -t aarch64-linux-android build --release
