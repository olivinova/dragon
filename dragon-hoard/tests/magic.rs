mod common;
use common::make_test_game;
use dragon_hoard::GameTrack;

#[test]
fn learn_magic_basic() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, game.magic_cost());
    
    assert!(game.learn_magic());
    assert_eq!(game.track_value(GameTrack::MagicLevel), 1.0);
    assert!(game.track_value(GameTrack::ManaCapacity) > 10.0);
    assert!(game.track_value(GameTrack::ManaRegenPerSec) > 0.5);
}

#[test]
fn specializations_require_magic_level() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 1000.0);
    game.update_track(GameTrack::AssignedResearch, 2.0);
    
    assert!(!game.learn_necromancy(), "Should fail without magic");
    
    game.update_track(GameTrack::Gold, game.magic_cost());
    game.learn_magic();
    
    game.update_track(GameTrack::Gold, game.specialization_cost(0));
    assert!(game.learn_necromancy(), "Should succeed with magic");
}

#[test]
fn learn_magic_and_specializations_require_research() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, game.magic_cost());
    assert!(game.learn_magic());
    assert_eq!(game.track_value(GameTrack::MagicLevel), 1.0);

    game.update_track(GameTrack::Gold, game.specialization_cost(0));
    game.update_track(GameTrack::AssignedResearch, 2.0);

    assert!(game.learn_necromancy());
    assert_eq!(game.track_value(GameTrack::NecromancyLevel), 1.0);
    assert_eq!(game.specialization_research_cost(), 3);
}

#[test]
fn craft_enchantment_consumes_resources_and_adds_bonus() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 1000.0);
    game.update_track(GameTrack::Mana, 100.0);

    assert!(game.craft_enchantment());
    assert_eq!(game.enchantments.len(), 1);
    assert!(game.track_value(GameTrack::GoldPerSec) > 0.0);
}

#[test]
fn sell_enchantment_refunds_gold() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 1000.0);
    game.update_track(GameTrack::Mana, 100.0);
    
    game.craft_enchantment();
    let gold_after_craft = game.track_value(GameTrack::Gold);
    
    game.sell_enchantment(0);
    let gold_after_sell = game.track_value(GameTrack::Gold);
    
    assert!(gold_after_sell > gold_after_craft, "Should receive refund");
    assert_eq!(game.enchantments.len(), 0);
}

#[test]
fn total_specialization_levels_sum_correctly() {
    let mut game = make_test_game();
    game.update_track(GameTrack::NecromancyLevel, 2.0);
    game.update_track(GameTrack::AlchemyLevel, 1.0);
    game.update_track(GameTrack::ElementalLevel, 3.0);
    
    assert_eq!(game.total_specialization_levels(), 6);
}

#[test]
fn specialization_research_cost_scales_with_levels() {
    let mut game = make_test_game();
    let cost1 = game.specialization_research_cost();
    
    game.update_track(GameTrack::NecromancyLevel, 1.0);
    let cost2 = game.specialization_research_cost();
    
    assert!(cost2 > cost1, "Research cost should scale with specialization levels");
}

#[test]
fn enchanting_creates_unique_items() {
    let mut game = make_test_game();
    game.update_track(GameTrack::Gold, 5000.0);
    game.update_track(GameTrack::Mana, 500.0);
    
    game.craft_enchantment();
    game.craft_enchantment();
    
    assert_eq!(game.enchantments.len(), 2);
    assert_ne!(game.enchantments[0].kind, game.enchantments[1].kind);
}
