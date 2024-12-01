//! Asset embedding helpers.

use crate::resources::*;
use crate::states::*;
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
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // Set up image assets
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

    // Set up audio assets
    let audio_asset_map = asset_map_paths
        .audio_asset_paths
        .iter()
        .map(|asset_path| {
            (
                asset_path.to_owned(),
                load_embedded_asset(&asset_server, asset_path),
            )
        })
        .collect();
    commands.insert_resource(AudioAssetMap(audio_asset_map));

    // Set up music assets
    let music_asset_map = asset_map_paths
        .music_asset_paths
        .iter()
        .map(|asset_path| {
            (
                asset_path.to_owned(),
                load_embedded_asset(&asset_server, asset_path),
            )
        })
        .collect();
    commands.insert_resource(MusicAssetMap(music_asset_map));

    // Go to the main menu
    next_game_state.set(GameState::InMenu);
}

/// Plugin to embed app assets.
pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        let image_asset_paths = insert_embedded_assets!(
            app,
            "../assets/images/age-hostage-10-wounded.png",
            "../assets/images/age-hostage-10.png",
            "../assets/images/born-lever-puller-normal.png",
            "../assets/images/born-lever-puller-switched.png",
            "../assets/images/cliff.png",
            "../assets/images/darwinism-hostage-1-wounded.png",
            "../assets/images/darwinism-hostage-1.png",
            "../assets/images/darwinism-hostage-5-wounded.png",
            "../assets/images/darwinism-hostage-5.png",
            "../assets/images/double-it-hostage-2-wounded.png",
            "../assets/images/double-it-hostage-2.png",
            "../assets/images/double-it-left-normal.png",
            "../assets/images/double-it-left-switched.png",
            "../assets/images/double-it-right-normal.png",
            "../assets/images/double-it-right-switched.png",
            "../assets/images/hat-hostage-wounded.png",
            "../assets/images/hat-hostage.png",
            "../assets/images/hat-lever-normal.png",
            "../assets/images/hat-lever-switched.png",
            "../assets/images/hat.png",
            "../assets/images/lobster-hostage-5-wounded.png",
            "../assets/images/lobster-hostage-5.png",
            "../assets/images/loop-normal.png",
            "../assets/images/loop-switched.png",
            "../assets/images/original-hostage-1-wounded.png",
            "../assets/images/original-hostage-1.png",
            "../assets/images/original-hostage-5-wounded.png",
            "../assets/images/original-hostage-5.png",
            "../assets/images/original-lever-normal.png",
            "../assets/images/original-lever-switched.png",
            "../assets/images/original-tracks-normal.png",
            "../assets/images/original-tracks-switched.png",
            "../assets/images/original-trolley-front.png",
            "../assets/images/original-trolley-switched.png",
            "../assets/images/original-trolley-side.png",
            "../assets/images/original-trolley-turn.png",
            "../assets/images/self-one-track.png",
            "../assets/images/self-standing.png",
            "../assets/images/self-wounded.png",
            "../assets/images/self.png",
            "../assets/images/shopping-cart-return-tracks.png",
            "../assets/images/shopping-cart-return.png",
            "../assets/images/shopping-cart-tracks-normal.png",
            "../assets/images/shopping-cart-tracks-switched.png",
            "../assets/images/shopping-cart.png",
            "../assets/images/thomas-the-tank-engine.png",
            "../assets/images/victim-wounded.png",
            "../assets/images/victim.png",
            "../assets/images/youtube-prank-dummy-wounded.png",
            "../assets/images/youtube-prank-dummy.png",
            "../assets/images/youtube-prank-tripod.png",
            "../assets/images/youtube-prank-youtubers-wounded.png",
            "../assets/images/youtube-prank-youtubers.png",
        );

        let audio_asset_paths = insert_embedded_assets!(
            app,
            "../assets/sounds/crash.mp3",
            "../assets/sounds/scream-1.mp3",
            "../assets/sounds/scream-2.mp3",
            "../assets/sounds/scream-3.mp3",
            "../assets/sounds/scream-4.mp3",
            "../assets/sounds/scream-5.mp3",
            "../assets/sounds/scream-6.mp3",
            "../assets/sounds/scream-7.mp3",
            "../assets/sounds/scream-8.mp3",
            "../assets/sounds/scream-9.mp3",
            "../assets/sounds/scream-10.mp3",
            "../assets/sounds/scream-11.mp3",
            "../assets/sounds/scream-12.mp3",
            "../assets/sounds/scream-13.mp3",
            "../assets/sounds/scream-14.mp3",
            "../assets/sounds/scream-15.mp3",
            "../assets/sounds/scream-16.mp3",
            "../assets/sounds/scream-17.mp3",
            "../assets/sounds/scream-18.mp3",
            "../assets/sounds/scream-19.mp3",
            "../assets/sounds/scream-20.mp3",
            "../assets/sounds/scream-21.mp3",
            "../assets/sounds/scream-22.mp3",
            "../assets/sounds/scream-23.mp3",
            "../assets/sounds/scream-24.mp3",
            "../assets/sounds/self-scream.mp3",
            "../assets/sounds/squash.mp3",
            "../assets/sounds/switch.mp3",
            "../assets/sounds/train-approaching.mp3",
        );

        let music_asset_paths = insert_embedded_assets!(
            app,
            "../assets/music/blue-lobster.mp3",
            "../assets/music/thomas-theme.mp3",
            "../assets/music/trolley-main.mp3",
            "../assets/music/win.mp3",
        );

        app.insert_resource(AssetMapPaths {
            image_asset_paths: image_asset_paths
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
            audio_asset_paths: audio_asset_paths
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
            music_asset_paths: music_asset_paths
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        })
        .add_systems(Startup, setup_asset_maps);
    }
}
