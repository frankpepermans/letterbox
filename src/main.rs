use bevy::prelude::*;
use letterbox::{
    actors::{grid::GridPlugin, robot::RobotPlugin},
    game::coordinates::Coordinates,
    GridSize, NodeSize, RobotCount,
};

// (rows, cols)
const GRID_SIZE: (usize, usize) = (24, 50);
// (width, height) in pixels
const NODE_SIZE: (f32, f32) = (20., 20.);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: GRID_SIZE.1 as f32 * NODE_SIZE.0,
            height: GRID_SIZE.0 as f32 * NODE_SIZE.1,
            ..Default::default()
        })
        .insert_resource(GridSize(GRID_SIZE))
        .insert_resource(NodeSize(NODE_SIZE))
        .insert_resource(RobotCount(20))
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(GridPlugin)
        .add_plugin(RobotPlugin)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle::default());
}

#[derive(Component)]
struct Player(Coordinates);
