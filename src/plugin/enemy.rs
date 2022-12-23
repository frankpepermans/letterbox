use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    game::{
        astar::{manhattan_heuristic, AStar},
        node::{Entry, Node},
    },
    game::{coordinates::Coordinates, matrix::Matrix},
    EndPosition, EnemyCount, EnemySprites, EnemyType, EnemyTypeValue, FragSprites, LivePosition,
    NodeSize, Path, Player, PlayerPosition, Position, ProjectilePosition, TraversalIndex,
};

use super::grid::OpenNodes;

#[derive(Bundle)]
struct PathInstructionsBundle {
    end_position: EndPosition,
    current_position: Position,
}

#[derive(Bundle)]
struct PathBundle {
    path: Path,
    traversal_index: TraversalIndex,
}

#[derive(Component)]
struct CheckPath(bool);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct WalkAnimationTimer(Timer);

#[derive(Component)]
struct FraggedAt((f32, f32));

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_system)
            .add_system(track_player_system)
            .add_system(calc_path)
            .add_system(check_path_after_matrix_change)
            .add_system(traverse_path.after(calc_path))
            .add_system(increment_path_traversal.after(traverse_path))
            .add_system(animate_sprite)
            .add_system(hit_test_projectiles)
            .add_system(animate_frag_sprite);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(
    mut commands: Commands,
    enemy_count: Res<EnemyCount>,
    node_size: Res<NodeSize>,
    open_nodes: Res<OpenNodes>,
    enemy_sprites: Res<EnemySprites>,
) {
    let mut rng = rand::thread_rng();

    (0..enemy_count.0).for_each(|_| {
        let index = (rng.gen::<f32>() * 24.) as usize;

        spawn_enemy(
            &mut commands,
            (index, 49),
            &node_size,
            &open_nodes,
            &enemy_sprites,
        );
    });
}

fn check_path_after_matrix_change(
    n_query: Query<(&Position, &Node), Changed<Node>>,
    mut query: Query<(&Path, &TraversalIndex, &mut CheckPath), With<EnemyType>>,
) {
    for (position, node) in &n_query {
        for (path, traversal_index, mut check_path) in &mut query {
            let no_path = path.0.is_none() || traversal_index.0.is_none();

            let affects_path = match node[Entry::LEFT] {
                true => true,
                false => match &path.0 {
                    Some(path) => path.contains(&position.0),
                    _ => false,
                },
            };

            if affects_path || (no_path && !check_path.0) {
                *check_path = CheckPath(true);
            }
        }
    }
}

fn calc_path(
    matrix: Res<Matrix<Node>>,
    windows: Res<Windows>,
    node_size: Res<NodeSize>,
    mut query: Query<(
        &Position,
        &EndPosition,
        &mut Path,
        &mut TraversalIndex,
        &mut CheckPath,
    )>,
) {
    let partial_paths = Arc::new(Mutex::new(HashMap::new()));
    let window = windows.primary();
    let g_w = window.width() / node_size.0 .0;
    let g_h = window.height() / node_size.0 .1;
    let g_s = (g_w * g_w + g_h * g_h).sqrt().ceil() as i32;

    query.par_for_each_mut(
        64,
        |(current_position, end_position, mut path, mut traversal_index, mut check_path)| {
            let start_position = if let (Some(path), Some(index)) = (&path.0, &traversal_index.0) {
                if *index < path.len() - 1 {
                    path[index + 1]
                } else {
                    current_position.0
                }
            } else {
                current_position.0
            };
            let mut partial_paths = partial_paths.lock().unwrap();

            if check_path.0 {
                if manhattan_heuristic(&start_position, &end_position.0) >= g_s {
                    *path = Path(None);
                    *traversal_index = TraversalIndex(None);
                    *check_path = CheckPath(false);
                } else {
                    let d_p = matrix.astar(
                        start_position,
                        end_position.0,
                        &manhattan_heuristic,
                        &partial_paths,
                    );

                    if let Some(d_p) = &d_p {
                        let size = d_p.len();

                        d_p.iter()
                            .enumerate()
                            .filter(|tuple| tuple.0 + 1 < size)
                            .for_each(|tuple| {
                                partial_paths
                                    .entry(*tuple.1)
                                    .or_insert_with(|| d_p[tuple.0 + 1..].to_vec());
                            });

                        if traversal_index.0 != Some(0) {
                            *traversal_index = TraversalIndex(Some(0));
                        }

                        *check_path = CheckPath(false);
                    }

                    if start_position != current_position.0 {
                        *path = Path(Some(
                            [Vec::from([current_position.0]), d_p.unwrap_or_default()].concat(),
                        ));
                    } else {
                        *path = Path(d_p);
                    }

                    if !path.0.is_some() {
                        *traversal_index = TraversalIndex(None);
                    }
                }
            }
        },
    )
}

