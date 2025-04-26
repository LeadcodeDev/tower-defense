use crate::application::engine::towers::{BasicTower, FireBasicTower};

use super::{
    behavior::TowerBehavior, element::Element, monster::Monster, position::Position, wave::Wave,
};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct TowerBase {
    pub position: Position,
    pub range: f32,
    pub element: Element,
    pub damage: f32,
    pub attacks_per_second: f32,
    pub aoe: bool,
    pub behavior: TowerBehavior,
}

pub trait Tower: Debug + Clone {
    fn position(&self) -> &Position;
    fn range(&self) -> f32;
    fn get_element(&self) -> Element;
    fn damage(&self) -> f32;
    fn attacks_per_second(&self) -> f32;
    fn is_aoe(&self) -> bool;
    fn behavior(&self) -> &TowerBehavior;
    fn calculate_damage(&self, monster: &Monster) -> f32 {
        let damage_factor = monster.resistances.damage_factor(&self.get_element());
        self.damage() * damage_factor
    }

    // Added methods to define shooting behavior
    fn can_shoot(&self, current_time: f32) -> bool {
        let attack_period = 1.0 / self.attacks_per_second();
        current_time % attack_period < 0.01 // Shoot at regular intervals
    }

    fn shoot(&mut self, wave: &mut Wave, current_time: f32) {
        let targets = if self.is_aoe() {
            // For AOE attacks, target all monsters within range
            wave.monsters
                .iter_mut()
                .filter(|m| m.is_alive() && self.is_in_range(m))
                .collect::<Vec<_>>()
        } else {
            // For single-target attacks, pick the first monster within range
            if let Some(target) = wave
                .monsters
                .iter_mut()
                .find(|m| m.is_alive() && self.is_in_range(m))
            {
                vec![target]
            } else {
                vec![]
            }
        };

        for target in targets {
            let damage = self.calculate_damage(target);
            let applied_damage = self.behavior().apply(target, damage);
            target.hp -= applied_damage;
            println!(
                "🔥 Tour {} ({}) → Monstre: -{:.1} hp | {} hp restants",
                self.get_element().get_name(),
                self.behavior().name(),
                applied_damage,
                target.hp
            );
        }
    }

    fn is_in_range(&self, monster: &Monster) -> bool {
        let dx = (self.position().x - monster.position.x) as f32;
        let dy = (self.position().y - monster.position.y) as f32;
        let distance = (dx * dx + dy * dy).sqrt();
        distance <= self.range()
    }
}

// Enum to hold all tower types
#[derive(Debug, Clone)]
pub enum TowerType {
    Basic(BasicTower),
    Fire(FireBasicTower),
}

// Implement the Tower trait for the enum
impl Tower for TowerType {
    fn position(&self) -> &Position {
        match self {
            TowerType::Basic(base) => &base.position(),
            TowerType::Fire(base) => &base.position(),
        }
    }

    fn range(&self) -> f32 {
        match self {
            TowerType::Basic(base) => base.range(),
            TowerType::Fire(base) => base.range(),
        }
    }

    fn get_element(&self) -> Element {
        match self {
            TowerType::Basic(base) => base.get_element(),
            TowerType::Fire(base) => base.get_element(),
        }
    }

    fn damage(&self) -> f32 {
        match self {
            TowerType::Basic(base) => base.damage(),
            TowerType::Fire(base) => base.damage(),
        }
    }

    fn attacks_per_second(&self) -> f32 {
        match self {
            TowerType::Basic(base) => base.attacks_per_second(),
            TowerType::Fire(base) => base.attacks_per_second(),
        }
    }

    fn is_aoe(&self) -> bool {
        match self {
            TowerType::Basic(base) => base.is_aoe(),
            TowerType::Fire(base) => base.is_aoe(),
        }
    }

    fn behavior(&self) -> &TowerBehavior {
        match self {
            TowerType::Basic(base) => base.behavior(),
            TowerType::Fire(base) => base.behavior(),
        }
    }
}
