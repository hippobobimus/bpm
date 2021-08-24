# Overview

A rudimentary 3D rigid-body physics simulation engine, integrated with the
[Bevy](https://bevyengine.org/) game engine.

# Roadmap

- [x] Force & torque generation
- [x] Collision detection
- [ ] Collision resolution

# Implementation

The physics engine has been built to utilise Bevy's ECS (Entity Component System) and includes a
Bevy plugin and custom components that can be used to add physics simulation to a Bevy project.

### Forces and Torques

A variety of specific components are used to generate the forces and torques acting on an Entity.
For example, the Gravity component generates a downward force proportional to the Entity's mass.
This is then added to any other forces and torques that have been generated in the current frame
and used to calculate the Entity's linear/angular velocity and updated transform.

### Collision Detection

This is split into two main parts; broad-phase and narrow-phase.

The __broad-phase__ is used to perform a relatively 'cheap' procedure to determine a subset of entity
pairs that may be intersecting. All of the primitive shapes in use in the physics engine have a
bounding sphere which is utilised to insert them into a spatial partitioning data structure.
Specifically, I created an oct-tree implementation for this purpose. Once every frame the oct-tree
is queried for entities in close proximity and these are passed in pairs to the narrow-phase.

In the __narrow-phase__, candidate pairs are evaluated. When found to be intersecting, a Contact is
generated which contains key information regarding penetration depth, contact point and contact
normal that will be used for collision resolution.

### Collision Resolution

In development...

# Examples

Currently only a very basic example is available. This will change once collision resolution has
been implemented, allowing for more interesting interactions.

### Pre-requisites

An up-to-date Rust installation is required (tested with v1.53). To install Rust, please visit
their website [here](https://www.rust-lang.org/tools/install).

### Run

Clone this GitHub repository, navigate to its directory and run the following terminal command.

    $ cargo --examples <EXAMPLE_NAME>

The following examples are currently available:

#### primative_playground

A very simple example in which a number of static cuboids and spheres are spawned randomly in 3D
space, along with a rotating 'fan' (elongated cuboid). A single sphere can be controlled with the
keyboard and upon 'collision' with any other object or plane, the contact data generated will be
printed to the terminal.

Controls:

- Up/Down/Left/Right = movement in the x-z plane.
- Spacebar = movement upwards in the positive y direction.

# References

While developing this library I used a wide variety of resources. The most significant of these
were:

- Ian Millington, Game Physics Engine Development (Second Edition), CRC Press [2010].
- Christer Ericson, Real-Time Collision Detection, CRC Press [2005].
