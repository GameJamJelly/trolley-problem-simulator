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
            3.0,
            Transform::from_translation(Vec3::new(900.0, 260.0, 0.0)),
        ));

    if let Some(texture) = wounded_texture {
        animation = animation.with_wounded_texture(texture);
    }

    animation
}

/// The transform function for the loop animation.
fn loop_animation(
    start_transform: Transform,
    _end_transform: Transform,
    progress: f32,
) -> Transform {
    let loop_radius = 70.0;
    let loop_center = Vec2::new(
        start_transform.translation.x,
        start_transform.translation.y - loop_radius,
    );
    let progress_angle = -std::f32::consts::TAU * (progress - 0.25);

    Transform::IDENTITY
        .with_translation(Vec3::new(
            loop_center.x + loop_radius * progress_angle.cos(),
            loop_center.y + loop_radius * progress_angle.sin(),
            0.0,
        ))
        .with_rotation(Quat::from_rotation_z(std::f32::consts::TAU * progress))
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

        // Clone
        let scenario_clone = Scenario::builder()
            .text("If you pull the lever, the trolley will kill a clone of Hitler. The clone has all of Hitler's memories and fully believes that they are him and that they committed all of his actions, even though they didn't. Do you pull the lever?")
            .duration(25.0)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_b_normal_texture("original-hostage-1")
            .animation(standard_animation_track_a(None))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .build();

        // TODO: Cliff

        // Cool hat
        let scenario_cool_hat = Scenario::builder()
            .text("You can pull the lever to save five people, but you would kill the guy with the really cool hat. Do you pull the lever?")
            .duration(15.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .hostages_track_b_normal_texture("hat-hostage")
            .animation(standard_animation_track_a(Some("original-hostage-5-wounded")))
            .animation(standard_animation_track_b(Some("hat-hostage-wounded")))
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
        let scenario_darwinism = Scenario::builder()
            .text("A person on the lower track is not tied down, and can walk away if he is smart enough to recognize the danger of the trolley. If he is not smart enough, natural selection will do its job and eliminate him from the gene pool, making humanity smarter. The upper track has 5 untied persons, which could potentially apply natural selection to 4 extra people, making humanity even smarter. Do you pull the lever?")
            .duration(30.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("darwinism-hostage-1")
            .hostages_track_b_normal_texture("darwinism-hostage-5")
            .animation(standard_animation_track_a(Some("darwinism-hostage-1-wounded")))
            .animation(standard_animation_track_b(Some("darwinism-hostage-5-wounded")))
            .build();

        // Loop
        let scenario_loop = Scenario::builder()
            .text("A trolley is headed towards a group of five people. You can pull the lever to only kill one person, but then the trolley won't do the totally sick loop-da-loop. Do you pull the lever?")
            .duration(20.0)
            .hostages_track_a_pos(STANDARD_HOSTAGES_POS_TRACK_A)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("loop-normal")
            .tracks_switched_texture("loop-switched")
            .lever_normal_texture("original-lever-normal")
            .lever_switched_texture("original-lever-switched")
            .hostages_track_a_normal_texture("original-hostage-5")
            .hostages_track_b_normal_texture("original-hostage-1")
            .animation(
                Animation::new(APPROACHING_TROLLEY_SIDE_END_TRANSFORM)
                    .on_lever_state(LeverState::Normal)
                    .with_wounded_texture("original-hostage-5-wounded")
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
                    .node(AnimationNode::new(0.25, Transform::from_xyz(570.0, 305.0, 0.0)))
                    .node(AnimationNode::new(2.0, Transform::from_xyz(570.0, 305.0, 0.0)).animation_fn(loop_animation))
                    .node(AnimationNode::new(
                        2.0,
                        Transform::from_translation(Vec3::new(900.0, 445.0, 0.0)),
                    )))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .build();

        // Professors
        let scenario_professors = Scenario::builder()
            .text("A trolley is headed towards five philosophy professors who like to invent trolley problems. You can pull the lever to switch the track, but then the trolley will kill the nice guy who never had a thought of inventing trolley problems.")
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

        // TODO: Loan forgiveness

        // TODO: Lobster

        // TODO: Shopping cart

        // Born lever puller
        let scenario_born_lever_puller = Scenario::builder()
            .text("If you do nothing, nobody will get hurt. However, you are a born lever-puller. Do you pull the lever?")
            .duration(15.0)
            .hostages_track_b_pos(STANDARD_HOSTAGES_POS_TRACK_B)
            .tracks_normal_texture("original-tracks-normal")
            .tracks_switched_texture("original-tracks-switched")
            .lever_normal_texture("born-lever-puller-normal")
            .lever_switched_texture("born-lever-puller-switched")
            .hostages_track_b_normal_texture("original-hostage-1")
            .animation(standard_animation_track_a(None))
            .animation(standard_animation_track_b(Some("original-hostage-1-wounded")))
            .build();

        // TODO: Double it

        // TODO: Thomas the tank engine

        // TODO: Youtube prank

        // TODO: Self

        // Add scenarios
        app.add_plugins(
            ScenarioCollectionPlugin::builder()
                .scenario(scenario_original)
                .scenario(scenario_age)
                .scenario(scenario_clone)
                .scenario(scenario_cool_hat)
                .scenario(scenario_victim)
                .scenario(scenario_darwinism)
                .scenario(scenario_loop)
                .scenario(scenario_professors)
                .scenario(scenario_born_lever_puller)
                .build(),
        );
    }
}
