//! Trolley animation implementation.

use crate::components::*;
use crate::resources::*;
use crate::states::*;
use crate::util::*;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

/// A linear animation transformation. This is the default animation function.
pub fn linear_animation(
    start_transform: Transform,
    end_transform: Transform,
    progress: f32,
) -> Transform {
    let start_point_x = start_transform.translation.x;
    let start_point_y = start_transform.translation.y;
    let start_rotation = start_transform.rotation.z;
    let start_scale = start_transform.scale.x;
    let end_point_x = end_transform.translation.x;
    let end_point_y = end_transform.translation.y;
    let end_rotation = end_transform.rotation.z;
    let end_scale = end_transform.scale.x;

    let transformed_x = point_between(start_point_x, end_point_x, progress);
    let transformed_y = point_between(start_point_y, end_point_y, progress);
    let transformed_rotation = point_between(start_rotation, end_rotation, progress);
    let transformed_scale = point_between(start_scale, end_scale, progress);

    Transform::IDENTITY
        .with_translation(Vec3::new(transformed_x, transformed_y, 0.0))
        .with_rotation(Quat::from_rotation_z(transformed_rotation))
        .with_scale(Vec3::new(transformed_scale, transformed_scale, 1.0))
}

/// Sets the animation index state once [`AnimationState::Running`] is entered.
fn set_animation_index_state(
    scenario_index_state: Res<State<ScenarioIndexState>>,
    mut next_animation_index_state: ResMut<NextState<AnimationIndexState>>,
    mut next_animation_node_index_state: ResMut<NextState<AnimationNodeIndexState>>,
    animation_config: Res<AnimationConfigRes>,
    lever_state: Res<State<LeverState>>,
) {
    let scenario_index = scenario_index_state.unwrap();

    next_animation_node_index_state.set(AnimationNodeIndexState(Some(0)));

    for (animation_index, animation) in animation_config.0[scenario_index].iter().enumerate() {
        if match animation.lever_state_condition {
            Some(desired_state) => desired_state == **lever_state,
            None => true,
        } {
            next_animation_index_state.set(AnimationIndexState(Some(animation_index)));
        }
    }
}

/// Unsets the animation index state once [`AnimationState::Running`] is exited.
fn unset_animation_index_state(
    mut commands: Commands,
    mut next_animation_index_state: ResMut<NextState<AnimationIndexState>>,
    mut next_animation_node_index_state: ResMut<NextState<AnimationNodeIndexState>>,
) {
    next_animation_index_state.set(AnimationIndexState(None));
    next_animation_node_index_state.set(AnimationNodeIndexState(None));
    commands.remove_resource::<AnimationSectionTimer>();
}

/// Immediately sets the animation state to [`AnimationState::Complete`].
fn goto_end_animation(mut next_animation_state: ResMut<NextState<AnimationState>>) {
    next_animation_state.set(AnimationState::Complete);
}

/// Sets the animation section timer resource.
fn set_animation_section_timer(
    mut commands: Commands,
    animation_config: Res<AnimationConfigRes>,
    scenario_index: Res<State<ScenarioIndexState>>,
    animation_index: Res<State<AnimationIndexState>>,
    animation_node_index: Res<State<AnimationNodeIndexState>>,
) {
    let this_scenario_animations = &animation_config[scenario_index.unwrap()];
    let this_animation = &this_scenario_animations[animation_index.unwrap()];
    let this_node = &this_animation.nodes[animation_node_index.unwrap()];
    let duration = this_node.duration;

    commands.insert_resource(AnimationSectionTimer(Timer::from_seconds(
        duration,
        TimerMode::Once,
    )));
}