fn track_player_system(
    p_query: Query<&PlayerPosition, (With<Player>, Changed<PlayerPosition>)>,
    mut query: Query<(&mut EndPosition, &mut CheckPath)>,
) {
    for position in &p_query {
        for (mut end_position, mut check_path) in &mut query {
            if end_position.0 != position.current_position.0 {
                *end_position = position.current_position.0.into();
            }

            if !check_path.0 {
                *check_path = CheckPath(true);
            }
        }
    }
}

fn increment_path_traversal(
    enemy_sprites: Res<EnemySprites>,
    mut query: Query<
        (
            &Path,
            &TraversalIndex,
            &EnemyType,
            &mut Position,
            &mut Handle<TextureAtlas>,
        ),
        Or<(Changed<TraversalIndex>, Changed<Path>)>,
    >,
) {
    for (path, traversal_index, enemy_type, mut current_position, mut texture_atlas_handle) in
        &mut query
    {
        if let (Some(p), Some(index)) = (&path.0, traversal_index.0) {
            if index < p.len() - 1 {
                let did_pos_change = current_position.0 != p[index];

                if did_pos_change {
                    *current_position = p[index].into();

                    if index < p.len() - 2 {
                        if let Some(handle) =
                            enemy_sprites.find(&p[index], &p[index + 1], &enemy_type.type_value)
                        {
                            *texture_atlas_handle = handle;
                        }
                    }
                }
            }
        }
    }
}

fn traverse_path(
    time: Res<Time>,
    node_size: Res<NodeSize>,
    mut query: Query<(
        &Path,
        &mut Transform,
        &mut TraversalIndex,
        &mut Visibility,
        &mut WalkAnimationTimer,
    )>,
    p_query: Query<&LivePosition>,
) {
    if p_query.is_empty() {
        return;
    }

    let live_position = p_query.single();

    for (path, mut transform, mut traversal_index, mut visibility, mut walk_animation_timer) in
        &mut query
    {
        let params = (&path.0, traversal_index.0);

        walk_animation_timer.tick(time.delta());

        if let (Some(path), Some(mut index)) = params {
            if index < path.len() - 1 {
                let mut delta_factor = walk_animation_timer.elapsed().as_millis() as f32
                    / walk_animation_timer.duration().as_millis() as f32;

                delta_factor = delta_factor.clamp(0., 1.);

                if walk_animation_timer.just_finished() {
                    index += 1;
                    delta_factor = 0.;

                    *traversal_index = TraversalIndex(Some(index));
                }

                let from = path[index];

                if index < path.len() - 1 {
                    let to = path[index + 1];
                    let row_0 = from.0 as f32;
                    let row_1 = to.0 as f32;
                    let col_0 = from.1 as f32;
                    let col_1 = to.1 as f32;
                    let position = (
                        row_0 + (row_1 - row_0) * delta_factor,
                        col_0 + (col_1 - col_0) * delta_factor,
                    );

                    transform.translation.x =
                        (position.1 - live_position.0 .1) * node_size.0 .0 + node_size.0 .0 / 2.;
                    transform.translation.y =
                        (live_position.0 .0 - position.0) * node_size.0 .1 - node_size.0 .1 / 2.;
                } else {
                    transform.translation.x = from.0 as f32;
                    transform.translation.y = from.1 as f32;
                }

                *visibility = Visibility::VISIBLE;
            }
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<EnemyType>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(handle) = texture_atlases.get(texture_atlas_handle) {
                sprite.index = (sprite.index + 1) % handle.textures.len();
            }
        }
    }
}

