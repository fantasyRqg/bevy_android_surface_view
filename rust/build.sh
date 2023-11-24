#get ANDROID_NDK_HOME from parameter
if [ -z "$1" ]; then
    echo "ANDROID_NDK_HOME is not set"
    exit 1
fi
export ANDROID_NDK_HOME=$1

DST_DIR=../app/src/main/cpp/libs/arm64-v8a/
mkdir -p $DST_DIR
cargo ndk -t arm64-v8a  build
#echo "copy libbevy_surface.a to $DST_DIR"
#cp ./target/aarch64-linux-android/debug/libbevy_surface.a $DST_DIR