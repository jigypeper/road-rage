use std::{thread::sleep, time::Duration};

use rand::Rng;
use rusty_engine::prelude::{bevy::math::Vec2Swizzles, *};

#[derive(Resource)]
struct GameState {
    score: u32,
    high_score: u32,
    hp: u32,
    enemy_number: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            high_score: 0,
            hp: 100,
            enemy_number: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();

    let game_state = GameState::default();
    let player = game.add_sprite("Player", SpritePreset::RacingCarRed);
    player.translation = Vec2::new(-650.0, 0.0);
    player.collision = true;
    let hp = game.add_text("hp", "HP: 100");

    game.add_logic(progress_logic);
    game.add_logic(control_logic);
    game.add_logic(spawn_enemy_logic);
    game.add_logic(move_enemy);
    game.run(game_state);
}

fn control_logic(engine: &mut Engine, game_state: &mut GameState) {
    let player = engine.sprites.get_mut("Player").unwrap();
    let current_y = player.translation.y;
    let acceptable_y_up = (engine.window_dimensions.y / 2.0) - 45.0;
    let acceptable_y_down = -((engine.window_dimensions.y / 2.0) - 45.0);
    if engine.keyboard_state.pressed(KeyCode::Up) && current_y < acceptable_y_up {
        player.rotation = std::f32::consts::FRAC_PI_6;
        player.translation += Vec2::new(0.0, 5.0);
    }
    if engine.keyboard_state.just_released(KeyCode::Up) {
        player.rotation -= std::f32::consts::FRAC_PI_6;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) && current_y > acceptable_y_down {
        player.rotation = -std::f32::consts::FRAC_PI_6;
        player.translation += Vec2::new(0.0, -5.0);
    }
    if engine.keyboard_state.just_released(KeyCode::Down) {
        player.rotation += std::f32::consts::FRAC_PI_6;
    }
}

fn spawn_enemy_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        game_state.enemy_number += 1;
        game_state
            .enemy_labels
            .push(format!("enemy_{0}", game_state.enemy_number));
        let acceptable_y_up = (engine.window_dimensions.y / 2.0) - 45.0;
        let acceptable_y_down = -((engine.window_dimensions.y / 2.0) - 45.0);
        let spawn_x = (engine.window_dimensions.x / 2.0) + 100.0;
        let enemy: &mut Sprite = match game_state.enemy_number {
            x if x % 5 == 0 && x % 3 == 0 => engine.add_sprite(
                format!("enemy_{0}", game_state.enemy_number),
                SpritePreset::RacingCarYellow,
            ),
            x if x % 5 == 0 => engine.add_sprite(
                format!("enemy_{0}", game_state.enemy_number),
                SpritePreset::RacingCarBlue,
            ),
            x if x % 3 == 0 => engine.add_sprite(
                format!("enemy_{0}", game_state.enemy_number),
                SpritePreset::RacingCarBlack,
            ),
            _ => engine.add_sprite(
                format!("enemy_{0}", game_state.enemy_number),
                SpritePreset::RacingConeStraight,
            ),
        };
        let mut rng = rand::rng();
        let random_y = rng.random_range(-300.0..300.0);
        enemy.collision = true;
        enemy.translation += Vec2::new(spawn_x, random_y);
    }
}

fn move_enemy(engine: &mut Engine, game_state: &mut GameState) {
    for enemy_label in game_state.enemy_labels.iter_mut() {
        let edge_x = engine.window_dimensions.x / 2.0 - 25.0;
        if engine.sprites.get(enemy_label).is_some() {
            let enemy = engine.sprites.get_mut(enemy_label).unwrap();
            enemy.translation += Vec2::new(-20.0, 0.0);
            let current_x = enemy.translation.x;
            if current_x == edge_x {
                engine.sprites.remove(enemy_label);
            }
            for event in engine.collision_events.drain(..) {
                if event.state == CollisionState::Begin {
                    let enemy_type = enemy_label.clone();
                    match enemy_type
                        .split("_")
                        .nth(1)
                        .unwrap()
                        .parse::<u32>()
                        .unwrap()
                    {
                        x if x % 5 == 0 && x % 3 == 0 => {
                            game_state.hp -= 5;
                        }
                        x if x % 5 == 0 => {
                            game_state.hp -= 4;
                        }
                        x if x % 3 == 0 => {
                            game_state.hp -= 6;
                        }
                        _ => {
                            game_state.hp -= 2;
                        }
                    }

                    engine.sprites.remove(enemy_label);
                }
            }
        }
    }
}

fn progress_logic(engine: &mut Engine, game_state: &mut GameState) {
    let label_y_position = (engine.window_dimensions.y / 2.0) - 50.0;
    let label_x_position = (engine.window_dimensions.x / 2.0) - 50.0;
    let hp = engine.texts.get_mut("hp").unwrap();
    hp.translation = Vec2::new(label_x_position, label_y_position + 10.0);
    hp.value = format!("HP: {0}", game_state.hp);
}
