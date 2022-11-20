use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

use letterbox::{
    actors::{grid::GridPlugin, player::PlayerPlugin, robot::RobotPlugin},
    game::coordinates::Coordinates,
    GridSize, NodeSize, RobotCount,
};

// (rows, cols)
const GRID_SIZE: (usize, usize) = (24, 50);
// (width, height) in pixels
const NODE_SIZE: (f32, f32) = (20., 20.);

fn main() {
    App::new()
        .insert_resource(GridSize(GRID_SIZE))
        .insert_resource(NodeSize(NODE_SIZE))
        .insert_resource(RobotCount(100))
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: GRID_SIZE.1 as f32 * NODE_SIZE.0,
                height: GRID_SIZE.0 as f32 * NODE_SIZE.1,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(GridPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RobotPlugin)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Player(Coordinates);
