#![feature(binary_heap_retain)]

use bevy::prelude::*;
use game::coordinates::Coordinates;

pub mod actors;
pub mod game;

pub struct GridSize(pub (usize, usize));

pub struct NodeSize(pub (f32, f32));

#[derive(Component, Debug)]
pub struct Position(pub Coordinates);

pub struct RobotCount(pub i8);
