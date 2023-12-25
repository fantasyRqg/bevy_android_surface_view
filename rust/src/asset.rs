use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::Poll;
use bevy::asset::BoxedFuture;
use bevy::asset::io::{AssetReader, AssetReaderError, AssetSource, AssetSourceId, PathStream, Reader, VecReader};
use bevy::prelude::*;
use bevy::tasks::futures_lite::Stream;
use crate::get_asset_manager;

pub struct AndroidAssetReaderPlugin;

impl Plugin for AndroidAssetReaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(|| {
                Box::new(CustomAssetReader(
                    // This is the default reader for the current platform
                    AssetSource::get_default_reader("assets".to_string())(),
                ))
            }),
        );
    }
}


struct CustomAssetReader(Box<dyn AssetReader>);

impl AssetReader for CustomAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let asset_manager = get_asset_manager()
                .expect("Bevy must be setup with the #[bevy_main] macro on Android");
            let mut opened_asset = asset_manager
                .open(&CString::new(path.to_str().unwrap()).unwrap())
                .ok_or(AssetReaderError::NotFound(path.to_path_buf()))?;
            let bytes = opened_asset.get_buffer()?;
            let reader: Box<Reader> = Box::new(VecReader::new(bytes.to_vec()));
            Ok(reader)
        })
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let meta_path = get_meta_path(path);
            let asset_manager = get_asset_manager()
                .expect("Bevy must be setup with the #[bevy_main] macro on Android");
            let mut opened_asset = asset_manager
                .open(&CString::new(meta_path.to_str().unwrap()).unwrap())
                .ok_or(AssetReaderError::NotFound(meta_path))?;
            let bytes = opened_asset.get_buffer()?;
            let reader: Box<Reader> = Box::new(VecReader::new(bytes.to_vec()));
            Ok(reader)
        })
    }

    fn read_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        let stream: Box<PathStream> = Box::new(EmptyPathStream);
        error!("Reading directories is not supported with the AndroidAssetReader");
        Box::pin(async move { Ok(stream) })
    }

    fn is_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, std::result::Result<bool, AssetReaderError>> {
        error!("Reading directories is not supported with the AndroidAssetReader");
        Box::pin(async move { Ok(false) })
    }
}

pub(crate) fn get_meta_path(path: &Path) -> PathBuf {
    let mut meta_path = path.to_path_buf();
    let mut extension = path
        .extension()
        .expect("asset paths must have extensions")
        .to_os_string();
    extension.push(".meta");
    meta_path.set_extension(extension);
    meta_path
}

/// A [`PathBuf`] [`Stream`] implementation that immediately returns nothing.
struct EmptyPathStream;

impl Stream for EmptyPathStream {
    type Item = PathBuf;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}