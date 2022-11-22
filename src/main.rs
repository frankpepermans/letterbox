use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

use letterbox::{
    game::coordinates::Coordinates,
    plugin::{assets::AssetsPlugin, grid::GridPlugin, player::PlayerPlugin, robot::RobotPlugin},
    GridSize, NodeSize, RobotCount,
};

// (rows, cols)
const GRID_SIZE: (usize, usize) = (64, 64);
// (width, height) in pixels
const NODE_SIZE: (f32, f32) = (35., 35.);

fn main() {
    App::new()
        .insert_resource(GridSize(GRID_SIZE))
        .insert_resource(NodeSize(NODE_SIZE))
        .insert_resource(RobotCount(200))
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1024.,
                height: 768.,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AssetsPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RobotPlugin)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0., 0., 0.)),
        },
        ..Default::default()
    });
}

#[derive(Component)]
struct Player(Coordinates);