/// Updates an animation every tick.
fn animation_update(
    animation_config: Res<AnimationConfigRes>,
    scenario_index: Res<State<ScenarioIndexState>>,
    animation_index: Res<State<AnimationIndexState>>,
    animation_node_index: Res<State<AnimationNodeIndexState>>,
    mut next_animation_node_index_state: ResMut<NextState<AnimationNodeIndexState>>,
    time: Res<Time>,
    mut animation_section_timer: ResMut<AnimationSectionTimer>,
    mut trolley_transform: Query<&mut Transform, With<TrolleyTexture>>,
) {
    if animation_section_timer.tick(time.delta()).just_finished() {
        next_animation_node_index_state.set(AnimationNodeIndexState(Some(
            animation_node_index.unwrap() + 1,
        )));
    }

    let this_scenario_animations = &animation_config[scenario_index.unwrap()];
    let this_animation = &this_scenario_animations[animation_index.unwrap()];
    let this_node = &this_animation.nodes[animation_node_index.unwrap()];

    let from_transform = if animation_node_index.unwrap() == 0 {
        this_animation.start_transform
    } else {
        this_animation.nodes[animation_node_index.unwrap() - 1].transform
    };

    let progress = 1.0 - (animation_section_timer.remaining_secs() / this_node.duration);

    let new_transform = (this_node.animation_fn)(from_transform, this_node.transform, progress);
    *trolley_transform.single_mut() = normalize_transform_to_canvas(new_transform);
}

/// A wrapper around an animation function.
///
/// Animation functions must take the following parameters:
///  - The starting `Transform`.
///  - The ending `Transform`.
///  - An `f32` representing the progress of the animation as a value between 0
///    and 1.
///
/// Animation functions must return the resulting `Transform`.
#[derive(Clone, Deref)]
pub struct AnimationFn(Arc<dyn Fn(Transform, Transform, f32) -> Transform + Send + Sync>);

impl<F> From<F> for AnimationFn
where
    F: Fn(Transform, Transform, f32) -> Transform + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self(Arc::new(value))
    }
}

/// A single node in an animation. Construct this using the builder pattern.
pub struct AnimationNode {
    /// The duration in seconds of this section of the animation.
    duration: f32,
    /// The end transformation value.
    transform: Transform,
    /// The function to model this section of the animation transformation.
    animation_fn: AnimationFn,
    /// An optional system to run at the end of this section of the animation.
    end_action: Option<SystemConfigs>,
}

impl AnimationNode {
    /// Creates a new animation node. A linear animation transformation is used
    /// by default.
    pub fn new(duration: f32, transform: Transform) -> Self {
        Self {
            duration,
            transform,
            animation_fn: linear_animation.into(),
            end_action: None,
        }
    }

    /// Configures the animation function for this node.
    ///
    /// Animation functions must take the following parameters:
    ///  - The starting `Transform`.
    ///  - The ending `Transform`.
    ///  - An `f32` representing the progress of the animation as a value between 0
    ///    and 1.
    ///
    /// Animation functions must return the resulting `Transform`.
    pub fn animation_fn(mut self, f: impl Into<AnimationFn>) -> Self {
        self.animation_fn = f.into();
        self
    }

    /// Configures a system to run at the end of this section of the animation.
    pub fn end_action<M>(mut self, action: impl IntoSystemConfigs<M>) -> Self {
        self.end_action = Some(action.into_configs());
        self
    }
}

/// A trolley animation within a scenario. Construct this using the builder
/// pattern.
pub struct Animation {
    /// An optional lever-state-related condition to decide whether to run the
    /// animation.
    lever_state_condition: Option<LeverState>,
    /// The animation start transformation.
    start_transform: Transform,
    /// An optional system to run at the start of the animation.
    start_action: Option<SystemConfigs>,
    /// The collection of animation nodes.
    nodes: Vec<AnimationNode>,
    /// The optional wounded texture.
    wounded_texture: Option<String>,
}

impl Animation {
    /// Creates a new animation, given the starting transformation.
    pub fn new(start_transform: Transform) -> Self {
        Self {
            lever_state_condition: None,
            start_transform,
            start_action: None,
            nodes: Vec::new(),
            wounded_texture: None,
        }
    }

    /// Configures a system to run at the start of the animation.
    pub fn with_start_action<M>(mut self, action: impl IntoSystemConfigs<M>) -> Self {
        self.start_action = Some(action.into_configs());
        self
    }

    /// Configures the lever state condition that will decide whether to run the
    /// animation.
    pub const fn on_lever_state(mut self, lever_state_condition: LeverState) -> Self {
        self.lever_state_condition = Some(lever_state_condition);
        self
    }

