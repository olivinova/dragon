mod common;
use common::make_test_game;
use dragon_hoard::GameTrack;

#[test]
fn assign_and_unassign_workers_updates_free_kobolds() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 1000.0);
    game.update_track(GameTrack::Food, 100.0);
    game.recruit_kobold();
    game.recruit_kobold();
    game.recruit_kobold();
    game.recruit_kobold();
    game.recruit_kobold();

    assert_eq!(game.free_kobolds(), 5);
    assert!(game.assign_mining());
    assert!(game.assign_farming());
    assert!(game.assign_digging());
    assert!(game.assign_military());
    assert!(game.assign_research());
    assert_eq!(game.free_kobolds(), 0);

    assert!(game.unassign_mining());
    assert!(game.unassign_farming());
    assert!(game.unassign_digging());
    assert!(game.unassign_military());
    assert!(game.unassign_research());
    assert_eq!(game.free_kobolds(), 5);
}

#[test]
fn cannot_assign_more_workers_than_available() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 100.0);
    game.update_track(GameTrack::Food, 50.0);
    game.recruit_kobold();

    assert_eq!(game.free_kobolds(), 1);
    assert!(game.assign_mining());
    assert_eq!(game.free_kobolds(), 0);
    assert!(!game.assign_farming(), "Should fail with no free kobolds");
}

#[test]
fn designate_storage_to_housing() {
    let mut game = make_test_game();
    let starting_storage = game.storage_slots;
    
    assert!(game.designate_storage_to_housing());
    assert_eq!(game.storage_slots, starting_storage - 1);
    assert_eq!(game.housing_slots, 11);
}

#[test]
fn reclaim_housing_to_storage() {
    let mut game = make_test_game();
    game.designate_storage_to_housing();
    
    assert!(game.reclaim_housing_to_storage());
    assert_eq!(game.storage_slots, 5);
    assert_eq!(game.housing_slots, 10);
}

#[test]
fn designate_storage_to_furniture() {
    let mut game = make_test_game();
    let starting_furniture = game.furniture_slots;
    
    assert!(game.designate_storage_to_furniture());
    assert_eq!(game.furniture_slots, starting_furniture + 1);
}

#[test]
fn reclaim_furniture_to_storage() {
    let mut game = make_test_game();
    game.designate_storage_to_furniture();
    
    assert!(game.reclaim_furniture_to_storage());
    assert_eq!(game.furniture_slots, 0);
}

#[test]
fn kobold_upkeep_scales_with_population() {
    let mut game = make_test_game();
    let upkeep1 = game.kobold_upkeep();
    
    game.add_track(GameTrack::Kobolds, 1.0);
    let upkeep2 = game.kobold_upkeep();
    
    assert!(upkeep2 > upkeep1, "Upkeep should increase with population");
}
