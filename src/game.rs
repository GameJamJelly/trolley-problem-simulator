//! Top-level game logic.

use crate::animation::*;
use crate::assets::*;
use crate::components::*;
use crate::constants::*;
use crate::end_screen::*;
use crate::menu::*;
use crate::resources::*;
use crate::scenario::*;
use crate::states::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;

/// Loads game assets and stores them as resources.
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Store assets as resources
    commands.insert_resource(TrolleyFrontRes(load_embedded_image(
        &asset_server,
        "original-trolley-front",
    )));
    commands.insert_resource(TrolleyTurnRes(load_embedded_image(
        &asset_server,
        "original-trolley-turn",
    )));
    commands.insert_resource(TrolleySideRes(load_embedded_image(
        &asset_server,
        "original-trolley-side",
    )));
    commands.insert_resource(TrolleySwitchedRes(load_embedded_image(
        &asset_server,
        "original-trolley-switched",
    )));

    // UI camera
    commands.spawn(Camera2dBundle::default());
}

/// Turns the trolley as it goes to track B.
fn turn_trolley_switched_start(
    mut trolley_texture: Query<&mut Handle<Image>, With<TrolleyTexture>>,
    trolley_switched_texture: Res<TrolleySwitchedRes>,
) {
    *trolley_texture.single_mut() = trolley_switched_texture.clone();
}

/// Turns the trolley back to normal as it continues down track B.
fn turn_trolley_switched_end(
    mut trolley_texture: Query<&mut Handle<Image>, With<TrolleyTexture>>,
    trolley_side_texture: Res<TrolleySideRes>,
) {
    *trolley_texture.single_mut() = trolley_side_texture.clone();
}

/// Displays the configured wounded texture on track A when appropriate.
fn show_wounded_track_a(
    animation_config: Res<AnimationConfigRes>,
    scenario_index: Res<State<ScenarioIndexState>>,
    animation_index: Res<State<AnimationIndexState>>,
    mut hostage_texture: Query<&mut Handle<Image>, With<HostagesTrackATexture>>,
    image_assets: Res<ImageAssetMap>,
) {
    let this_scenario_animations = &animation_config[scenario_index.unwrap()];
    let this_animation = &this_scenario_animations[animation_index.unwrap()];

    if let Some(wounded_texture_name) = &this_animation.wounded_texture {
        let wounded_texture = image_assets.get_by_name(wounded_texture_name);
        *hostage_texture.single_mut() = wounded_texture;
    }
}

/// Displays the configured wounded texture on track B when appropriate.
fn show_wounded_track_b(
    animation_config: Res<AnimationConfigRes>,
    scenario_index: Res<State<ScenarioIndexState>>,
    animation_index: Res<State<AnimationIndexState>>,
    mut hostage_texture: Query<&mut Handle<Image>, With<HostagesTrackBTexture>>,
    image_assets: Res<ImageAssetMap>,
) {
    let this_scenario_animations = &animation_config[scenario_index.unwrap()];
    let this_animation = &this_scenario_animations[animation_index.unwrap()];

    if let Some(wounded_texture_name) = &this_animation.wounded_texture {
        let wounded_texture = image_assets.get_by_name(wounded_texture_name);
        *hostage_texture.single_mut() = wounded_texture;
    }
}

/// Generates a standard animation for the trolley on track A.
fn standard_animation_track_a(wounded_texture: Option<&str>) -> Animation {
    let mut animation = Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
        .on_lever_state(LeverState::Normal)
        .node(
            AnimationNode::new(
                2.0,
                Transform::from_xyz(
                    STANDARD_HOSTAGES_POS_TRACK_A.x,
                    STANDARD_HOSTAGES_POS_TRACK_A.y,
                    0.0,
                ),
            )
            .end_action(show_wounded_track_a),
        )
        .node(AnimationNode::new(
            4.0,
            Transform::from_translation(Vec3::new(900.0, 445.0, 0.0)),
        ));

    if let Some(texture) = wounded_texture {
        animation = animation.with_wounded_texture(texture);
    }

    animation
}

