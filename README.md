# Appetizer [![dependency status](https://deps.rs/repo/github/emiflake/appetizer/status.svg)](https://deps.rs/repo/github/emiflake/appetizer) [![build status](https://travis-ci.org/emiflake/appetizer.svg?branch=master)](https://travis-ci.org/emiflake/appetizer) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![Language](https://img.shields.io/github/languages/top/emiflake/appetizer)

A work in progress OpenGL Game Engine, written in Rust. 
It uses ECS for managing the world state and entities.

# Usage
In order to test the current state of the engine, you must run these commands.
```
git clone git@github.com:emiflake/appetizer.git
cd appetizer
cargo run
```
# ECS
It uses SPECS Parallel ECS to create the ECS system.
It uses GLFW for handling the window management and the Rust GL bindings for rendering the world.
