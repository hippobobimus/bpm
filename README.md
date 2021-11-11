# Overview

A rudimentary 3D rigid-body physics simulation engine, integrated with the
[Bevy](https://bevyengine.org/) game engine.

# Roadmap

- [x] Force & torque
    - [x] Force & torque generators
    - [x] Force & torque accumulation

- [x] Collision detection
    - [x] Broad phase; spatial partitioning / oct-tree implementation
    - [x] Narrow phase; contact generation
- [ ] Collision resolution
    - [x] Spheres and planes
    - [ ] Cuboids
    - [ ] Friction

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
Specifically, an oct-tree implementation was created for this purpose. Once every frame the oct-tree
is queried for entities in close proximity and these are passed in pairs to the narrow-phase.

In the __narrow-phase__, candidate pairs are evaluated. When found to be intersecting, a Contact is
generated which contains key information regarding penetration depth, contact point and contact
normal that will be used for collision resolution.

### Collision Resolution

This takes the form of corrections to colliding bodies positions and motion, to simulate the results
of the collision.

First, appropriate impulsive forces and impulsive torques are calculated and used to update the
linear and angular velocities of the bodies. Then linear movements and rotations are applied to the
bodies to remove any interpenetration between them.

# Examples

Some basic examples are available which, when run, show the simulation in action. Video clips of the
examples can be viewed above.

### Pre-requisites

An up-to-date Rust installation is required (tested with v1.53). To install Rust, please visit
their website [here](https://www.rust-lang.org/tools/install).

### Run

Clone this GitHub repository, navigate to its directory and run the following terminal command.

    $ cargo --examples <EXAMPLE_NAME> --release

In the examples, the large red ball is user controllable with the following key bindings:

- 'Up/Down/Left/Right' or 'h/j/k/l' = movement in the x-z plane.
- 'Spacebar' or 'w' = movement up in the positive y direction.
- 's' = movement down in the positive y direction.

The following examples are currently available:

#### drop_random_balls

Upon pressing the 'Return' key, 1000 coloured balls of variable size and position, along with the
user controllable red ball, are dropped above a green plane.

#### drop_stack_of_balls

Upon pressing the 'Return' key, a stack of coloured balls, along with the user controllable red ball, are
dropped above a green plane.

# References

While developing this library I used a wide variety of resources. The most significant of these
were:

- Ian Millington, Game Physics Engine Development (Second Edition), CRC Press [2010].
- Christer Ericson, Real-Time Collision Detection, CRC Press [2005].
