use bevy::prelude::*;
use rand::prelude::*;

use crate::{LivePosition, NodeSize, PlayerPosition, Position, PowerUpSprites, WalkAnimationTimer};

use super::{grid::OpenNodes, projectile::ProjectileCount};

enum PowerUpType {
    Speed,
    ProjectileCount,
}

#[derive(Component)]
struct PowerUp {
    power_up_type: PowerUpType,
}

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_system)
            .add_system(render_system)
            .add_system(player_obtains_system);
    }
}

fn setup_system(
    open_nodes: Res<OpenNodes>,
    power_up_sprites: Res<PowerUpSprites>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    let len = open_nodes.0.len();

    (0..100).for_each(|_| {
        let index = rng.gen_range(0..len);
        let pu_type = rng.gen_range(0..2);
        let position = open_nodes.0[index];

        commands
            .spawn(Position(position))
            .insert(PowerUp {
                power_up_type: match pu_type {
                    0 => PowerUpType::ProjectileCount,
                    _ => PowerUpType::Speed,
                },
            })
            .insert(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 100.,
                    },
                    ..default()
                },
                texture_atlas: match pu_type {
                    0 => power_up_sprites.projectile_count.clone(),
                    _ => power_up_sprites.speed.clone(),
                },
                visibility: Visibility::INVISIBLE,
                ..default()
            });
    });
}

fn render_system(
    node_size: Res<NodeSize>,
    mut query: Query<(&Position, &mut Transform, &mut Visibility), With<PowerUp>>,
    p_query: Query<&LivePosition>,
) {
    for (position, mut transform, mut visibility) in &mut query {
        for live_position in &p_query {
            transform.translation.x =
                (position.0 .1 as f32 - live_position.0 .1) * node_size.0 .0 + node_size.0 .0 / 2.;
            transform.translation.y =
                (live_position.0 .0 - position.0 .0 as f32) * node_size.0 .1 - node_size.0 .1 / 2.;

            *visibility = Visibility::VISIBLE;
        }
    }
}

fn player_obtains_system(
    mut commands: Commands,
    query: Query<(Entity, &Position, &PowerUp)>,
    mut p_query: Query<(&PlayerPosition, &mut WalkAnimationTimer)>,
    mut projectile_query: Query<&mut ProjectileCount>,
) {
    for (entity, position, power_up) in &query {
        for (player_position, mut walk_animation_timer) in &mut p_query {
            if let Some(next_position) = player_position.next_position {
                if position.0 == next_position.0 {
                    commands.entity(entity).despawn();

                    match power_up.power_up_type {
                        PowerUpType::Speed => {
                            *walk_animation_timer = WalkAnimationTimer(Timer::from_seconds(
                                walk_animation_timer.duration().as_secs_f32() - 0.02,
                                TimerMode::Repeating,
                            ))
                        }
                        PowerUpType::ProjectileCount => {
                            for mut projectile_count in &mut projectile_query {
                                *projectile_count = ProjectileCount(projectile_count.0 + 1);
                            }
                        }
                    }
                }
            }
        }
    }
}
