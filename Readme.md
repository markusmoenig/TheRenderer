A fast, state based 2D renderer for apps and games.

TheRenderer is integrated in (TheFramework)[https://github.com/markusmoenig/TheFramework], a trait based app and game framework, but can be used independently as well.

## Features

* Lay out shapes in spaces, which are rectangular areas on your screen.
* Apply inbuilt or custom shaders to shapes and space backgrounds.
* Shapes have states, each state can have a set of properties (like the radius of a circle). TheRenderer smoothly animates between state changes, for example in an app when a user clicks a button, or in a game when the shape collides with another shape.
* TheRenderer only updates the screen when needed.
* Fast multi-threaded rendering and blending of shapes and spaces.

The integration in (TheFramework)[https://github.com/markusmoenig/TheFramework] allows the creation of native, cross platform applications and games.

## Goals

* Fast rendering of 2D shapes on the CPU.
* Fast rendering of 3D triangles (coming soon).
* Physics integration for games.
* More to come.

