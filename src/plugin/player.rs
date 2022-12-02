use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    game::matrix::Matrix, game::movement::Movement, game::node::Node, AnimationSequence,
    LivePosition, NodeSize, Player, PlayerPosition, PlayerSprites, Position,
};

use super::{grid::OpenNodes, projectile::ProjectilePlugin};

pub struct PlayerPlugin;

#[derive(Component)]
struct KeyState {
    down_key: Option<KeyCode>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_system)
            .add_system(update_player_position_system)
            .add_system(traverse_path)
            .add_system(update_sprite)
            .add_system(animate_sprite)
            .add_plugin(ProjectilePlugin);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(
    mut commands: Commands,
    node_size: Res<NodeSize>,
    open_nodes: Res<OpenNodes>,
    player_sprites: Res<PlayerSprites>,
) {
    let mut rng = rand::thread_rng();
    let start_position = open_nodes.0[(rng.gen::<f32>() * open_nodes.0.len() as f32) as usize];

    commands
        .spawn_empty()
        .insert(Player {})
        .insert(PlayerPosition {
            current_position: Position(start_position),
            next_position: None,
        })
        .insert(LivePosition((0., 0.)))
        .insert((
            SpriteSheetBundle {
                texture_atlas: player_sprites.hero_down.clone(),
                transform: Transform {
                    scale: Vec3::splat(node_size.0 .0 as f32 / player_sprites.size),
                    translation: Vec3 {
                        x: node_size.0 .0 / 2.,
                        y: -node_size.0 .1 / 2.,
                        z: 101.,
                    },
                    ..default()
                },
                visibility: Visibility::INVISIBLE,
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        ))
        .insert(AnimationSequence {
            duration: Duration::from_millis(200),
            snap: None,
        })
        .insert(KeyState { down_key: None });
}

fn update_player_position_system(
    key_code: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut PlayerPosition, &mut AnimationSequence, &mut KeyState), With<Player>>,
    matrix: Res<Matrix<Node>>,
) {
    for (mut position, mut animation_sequence, mut key_state) in &mut query {
        if key_code.just_released(KeyCode::Left) && key_state.down_key == Some(KeyCode::Left) {
            *key_state = KeyState { down_key: None };
        } else if key_code.just_released(KeyCode::Right)
            && key_state.down_key == Some(KeyCode::Right)
        {
            *key_state = KeyState { down_key: None };
        } else if key_code.just_released(KeyCode::Up) && key_state.down_key == Some(KeyCode::Up) {
            *key_state = KeyState { down_key: None };
        } else if key_code.just_released(KeyCode::Down) && key_state.down_key == Some(KeyCode::Down)
        {
            *key_state = KeyState { down_key: None };
        }

        if key_code.just_pressed(KeyCode::Left) {
            *key_state = KeyState {
                down_key: Some(KeyCode::Left),
            };
        } else if key_code.just_pressed(KeyCode::Right) {
            *key_state = KeyState {
                down_key: Some(KeyCode::Right),
            };
        } else if key_code.just_pressed(KeyCode::Up) {
            *key_state = KeyState {
                down_key: Some(KeyCode::Up),
            };
        } else if key_code.just_pressed(KeyCode::Down) {
            *key_state = KeyState {
                down_key: Some(KeyCode::Down),
            };
        }

        if position.next_position.is_none() {
            if key_code.just_pressed(KeyCode::Left) {
                if let Some(next_node) = matrix.left(&position.current_position.0) {
                    *position = PlayerPosition {
                        current_position: position.current_position,
                        next_position: Some(Position(next_node)),
                    };
                }
            } else if key_code.just_pressed(KeyCode::Right) {
                if let Some(next_node) = matrix.right(&position.current_position.0) {
                    *position = PlayerPosition {
                        current_position: position.current_position,
                        next_position: Some(Position(next_node)),
                    };
                }
            } else if key_code.just_pressed(KeyCode::Up) {
                if let Some(next_node) = matrix.up(&position.current_position.0) {
                    *position = PlayerPosition {
                        current_position: position.current_position,
                        next_position: Some(Position(next_node)),
                    };
                }
            } else if key_code.just_pressed(KeyCode::Down) {
                if let Some(next_node) = matrix.down(&position.current_position.0) {
                    *position = PlayerPosition {
                        current_position: position.current_position,
                        next_position: Some(Position(next_node)),
                    };
                }
            }

            if key_code.any_just_pressed([
                KeyCode::Left,
                KeyCode::Right,
                KeyCode::Up,
                KeyCode::Down,
            ]) {
                *animation_sequence = AnimationSequence {
                    duration: animation_sequence.duration,
                    snap: Some(time.elapsed()),
                };
            }
        }
    }
}

