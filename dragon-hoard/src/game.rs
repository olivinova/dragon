use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GameState {
    pub gold: f64,
    pub gold_per_sec: f64,
    pub kobolds: u32,
    pub food: f64,
    pub housing: u32,
    pub housing_progress: f64,
    pub assigned_mining: u32,
    pub assigned_farming: u32,
    pub assigned_digging: u32,
    pub kobold_efficiency: f64,
    pub kobold_upgrade_level: u32,
    pub click_multiplier: f64,
    pub training_level: u32,
    pub vault_unlocked: bool,
    pub magic_level: u32,
    pub mana: f64,
    pub mana_capacity: f64,
    pub mana_regen_per_sec: f64,
    pub enchantments: Vec<Enchantment>,
    pub towns: Vec<Town>,
    pub dungeons: Vec<Dungeon>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            gold: 0.0,
            gold_per_sec: 0.0,
            kobolds: 0,
            food: 20.0,
            housing: 5,
            housing_progress: 0.0,
            assigned_mining: 0,
            assigned_farming: 0,
            assigned_digging: 0,
            kobold_efficiency: 1.0,
            kobold_upgrade_level: 0,
            click_multiplier: 1.0,
            training_level: 0,
            vault_unlocked: false,
            magic_level: 0,
            mana: 0.0,
            mana_capacity: 10.0,
            mana_regen_per_sec: 0.5,
            enchantments: Vec::new(),
            towns: vec![
                Town {
                    name: "Pebbleton".to_string(),
                    level: 1,
                    conquered: false,
                    reward_gold_per_sec: 1.0,
                    difficulty: 2.0,
                },
                Town {
                    name: "Goldbridge".to_string(),
                    level: 2,
                    conquered: false,
                    reward_gold_per_sec: 4.0,
                    difficulty: 6.0,
                },
                Town {
                    name: "Ironkeep".to_string(),
                    level: 3,
                    conquered: false,
                    reward_gold_per_sec: 12.0,
                    difficulty: 14.0,
                },
            ],
            dungeons: vec![
                Dungeon {
                    name: "Old Cave".to_string(),
                    level: 1,
                    cleared: false,
                    difficulty: 3.0,
                    reward_gold: 80.0,
                },
                Dungeon {
                    name: "Fallen Crypt".to_string(),
                    level: 2,
                    cleared: false,
                    difficulty: 8.0,
                    reward_gold: 220.0,
                },
                Dungeon {
                    name: "Dragon's Maw".to_string(),
                    level: 4,
                    cleared: false,
                    difficulty: 25.0,
                    reward_gold: 1200.0,
                },
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Enchantment {
    pub name: String,
    pub kind: String,
    pub effect: String,
    pub power: f64,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Town {
    pub name: String,
    pub level: u32,
    pub conquered: bool,
    pub reward_gold_per_sec: f64,
    pub difficulty: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Dungeon {
    pub name: String,
    pub level: u32,
    pub cleared: bool,
    pub difficulty: f64,
    pub reward_gold: f64,
}

/// Supported tracks for programmatic game state adjustments.
///
/// This enum makes it easier to apply delta-based changes to the game state
/// in a generic way, rather than mutating fields directly from UI code.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameTrack {
    Gold,
    GoldPerSec,
    Food,
    Mana,
    ManaCapacity,
    ManaRegenPerSec,
    Housing,
    HousingProgress,
    Kobolds,
    AssignedMining,
    AssignedFarming,
    AssignedDigging,
    KoboldEfficiency,
    TrainingLevel,
    MagicLevel,
    ClickMultiplier,
}

/// Property types that may be associated with a tracked game value.
///
/// Each track can expose an optional current value, per-second rate, capacity,
/// and/or a modifier. This makes it easier to render a generic resource UI.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameTrackProperty {
    Current,
    PerSecond,
    Capacity,
    Modifier,
}

/// A simple metadata container for a tracked value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GameTrackStats {
    pub current: Option<f64>,
    pub per_second: Option<f64>,
    pub capacity: Option<f64>,
    pub modifier: Option<f64>,
}

impl GameTrackStats {
    pub fn new(
        current: Option<f64>,
        per_second: Option<f64>,
        capacity: Option<f64>,
        modifier: Option<f64>,
    ) -> Self {
        Self {
            current,
            per_second,
            capacity,
            modifier,
        }
    }

    pub fn property(&self, property: GameTrackProperty) -> Option<f64> {
        match property {
            GameTrackProperty::Current => self.current,
            GameTrackProperty::PerSecond => self.per_second,
            GameTrackProperty::Capacity => self.capacity,
            GameTrackProperty::Modifier => self.modifier,
        }
    }
}

impl GameState {
    pub fn tick(&mut self, dt_seconds: f64) {
        self.gold += self.gold_per_sec * dt_seconds;
        self.gold += self.assigned_mining as f64 * 0.6 * self.kobold_efficiency * dt_seconds;
        let upkeep = self.kobold_upkeep() * dt_seconds;
        self.food += self.assigned_farming as f64 * 0.35 * self.kobold_efficiency * dt_seconds;
        self.food = (self.food - upkeep).max(0.0);
        if self.food > 99999.0 {
            self.food = 99999.0;
        }
        self.housing_progress +=
            self.assigned_digging as f64 * 0.04 * self.kobold_efficiency * dt_seconds;
        while self.housing_progress >= 1.0 {
            self.housing += 1;
            self.housing_progress -= 1.0;
        }
        // mana regeneration
        if self.mana < self.mana_capacity {
            self.mana += self.mana_regen_per_sec * dt_seconds;
            if self.mana > self.mana_capacity {
                self.mana = self.mana_capacity;
            }
        }
    }

    pub fn click_loot(&mut self) {
        self.add_track(GameTrack::Gold, 1.0 * self.click_multiplier);
    }

    pub fn kobold_cost(&self) -> f64 {
        20.0 * 1.25_f64.powi(self.kobolds as i32)
    }

    pub fn recruit_kobold(&mut self) -> bool {
        let cost = self.kobold_cost();
        if self.gold >= cost && self.food >= 5.0 && self.kobolds < self.housing {
            self.subtract_track(GameTrack::Gold, cost);
            self.subtract_track(GameTrack::Food, 5.0);
            self.add_track(GameTrack::Kobolds, 1.0);
            true
        } else {
            false
        }
    }

    pub fn free_kobolds(&self) -> u32 {
        self.kobolds
            .saturating_sub(self.assigned_mining + self.assigned_farming + self.assigned_digging)
    }

    pub fn assign_mining(&mut self) -> bool {
        if self.free_kobolds() > 0 {
            self.add_track(GameTrack::AssignedMining, 1.0);
            true
        } else {
            false
        }
    }

    pub fn unassign_mining(&mut self) -> bool {
        if self.assigned_mining > 0 {
            self.subtract_track(GameTrack::AssignedMining, 1.0);
            true
        } else {
            false
        }
    }

    pub fn assign_farming(&mut self) -> bool {
        if self.free_kobolds() > 0 {
            self.add_track(GameTrack::AssignedFarming, 1.0);
            true
        } else {
            false
        }
    }

    pub fn unassign_farming(&mut self) -> bool {
        if self.assigned_farming > 0 {
            self.subtract_track(GameTrack::AssignedFarming, 1.0);
            true
        } else {
            false
        }
    }

    pub fn assign_digging(&mut self) -> bool {
        if self.free_kobolds() > 0 {
            self.add_track(GameTrack::AssignedDigging, 1.0);
            true
        } else {
            false
        }
    }

    pub fn unassign_digging(&mut self) -> bool {
        if self.assigned_digging > 0 {
            self.subtract_track(GameTrack::AssignedDigging, 1.0);
            true
        } else {
            false
        }
    }

    pub fn kobold_upkeep(&self) -> f64 {
        self.kobolds as f64 * 0.2
    }

    pub fn kobold_upgrade_cost(&self) -> (f64, f64) {
        let gold_cost = 120.0 * 1.5_f64.powi(self.kobold_upgrade_level as i32);
        let mana_cost = 12.0 + self.kobold_upgrade_level as f64 * 4.0;
        (gold_cost, mana_cost)
    }

    pub fn upgrade_kobold_efficiency(&mut self) -> bool {
        let (gold_cost, mana_cost) = self.kobold_upgrade_cost();
        if self.track_value(GameTrack::Gold) >= gold_cost
            && self.track_value(GameTrack::Mana) >= mana_cost
            && self.magic_level > 0
        {
            self.subtract_track(GameTrack::Gold, gold_cost);
            self.subtract_track(GameTrack::Mana, mana_cost);
            self.kobold_upgrade_level += 1;
            self.add_track(GameTrack::KoboldEfficiency, 0.18);
            true
        } else {
            false
        }
    }

    pub fn training_cost(&self) -> f64 {
        50.0 * 1.7_f64.powi(self.training_level as i32)
    }

    pub fn buy_training(&mut self) -> bool {
        let cost = self.training_cost();
        if self.track_value(GameTrack::Gold) >= cost {
            self.subtract_track(GameTrack::Gold, cost);
            self.add_track(GameTrack::TrainingLevel, 1.0);
            self.add_track(GameTrack::ClickMultiplier, 0.5); // stronger clicks
            true
        } else {
            false
        }
    }

    pub fn vault_cost() -> f64 {
        500.0
    }

    pub fn buy_vault(&mut self) -> bool {
        let cost = Self::vault_cost();
        if self.track_value(GameTrack::Gold) >= cost && !self.vault_unlocked {
            self.subtract_track(GameTrack::Gold, cost);
            self.vault_unlocked = true;
            self.add_track(GameTrack::GoldPerSec, 20.0);
            true
        } else {
            false
        }
    }

    /// Reset the game state to the default starting values.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Read the current value of a tracked game field.
    ///
    /// Uses the same track enum as the update helpers for consistency.
    #[allow(dead_code)]
    pub fn track_value(&self, track: GameTrack) -> f64 {
        self.track_stats(track)
            .current
            .unwrap_or(0.0)
    }

    #[allow(dead_code)]
    pub fn track_stats(&self, track: GameTrack) -> GameTrackStats {
        match track {
            GameTrack::Gold => GameTrackStats::new(
                Some(self.gold),
                Some(self.gold_per_sec),
                None,
                None,
            ),
            GameTrack::GoldPerSec => GameTrackStats::new(Some(self.gold_per_sec), None, None, None),
            GameTrack::Food => GameTrackStats::new(Some(self.food), None, None, None),
            GameTrack::Mana => GameTrackStats::new(
                Some(self.mana),
                Some(self.mana_regen_per_sec),
                Some(self.mana_capacity),
                None,
            ),
            GameTrack::ManaCapacity => GameTrackStats::new(Some(self.mana_capacity), None, None, None),
            GameTrack::ManaRegenPerSec => GameTrackStats::new(Some(self.mana_regen_per_sec), None, None, None),
            GameTrack::Housing => GameTrackStats::new(Some(self.housing as f64), None, None, None),
            GameTrack::HousingProgress => GameTrackStats::new(Some(self.housing_progress), None, None, None),
            GameTrack::Kobolds => GameTrackStats::new(
                Some(self.kobolds as f64),
                None,
                Some(self.housing as f64),
                Some(self.kobold_efficiency),
            ),
            GameTrack::AssignedMining => GameTrackStats::new(
                Some(self.assigned_mining as f64),
                None,
                Some(self.kobolds as f64),
                Some(self.kobold_efficiency),
            ),
            GameTrack::AssignedFarming => GameTrackStats::new(
                Some(self.assigned_farming as f64),
                None,
                Some(self.kobolds as f64),
                Some(self.kobold_efficiency),
            ),
            GameTrack::AssignedDigging => GameTrackStats::new(
                Some(self.assigned_digging as f64),
                None,
                Some(self.kobolds as f64),
                Some(self.kobold_efficiency),
            ),
            GameTrack::KoboldEfficiency => GameTrackStats::new(Some(self.kobold_efficiency), None, None, None),
            GameTrack::TrainingLevel => GameTrackStats::new(Some(self.training_level as f64), None, None, None),
            GameTrack::MagicLevel => GameTrackStats::new(Some(self.magic_level as f64), None, None, None),
            GameTrack::ClickMultiplier => GameTrackStats::new(Some(self.click_multiplier), None, None, None),
        }
    }

    #[allow(dead_code)]
    pub fn track_property_value(
        &self,
        track: GameTrack,
        property: GameTrackProperty,
    ) -> Option<f64> {
        self.track_stats(track).property(property)
    }

    /// Apply a delta adjustment to a tracked game value.
    ///
    /// Positive `delta` values add, negative values subtract.
    /// This helper keeps clamping and special bounds logic in one place.
    #[allow(dead_code)]
    pub fn adjust_track(&mut self, track: GameTrack, delta: f64) {
        match track {
            GameTrack::Gold => {
                self.gold = (self.gold + delta).max(0.0);
            }
            GameTrack::GoldPerSec => {
                self.gold_per_sec = (self.gold_per_sec + delta).max(0.0);
            }
            GameTrack::Food => {
                self.food = (self.food + delta).max(0.0);
            }
            GameTrack::Mana => {
                self.mana += delta;
                if self.mana < 0.0 {
                    self.mana = 0.0;
                }
                if self.mana > self.mana_capacity {
                    self.mana = self.mana_capacity;
                }
            }
            GameTrack::ManaCapacity => {
                self.mana_capacity = (self.mana_capacity + delta).max(0.0);
                if self.mana > self.mana_capacity {
                    self.mana = self.mana_capacity;
                }
            }
            GameTrack::ManaRegenPerSec => {
                self.mana_regen_per_sec = (self.mana_regen_per_sec + delta).max(0.0);
            }
            GameTrack::Housing => {
                if delta >= 0.0 {
                    self.housing = self.housing.saturating_add(delta.round() as u32);
                } else {
                    self.housing = self.housing.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::HousingProgress => {
                self.housing_progress = (self.housing_progress + delta).max(0.0);
            }
            GameTrack::Kobolds => {
                if delta >= 0.0 {
                    self.kobolds = self.kobolds.saturating_add(delta.round() as u32);
                } else {
                    self.kobolds = self.kobolds.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::AssignedMining => {
                if delta >= 0.0 {
                    self.assigned_mining = self.assigned_mining.saturating_add(delta.round() as u32);
                } else {
                    self.assigned_mining = self.assigned_mining.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::AssignedFarming => {
                if delta >= 0.0 {
                    self.assigned_farming = self.assigned_farming.saturating_add(delta.round() as u32);
                } else {
                    self.assigned_farming = self.assigned_farming.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::AssignedDigging => {
                if delta >= 0.0 {
                    self.assigned_digging = self.assigned_digging.saturating_add(delta.round() as u32);
                } else {
                    self.assigned_digging = self.assigned_digging.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::KoboldEfficiency => {
                self.kobold_efficiency = (self.kobold_efficiency + delta).max(0.0);
            }
            GameTrack::TrainingLevel => {
                if delta >= 0.0 {
                    self.training_level = self.training_level.saturating_add(delta.round() as u32);
                } else {
                    self.training_level = self.training_level.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::MagicLevel => {
                if delta >= 0.0 {
                    self.magic_level = self.magic_level.saturating_add(delta.round() as u32);
                } else {
                    self.magic_level = self.magic_level.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::ClickMultiplier => {
                self.click_multiplier = (self.click_multiplier + delta).max(0.0);
            }
        }
    }

    /// Update a tracked game value to an exact value.
    ///
    /// This is useful when you want to set a track directly instead of applying
    /// a delta adjustment.
    #[allow(dead_code)]
    pub fn update_track(&mut self, track: GameTrack, value: f64) {
        match track {
            GameTrack::Gold => self.gold = value.max(0.0),
            GameTrack::GoldPerSec => self.gold_per_sec = value.max(0.0),
            GameTrack::Food => self.food = value.max(0.0),
            GameTrack::Mana => {
                self.mana = value.max(0.0);
                if self.mana > self.mana_capacity {
                    self.mana = self.mana_capacity;
                }
            }
            GameTrack::ManaCapacity => {
                self.mana_capacity = value.max(0.0);
                if self.mana > self.mana_capacity {
                    self.mana = self.mana_capacity;
                }
            }
            GameTrack::ManaRegenPerSec => self.mana_regen_per_sec = value.max(0.0),
            GameTrack::Housing => self.housing = value.max(0.0).round() as u32,
            GameTrack::HousingProgress => self.housing_progress = value.max(0.0),
            GameTrack::Kobolds => self.kobolds = value.max(0.0).round() as u32,
            GameTrack::AssignedMining => self.assigned_mining = value.max(0.0).round() as u32,
            GameTrack::AssignedFarming => self.assigned_farming = value.max(0.0).round() as u32,
            GameTrack::AssignedDigging => self.assigned_digging = value.max(0.0).round() as u32,
            GameTrack::KoboldEfficiency => self.kobold_efficiency = value.max(0.0),
            GameTrack::TrainingLevel => self.training_level = value.max(0.0).round() as u32,
            GameTrack::MagicLevel => self.magic_level = value.max(0.0).round() as u32,
            GameTrack::ClickMultiplier => self.click_multiplier = value.max(0.0),
        }
    }

    /// Convenience wrapper for adding to a track.
    #[allow(dead_code)]
    pub fn add_track(&mut self, track: GameTrack, amount: f64) {
        self.adjust_track(track, amount);
    }

    /// Convenience wrapper for subtracting from a track.
    #[allow(dead_code)]
    pub fn subtract_track(&mut self, track: GameTrack, amount: f64) {
        self.adjust_track(track, -amount);
    }

    // Magic: learn and craft enchantments
    pub fn magic_cost(&self) -> f64 {
        200.0 * 2.0_f64.powi(self.magic_level as i32)
    }

    pub fn learn_magic(&mut self) -> bool {
        let cost = self.magic_cost();
        if self.track_value(GameTrack::Gold) >= cost {
            self.subtract_track(GameTrack::Gold, cost);
            self.add_track(GameTrack::MagicLevel, 1.0);
            self.add_track(GameTrack::ManaCapacity, 10.0);
            self.add_track(GameTrack::ManaRegenPerSec, 0.5);
            true
        } else {
            false
        }
    }

    pub fn enchant_cost(&self) -> (f64, f64) {
        // returns (gold_cost, mana_cost)
        let gold_cost = 100.0 * (1.0 + 0.5 * (self.magic_level as f64));
        let mana_cost = 10.0 + 5.0 * (self.magic_level as f64);
        (gold_cost, mana_cost)
    }

    pub fn craft_enchantment(&mut self) -> bool {
        let (gold_cost, mana_cost) = self.enchant_cost();
        if self.track_value(GameTrack::Gold) >= gold_cost && self.track_value(GameTrack::Mana) >= mana_cost {
            self.subtract_track(GameTrack::Gold, gold_cost);
            self.subtract_track(GameTrack::Mana, mana_cost);

            let index = self.enchantments.len() % 5;
            let (kind, effect) = match index {
                0 => (
                    "Flame Glyph",
                    "Adds burning power to each treasure you keep".to_string(),
                ),
                1 => (
                    "Storm Sigil",
                    "Harnesses lightning to charge your hoard".to_string(),
                ),
                2 => (
                    "Spirit Crystal",
                    "Draws on dragon spirits for extra strength".to_string(),
                ),
                3 => (
                    "Runic Scale",
                    "Hardens your hoard and increases passive flow".to_string(),
                ),
                _ => (
                    "Moonshard",
                    "Infuses the hoard with mysterious lunar magic".to_string(),
                ),
            };

            let power = 1.0
                + 0.7 * (self.magic_level as f64)
                + 0.25 * (self.training_level as f64)
                + 0.3 * (index as f64);
            let value = gold_cost * 0.7 + power * 12.0;
            let name = format!(
                "{} of the {}",
                kind,
                if self.magic_level > 0 {
                    format!("Mage L{}", self.magic_level)
                } else {
                    "Wyrm".to_string()
                }
            );

            let ench = Enchantment {
                name,
                kind: kind.to_string(),
                effect,
                power,
                value,
            };
            // immediate passive benefit
            self.add_track(GameTrack::GoldPerSec, ench.power * 2.5);
            self.enchantments.push(ench);
            true
        } else {
            false
        }
    }

    pub fn sell_enchantment(&mut self, index: usize) -> bool {
        if index < self.enchantments.len() {
            let ench = self.enchantments.remove(index);
            // refund some gold and remove passive bonus
            let refund = (ench.value * 0.7).round();
            self.add_track(GameTrack::Gold, refund);
            self.subtract_track(GameTrack::GoldPerSec, ench.power * 2.0);
            true
        } else {
            false
        }
    }

    pub fn total_enchant_power(&self) -> f64 {
        self.enchantments.iter().map(|e| e.power).sum()
    }

    // Conquest & Dungeons
    pub fn town_cost(&self, idx: usize) -> f64 {
        self.towns
            .get(idx)
            .map(|t| 120.0 * (t.level as f64))
            .unwrap_or(99999.0)
    }

    pub fn try_conquer_town(&mut self, idx: usize) -> (bool, String) {
        if idx >= self.towns.len() {
            return (false, "Invalid town".to_string());
        }
        let cost = self.town_cost(idx);
        if self.track_value(GameTrack::Gold) < cost {
            return (false, "Not enough gold".to_string());
        }
        // compute strength vs difficulty before mutably borrowing town
        let strength = (self.training_level as f64) * 1.5
            + (self.magic_level as f64) * 1.2
            + self.total_enchant_power();
        if self.towns[idx].conquered {
            return (false, format!("{} already conquered", self.towns[idx].name));
        }
        // consume cost
        self.subtract_track(GameTrack::Gold, cost);
        let town = &mut self.towns[idx];
        if strength >= town.difficulty {
            let reward = town.reward_gold_per_sec;
            let name = town.name.clone();
            town.conquered = true;
            self.add_track(GameTrack::GoldPerSec, reward);
            return (
                true,
                format!("Conquered {}! +{:.1} gold/sec", name, reward),
            );
        } else {
            let name = town.name.clone();
            let penalty = (cost * 0.25).round();
            self.subtract_track(GameTrack::Gold, penalty);
            return (
                false,
                format!("Failed to conquer {}. Lost {:.0} gold.", name, penalty),
            );
        }
    }

    pub fn dungeon_cost(&self, idx: usize) -> (f64, f64) {
        // returns (gold_cost, mana_required)
        self.dungeons
            .get(idx)
            .map(|d| (d.reward_gold * 0.2, 5.0 + d.level as f64 * 3.0))
            .unwrap_or((99999.0, 99999.0))
    }

    pub fn explore_dungeon(&mut self, idx: usize) -> (bool, String) {
        if idx >= self.dungeons.len() {
            return (false, "Invalid dungeon".to_string());
        }
        let (gold_cost, mana_req) = self.dungeon_cost(idx);
        if self.gold < gold_cost {
            return (false, "Not enough gold".to_string());
        }
        if self.mana < mana_req {
            return (false, "Not enough mana".to_string());
        }
        // compute strength before mutable borrow
        let strength = (self.training_level as f64) * 1.5
            + (self.magic_level as f64) * 1.2
            + self.total_enchant_power();
        if self.dungeons[idx].cleared {
            return (false, format!("{} already cleared", self.dungeons[idx].name));
        }
        self.subtract_track(GameTrack::Gold, gold_cost);
        self.subtract_track(GameTrack::Mana, mana_req);
        let dungeon = &mut self.dungeons[idx];
        if strength >= dungeon.difficulty {
            dungeon.cleared = true;
            let reward = dungeon.reward_gold;
            let name = dungeon.name.clone();
            let level = dungeon.level;
            // reward: gold and possible enchantment
            self.add_track(GameTrack::Gold, reward);
            // create a special enchantment
            let power = 2.0 + (level as f64) * 1.2 + (self.magic_level as f64) * 0.5;
            let value = reward * 0.7 + power * 20.0;
            let (kind, effect) = if level >= 4 {
                (
                    "Abyssal Talisman".to_string(),
                    "Steel your mind and drain enemy willpower.".to_string(),
                )
            } else if level >= 2 {
                (
                    "Runic Fang".to_string(),
                    "Imbues treasure with fierce strike power.".to_string(),
                )
            } else {
                (
                    "Cave Charm".to_string(),
                    "Strengthens your grip on the gold.".to_string(),
                )
            };
            let enchant_name = format!("{} of {}", kind, name);
            let ench = Enchantment {
                name: enchant_name.clone(),
                kind,
                effect,
                power,
                value,
            };
            self.add_track(GameTrack::GoldPerSec, ench.power * 2.0);
            self.enchantments.push(ench);
            return (
                true,
                format!(
                    "Cleared {}! Found an enchanted relic and {:.0} gold.",
                    enchant_name, reward
                ),
            );
        } else {
            let name = dungeon.name.clone();
            // fail: take damage to gold
            let loss = (gold_cost * 0.5).round();
            self.subtract_track(GameTrack::Gold, loss);
            return (
                false,
                format!("Failed in {}. Lost {:.0} gold.", name, loss),
            );
        }
    }

    pub fn save(&self) {
        let _ = LocalStorage::set("dragon_hoard_save", self);
    }

    pub fn load() -> Option<Self> {
        LocalStorage::get("dragon_hoard_save").ok()
    }
}
