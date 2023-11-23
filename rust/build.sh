export ANDROID_NDK_HOME=/Users/rqg/Library/Android/sdk/ndk/25.2.9519653

DST_DIR=../app/src/main/cpp/libs/arm64-v8a/
mkdir -p $DST_DIR
cargo ndk -t arm64-v8a  build && cp ./target/aarch64-linux-android/debug/libbevy_surface.a $DST_DIR