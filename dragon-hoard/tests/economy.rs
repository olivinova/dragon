mod common;
use common::make_test_game;
use dragon_hoard::GameTrack;

#[test]
fn click_loot_increases_gold() {
    let mut game = make_test_game();
    let starting_gold = game.track_value(GameTrack::Gold);
    game.click_loot();
    assert_eq!(game.track_value(GameTrack::Gold), starting_gold + 10.0);
}

#[test]
fn recruit_kobold_requires_gold_and_food() {
    let mut game = make_test_game();
    let cost = game.kobold_cost();
    game.update_track(GameTrack::Gold, cost);
    game.update_track(GameTrack::Food, 5.0);

    let success = game.recruit_kobold();
    assert!(success);
    assert_eq!(game.kobolds, 1);
    assert_eq!(game.track_value(GameTrack::Gold), 0.0);
    assert_eq!(game.track_value(GameTrack::Food), 0.0);
}

#[test]
fn buy_training_increases_stats() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, game.training_cost());

    assert!(game.buy_training());
    assert_eq!(game.training_level, 1);
    assert_eq!(game.track_value(GameTrack::ClickMultiplier), 1.5);
}

#[test]
fn buy_vault_grants_gold_per_sec_and_only_once() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 1000.0);

    assert!(game.buy_vault());
    assert!(game.vault_unlocked);
    assert_eq!(game.track_value(GameTrack::GoldPerSec), 20.0);
    assert!(!game.buy_vault());
}

#[test]
fn tick_produces_gold_from_mining() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 0.0);
    game.add_track(GameTrack::Kobolds, 1.0);
    assert!(game.assign_mining());
    game.tick(1.0);

    assert!(game.track_value(GameTrack::Gold) > 0.0);
}

#[test]
fn recruit_kobold_cost_increases_with_population() {
    let mut game = make_test_game();
    let cost1 = game.kobold_cost();
    
    game.update_track(GameTrack::Kobolds, 1.0);
    let cost2 = game.kobold_cost();
    
    assert!(cost2 > cost1, "Second recruit should cost more");
}
