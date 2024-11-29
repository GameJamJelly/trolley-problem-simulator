//! Asset embedding helpers.

use crate::resources::*;
use bevy::prelude::*;

/// Embeds a binary asset.
macro_rules! insert_embedded_assets {
    ( $app:ident, $( $path:expr ),+ $(,)? ) => {{
        let embedded = $app
            .world_mut()
            .resource_mut::<::bevy::asset::io::embedded::EmbeddedAssetRegistry>();
        let mut asset_paths = Vec::new();

        $(
            let asset_path = $path.strip_prefix("../").unwrap_or($path);
            embedded.insert_asset(
                ::std::path::PathBuf::from(file!()).join($path),
                ::std::path::Path::new(asset_path),
                include_bytes!($path),
            );
            asset_paths.push(asset_path);
        )+

        asset_paths
    }};
}

/// Loads an embedded asset.
pub fn load_embedded_asset<A>(asset_server: &AssetServer, path: &str) -> Handle<A>
where
    A: Asset,
{
    asset_server.load(format!("embedded://{}", path))
}

/// Loads an embedded image asset.
pub fn load_embedded_image(asset_server: &AssetServer, image_name: &str) -> Handle<Image> {
    load_embedded_asset(asset_server, &format!("assets/images/{}.png", image_name))
}

/// Sets up asset mapping resources.
fn setup_asset_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    asset_map_paths: Res<AssetMapPaths>,
) {
    let image_asset_map = asset_map_paths
        .image_asset_paths
        .iter()
        .map(|asset_path| {
            (
                asset_path.to_owned(),
                load_embedded_asset(&asset_server, asset_path),
            )
        })
        .collect();
    commands.insert_resource(ImageAssetMap(image_asset_map));
}

/// Plugin to embed app assets.
pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        let image_asset_paths = insert_embedded_assets!(
            app,
            "../assets/images/tracks-default.png",
            "../assets/images/tracks-switched.png",
            "../assets/images/lever-default.png",
            "../assets/images/lever-pull.png",
            "../assets/images/hostage-1.png",
            "../assets/images/hostage-5.png",
            "../assets/images/trolley-front.png",
            "../assets/images/trolley-turn.png",
            "../assets/images/trolley-side.png",
        );

        app.insert_resource(AssetMapPaths {
            image_asset_paths: image_asset_paths
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        })
        .add_systems(Startup, setup_asset_maps);
    }
}
