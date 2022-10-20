#![feature(binary_heap_retain)]

use std::time::Duration;

use bevy::prelude::*;
use game::coordinates::Coordinates;

pub mod actors;
pub mod game;

pub struct GridSize(pub Coordinates);

#[derive(Component, Debug)]
pub struct Position(pub Coordinates);

// region:    --- Robot Actor

#[derive(Component)]
pub struct Robot;

pub struct RobotCount(pub i8);

#[derive(Component)]
pub struct StartPosition(pub Coordinates);

#[derive(Component)]
pub struct EndPosition(pub Coordinates);

#[derive(Component)]
pub struct PathPosition(pub Coordinates);

#[derive(Component)]
pub struct Path(Option<Vec<Coordinates>>);

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Bundle)]
pub struct RobotMappingBundle {
    pub start_position: StartPosition,
    pub end_position: EndPosition,
    pub path: Path,
}

#[derive(Component)]
pub struct AnimationValue(pub f32);

#[derive(Component)]
pub struct AnimationSequence {
    pub range_values: (AnimationValue, AnimationValue),
    pub velocity: Velocity,
    pub snap: Option<Duration>,
    pub duration: Duration,
}

#[derive(Component)]
pub struct SinglePathVector {
    pub from: StartPosition,
    pub to: EndPosition,
}

#[derive(Component)]
pub struct PathTraversalIndex(pub usize);

// endregion: --- Robot Actor