    /// Adds a new node to the animation.
    pub fn node(mut self, node: AnimationNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// Configures the wounded texture.
    pub fn with_wounded_texture(mut self, wounded_texture: &str) -> Self {
        self.wounded_texture = Some(wounded_texture.to_owned());
        self
    }
}

/// A plugin to simplify the configuration of trolley animations.
pub struct AnimationCollectionPlugin {
    /// The list of scenario animations. Normally, this could just be a
    /// `Vec<Vec<Animation>>`, but [`Plugin::build`] takes `&self`.
    animations: Mutex<Option<Vec<Vec<Animation>>>>,
}

impl AnimationCollectionPlugin {
    /// Creates a new animation collection plugin, given a collection of
    /// scenario animations. The outer `Vec` represents the collection of
    /// scenarios, and each inner `Vec` represents the specific scenario's
    /// animations.
    pub const fn new(animations: Vec<Vec<Animation>>) -> Self {
        Self {
            animations: Mutex::new(Some(animations)),
        }
    }
}

impl Plugin for AnimationCollectionPlugin {
    fn build(&self, app: &mut App) {
        let maybe_animations = self.animations.lock().unwrap().take();

        if let Some(scenario_animations) = maybe_animations {
            let (animation_config, animations) = scenario_animations
                .into_iter()
                .map(|animations| {
                    animations
                        .into_iter()
                        .map(|animation| {
                            (
                                AnimationConfig {
                                    lever_state_condition: animation.lever_state_condition,
                                    start_transform: animation.start_transform,
                                    nodes: animation
                                        .nodes
                                        .iter()
                                        .map(|node| AnimationNodeConfig {
                                            duration: node.duration,
                                            transform: node.transform,
                                            animation_fn: node.animation_fn.clone(),
                                        })
                                        .collect(),
                                    wounded_texture: animation.wounded_texture.clone(),
                                },
                                animation,
                            )
                        })
                        .unzip::<_, _, Vec<_>, Vec<_>>()
                })
                .unzip::<_, _, Vec<_>, Vec<_>>();

            // Insert animation state, animation index state, and animation node
            // index state
            app.insert_state(AnimationState::Waiting);
            app.insert_state(AnimationIndexState(None));
            app.insert_state(AnimationNodeIndexState(None));

            // Insert animation config resource
            app.insert_resource(AnimationConfigRes(animation_config));

            // Add set and unset animation index systems, which will also clean
            // up the animation section timer resource
            app.add_systems(OnEnter(AnimationState::Running), set_animation_index_state);
            app.add_systems(OnExit(AnimationState::Running), unset_animation_index_state);

            for (scenario_index, scenario) in animations.into_iter().enumerate() {
                for (animation_index, animation) in scenario.into_iter().enumerate() {
                    let num_nodes = animation.nodes.len();

                    // Add animation start action system
                    if let Some(start_action) = animation.start_action {
                        app.add_systems(
                            OnEnter(AnimationIndexState(Some(animation_index))),
                            start_action.run_if(in_state(ScenarioIndexState(Some(scenario_index)))),
                        );
                    }

                    for (node_index, node) in animation.nodes.into_iter().enumerate() {
                        // Add system to set new animation section timer
                        // resource with `OnEnter` transition
                        app.add_systems(
                            OnEnter(AnimationNodeIndexState(Some(node_index))),
                            set_animation_section_timer.run_if(
                                in_state(ScenarioIndexState(Some(scenario_index)))
                                    .and_then(in_state(AnimationIndexState(Some(animation_index)))),
                            ),
                        );

                        // Add system to move the trolley and set next animation
                        // node index state when timer finishes
                        app.add_systems(
                            Update,
                            animation_update.run_if(
                                in_state(ScenarioIndexState(Some(scenario_index))).and_then(
                                    in_state(AnimationIndexState(Some(animation_index))).and_then(
                                        in_state(AnimationNodeIndexState(Some(node_index))),
                                    ),
                                ),
                            ),
                        );

                        // Add animation node action system
                        if let Some(end_action) = node.end_action {
                            app.add_systems(
                                OnExit(AnimationNodeIndexState(Some(node_index))),
                                end_action.run_if(
                                    in_state(ScenarioIndexState(Some(scenario_index))).and_then(
                                        in_state(AnimationIndexState(Some(animation_index))),
                                    ),
                                ),
                            );
                        }

                        // Add goto end system which sets the animation state
                        app.add_systems(
                            OnEnter(AnimationNodeIndexState(Some(num_nodes))),
                            goto_end_animation.run_if(
                                in_state(ScenarioIndexState(Some(scenario_index)))
                                    .and_then(in_state(AnimationIndexState(Some(animation_index)))),
                            ),
                        );
                    }
                }
            }
        }
    }
}
