use std::time::Duration;

use bevy::prelude::*;

use crate::{
    game::matrix::Matrix, game::movement::Movement, game::node::Node, AnimationSequence, NodeSize,
    Player, PlayerPosition, Position,
};

pub struct PlayerPlugin;

#[derive(Component)]
struct KeyState {
    down_key: Option<KeyCode>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(update_player_position_system)
            .add_system(traverse_path);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _asset_server: Res<AssetServer>,
) {
    commands
        .spawn_empty()
        .insert(Player {})
        .insert(PlayerPosition {
            current_position: Position((20, 20)),
            next_position: None,
        })
        .insert(SpriteBundle {
            texture: asset_server.load("player.png"),
            visibility: Visibility::INVISIBLE,
            ..default()
        })
        .insert(AnimationSequence {
            duration: Duration::from_millis(100),
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
                if let Some(next_node) = matrix.left(position.current_position.0) {
                    *position = PlayerPosition {
                        current_position: position.current_position,
                        next_position: Some(Position(next_node)),
                    };
                }
            } else if key_code.just_pressed(KeyCode::Right) {
                if let Some(next_node) = matrix.right(position.current_position.0) {
                    *position = PlayerPosition {
                        current_position: position.current_position,
                        next_position: Some(Position(next_node)),
                    };
                }
            } else if key_code.just_pressed(KeyCode::Up) {
                if let Some(next_node) = matrix.up(position.current_position.0) {
                    *position = PlayerPosition {
                        current_position: position.current_position,
                        next_position: Some(Position(next_node)),
                    };
                }
            } else if key_code.just_pressed(KeyCode::Down) {
                if let Some(next_node) = matrix.down(position.current_position.0) {
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
    windows: Res<Windows>,
    node_size: Res<NodeSize>,
    mut query: Query<
        (
            &mut AnimationSequence,
            &mut PlayerPosition,
            &mut Transform,
            &mut Visibility,
            &KeyState,
        ),
        With<Player>,
    >,
    matrix: Res<Matrix<Node>>,
) {
    let window = windows.primary();

    for (mut animation_sequence, mut player_position, mut transform, mut visibility, key_state) in
        &mut query
    {
        let params = (player_position.next_position, animation_sequence.snap);
        let (w, h) = (window.width(), window.height());

        if let (Some(next_position), Some(start_duration)) = params {
            let delta = time.elapsed() - start_duration;
            let mut delta_factor =
                delta.as_millis() as f32 / animation_sequence.duration.as_millis() as f32;
            let at_end = delta_factor > 1.;

            delta_factor = delta_factor.clamp(0., 1.);

            let from = player_position.current_position.0;
            let to = next_position;
            let row_0 = from.0 as f32;
            let row_1 = to.0 .0 as f32;
            let col_0 = from.1 as f32;
            let col_1 = to.0 .1 as f32;
            let position = (
                row_0 + (row_1 - row_0) * delta_factor,
                col_0 + (col_1 - col_0) * delta_factor,
            );

            transform.translation.x =
                -w / 2. + position.1 as f32 * node_size.0 .0 + node_size.0 .0 / 2.;
            transform.translation.y =
                h / 2. - position.0 as f32 * node_size.0 .1 - node_size.0 .1 / 2.;
            transform.translation.z = 100.;

            if at_end {
                if let Some(down_key) = key_state.down_key {
                    match down_key {
                        KeyCode::Left => {
                            if let Some(next_node) = matrix.left(next_position.0) {
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
                            if let Some(next_node) = matrix.right(next_position.0) {
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
                            if let Some(next_node) = matrix.up(next_position.0) {
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
                            if let Some(next_node) = matrix.down(next_position.0) {
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
            let window = windows.primary();

            let (w, h) = (window.width(), window.height());

            transform.translation.x = -w / 2.
                + player_position.current_position.0 .1 as f32 * node_size.0 .0
                + node_size.0 .0 / 2.;
            transform.translation.y = h / 2.
                - player_position.current_position.0 .0 as f32 * node_size.0 .1
                - node_size.0 .1 / 2.;
            transform.translation.z = 101.;
        }

        *visibility = Visibility::VISIBLE;
    }
}
