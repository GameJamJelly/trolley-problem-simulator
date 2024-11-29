//! Trolley animation implementation.

use crate::states::*;
use crate::util::*;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use std::sync::Arc;

/// A linear animation transformation. This is the default animation function.
pub fn linear_animation(
    start_transform: Transform,
    end_transform: Transform,
    progress: f32,
) -> Transform {
    let start_point_x = start_transform.translation.x;
    let start_point_y = start_transform.translation.y;
    let start_scale = start_transform.scale.x;
    let end_point_x = end_transform.translation.x;
    let end_point_y = end_transform.translation.y;
    let end_scale = end_transform.scale.x;

    let amount = 1.0 - progress;

    let transformed_x = point_between(start_point_x, end_point_x, amount);
    let transformed_y = point_between(start_point_y, end_point_y, amount);
    let transformed_scale = point_between(start_scale, end_scale, amount);

    Transform::IDENTITY
        .with_translation(normalize_translation_to_canvas(Vec2::new(
            transformed_x,
            transformed_y,
        )))
        .with_scale(Vec3::new(transformed_scale, transformed_scale, 1.0))
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
#[derive(Clone)]
pub struct AnimationNode {
    /// The duration in seconds of this section of the animation.
    duration: f32,
    /// The end transformation value.
    transform: Transform,
    /// The function to model this section of the animation transformation.
    animation_fn: AnimationFn,
    /// An optional system to run at the end of this section of the animation.
    end_action: Option<Arc<SystemConfigs>>,
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
        self.end_action = Some(Arc::new(action.into_configs()));
        self
    }
}

/// A trolley animation within a scenario. Construct this using the builder
/// pattern.
#[derive(Clone)]
pub struct Animation {
    /// An optional lever-state-related condition to decide whether to run the
    /// animation.
    lever_state_condition: Option<LeverState>,
    /// The animation start transformation.
    start_transform: Transform,
    /// An optional system to run at the start of the animation.
    start_action: Option<Arc<SystemConfigs>>,
    /// The collection of animation nodes.
    nodes: Vec<AnimationNode>,
}

impl Animation {
    /// Creates a new animation, given the starting transformation.
    pub fn new(start_transform: Transform) -> Self {
        Self {
            lever_state_condition: None,
            start_transform,
            start_action: None,
            nodes: Vec::new(),
        }
    }

    /// Configures a system to run at the start of the animation.
    pub fn with_start_action<M>(mut self, action: impl IntoSystemConfigs<M>) -> Self {
        self.start_action = Some(Arc::new(action.into_configs()));
        self
    }

    /// Configures the lever state condition that will decide whether to run the
    /// animation.
    pub const fn on_lever_state<M>(mut self, lever_state_condition: LeverState) -> Self {
        self.lever_state_condition = Some(lever_state_condition);
        self
    }

    /// Adds a new node to the animation.
    pub fn node(mut self, node: AnimationNode) -> Self {
        self.nodes.push(node);
        self
    }
}

/// A plugin to simplify the configuration of trolley animations.
pub struct AnimationCollectionPlugin(Vec<Vec<Animation>>);

impl AnimationCollectionPlugin {
    /// Creates a new animation collection plugin, given a collection of
    /// scenario animations. The outer `Vec` represents the collection of
    /// scenarios, and each inner `Vec` represents the specific scenario's
    /// animations.
    pub const fn new(animations: Vec<Vec<Animation>>) -> Self {
        Self(animations)
    }
}

impl Plugin for AnimationCollectionPlugin {
    fn build(&self, app: &mut App) {
        // NOTE: system starting the animation is unnecessary, as this will be triggered by the scenario setting the animation state when the trolley reaches the switch
        // NOTE: system checking if animation node timer just finished is unnecessary, as this can be checked in the system that moves the trolley

        // TODO: insert animation state
        // TODO: insert animation config resource (will require a clone of `self.0`)
        // TODO: add set and unset animation index systems, which will also remove the animation section timer resource when unset
        // TODO: add goto end system which sets the animation state

        for (scenario_index, scenario) in self.0.iter().enumerate() {
            for (animation_index, animation) in scenario.iter().enumerate() {
                // TODO: add animation start action system

                for (section_index, section) in animation.nodes.iter().enumerate() {
                    // TODO: add system to set new animation section timer resource with `OnEnter` transition
                    // TODO: add system to move the trolley and set next animation state when timer finishes
                    // TODO: add animation node action system
                }
            }
        }
    }
}