/// Generates a standard animation for the trolley on track B.
fn standard_animation_track_b(wounded_texture: Option<&str>) -> Animation {
    let mut animation = Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
        .on_lever_state(LeverState::Pulled)
        .with_start_action(turn_trolley_switched_start)
        .node(
            AnimationNode::new(
                1.0,
                Transform::from_translation(Vec3::new(400.0, 190.0, 0.0)),
            )
            .end_action(turn_trolley_switched_end),
        )
        .node(
            AnimationNode::new(
                1.0,
                Transform::from_xyz(
                    STANDARD_HOSTAGES_POS_TRACK_B.x,
                    STANDARD_HOSTAGES_POS_TRACK_B.y,
                    0.0,
                ),
            )
            .end_action(show_wounded_track_b),
        )
        .node(AnimationNode::new(
            4.0,
            Transform::from_translation(Vec3::new(900.0, 260.0, 0.0)),
        ));

    if let Some(texture) = wounded_texture {
        animation = animation.with_wounded_texture(texture);
    }

    animation
}

/// The plugin which orchestrates the game logic.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Add app plugins
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".to_owned()),
                    resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
            EmbeddedAssetPlugin,
        ));

        // Insert stateful values
        app.insert_state(GameState::InMenu)
            .insert_state(ScenarioIndexState(None))
            .insert_state(LeverState::Normal);

        // Add game setup and menu screen systems.
        // Because the first two are chained, the user cannot possibly proceed
        // before all assets are loaded, since the "Play" button won't be shown
        // until then.
        app.add_systems(Startup, setup_game);
        app.add_systems(OnEnter(GameState::InMenu), setup_menu_screen);
        app.add_systems(
            Update,
            update_menu_screen.run_if(in_state(GameState::InMenu)),
        );
        app.add_systems(OnExit(GameState::InMenu), cleanup_menu_screen);

        // Add end screen systems
        app.add_systems(OnEnter(GameState::EndScreen), setup_end_screen);
        app.add_systems(
            Update,
            update_end_screen.run_if(in_state(GameState::EndScreen)),
        );
        app.add_systems(OnExit(GameState::EndScreen), cleanup_end_screen);

        // Create scenarios

        // Original
        let scenario_original = Scenario::builder()
            .text("A trolley is headed towards a group of five people. You can intervene and pull the lever to switch the tracks so that only one person will be killed. Do you pull the lever?")
            .duration(20.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .hostages_track_b_normal_texture("original-hostage-1")
            .animation(standard_animation_track_a(Some("original-hostage-5-wounded")))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .build();

        // Age
        let scenario_age = Scenario::builder()
            .text("Everyone on the lower track is 90 years old. There is a child on the upper track. Do you pull the lever?")
            .duration(20.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("age-hostage-10")
            .hostages_track_b_normal_texture("original-hostage-1")
            .animation(standard_animation_track_a(Some("age-hostage-10-wounded")))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .build();

        // Victim
        let scenario_victim = Scenario::builder()
            .text("The person on the track claims that \"Society needs to pull the lever.\" You have told them to just walk off the tracks. Is this person really the victim if they have knowingly done this to themselves? Will you be responsible if they die?")
            .duration(25.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("victim")
            .animation(standard_animation_track_a(Some("victim-wounded")))
            .animation(standard_animation_track_b(None))
            .build();

        // Darwinism

        // Clone

        // Cool hat

        // Loop

        // Professors

        // Loan forgiveness

        // Lobster

        // Cliff

        // Shopping cart

        // Born lever puller

        // Double it

        // Thomas the tank engine

        // Youtube prank

        // Self

        // Add scenarios
        app.add_plugins(
            ScenarioCollectionPlugin::builder()
                .scenario(scenario_original)
                .scenario(scenario_age)
                .scenario(scenario_victim)
                .build(),
        );
    }
}
