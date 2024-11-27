//! Asset embedding helpers.

use bevy::prelude::*;

/// Embeds a binary asset.
macro_rules! insert_embedded_assets {
    ( $app:ident, $( $path:expr ),+ $(,)? ) => {{
        let embedded = $app
            .world_mut()
            .resource_mut::<::bevy::asset::io::embedded::EmbeddedAssetRegistry>();
        $(
            embedded.insert_asset(
                ::std::path::PathBuf::from(file!()).join($path),
                ::std::path::Path::new($path.strip_prefix("../").unwrap_or($path)),
                include_bytes!($path),
            );
        )+
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

/// Plugin to embed app assets.
pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        insert_embedded_assets!(
            app,
            "../assets/images/tracks-straight.png",
            "../assets/images/tracks-pull.png",
            "../assets/images/lever-default.png",
            "../assets/images/lever-pull.png",
            "../assets/images/hostage-1.png",
            "../assets/images/hostage-5.png",
            "../assets/images/trolley-front.png",
            "../assets/images/trolley-turn.png",
            "../assets/images/trolley-side.png",
        );
    }
}
