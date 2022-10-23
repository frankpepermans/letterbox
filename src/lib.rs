#![feature(binary_heap_retain)]

use bevy::prelude::*;
use game::coordinates::Coordinates;

pub mod actors;
pub mod game;

pub struct GridSize(pub Coordinates);

#[derive(Component, Debug)]
pub struct Position(pub Coordinates);

pub struct RobotCount(pub i8);
