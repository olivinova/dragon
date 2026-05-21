use dragon_hoard::{GameState, GameTrack};

pub fn make_test_game() -> GameState {
    let mut game = GameState::default();
    game.update_track(GameTrack::Gold, 5000.0);
    game.update_track(GameTrack::Food, 100.0);
    game.update_track(GameTrack::Mana, 100.0);
    game.update_track(GameTrack::ManaCapacity, 100.0);
    game.update_track(GameTrack::ManaRegenPerSec, 1.0);
    game.housing_slots = 10;
    game.gold_capacity = 10000.0;
    game
}
