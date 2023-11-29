cmake_minimum_required(VERSION 3.22.1)

project("surface")


set(RUST_DIR ${CMAKE_SOURCE_DIR}/../../../../rust)
set(RUST_LIB_PATH ${CMAKE_SOURCE_DIR}/libs/${ANDROID_ABI}/libbevy_surface.so)

add_custom_target(rust_lib ALL
        COMMAND sh ${RUST_DIR}/build.sh ${ANDROID_NDK}

        BYPRODUCTS ${RUST_LIB_PATH}

        WORKING_DIRECTORY ${RUST_DIR}
        COMMENT "Building Rust static library"
)

add_library(bevy_surface INTERFACE)
target_link_directories(bevy_surface INTERFACE ${CMAKE_SOURCE_DIR}/libs/${ANDROID_ABI})
#set_target_properties(bevy_surface PROPERTIES IMPORTED_LOCATION ${RUST_LIB_PATH})
set_target_properties(bevy_surface PROPERTIES INTERFACE_LINK_LIBRARIES libbevy_surface.so)
add_dependencies(bevy_surface rust_lib)


add_library(surface SHARED
        native-lib.cpp)

target_link_libraries(surface
        android
        log
        bevy_surface
        )