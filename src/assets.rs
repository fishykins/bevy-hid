use bevy::{
    asset::{AssetLoader, LoadContext, LoadedFolder, io::Reader},
    prelude::*,
};
use thiserror::Error;

use crate::{
    buffers::BufferMap,
    device::{DeviceMap, DeviceAsset},
};

#[derive(Clone, Asset, Reflect)]
pub struct HidAsset {
    pub(crate) name: String,
    pub(crate) input_mapping: DeviceMap,
    pub(crate) buffer_map: BufferMap,
}

#[derive(Default, Reflect)]
pub(crate) struct HidAssetLoader;

#[derive(Error, Debug)]
pub enum HidAssetLoadError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not deserialize RON: {0}")]
    Ron(#[from] ron::de::SpannedError),
}

#[derive(Resource, Clone)]
pub(crate) struct TemporaryDeviceAssets {
    pub(crate) device_assets: Handle<LoadedFolder>,
}

impl HidAsset {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl AssetLoader for HidAssetLoader {
    type Asset = DeviceAsset;
    type Settings = ();
    type Error = HidAssetLoadError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<DeviceAsset>(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["hid.ron"]
    }
}
