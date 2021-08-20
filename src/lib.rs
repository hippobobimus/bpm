//! # bpm - Bob's Physics Module
//!
//! A simple physics engine integrated with the Bevy game engine.
//!
//! Incorporates basic 3D rigid-body dynamics including collisions.

/// Default settings.
mod constants;
/// Components, component bundles and plugins used to add physics interactions to your Bevy
/// application.
pub mod physics;
/// Components and plugins that support specific user interactions with Entitys' physics components.
pub mod user_interaction;
