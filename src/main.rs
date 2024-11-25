#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod embed_assets;

use crate::embed_assets::*;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

/// The track texture component.
#[derive(Component)]
struct TrackTexture;

/// The resource for the track texture.
#[derive(Resource, Deref, DerefMut)]
struct TracksNormalRes(Handle<Image>);

/// The resource for the track texture with the lever pulled.
#[derive(Resource, Deref, DerefMut)]
struct TracksSwitchedRes(Handle<Image>);

/// The lever/player texture component.
#[derive(Component)]
struct LeverPlayerTexture;

/// The resource for the lever/player texture.
#[derive(Resource, Deref, DerefMut)]
struct LeverPlayerNormalRes(Handle<Image>);

/// The resource for the lever/player texture with the lever pulled.
#[derive(Resource, Deref, DerefMut)]
struct LeverPlayerSwitchedRes(Handle<Image>);

/// The state of the lever.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum LeverState {
    /// The lever has not been pulled.
    #[default]
    Normal,
    /// The lever has been pulled.
    Pulled,
}

/// Sets up the track background.
fn setup_track_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tracks_normal = load_embedded_asset(&asset_server, "assets/images/tracks-straight.png");
    let tracks_switched = load_embedded_asset(&asset_server, "assets/images/tracks-pull.png");
    let lever_player_normal = load_embedded_asset(&asset_server, "assets/images/lever-default.png");
    let lever_player_switched = load_embedded_asset(&asset_server, "assets/images/lever-pull.png");

    // UI camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the track texture
    commands.spawn((
        SpriteBundle {
            texture: tracks_normal.clone(),
            ..default()
        },
        TrackTexture,
    ));

    // Spawn the lever/player texture
    commands.spawn((
        SpriteBundle {
            texture: lever_player_normal.clone(),
            ..default()
        },
        LeverPlayerTexture,
    ));

    // Add track texture and lever/player resources
    commands.insert_resource(TracksNormalRes(tracks_normal));
    commands.insert_resource(TracksSwitchedRes(tracks_switched));
    commands.insert_resource(LeverPlayerNormalRes(lever_player_normal));
    commands.insert_resource(LeverPlayerSwitchedRes(lever_player_switched));
}

/// Handles a left click event.
fn handle_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    lever_state: Res<State<LeverState>>,
    mut next_lever_state: ResMut<NextState<LeverState>>,
    mut texture_set: ParamSet<(
        Query<&mut Handle<Image>, With<TrackTexture>>,
        Query<&mut Handle<Image>, With<LeverPlayerTexture>>,
    )>,
    track_normal_texture: Res<TracksNormalRes>,
    track_switched_texture: Res<TracksSwitchedRes>,
    lever_player_normal_texture: Res<LeverPlayerNormalRes>,
    lever_player_switched_texture: Res<LeverPlayerSwitchedRes>,
) {
    let lever_rect = Rect::new(346.0, 135.0, 410.0, 202.0);

    if let Some(mouse_pos) = windows.single().cursor_position() {
        if lever_rect.contains(mouse_pos) {
            match lever_state.get() {
                LeverState::Normal => {
                    next_lever_state.set(LeverState::Pulled);
                    *texture_set.p0().single_mut() = track_switched_texture.clone();
                    *texture_set.p1().single_mut() = lever_player_switched_texture.clone();
                }
                LeverState::Pulled => {
                    next_lever_state.set(LeverState::Normal);
                    *texture_set.p0().single_mut() = track_normal_texture.clone();
                    *texture_set.p1().single_mut() = lever_player_normal_texture.clone();
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".to_owned()),
                    resolution: WindowResolution::new(800.0, 600.0),
                    ..default()
                }),
                ..default()
            }),
            EmbeddedAssetPlugin,
        ))
        .init_state::<LeverState>()
        .add_systems(Startup, setup_track_background)
        .add_systems(
            Update,
            handle_click.run_if(input_just_pressed(MouseButton::Left)),
        )
        .run();
}
