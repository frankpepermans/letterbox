use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, time::FixedTimestep};
use rand::prelude::*;

use crate::{
    game::{
        astar::{manhattan_heuristic, AStar},
        node::{Entry, Node},
    },
    game::{coordinates::Coordinates, matrix::Matrix},
    NodeSize, Player, Position, RobotCount,
};

#[derive(Component, Clone, Copy)]
struct EndPosition(Coordinates);

#[derive(Component)]
struct Path(Option<Vec<Coordinates>>);

#[derive(Component)]
struct TraversalIndex(Option<usize>);

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
struct AnimationSequence {
    snap: Option<Duration>,
    duration: Duration,
}

#[derive(Component)]
struct CheckPath(bool);

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(check_path)
            .add_system(track_player_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1. / 10.))
                    .with_system(calc_path),
            )
            .add_system(traverse_path)
            .add_system(increment_path_traversal.after(traverse_path));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    robot_count: Res<RobotCount>,
) {
    let mut rng = rand::thread_rng();

    (0..robot_count.0).for_each(|_| {
        let index = (rng.gen::<f32>() * 24.) as usize;

        spawn_robot(&mut commands, (index, 49), &asset_server);
    });
}

fn check_path(
    n_query: Query<(&Position, &Node), Changed<Node>>,
    mut query: Query<(&Path, &TraversalIndex, &mut CheckPath), With<AnimationSequence>>,
) {
    for (path, traversal_index, mut check_path) in &mut query {
        let mut affects_path = false;
        let no_path = path.0.is_none() || traversal_index.0.is_none();

        for (position, node) in &n_query {
            if node[Entry::LEFT] {
                affects_path = true;

                break;
            }

            if let Some(path) = &path.0 {
                path.contains(&position.0)
            } else {
                false
            }
            .then(|| {
                affects_path = true;
            });

            if affects_path {
                break;
            }
        }

        if affects_path || (no_path && !check_path.0) {
            *check_path = CheckPath(true);
        }
    }
}

fn calc_path(
    m_query: Query<&Matrix<Node>>,
    mut query: Query<
        (
            &Position,
            &EndPosition,
            &mut Path,
            &mut TraversalIndex,
            &mut CheckPath,
        ),
        Changed<CheckPath>,
    >,
) {
    for matrix in &m_query {
        let mut partial_paths = HashMap::new();

        for (current_position, end_position, mut path, mut traversal_index, mut check_path) in
            &mut query
        {
            let mut start_position = current_position.0;

            if let Some(path) = &path.0 {
                if let Some(index) = &traversal_index.0 {
                    if *index < path.len() - 1 {
                        start_position = path[index + 1];
                    }
                }
            }

            if check_path.0 {
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
    }
}

fn track_player_system(
    p_query: Query<&Position, (With<Player>, Changed<Position>)>,
    mut query: Query<(&mut EndPosition, &mut CheckPath)>,
) {
    for position in &p_query {
        for (mut end_position, mut check_path) in &mut query {
            *end_position = EndPosition(position.0);
            *check_path = CheckPath(true);
        }
    }
}

fn increment_path_traversal(
    time: Res<Time>,
    mut query: Query<
        (
            &Path,
            &TraversalIndex,
            &mut AnimationSequence,
            &mut Position,
        ),
        Or<(Changed<TraversalIndex>, Changed<Path>)>,
    >,
) {
    for (path, traversal_index, mut animation_sequence, mut current_position) in &mut query {
        if let (Some(p), Some(index)) = (&path.0, traversal_index.0) {
            if index < p.len() - 1 {
                let did_pos_change = current_position.0 != p[index];

                if did_pos_change {
                    *current_position = Position(p[index]);
                }

                let at_end = did_pos_change
                    || match animation_sequence.snap {
                        Some(snap) => {
                            (time.elapsed() - snap).as_millis()
                                >= animation_sequence.duration.as_millis()
                        }
                        None => true,
                    };

                if at_end {
                    *animation_sequence = AnimationSequence {
                        duration: animation_sequence.duration,
                        snap: Some(time.elapsed()),
                    };
                }
            }
        }
    }
}

fn traverse_path(
    time: Res<Time>,
    windows: Res<Windows>,
    node_size: Res<NodeSize>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Path,
        &AnimationSequence,
        &mut Transform,
        &mut TraversalIndex,
        &mut Visibility,
    )>,
    p_query: Query<&Position, With<Player>>,
) {
    let window = windows.primary();

    for (entity, path, animation_sequence, mut transform, mut traversal_index, mut visibility) in
        &mut query
    {
        let params = (&path.0, traversal_index.0, animation_sequence.snap);

        if let (Some(path), Some(index), Some(start_duration)) = params {
            if index < path.len() - 1 {
                let delta = time.elapsed() - start_duration;
                let mut delta_factor =
                    delta.as_millis() as f32 / animation_sequence.duration.as_millis() as f32;
                let at_end = delta_factor > 1.;

                delta_factor = delta_factor.clamp(0., 1.);

                let from = path[index];
                let to = path[index + 1];
                let row_0 = from.0 as f32;
                let row_1 = to.0 as f32;
                let col_0 = from.1 as f32;
                let col_1 = to.1 as f32;
                let position = (
                    row_0 + (row_1 - row_0) * delta_factor,
                    col_0 + (col_1 - col_0) * delta_factor,
                );
                let (w, h) = (window.width(), window.height());

                transform.translation.x =
                    -w / 2. + position.1 as f32 * node_size.0 .0 + node_size.0 .0 / 2.;
                transform.translation.y =
                    h / 2. - position.0 as f32 * node_size.0 .1 - node_size.0 .1 / 2.;
                transform.translation.z = 100.;

                *visibility = Visibility::VISIBLE;

                if at_end {
                    *traversal_index = TraversalIndex(Some(index + 1));
                }
            } else {
                commands.entity(entity).despawn();

                for position in &p_query {
                    spawn_robot(&mut commands, position.0, &asset_server);
                }
            }
        }
    }
}

fn spawn_robot(
    commands: &mut Commands,
    end_position: Coordinates,
    asset_server: &Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    let index = (rng.gen::<f32>() * 24.) as usize;

    commands
        .spawn(PathInstructionsBundle {
            end_position: EndPosition(end_position),
            current_position: Position((index, 0)),
        })
        .insert(PathBundle {
            path: Path(None),
            traversal_index: TraversalIndex(None),
        })
        .insert(CheckPath(true))
        .insert(AnimationSequence {
            duration: Duration::from_millis(25 + (rng.gen::<f32>() * 2000.) as u64),
            snap: None,
        })
        .insert(SpriteBundle {
            texture: asset_server.load("robot.png"),
            visibility: Visibility::INVISIBLE,
            ..default()
        });
}
