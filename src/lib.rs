#![feature(binary_heap_retain)]

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

#[derive(Resource)]
pub struct ProjectileReach(pub i8);

#[derive(Component)]
pub struct UserPosition {
    pub coordinates: Option<Coordinates>,
    pub cursor_pressed_state: Option<UserCursorPressedState>,
    pub target_modification: Option<Node>,
}

#[derive(Component, Debug, Clone, Copy)]
struct ProjectilePosition(pub (f32, f32));

#[derive(Component, Debug)]
struct PlayerPosition {
    current_position: Position,
    next_position: Option<Position>,
}

#[derive(Component)]
struct Path(Option<Vec<Coordinates>>);

#[derive(Component)]
struct TraversalIndex(Option<usize>);

#[derive(Component, Clone, Copy)]
struct EndPosition(Coordinates);

impl Into<EndPosition> for Coordinates {
    fn into(self) -> EndPosition {
        EndPosition(self)
    }
}

#[derive(Component, Deref, DerefMut)]
struct WalkAnimationTimer(Timer);

#[derive(Component)]
struct EnemyType {
    type_value: EnemyTypeValue,
}

#[derive(Resource)]
pub struct EnemySprites {
    pub size: f32,
    pub bat_up: Handle<TextureAtlas>,
    pub bat_down: Handle<TextureAtlas>,
    pub bat_left: Handle<TextureAtlas>,
    pub bat_right: Handle<TextureAtlas>,
    pub spider_up: Handle<TextureAtlas>,
    pub spider_down: Handle<TextureAtlas>,
    pub spider_left: Handle<TextureAtlas>,
    pub spider_right: Handle<TextureAtlas>,
    pub skeleton_up: Handle<TextureAtlas>,
    pub skeleton_down: Handle<TextureAtlas>,
    pub skeleton_left: Handle<TextureAtlas>,
    pub skeleton_right: Handle<TextureAtlas>,
}

pub enum EnemyTypeValue {
    Bat,
    Spider,
    Skeleton,
}

impl EnemySprites {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            size: 32.,
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
            spider_up: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("spider_up.png"),
                Vec2::new(32., 32.),
                6,
                1,
                None,
                None,
            )),
            spider_down: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("spider_down.png"),
                Vec2::new(32., 32.),
                6,
                1,
                None,
                None,
            )),
            spider_left: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("spider_left.png"),
                Vec2::new(32., 32.),
                6,
                1,
                None,
                None,
            )),
            spider_right: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("spider_right.png"),
                Vec2::new(32., 32.),
                6,
                1,
                None,
                None,
            )),
            skeleton_up: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("skeleton_up.png"),
                Vec2::new(32., 32.),
                8,
                1,
                None,
                None,
            )),
            skeleton_down: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("skeleton_down.png"),
                Vec2::new(32., 32.),
                8,
                1,
                None,
                None,
            )),
            skeleton_left: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("skeleton_left.png"),
                Vec2::new(32., 32.),
                8,
                1,
                None,
                None,
            )),
            skeleton_right: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("skeleton_right.png"),
                Vec2::new(32., 32.),
                8,
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
            EnemyTypeValue::Spider => Some(if from.1 > to.1 {
                self.spider_left.clone()
            } else if from.1 < to.1 {
                self.spider_right.clone()
            } else if from.0 > to.0 {
                self.spider_up.clone()
            } else {
                self.spider_down.clone()
            }),
            EnemyTypeValue::Skeleton => Some(if from.1 > to.1 {
                self.skeleton_left.clone()
            } else if from.1 < to.1 {
                self.skeleton_right.clone()
            } else if from.0 > to.0 {
                self.skeleton_up.clone()
            } else {
                self.skeleton_down.clone()
            }),
        }
    }
}

#[derive(Resource)]
pub struct PlayerSprites {
    pub size: f32,
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
            size: 25.,
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

#[derive(Resource)]
pub struct AttackSprites {
    pub size: f32,
    pub sword: Handle<TextureAtlas>,
}

impl AttackSprites {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            size: 16.,
            sword: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("sword_attack.png"),
                Vec2::new(32., 32.),
                4,
                1,
                None,
                None,
            )),
        }
    }
}

#[derive(Resource)]
pub struct ProjectileSprites {
    pub size: f32,
    pub knife: Handle<TextureAtlas>,
}

impl ProjectileSprites {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            size: 64.,
            knife: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("projectile_knife.png"),
                Vec2::new(32., 32.),
                4,
                1,
                None,
                None,
            )),
        }
    }
}

#[derive(Resource)]
pub struct FragSprites {
    pub size: f32,
    pub blood: Handle<TextureAtlas>,
}

impl FragSprites {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            size: 24.,
            blood: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("enemy_frag.png"),
                Vec2::new(32., 32.),
                6,
                1,
                None,
                None,
            )),
        }
    }
}

#[derive(Resource)]
pub struct PowerUpSprites {
    pub size: f32,
    pub speed: Handle<TextureAtlas>,
    pub projectile_count: Handle<TextureAtlas>,
}

impl PowerUpSprites {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            size: 32.,
            speed: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("power_up_speed.png"),
                Vec2::new(32., 32.),
                6,
                1,
                None,
                None,
            )),
            projectile_count: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("power_up_projectile.png"),
                Vec2::new(32., 32.),
                6,
                1,
                None,
                None,
            )),
        }
    }
}
