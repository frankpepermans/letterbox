#![feature(binary_heap_retain)]

use std::time::Duration;

use bevy::prelude::*;
use game::{coordinates::Coordinates, node::Node};

pub mod actors;
pub mod game;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UserCursorPressedState {
    UP,
    DOWN,
}

#[derive(Resource)]
pub struct GridSize(pub (usize, usize));

#[derive(Resource)]
pub struct NodeSize(pub (f32, f32));

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Coordinates);

#[derive(Component, Debug, Clone, Copy)]
pub struct LivePosition(pub (f32, f32));

#[derive(Resource)]
pub struct RobotCount(pub i16);

#[derive(Component, Debug)]
struct Player {}

#[derive(Component)]
pub struct UserPosition {
    pub coordinates: Option<Coordinates>,
    pub cursor_pressed_state: Option<UserCursorPressedState>,
    pub target_modification: Option<Node>,
}

#[derive(Component)]
struct AnimationSequence {
    snap: Option<Duration>,
    duration: Duration,
}

#[derive(Component, Debug)]
struct PlayerPosition {
    current_position: Position,
    next_position: Option<Position>,
}
