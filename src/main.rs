use bevy::{asset::LoadedFolder, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            JsonAssetPlugin::<TerrainLine>::new(&["TerrainLine.json"]),
        ))
        .add_systems(Startup, setup)        
        .run();
}

#[derive(Serialize, Deserialize, Asset, TypePath)]
struct TerrainLine {

}

fn setup(asset_server: Res<AssetServer>, mut local: Local<Option<Handle<LoadedFolder>>>) {
    *local = Some(asset_server.load_folder(""));
}