# Overview

TBD

# Implementation

TBD

# Roadmap

- [x] Force & torque generation
- [x] Collision detection
- [ ] Collision resolution

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

# References

While developing this library I used a wide variety of resources. The most significant of these
were:

Ian Millington, Game Physics Engine Development (Second Edition), CRC Press [2010].
Christer Ericson, Real-Time Collision Detection, CRC Press [2005].
