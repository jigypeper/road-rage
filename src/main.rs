use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    score: u32,
    high_score: u32,
    hp: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            high_score: 0,
            hp: 100,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("Player", SpritePreset::RacingCarRed);
    player.translation = Vec2::new(-650.0, 0.0);

    let game_state = GameState::default();

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let player = engine.sprites.get_mut("Player").unwrap();
    if engine.keyboard_state.pressed(KeyCode::Up) {
        player.rotation = std::f32::consts::FRAC_PI_6;
        player.translation += Vec2::new(0.0, 5.0);
    }
    if engine.keyboard_state.just_released(KeyCode::Up) {
        player.rotation -= std::f32::consts::FRAC_PI_6;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        player.rotation = -std::f32::consts::FRAC_PI_6;
        player.translation += Vec2::new(0.0, -5.0);
    }
    if engine.keyboard_state.just_released(KeyCode::Down) {
        player.rotation += std::f32::consts::FRAC_PI_6;
    }
}
