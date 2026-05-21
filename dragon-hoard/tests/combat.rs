mod common;
use common::make_test_game;
use dragon_hoard::GameTrack;

#[test]
fn try_conquer_town_fails_without_gold() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 0.0);
    let (success, message) = game.try_conquer_town(0);
    assert!(!success);
    assert!(message.contains("Not enough gold"));
}

#[test]
fn military_power_increases_with_soldiers() {
    let mut game = make_test_game();
    let power1 = game.military_power();
    
    game.add_track(GameTrack::AssignedMilitary, 1.0);
    let power2 = game.military_power();
    
    assert!(power2 > power1, "Military power should increase with assigned soldiers");
}

#[test]
fn elemental_specialization_boosts_military_power() {
    let mut game = make_test_game();
    game.add_track(GameTrack::AssignedMilitary, 5.0);
    
    let base_power = game.military_power();
    
    game.update_track(GameTrack::ElementalLevel, 1.0);
    let boosted_power = game.military_power();
    
    assert!(boosted_power > base_power, "Elemental should boost military power");
}

#[test]
fn explore_dungeon_fails_without_resources() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 0.0);
    game.update_track(GameTrack::Mana, 0.0);
    
    let (success, message) = game.explore_dungeon(0);
    assert!(!success);
    assert!(message.contains("Not enough") || message.contains("Invalid"));
}

#[test]
fn conquer_town_consumes_gold() {
    let mut game = make_test_game();
    let starting_gold = game.track_value(GameTrack::Gold);
    
    game.try_conquer_town(0);
    let remaining_gold = game.track_value(GameTrack::Gold);
    
    // Either conquered or failed with penalty
    assert!(remaining_gold <= starting_gold);
}
