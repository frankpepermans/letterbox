use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    game::{astar::manhattan_heuristic, coordinates::Coordinates, matrix::Matrix, node::Node},
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
            count: ProjectileCount(5),
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
            let mut closest_target: Option<(i32, Coordinates)> = None;

            e_query.into_iter().for_each(|(path, traversal_index)| {
                if let (Some(path), Some(traversal_index)) = (&path.0, &traversal_index.0) {
                    let position = path[*traversal_index];
                    let mut delta = l.0;
                    let angle = (position.1 as f32 - l.0 .1 as f32)
                        .atan2(position.0 as f32 - l.0 .0 as f32)
                        + PI;
                    let a_cos = angle.cos() / 6.;
                    let a_sin = angle.sin() / 6.;
                    let mut delta_r = (delta.0.round() as usize, delta.1.round() as usize);
                    let mut reachable = true;

                    while delta_r != position {
                        delta = (delta.0 - a_cos, delta.1 - a_sin);
                        delta_r = (delta.0.round() as usize, delta.1.round() as usize);

                        if matrix[delta_r] == Node::closed() {
                            reachable = false;
                        }
                    }

                    if reachable {
                        let d = manhattan_heuristic(&p.0, &position);

                        if d <= 10 {
                            closest_target = match closest_target {
                                Some(value) => {
                                    if d < value.0 {
                                        Some((d, position))
                                    } else {
                                        closest_target
                                    }
                                }
                                _ => Some((d, position)),
                            }
                        }
                    }
                }
            });

            if let Some(closest_target) = closest_target {
                let degree_delta = PI / 18.;
                let angle_delta = (count.0 - 1) as f32 * degree_delta / 2.;

                for i in 0..count.0 {
                    let target_position = closest_target.1;
                    let angle = (target_position.1 as f32 - l.0 .1 as f32)
                        .atan2(target_position.0 as f32 - l.0 .0 as f32)
                        + PI
                        - angle_delta
                        + i as f32 * degree_delta;

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
                }
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
    if p_query.is_empty() {
        return;
    }

    for (angle, mut initial_position, mut timer, mut transform, mut visibility) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let live_position = p_query.single();

            *initial_position = ProjectilePosition((
                initial_position.0 .0 - angle.0.cos() / 6.,
                initial_position.0 .1 - angle.0.sin() / 6.,
            ));

            transform.translation.x =
                (initial_position.0 .1 - live_position.0 .1) * node_size.0 .0 + node_size.0 .0 / 2.;
            transform.translation.y =
                (live_position.0 .0 - initial_position.0 .0) * node_size.0 .1 - node_size.0 .1 / 2.;

            if !visibility.is_visible {
                *visibility = Visibility::VISIBLE;
            }
        }
    }
}
