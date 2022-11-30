use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    game::{astar::manhattan_heuristic, matrix::Matrix, node::Node},
    LivePosition, NodeSize, Path, Player, PlayerPosition, ProjectilePosition, ProjectileSprites,
    TraversalIndex,
};

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub(crate) struct ProjectileCount(i8);

#[derive(Component)]
pub(crate) struct PiercingCount(i8);

#[derive(Bundle)]
pub(crate) struct Projectile {
    pub(crate) count: ProjectileCount,
    pub(crate) piercing_count: PiercingCount,
}

#[derive(Component)]
struct Angle(f32);

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_system)
            .add_system(launch_projectiles)
            .add_system(animate_projectiles)
            .add_system(hit_test_projectiles);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(mut commands: Commands) {
    commands
        .spawn(Projectile {
            count: ProjectileCount(1),
            piercing_count: PiercingCount(0),
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )));
}

fn launch_projectiles(
    mut commands: Commands,
    matrix: Res<Matrix<Node>>,
    node_size: Res<NodeSize>,
    projectile_sprites: Res<ProjectileSprites>,
    time: Res<Time>,
    mut query: Query<(&ProjectileCount, &PiercingCount, &mut AnimationTimer)>,
    e_query: Query<(&Path, &TraversalIndex)>,
    p_query: Query<(&PlayerPosition, &LivePosition)>,
) {
    for (count, _piercing_count, mut timer) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let tuple = p_query.single();
            let (p, l) = (tuple.0.current_position, tuple.1);

            let mut valid_positions = e_query
                .into_iter()
                .filter_map(|(path, traversal_index)| {
                    if let (Some(path), Some(traversal_index)) = (&path.0, &traversal_index.0) {
                        let position = path[*traversal_index];
                        let mut delta = l.0;
                        let angle = (position.1 as f32 - l.0 .1 as f32)
                            .atan2(position.0 as f32 - l.0 .0 as f32)
                            + PI;
                        let a_cos = angle.cos() / 6.;
                        let a_sin = angle.sin() / 6.;
                        let mut delta_r = (delta.0.round() as usize, delta.1.round() as usize);

                        while delta_r != position {
                            delta = (delta.0 - a_cos, delta.1 - a_sin);
                            delta_r = (delta.0.round() as usize, delta.1.round() as usize);

                            if matrix[delta_r] == Node::closed() {
                                return None;
                            }
                        }

                        let d = manhattan_heuristic(&p.0, &position);

                        if d <= 10 {
                            Some((d, position))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            valid_positions.sort_by(|a, b| a.0.cmp(&b.0));

            if valid_positions.len() > 0 {
                let size = (valid_positions.len() - 1).min(count.0 as usize);

                [0..size].iter().enumerate().for_each(|i| {
                    let target_position = valid_positions[i.0].1;
                    let angle = (target_position.1 as f32 - l.0 .1 as f32)
                        .atan2(target_position.0 as f32 - l.0 .0 as f32)
                        + PI;

                    commands.spawn((
                        SpriteSheetBundle {
                            texture_atlas: projectile_sprites.knife.clone(),
                            transform: Transform {
                                scale: Vec3::splat(node_size.0 .0 as f32 / projectile_sprites.size),
                                rotation: Quat::from_rotation_z(angle),
                                translation: Vec3 {
                                    x: node_size.0 .0 / 2.,
                                    y: -node_size.0 .1 / 2.,
                                    z: 100.,
                                },
                                ..default()
                            },
                            visibility: Visibility::INVISIBLE,
                            ..default()
                        },
                        Angle(angle),
                        AnimationTimer(Timer::from_seconds(1. / 60., TimerMode::Repeating)),
                        ProjectilePosition(l.0),
                    ));
                });
            }
        }
    }
}

fn hit_test_projectiles(
    mut commands: Commands,
    matrix: Res<Matrix<Node>>,
    query: Query<(Entity, &ProjectilePosition), Changed<ProjectilePosition>>,
) {
    for (entity, position) in &query {
        let (row, col) = (position.0 .0, position.0 .1);

        if row < 0.
            || row.round() as usize >= matrix.rows
            || col < 0.
            || col.round() as usize >= matrix.cols
            || matrix[(row.round() as usize, col.round() as usize)] == Node::closed()
        {
            return commands.entity(entity).despawn();
        }
    }
}

fn animate_projectiles(
    time: Res<Time>,
    node_size: Res<NodeSize>,
    mut query: Query<(
        &Angle,
        &mut ProjectilePosition,
        &mut AnimationTimer,
        &mut Transform,
        &mut Visibility,
    )>,
    p_query: Query<&LivePosition, With<Player>>,
) {
    for (angle, mut initial_position, mut timer, mut transform, mut visibility) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            for live_position in &p_query {
                *initial_position = ProjectilePosition((
                    initial_position.0 .0 - angle.0.cos() / 6.,
                    initial_position.0 .1 - angle.0.sin() / 6.,
                ));

                transform.translation.x = (initial_position.0 .1 - live_position.0 .1)
                    * node_size.0 .0
                    + node_size.0 .0 / 2.;
                transform.translation.y = (live_position.0 .0 - initial_position.0 .0)
                    * node_size.0 .1
                    - node_size.0 .1 / 2.;

                if !visibility.is_visible {
                    *visibility = Visibility::VISIBLE;
                }
            }
        }
    }
}
