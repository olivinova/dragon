use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct GameState {
    pub gold: f64,
    pub gold_per_sec: f64,
    pub kobolds: u32,
    pub food: f64,
    #[serde(alias = "housing")]
    pub space: u32,
    #[serde(alias = "housing_progress")]
    pub space_progress: f64,
    pub space_soft_cap: f64,
    pub housing_slots: u32,
    pub storage_slots: u32,
    pub furniture_slots: u32,
    pub gold_capacity: f64,
    pub assigned_mining: u32,
    pub assigned_farming: u32,
    pub assigned_digging: u32,
    pub assigned_military: u32,
    pub assigned_research: u32,
    pub kobold_efficiency: f64,
    pub kobold_upgrade_level: u32,
    pub click_multiplier: f64,
    pub training_level: u32,
    pub vault_unlocked: bool,
    pub magic_level: u32,
    pub necromancy_level: u32,
    pub alchemy_level: u32,
    pub restoration_level: u32,
    pub elemental_level: u32,
    pub summoning_level: u32,
    pub enchanting_level: u32,
    pub mana: f64,
    pub mana_capacity: f64,
    pub mana_regen_per_sec: f64,
    pub enchantments: Vec<Enchantment>,
    pub towns: Vec<Town>,
    pub conquered_towns: u32,
    pub dungeons: Vec<Dungeon>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            gold: 0.0,
            gold_per_sec: 0.0,
            kobolds: 0,
            food: 20.0,
            space: 5,
            space_progress: 0.0,
            space_soft_cap: 1000.0,
            housing_slots: 0,
            storage_slots: 5,
            furniture_slots: 0,
            gold_capacity: 1000.0,
            assigned_mining: 0,
            assigned_farming: 0,
            assigned_digging: 0,
            assigned_military: 0,
            assigned_research: 0,
            kobold_efficiency: 1.0,
            kobold_upgrade_level: 0,
            click_multiplier: 1.0,
            training_level: 0,
            vault_unlocked: false,
            magic_level: 0,
            necromancy_level: 0,
            alchemy_level: 0,
            restoration_level: 0,
            elemental_level: 0,
            summoning_level: 0,
            enchanting_level: 0,
            mana: 0.0,
            mana_capacity: 10.0,
            mana_regen_per_sec: 0.5,
            enchantments: Vec::new(),
            conquered_towns: 0,
            towns: vec![
                Town {
                    name: "Pebbleton".to_string(),
                    level: 1,
                    conquered: false,
                    reward_gold_per_sec: 1.0,
                    difficulty: 2.0,
                    wants: "food".to_string(),
                    wants_amount: 10.0,
                    offers: "gold".to_string(),
                    offers_amount: 50.0,
                },
                Town {
                    name: "Goldbridge".to_string(),
                    level: 2,
                    conquered: false,
                    reward_gold_per_sec: 4.0,
                    difficulty: 6.0,
                    wants: "mana".to_string(),
                    wants_amount: 10.0,
                    offers: "gold".to_string(),
                    offers_amount: 200.0,
                },
                Town {
                    name: "Ironkeep".to_string(),
                    level: 3,
                    conquered: false,
                    reward_gold_per_sec: 12.0,
                    difficulty: 14.0,
                    wants: "gold".to_string(),
                    wants_amount: 300.0,
                    offers: "food".to_string(),
                    offers_amount: 90.0,
                },
                Town {
                    name: "Dragonhold Fortress".to_string(),
                    level: 4,
                    conquered: false,
                    reward_gold_per_sec: 32.0,
                    difficulty: 24.0,
                    wants: "gold".to_string(),
                    wants_amount: 450.0,
                    offers: "mana".to_string(),
                    offers_amount: 40.0,
                },
                Town {
                    name: "Crystal Spire".to_string(),
                    level: 5,
                    conquered: false,
                    reward_gold_per_sec: 80.0,
                    difficulty: 38.0,
                    wants: "food".to_string(),
                    wants_amount: 100.0,
                    offers: "mana".to_string(),
                    offers_amount: 50.0,
                },
                Town {
                    name: "Shadowmere Keep".to_string(),
                    level: 6,
                    conquered: false,
                    reward_gold_per_sec: 192.0,
                    difficulty: 54.0,
                    wants: "mana".to_string(),
                    wants_amount: 48.0,
                    offers: "gold".to_string(),
                    offers_amount: 720.0,
                },
                Town {
                    name: "Starlight Harbor".to_string(),
                    level: 7,
                    conquered: false,
                    reward_gold_per_sec: 448.0,
                    difficulty: 72.0,
                    wants: "food".to_string(),
                    wants_amount: 140.0,
                    offers: "mana".to_string(),
                    offers_amount: 70.0,
                },
                Town {
                    name: "Runehold Citadel".to_string(),
                    level: 8,
                    conquered: false,
                    reward_gold_per_sec: 1024.0,
                    difficulty: 92.0,
                    wants: "gold".to_string(),
                    wants_amount: 1200.0,
                    offers: "mana".to_string(),
                    offers_amount: 80.0,
                },
                Town {
                    name: "Obsidian Palace".to_string(),
                    level: 9,
                    conquered: false,
                    reward_gold_per_sec: 2304.0,
                    difficulty: 114.0,
                    wants: "mana".to_string(),
                    wants_amount: 72.0,
                    offers: "gold".to_string(),
                    offers_amount: 2880.0,
                },
                Town {
                    name: "Celestial Throne".to_string(),
                    level: 10,
                    conquered: false,
                    reward_gold_per_sec: 5120.0,
                    difficulty: 138.0,
                    wants: "food".to_string(),
                    wants_amount: 200.0,
                    offers: "mana".to_string(),
                    offers_amount: 100.0,
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
    /// What resource this town wants ("food", "mana", "gold")
    pub wants: String,
    pub wants_amount: f64,
    /// What resource this town offers in trade
    pub offers: String,
    pub offers_amount: f64,
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
#[derive(Copy, Clone, Debug)]
pub enum GameTrack {
    Gold,
    GoldPerSec,
    Food,
    Mana,
    ManaCapacity,
    ManaRegenPerSec,
    Space,
    SpaceProgress,
    SpaceSoftCap,
    Kobolds,
    AssignedMining,
    AssignedFarming,
    AssignedDigging,
    AssignedMilitary,
    AssignedResearch,
    KoboldEfficiency,
    TrainingLevel,
    MagicLevel,
    NecromancyLevel,
    AlchemyLevel,
    RestorationLevel,
    ElementalLevel,
    SummoningLevel,
    EnchantingLevel,
    ClickMultiplier,
}

/// Property types that may be associated with a tracked game value.
///
/// Each track can expose an optional current value, per-second rate, capacity,
/// and/or a modifier. This makes it easier to render a generic resource UI.
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
    /// Generate a town for a given index.
    /// Towns scale in difficulty and rewards based on their index.
    fn generate_town_at_index(idx: usize) -> Town {
        let level = (idx as u32) + 1;
        let base_difficulty = 2.0 + (level as f64) * 5.0;
        let reward = ((1.3_f64).powi(level as i32) * 0.5).floor();
        
        // Town names cycle through a pattern
        let templates = [
            "Harbor", "Keep", "Fortress", "Tower", "Palace",
            "Citadel", "Spire", "Haven", "Reach", "Hall",
        ];
        let prefixes = [
            "North", "South", "East", "West", "High", "Deep",
            "Shadow", "Star", "Mystic", "Ancient", "Lost", "Silent",
        ];
        let template = templates[idx % templates.len()];
        let prefix = prefixes[(idx / templates.len()) % prefixes.len()];
        let name = format!("{}{}", prefix, template);
        
        // Vary trades based on town index
        let trade_rotation = idx % 6;
        let (wants, wants_amount, offers, offers_amount) = match trade_rotation {
            0 => ("food".to_string(), 10.0 * (level as f64), "gold".to_string(), 50.0 * (level as f64)),
            1 => ("mana".to_string(), 5.0 * (level as f64), "gold".to_string(), 100.0 * (level as f64)),
            2 => ("gold".to_string(), 100.0 * (level as f64), "food".to_string(), 30.0 * (level as f64)),
            3 => ("gold".to_string(), 150.0 * (level as f64), "mana".to_string(), 10.0 * (level as f64)),
            4 => ("food".to_string(), 20.0 * (level as f64), "mana".to_string(), 10.0 * (level as f64)),
            _ => ("mana".to_string(), 8.0 * (level as f64), "food".to_string(), 40.0 * (level as f64)),
        };
        
        Town {
            name,
            level,
            conquered: false,
            reward_gold_per_sec: reward,
            difficulty: base_difficulty,
            wants,
            wants_amount,
            offers,
            offers_amount,
        }
    }

    /// Ensure that a town exists at the given index, generating it if needed.
    /// Conquered towns are replaced with newly generated towns.
    pub fn ensure_town_exists(&mut self, idx: usize) {
        while self.towns.len() <= idx {
            let town = Self::generate_town_at_index(self.towns.len());
            self.towns.push(town);
        }
        // Replace conquered towns with new generated towns
        for i in 0..self.towns.len() {
            if self.towns[i].conquered && i < idx {
                let new_town = Self::generate_town_at_index(i);
                self.towns[i] = new_town;
            }
        }
    }

    pub fn tick(&mut self, dt_seconds: f64) {
        // Effective kobold efficiency includes enchanting bonuses (small bonus to all output)
        let effective_eff = self.kobold_efficiency * (1.0 + 0.04 * (self.enchanting_level as f64));

        // Restoration reduces military upkeep requirements for conquered towns
        let required_per_town = (1.0 - 0.15 * (self.restoration_level as f64)).max(0.0);
        let military_per_town = if self.conquered_towns > 0 {
            (self.assigned_military as f64) / (self.conquered_towns as f64)
        } else {
            1.0
        };

        // If maintenance not met, disable conquered-town passive income
        let effective_gold_per_sec = if military_per_town < required_per_town {
            self.gold_per_sec - (self.towns.iter()
                .filter(|t| t.conquered)
                .map(|t| t.reward_gold_per_sec)
                .sum::<f64>())
        } else {
            self.gold_per_sec
        };
        
        // --- Necromancy: undead workers ---
        let undead_workers = if self.necromancy_level > 0 {
            let level = self.necromancy_level as f64;

            // base workers scale with level
            let base_workers = level * 2.0;

            // efficiency improves with level (more workers per mana)
            let efficiency = 1.0 + 0.25 * level;

            base_workers * efficiency
        } else {
            0.0
        };
        
        let mana_cost_per_worker = 0.2;

        let total_mana_cost = undead_workers * mana_cost_per_worker * dt_seconds;

        let active_undead = if self.mana >= total_mana_cost {
            self.subtract_track(GameTrack::Mana, total_mana_cost);
            undead_workers
        } else if self.mana > 0.0 {
            // partial operation if low mana
            let fraction = self.mana / total_mana_cost;
            self.subtract_track(GameTrack::Mana, self.mana);
            undead_workers * fraction
        } else {
            0.0
        };

        // Base passive gold and kobold-produced gold
        self.adjust_track(GameTrack::Gold, effective_gold_per_sec * dt_seconds);

        // Alchemy increases miner output (+12% per level)
        let alchemy_mult = 1.0 + 0.12 * (self.alchemy_level as f64);
        self.adjust_track(GameTrack::Gold, (self.assigned_mining as f64 + active_undead) * 0.6 * effective_eff * alchemy_mult * dt_seconds);

        // Military kobolds provide minor gold when assigned
        self.adjust_track(GameTrack::Gold, self.assigned_military as f64 * 0.4 * effective_eff * dt_seconds);

        // Food production and upkeep
        self.adjust_track(GameTrack::Food, (self.assigned_farming as f64 + active_undead * 0.5) * 0.35 * effective_eff* 0.35 * effective_eff * dt_seconds);
        let upkeep = self.kobold_upkeep() * dt_seconds;
        self.adjust_track(GameTrack::Food, -upkeep);
        if self.food > 99999.0 {
            self.food = 99999.0;
        }

        // Digging / space progress
        self.adjust_track(GameTrack::SpaceProgress, (self.assigned_digging as f64 + active_undead * 0.5) * 0.04 * effective_eff * dt_seconds);
        let excess = (self.space as f64 - self.space_soft_cap).max(0.0);
        let multiplier = (-0.01 * excess).exp();
        let space_to_add = (self.space_progress * multiplier).floor() as u32;
        self.add_track(GameTrack::Space, space_to_add as f64);
        self.adjust_track(GameTrack::SpaceProgress, -(space_to_add as f64));
        self.storage_slots += space_to_add;
        self.update_gold_capacity();



        // Summoning: more efficient helpers but require soldiers to maintain efficiency
        if self.summoning_level > 0 {
            let summon_count = self.summoning_level as f64;
            let mana_cost = 0.5 * summon_count * dt_seconds;
            // Efficiency scales with soldiers; bonus per soldier assigned
            let soldier_factor = 1.0 + 0.08 * (self.assigned_military as f64);
            let production = 0.9 * summon_count * soldier_factor * dt_seconds;
            let mana_available = self.mana;
            if mana_available >= mana_cost {
                self.subtract_track(GameTrack::Mana, mana_cost);
                self.adjust_track(GameTrack::Gold, production);
            } else if mana_available > 0.0 {
                let frac = mana_available / mana_cost;
                self.subtract_track(GameTrack::Mana, mana_available);
                self.adjust_track(GameTrack::Gold, production * frac);
            }
        }

        // mana regeneration
        if self.mana < self.mana_capacity {
            self.adjust_track(GameTrack::Mana, self.mana_regen_per_sec * dt_seconds);
        }
    }

    pub fn click_loot(&mut self) {
        let gold_gain = (10.0 * self.click_multiplier).round();
        self.adjust_track(GameTrack::Gold, gold_gain);
    }

    pub fn kobold_cost(&self) -> f64 {
        20.0 * 1.25_f64.powi(self.kobolds as i32)
    }

    pub fn recruit_kobold(&mut self) -> bool {
        let cost = self.kobold_cost();
        if self.gold >= cost && self.food >= 5.0 && self.kobolds < self.housing_slots {
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
            .saturating_sub(self.assigned_mining + self.assigned_farming + self.assigned_digging + self.assigned_military + self.assigned_research)
    }

    pub fn military_power(&self) -> f64 {
        let base_power = (self.assigned_military as f64) * 2.0;
        // Elemental specialization increases combat power
        let elemental_mult = 1.0 + 0.18 * (self.elemental_level as f64);
        base_power * elemental_mult
    }

    pub fn total_allocated_space(&self) -> u32 {
        self.housing_slots + self.storage_slots + self.furniture_slots
    }

    pub fn available_space(&self) -> u32 {
        self.space.saturating_sub(self.total_allocated_space())
    }

    pub fn update_gold_capacity(&mut self) {
        self.gold_capacity = (self.storage_slots as f64) * 200.0;
    }

    pub fn designate_storage_to_housing(&mut self) -> bool {
        if self.storage_slots > 0 {
            self.storage_slots -= 1;
            self.housing_slots += 1;
            self.update_gold_capacity();
            true
        } else {
            false
        }
    }

    pub fn reclaim_housing_to_storage(&mut self) -> bool {
        if self.housing_slots > 0 {
            self.housing_slots -= 1;
            self.storage_slots += 1;
            self.update_gold_capacity();
            true
        } else {
            false
        }
    }

    pub fn designate_storage_to_furniture(&mut self) -> bool {
        if self.storage_slots > 0 {
            self.storage_slots -= 1;
            self.furniture_slots += 1;
            self.update_gold_capacity();
            true
        } else {
            false
        }
    }

    pub fn reclaim_furniture_to_storage(&mut self) -> bool {
        if self.furniture_slots > 0 {
            self.furniture_slots -= 1;
            self.storage_slots += 1;
            self.update_gold_capacity();
            true
        } else {
            false
        }
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

    pub fn assign_military(&mut self) -> bool {
        if self.free_kobolds() > 0 {
            self.add_track(GameTrack::AssignedMilitary, 1.0);
            true
        } else {
            false
        }
    }

    pub fn unassign_military(&mut self) -> bool {
        if self.assigned_military > 0 {
            self.subtract_track(GameTrack::AssignedMilitary, 1.0);
            true
        } else {
            false
        }
    }

    pub fn assign_research(&mut self) -> bool {
        if self.free_kobolds() > 0 {
            self.add_track(GameTrack::AssignedResearch, 1.0);
            true
        } else {
            false
        }
    }

    pub fn unassign_research(&mut self) -> bool {
        if self.assigned_research > 0 {
            self.subtract_track(GameTrack::AssignedResearch, 1.0);
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

    pub fn track_stats(&self, track: GameTrack) -> GameTrackStats {
        match track {
            GameTrack::Gold => GameTrackStats::new(
                Some(self.gold),
                Some(self.gold_per_sec),
                Some(self.gold_capacity),
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
            GameTrack::Space => GameTrackStats::new(Some(self.space as f64), None, None, None),
            GameTrack::SpaceProgress => GameTrackStats::new(Some(self.space_progress), None, None, None),
            GameTrack::SpaceSoftCap => GameTrackStats::new(Some(self.space_soft_cap), None, None, None),
            GameTrack::Kobolds => GameTrackStats::new(
                Some(self.kobolds as f64),
                None,
                Some(self.housing_slots as f64),
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
            GameTrack::AssignedMilitary => GameTrackStats::new(
                Some(self.assigned_military as f64),
                None,
                Some(self.kobolds as f64),
                Some(self.kobold_efficiency),
            ),
            GameTrack::AssignedResearch => GameTrackStats::new(
                Some(self.assigned_research as f64),
                None,
                Some(self.kobolds as f64),
                Some(self.kobold_efficiency),
            ),
            GameTrack::KoboldEfficiency => GameTrackStats::new(Some(self.kobold_efficiency), None, None, None),
            GameTrack::TrainingLevel => GameTrackStats::new(Some(self.training_level as f64), None, None, None),
            GameTrack::MagicLevel => GameTrackStats::new(Some(self.magic_level as f64), None, None, None),
            GameTrack::NecromancyLevel => GameTrackStats::new(Some(self.necromancy_level as f64), None, None, None),
            GameTrack::AlchemyLevel => GameTrackStats::new(Some(self.alchemy_level as f64), None, None, None),
            GameTrack::RestorationLevel => GameTrackStats::new(Some(self.restoration_level as f64), None, None, None),
            GameTrack::ElementalLevel => GameTrackStats::new(Some(self.elemental_level as f64), None, None, None),
            GameTrack::SummoningLevel => GameTrackStats::new(Some(self.summoning_level as f64), None, None, None),
            GameTrack::EnchantingLevel => GameTrackStats::new(Some(self.enchanting_level as f64), None, None, None),
            GameTrack::ClickMultiplier => GameTrackStats::new(Some(self.click_multiplier), None, None, None),
        }
    }

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
    pub fn adjust_track(&mut self, track: GameTrack, delta: f64) {
        match track {
            GameTrack::Gold => {
                self.gold = (self.gold + delta).max(0.0);
                if self.gold > self.gold_capacity {
                    self.gold = self.gold_capacity;
                }
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
            GameTrack::Space => {
                if delta >= 0.0 {
                    self.space = self.space.saturating_add(delta.round() as u32);
                } else {
                    self.space = self.space.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::SpaceProgress => {
                self.space_progress = (self.space_progress + delta).max(0.0);
            }
            GameTrack::SpaceSoftCap => {
                self.space_soft_cap = (self.space_soft_cap + delta).max(0.0);
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
            GameTrack::AssignedMilitary => {
                if delta >= 0.0 {
                    self.assigned_military = self.assigned_military.saturating_add(delta.round() as u32);
                } else {
                    self.assigned_military = self.assigned_military.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::AssignedResearch => {
                if delta >= 0.0 {
                    self.assigned_research = self.assigned_research.saturating_add(delta.round() as u32);
                } else {
                    self.assigned_research = self.assigned_research.saturating_sub(delta.abs().round() as u32);
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
            GameTrack::NecromancyLevel => {
                if delta >= 0.0 {
                    self.necromancy_level = self.necromancy_level.saturating_add(delta.round() as u32);
                } else {
                    self.necromancy_level = self.necromancy_level.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::AlchemyLevel => {
                if delta >= 0.0 {
                    self.alchemy_level = self.alchemy_level.saturating_add(delta.round() as u32);
                } else {
                    self.alchemy_level = self.alchemy_level.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::RestorationLevel => {
                if delta >= 0.0 {
                    self.restoration_level = self.restoration_level.saturating_add(delta.round() as u32);
                } else {
                    self.restoration_level = self.restoration_level.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::ElementalLevel => {
                if delta >= 0.0 {
                    self.elemental_level = self.elemental_level.saturating_add(delta.round() as u32);
                } else {
                    self.elemental_level = self.elemental_level.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::SummoningLevel => {
                if delta >= 0.0 {
                    self.summoning_level = self.summoning_level.saturating_add(delta.round() as u32);
                } else {
                    self.summoning_level = self.summoning_level.saturating_sub(delta.abs().round() as u32);
                }
            }
            GameTrack::EnchantingLevel => {
                if delta >= 0.0 {
                    self.enchanting_level = self.enchanting_level.saturating_add(delta.round() as u32);
                } else {
                    self.enchanting_level = self.enchanting_level.saturating_sub(delta.abs().round() as u32);
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
            GameTrack::Space => self.space = value.max(0.0).round() as u32,
            GameTrack::SpaceProgress => self.space_progress = value.max(0.0),
            GameTrack::SpaceSoftCap => self.space_soft_cap = value.max(0.0),
            GameTrack::Kobolds => self.kobolds = value.max(0.0).round() as u32,
            GameTrack::AssignedMining => self.assigned_mining = value.max(0.0).round() as u32,
            GameTrack::AssignedFarming => self.assigned_farming = value.max(0.0).round() as u32,
            GameTrack::AssignedDigging => self.assigned_digging = value.max(0.0).round() as u32,
            GameTrack::AssignedMilitary => self.assigned_military = value.max(0.0).round() as u32,
            GameTrack::AssignedResearch => self.assigned_research = value.max(0.0).round() as u32,
            GameTrack::KoboldEfficiency => self.kobold_efficiency = value.max(0.0),
            GameTrack::TrainingLevel => self.training_level = value.max(0.0).round() as u32,
            GameTrack::MagicLevel => self.magic_level = value.max(0.0).round() as u32,
            GameTrack::NecromancyLevel => self.necromancy_level = value.max(0.0).round() as u32,
            GameTrack::AlchemyLevel => self.alchemy_level = value.max(0.0).round() as u32,
            GameTrack::RestorationLevel => self.restoration_level = value.max(0.0).round() as u32,
            GameTrack::ElementalLevel => self.elemental_level = value.max(0.0).round() as u32,
            GameTrack::SummoningLevel => self.summoning_level = value.max(0.0).round() as u32,
            GameTrack::EnchantingLevel => self.enchanting_level = value.max(0.0).round() as u32,
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

    pub fn specialization_cost(&self, level: u32) -> f64 {
        500.0 * 2.5_f64.powi(level as i32)
    }

    pub fn total_specialization_levels(&self) -> u32 {
        self.necromancy_level + self.alchemy_level + self.restoration_level 
            + self.elemental_level + self.summoning_level + self.enchanting_level
    }

    pub fn specialization_research_cost(&self) -> u32 {
        2 + self.total_specialization_levels()
    }

    pub fn learn_necromancy(&mut self) -> bool {
        if self.magic_level == 0 {
            return false;
        }
        let research_needed = self.specialization_research_cost();
        if self.assigned_research < research_needed {
            return false;
        }
        let cost = self.specialization_cost(self.necromancy_level);
        if self.track_value(GameTrack::Gold) >= cost {
            self.subtract_track(GameTrack::Gold, cost);
            self.add_track(GameTrack::NecromancyLevel, 1.0);
            true
        } else {
            false
        }
    }

    pub fn learn_alchemy(&mut self) -> bool {
        if self.magic_level == 0 {
            return false;
        }
        let research_needed = self.specialization_research_cost();
        if self.assigned_research < research_needed {
            return false;
        }
        let cost = self.specialization_cost(self.alchemy_level);
        if self.track_value(GameTrack::Gold) >= cost {
            self.subtract_track(GameTrack::Gold, cost);
            self.add_track(GameTrack::AlchemyLevel, 1.0);
            true
        } else {
            false
        }
    }

    pub fn learn_restoration(&mut self) -> bool {
        if self.magic_level == 0 {
            return false;
        }
        let research_needed = self.specialization_research_cost();
        if self.assigned_research < research_needed {
            return false;
        }
        let cost = self.specialization_cost(self.restoration_level);
        if self.track_value(GameTrack::Gold) >= cost {
            self.subtract_track(GameTrack::Gold, cost);
            self.add_track(GameTrack::RestorationLevel, 1.0);
            true
        } else {
            false
        }
    }

    pub fn learn_elemental(&mut self) -> bool {
        if self.magic_level == 0 {
            return false;
        }
        let research_needed = self.specialization_research_cost();
        if self.assigned_research < research_needed {
            return false;
        }
        let cost = self.specialization_cost(self.elemental_level);
        if self.track_value(GameTrack::Gold) >= cost {
            self.subtract_track(GameTrack::Gold, cost);
            self.add_track(GameTrack::ElementalLevel, 1.0);
            true
        } else {
            false
        }
    }

    pub fn learn_summoning(&mut self) -> bool {
        if self.magic_level == 0 {
            return false;
        }
        let research_needed = self.specialization_research_cost();
        if self.assigned_research < research_needed {
            return false;
        }
        let cost = self.specialization_cost(self.summoning_level);
        if self.track_value(GameTrack::Gold) >= cost {
            self.subtract_track(GameTrack::Gold, cost);
            self.add_track(GameTrack::SummoningLevel, 1.0);
            true
        } else {
            false
        }
    }

    pub fn learn_enchanting(&mut self) -> bool {
        if self.magic_level == 0 {
            return false;
        }
        let research_needed = self.specialization_research_cost();
        if self.assigned_research < research_needed {
            return false;
        }
        let cost = self.specialization_cost(self.enchanting_level);
        if self.track_value(GameTrack::Gold) >= cost {
            self.subtract_track(GameTrack::Gold, cost);
            self.add_track(GameTrack::EnchantingLevel, 1.0);
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

            let index = self.enchantments.len() % 11;
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
                4 => (
                    "Moonshard",
                    "Infuses the hoard with mysterious lunar magic".to_string(),
                ),
                5 => (
                    "Necrotic Rune",
                    "Channels death energy to amplify your defenses".to_string(),
                ),
                6 => (
                    "Alchemical Flask",
                    "Transmutes resources into more valuable forms".to_string(),
                ),
                7 => (
                    "Restoration Ward",
                    "Heals and regenerates your magical reserves".to_string(),
                ),
                8 => (
                    "Elemental Core",
                    "Harnesses raw elemental forces for greater power".to_string(),
                ),
                9 => (
                    "Summoning Circle",
                    "Calls forth allies to aid in your conquests".to_string(),
                ),
                _ => (
                    "Enchantment Matrix",
                    "Creates complex magical patterns for advanced effects".to_string(),
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
        if idx < self.towns.len() {
            120.0 * (self.towns[idx].level as f64)
        } else {
            // Calculate cost for towns that haven't been generated yet
            let level = (idx as u32) + 1;
            120.0 * (level as f64)
        }
    }

    pub fn try_conquer_town(&mut self, idx: usize) -> (bool, String) {
        self.ensure_town_exists(idx);
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
            + self.military_power()
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
            self.adjust_track(GameTrack::SpaceSoftCap, 1000.0);
            self.conquered_towns += 1;
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

    pub fn try_trade_town(&mut self, idx: usize) -> (bool, String) {
        self.ensure_town_exists(idx);
        if idx >= self.towns.len() {
            return (false, "Invalid town".to_string());
        }

        let town = &self.towns[idx];
        if town.conquered {
            return (false, format!("{} is conquered and cannot be traded with", town.name));
        }

        let town_name = town.name.clone();
        let wants_str = town.wants.clone();
        let offers_str = town.offers.clone();
        let wants_amount = town.wants_amount;
        let offers_amount = town.offers_amount;
        
        let wants_track = match wants_str.as_str() {
            "gold" => GameTrack::Gold,
            "food" => GameTrack::Food,
            "mana" => GameTrack::Mana,
            _ => return (false, "Unknown trade".to_string()),
        };
        let offers_track = match offers_str.as_str() {
            "gold" => GameTrack::Gold,
            "food" => GameTrack::Food,
            "mana" => GameTrack::Mana,
            _ => return (false, "Unknown trade".to_string()),
        };

        // Check if player has what the town wants
        if self.track_value(wants_track) < wants_amount {
            return (
                false,
                format!(
                    "Not enough {}. Need {:.0}",
                    wants_str, wants_amount
                ),
            );
        }

        // Execute trade
        self.adjust_track(wants_track, -wants_amount);
        self.adjust_track(offers_track, offers_amount);

        (
            true,
            format!(
                "Traded with {}! Gave {:.0} {}, received {:.0} {}",
                town_name, wants_amount, wants_str, offers_amount, offers_str
            ),
        )
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
            + self.military_power()
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
