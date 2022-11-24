#![feature(binary_heap_retain)]

use std::time::Duration;

use bevy::prelude::*;
use game::{coordinates::Coordinates, node::Node};

pub mod game;
pub mod plugin;

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
pub struct EnemyCount(pub i16);

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

#[derive(Resource)]
pub struct EnemySprites {
    pub bat_up: Handle<TextureAtlas>,
    pub bat_down: Handle<TextureAtlas>,
    pub bat_left: Handle<TextureAtlas>,
    pub bat_right: Handle<TextureAtlas>,
}

pub enum EnemyTypeValue {
    Bat,
}

impl EnemySprites {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            bat_up: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("bat_up.png"),
                Vec2::new(32., 32.),
                3,
                1,
                None,
                None,
            )),
            bat_down: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("bat_down.png"),
                Vec2::new(32., 32.),
                3,
                1,
                None,
                None,
            )),
            bat_left: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("bat_left.png"),
                Vec2::new(32., 32.),
                3,
                1,
                None,
                None,
            )),
            bat_right: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("bat_right.png"),
                Vec2::new(32., 32.),
                3,
                1,
                None,
                None,
            )),
        }
    }

    pub fn find(
        &self,
        from: &Coordinates,
        to: &Coordinates,
        type_value: &EnemyTypeValue,
    ) -> Option<Handle<TextureAtlas>> {
        match type_value {
            EnemyTypeValue::Bat => Some(if from.1 > to.1 {
                self.bat_left.clone()
            } else if from.1 < to.1 {
                self.bat_right.clone()
            } else if from.0 > to.0 {
                self.bat_up.clone()
            } else {
                self.bat_down.clone()
            }),
        }
    }
}

#[derive(Resource)]
pub struct PlayerSprites {
    pub hero_up: Handle<TextureAtlas>,
    pub hero_down: Handle<TextureAtlas>,
    pub hero_left: Handle<TextureAtlas>,
    pub hero_right: Handle<TextureAtlas>,
}

impl PlayerSprites {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            hero_up: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("hero_up.png"),
                Vec2::new(25., 25.),
                1,
                4,
                None,
                None,
            )),
            hero_down: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("hero_down.png"),
                Vec2::new(25., 25.),
                1,
                4,
                None,
                None,
            )),
            hero_left: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("hero_left.png"),
                Vec2::new(25., 25.),
                1,
                4,
                None,
                None,
            )),
            hero_right: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("hero_right.png"),
                Vec2::new(25., 25.),
                1,
                4,
                None,
                None,
            )),
        }
    }

    pub fn find(&self, from: &Coordinates, to: &Coordinates) -> Handle<TextureAtlas> {
        if from.1 > to.1 {
            self.hero_left.clone()
        } else if from.1 < to.1 {
            self.hero_right.clone()
        } else if from.0 > to.0 {
            self.hero_up.clone()
        } else {
            self.hero_down.clone()
        }
    }
}