fn traverse_path(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationSequence,
            &mut PlayerPosition,
            &mut Visibility,
            &mut LivePosition,
            &KeyState,
        ),
        With<Player>,
    >,
    matrix: Res<Matrix<Node>>,
) {
    for (
        mut animation_sequence,
        mut player_position,
        mut visibility,
        mut live_position,
        key_state,
    ) in &mut query
    {
        let params = (player_position.next_position, animation_sequence.snap);

        if let (Some(next_position), Some(start_duration)) = params {
            let delta = time.elapsed() - start_duration;
            let mut delta_factor =
                delta.as_millis() as f32 / animation_sequence.duration.as_millis() as f32;
            let at_end = delta_factor >= 1.;

            delta_factor = delta_factor.clamp(0., 1.);

            let from = player_position.current_position.0;
            let to = next_position;
            let row_0 = from.0 as f32;
            let row_1 = to.0 .0 as f32;
            let col_0 = from.1 as f32;
            let col_1 = to.0 .1 as f32;

            *live_position = if row_0 != row_1 {
                LivePosition((
                    row_0 + (row_1 - row_0) * delta_factor,
                    col_0 + (col_1 - col_0) * delta_factor,
                ))
            } else {
                LivePosition((
                    row_0 + (row_1 - row_0) * delta_factor,
                    col_0 + (col_1 - col_0) * delta_factor,
                ))
            };

            if at_end {
                if let Some(down_key) = key_state.down_key {
                    match down_key {
                        KeyCode::Left => {
                            if let Some(next_node) = matrix.left(&next_position.0) {
                                *player_position = PlayerPosition {
                                    current_position: next_position,
                                    next_position: Some(Position(next_node)),
                                };
                            } else {
                                *player_position = PlayerPosition {
                                    current_position: next_position,
                                    next_position: None,
                                };
                            }
                        }
                        KeyCode::Right => {
                            if let Some(next_node) = matrix.right(&next_position.0) {
                                *player_position = PlayerPosition {
                                    current_position: next_position,
                                    next_position: Some(Position(next_node)),
                                };
                            } else {
                                *player_position = PlayerPosition {
                                    current_position: next_position,
                                    next_position: None,
                                };
                            }
                        }
                        KeyCode::Up => {
                            if let Some(next_node) = matrix.up(&next_position.0) {
                                *player_position = PlayerPosition {
                                    current_position: next_position,
                                    next_position: Some(Position(next_node)),
                                };
                            } else {
                                *player_position = PlayerPosition {
                                    current_position: next_position,
                                    next_position: None,
                                };
                            }
                        }
                        KeyCode::Down => {
                            if let Some(next_node) = matrix.down(&next_position.0) {
                                *player_position = PlayerPosition {
                                    current_position: next_position,
                                    next_position: Some(Position(next_node)),
                                };
                            } else {
                                *player_position = PlayerPosition {
                                    current_position: next_position,
                                    next_position: None,
                                };
                            }
                        }
                        _ => {}
                    }

                    *animation_sequence = AnimationSequence {
                        duration: animation_sequence.duration,
                        snap: Some(time.elapsed()),
                    };
                } else {
                    *player_position = PlayerPosition {
                        current_position: next_position,
                        next_position: None,
                    }
                }
            }
        } else {
            let l_p = (
                player_position.current_position.0 .0 as f32,
                player_position.current_position.0 .1 as f32,
            );

            if l_p != live_position.0 {
                *live_position = LivePosition(l_p);
            }
        }

        *visibility = Visibility::VISIBLE;
    }
}

fn update_sprite(
    player_sprites: Res<PlayerSprites>,
    mut query: Query<(&PlayerPosition, &mut Handle<TextureAtlas>), Changed<PlayerPosition>>,
) {
    for (position, mut texture_atlas_handle) in &mut query {
        if let Some(to) = position.next_position {
            *texture_atlas_handle = player_sprites.find(&position.current_position.0, &to.0);
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &PlayerPosition,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, position) in &mut query {
        if position.next_position.is_some() {
            timer.tick(time.delta());

            if timer.just_finished() {
                if let Some(handle) = texture_atlases.get(texture_atlas_handle) {
                    sprite.index = (sprite.index + 1) % handle.textures.len();
                }
            }
        }
    }
}
