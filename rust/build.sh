#get ANDROID_NDK_HOME from parameter
if [ -z "$1" ]; then
export ANDROID_NDK_HOME=/Users/rqg/Library/Android/sdk/ndk/25.2.9519653
else
export ANDROID_NDK_HOME=$1
fi


#export RUSTC_LOG=rustc_codegen_ssa::back::link=info
#cargo ndk -t arm64-v8a  build --verbose
cargo ndk -t arm64-v8a -o ../app/src/main/cpp/libs  build --release