fn animate_frag_sprite(
    mut commands: Commands,
    time: Res<Time>,
    node_size: Res<NodeSize>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &FraggedAt,
    )>,
    p_query: Query<&LivePosition>,
) {
    for (entity, mut transform, mut timer, mut sprite, texture_atlas_handle, fragged_at) in
        &mut query
    {
        let live_position = p_query.single();

        transform.translation.x =
            (fragged_at.0 .1 - live_position.0 .1) * node_size.0 .0 + node_size.0 .0 / 2.;
        transform.translation.y =
            (live_position.0 .0 - fragged_at.0 .0) * node_size.0 .1 - node_size.0 .1 / 2.;

        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(handle) = texture_atlases.get(texture_atlas_handle) {
                let next_index = sprite.index + 1;

                if next_index < handle.textures.len() {
                    sprite.index = next_index;
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn hit_test_projectiles(
    mut commands: Commands,
    node_size: Res<NodeSize>,
    open_nodes: Res<OpenNodes>,
    enemy_sprites: Res<EnemySprites>,
    frag_sprites: Res<FragSprites>,
    query: Query<(Entity, &Transform), With<ProjectilePosition>>,
    e_query: Query<(Entity, &Transform), With<TraversalIndex>>,
    p_query: Query<(&PlayerPosition, &LivePosition)>,
) {
    if p_query.is_empty() {
        return;
    }

    let (player_position, live_position) = p_query.single();
    let mut despawned = Vec::new();
    let mut spawn_count = 0;

    for (_entity, transform) in &query {
        for (enemy_entity, enemy_transform) in &e_query {
            if despawned.contains(&enemy_entity) {
                break;
            }

            let hit_box = Rect::new(
                enemy_transform.translation.x - node_size.0 .0 / 2.,
                enemy_transform.translation.y - node_size.0 .1 / 2.,
                enemy_transform.translation.x + node_size.0 .0 / 2.,
                enemy_transform.translation.y + node_size.0 .1 / 2.,
            );

            if hit_box.contains(Vec2::new(transform.translation.x, transform.translation.y)) {
                let frag_translation_x = live_position.0 .0
                    - (enemy_transform.translation.y + node_size.0 .1 / 2.) / node_size.0 .1;
                let frag_translation_y = (enemy_transform.translation.x - node_size.0 .0 / 2.)
                    / node_size.0 .0
                    + live_position.0 .1;

                despawned.push(enemy_entity);
                //commands.entity(entity).despawn();
                commands.entity(enemy_entity).despawn();

                commands.spawn((
                    SpriteSheetBundle {
                        transform: Transform {
                            scale: Vec3::splat(node_size.0 .0 as f32 / frag_sprites.size),
                            translation: enemy_transform.translation,
                            ..default()
                        },
                        texture_atlas: frag_sprites.blood.clone(),
                        ..default()
                    },
                    AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
                    FraggedAt((frag_translation_x, frag_translation_y)),
                ));

                spawn_count += 1;
            }
        }
    }

    for _i in 0..spawn_count {
        spawn_enemy(
            &mut commands,
            player_position.current_position.0,
            &node_size,
            &open_nodes,
            &enemy_sprites,
        );
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    end_position: Coordinates,
    node_size: &Res<NodeSize>,
    open_nodes: &Res<OpenNodes>,
    enemy_sprites: &Res<EnemySprites>,
) {
    let mut rng = rand::thread_rng();
    let start_position = open_nodes.0[(rng.gen::<f32>() * open_nodes.0.len() as f32) as usize];
    let rnd = rng.gen_range(0..3);
    let type_value = match rnd {
        0 => EnemyTypeValue::Bat,
        1 => EnemyTypeValue::Spider,
        _ => EnemyTypeValue::Skeleton,
    };

    commands
        .spawn(PathInstructionsBundle {
            end_position: end_position.into(),
            current_position: start_position.into(),
        })
        .insert(PathBundle {
            path: Path(None),
            traversal_index: TraversalIndex(None),
        })
        .insert(CheckPath(true))
        .insert(EnemyType { type_value })
        .insert((
            SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3::splat(
                        (rng.gen::<f32>() * 0.5 + 0.75) * node_size.0 .0 as f32
                            / enemy_sprites.size,
                    ),
                    translation: Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 100.5,
                    },
                    ..default()
                },
                visibility: Visibility::INVISIBLE,
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            WalkAnimationTimer(Timer::from_seconds(
                0.15 + (rng.gen::<f32>() * 1.5),
                TimerMode::Repeating,
            )),
        ));
}
