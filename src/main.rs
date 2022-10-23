use bevy::prelude::*;
use letterbox::{
    actors::{grid::GridPlugin, robot::RobotPlugin},
    game::coordinates::Coordinates,
    GridSize, RobotCount,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 50. * 20.,
            height: 24.0 * 20.,
            ..Default::default()
        })
        .insert_resource(GridSize((24, 50)))
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
