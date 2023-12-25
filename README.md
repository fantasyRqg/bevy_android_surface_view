# bevy_android_surface_view

Run Bevy on Android with SurfaceView. Can stop and start the bevy engine as you like.

## Main purpose

Replace parts of the old games with bevy to boost performance.

## How to compile

Run `./gradlew assembleDebug` in the root directory of this project. 

## Changes in the bevy  
none

## Problems
1. Memory leak. Bevy seems not care about the resource release after the app exit. So you have to take care of it yourself (remove all handle hold in `Resource`). Even though my goal is to enable reenter bevy, I think it's better to start bevy only once and keep it running until the app exit. We can manipulate with `SurfaceView` to show or hide the bevy window.