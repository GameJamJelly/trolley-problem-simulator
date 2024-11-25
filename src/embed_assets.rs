//! Asset embedding helpers.

use bevy::prelude::*;

/// Embed a binary asset.
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

/// Load an embedded asset.
pub fn load_embedded_asset<A>(asset_server: &AssetServer, path: &str) -> Handle<A>
where
    A: Asset,
{
    asset_server.load(format!("embedded://{}", path))
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
        );
    }
}
