# Unnamed
A work in progress OpenGL Game Engine, written in Rust. 
It uses ECS for managing the world state and entities.

# Usage
In order to test the current state of the engine, you must run these commands.
```
git clone git@github.com:emiflake/opengl-appetizer.git
cd opengl-appetizer
cargo run
```
# ECS
It uses SPECS Parallel ECS to create the ECS system.
It uses GLFW for handling the window management and the Rust GL bindings for rendering the world.
