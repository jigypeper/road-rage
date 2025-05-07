use std::time::Duration;

use rand::Rng;
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    score: u32,
    high_score: u32,
    hp: u32,
    enemy_number: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
    game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            high_score: 0,
            hp: 100,
            enemy_number: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            game_over: false,
        }
    }
}

fn main() {
    let mut game = Game::new();
    let binding = std::fs::read_to_string("./high_score.txt").unwrap_or(String::from("0"));
    let high_score = binding.trim();
    let game_state = GameState::default();
    let player = game.add_sprite("Player", SpritePreset::RacingCarRed);
    player.translation = Vec2::new(-650.0, 0.0);
    player.collision = true;
    player.layer = 10.0;
    let _ = game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);
    let _ = game.add_text("hp", "HP: 100");
    let _ = game.add_text("score", "Score: 0");
    let _ = game.add_text("high_score", format!("High Score: {0}", high_score));
    let _ = game.add_text("score", "0");
    let game_over_text = game.add_text("game_over", "GAME OVER!");
    game_over_text.translation = Vec2::new(0.0, 5000.0);
    game_over_text.font_size = 60.0;

    for i in 0..20 {
        let road_line = game.add_sprite(format!("roadline_{i}"), SpritePreset::RacingBarrierWhite);
        road_line.scale = 0.1;
        road_line.translation.x = (-600 + 150 * i) as f32;
    }

    let _ = game.add_logic(progress_logic);
    let _ = game.add_logic(control_logic);
    let _ = game.add_logic(spawn_enemy_logic);
    let _ = game.add_logic(move_enemy);
    let _ = game.add_logic(move_road_line);
    let _ = game.add_logic(collision_logic);
    let _ = game.add_logic(game_over_logic);
    let _ = game.run(game_state);
}

fn control_logic(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.game_over {
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
}

fn spawn_enemy_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.spawn_timer.tick(engine.delta).just_finished() && !game_state.game_over {
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
    if !game_state.game_over {
        let player_x = engine.sprites.get_mut("Player").unwrap().translation.x;
        for enemy_label in game_state.enemy_labels.iter_mut() {
            if let Some(enemy) = engine.sprites.get_mut(enemy_label) {
                enemy.translation += Vec2::new(-30.0, 0.0);
                if enemy.translation.x < -655.0 {
                    engine.sprites.remove(enemy_label);
                    game_state.score += 1;
                }
            }
        }
    }
}

fn move_road_line(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.game_over {
        for sprite in engine.sprites.values_mut() {
            if sprite.label.starts_with("roadline") {
                sprite.translation.x -= 30.0;
                if sprite.translation.x < -675.0 {
                    sprite.translation.x += 1500.0;
                }
            }
        }
    }
}

fn collision_logic(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.game_over {
        for event in engine.collision_events.drain(..) {
            if event.state == CollisionState::Begin && event.pair.one_starts_with("Player") {
                for label in [event.pair.0, event.pair.1] {
                    if label != "Player" {
                        match label.split("_").nth(1).unwrap().parse::<u32>().unwrap() {
                            x if x % 5 == 0 && x % 3 == 0 => {
                                game_state.hp = game_state.hp.checked_sub(5).unwrap_or(0);
                            }
                            x if x % 5 == 0 => {
                                game_state.hp = game_state.hp.checked_sub(4).unwrap_or(0);
                            }
                            x if x % 3 == 0 => {
                                game_state.hp = game_state.hp.checked_sub(6).unwrap_or(0);
                            }
                            _ => {
                                game_state.hp = game_state.hp.checked_sub(2).unwrap_or(0);
                            }
                        }

                        engine.sprites.remove(&label);
                    }
                }
                engine.sprites.get_mut("Player").unwrap().rotation += 1.25 * std::f32::consts::PI;
                engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.2);
            }
        }
    }
}

fn progress_logic(engine: &mut Engine, game_state: &mut GameState) {
    let label_y_position = (engine.window_dimensions.y / 2.0) - 50.0;
    let label_x_position = (engine.window_dimensions.x / 2.0) - 50.0;
    let hp = engine.texts.get_mut("hp").unwrap();
    hp.translation = Vec2::new(-label_x_position, label_y_position + 10.0);
    hp.value = format!("HP: {0}", game_state.hp);
    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation = Vec2::new(label_x_position - 60.0, label_y_position - 10.0);
    let score = engine.texts.get_mut("score").unwrap();
    score.translation = Vec2::new(label_x_position - 40.0, label_y_position - 40.0);
    score.value = format!("Score: {0}", game_state.score);
}

fn game_over_logic(engine: &mut Engine, game_state: &mut GameState) {
    // check health, if at 0, set game_over to true
    // delete player sprite?
    // display message with high score
    // save high score
    if game_state.hp == 0 {
        engine.audio_manager.stop_music();
        game_state.game_over = true;
        engine.sprites.drain();
        engine.texts.get_mut("game_over").unwrap().translation = Vec2::new(0.0, 0.0);
        game_state.high_score = if game_state.score > game_state.high_score {
            game_state.score
        } else {
            game_state.high_score
        };
        if engine.keyboard_state.pressed(KeyCode::Escape) {
            let _ = std::fs::write("./high_score.txt", format!("{0}", game_state.high_score));
            engine.should_exit;
        }
    }
}

fn game_difficulty_logic(engine: &mut Engine, game_state: &mut GameState) {
    // check score
    // decrease timer on point thresholds e.g. 80 -> 0.3, 150 -> 0.1
    match game_state.score {
        x if x > 60 => game_state.spawn_timer = Timer::from_seconds(0.3, TimerMode::Repeating),
        x if x > 150 => game_state.spawn_timer = Timer::from_seconds(0.2, TimerMode::Repeating),
        x if x > 250 => game_state.spawn_timer = Timer::from_seconds(0.1, TimerMode::Repeating),
        _ => game_state.spawn_timer = Timer::from_seconds(0.5, TimerMode::Repeating),
    }
}
