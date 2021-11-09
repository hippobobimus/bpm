//! # bpm - Bob's Physics Module
//!
//! A simple physics engine integrated with the Bevy game engine.
//!
//! Incorporates basic 3D rigid-body dynamics including collisions.

/// Default settings.
mod constants;

/// Components, bundles and plugins to add debugging support to physics interactions.
pub mod debug;

/// Components, component bundles and plugins used to add physics interactions to a Bevy
/// application.
pub mod physics;

/// Components and plugins that support specific user interactions with Entitys' physics components.
pub mod user_interaction;
