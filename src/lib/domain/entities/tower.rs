use ratatui::style::Color;
use uuid::Uuid;

use crate::domain::mediator::MediatorService;

use super::game::Game;
use super::{
    behavior::TowerBehavior, element::Element, monster::Monster, position::Position, wave::Wave,
};
use std::f32;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

/// Stratégie de sélection de cible pour les tourelles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetSelection {
    /// Cible les monstres volants
    Flying,
    /// Cible le monstre le plus proche
    Nearest,
    /// Cible le monstre le plus éloigné
    Farthest,
    /// Cible le monstre avec le plus de points de vie
    Strongest,
    /// Cible le monstre avec le moins de points de vie
    Weakest,
    /// Cible tous les monstres dans la portée (AOE)
    All,
}

impl Default for TargetSelection {
    fn default() -> Self {
        Self::Nearest
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TowerStatType {
    Range,
    Damage,
    AttackSpeed,
    Money,
}

#[derive(Clone)]
pub struct TowerStatUpgrade {
    pub price_multiplier: f32,
    pub value_multiplier: f32,
    pub value_multiplier_unit: TowerUpgradeElementUnit,
    pub max_level: u32,
}

impl TowerStatUpgrade {
    pub fn format(&self, stat: &TowerStats) -> Result<String, ()> {
        if self.max_level == stat.level {
            return Ok(format!("{} {:.2} {}", stat.icon, stat.base, stat.label));
        }

        let unit = match &self.value_multiplier_unit {
            TowerUpgradeElementUnit::Percent => "%",
            TowerUpgradeElementUnit::Unit => "",
        };

        let symbol = match self.value_multiplier_unit {
            TowerUpgradeElementUnit::Percent => "x",
            TowerUpgradeElementUnit::Unit => "+",
        };

        Ok(format!(
            "{} {:.2} {} ({}{:.2}{})",
            stat.icon, stat.base, stat.label, symbol, self.value_multiplier, unit
        ))
    }
}

#[derive(Clone)]
pub struct TowerStats {
    pub stat_type: TowerStatType,
    pub label: String,
    pub icon: String,
    pub base: f32,
    pub level: u32,
    pub upgrade: Option<TowerStatUpgrade>,
}

impl TowerStats {
    pub fn get_next_price(&self) -> Option<u32> {
        if let Some(upgrade) = &self.upgrade {
            let base = (self.base * 1.3_f32.powi(self.level as i32)).round() as u32;
            Some((base as f32 * upgrade.price_multiplier).round() as u32)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TowerUpgradeElementUnit {
    Percent,
    Unit,
}

#[derive(Debug, Clone)]
pub struct TowerUpgradeElement {
    pub price_multiplier: f32,
    pub value_multiplier: f32,
    pub value_multiplier_unit: TowerUpgradeElementUnit,
    pub level: u32,
}

impl TowerUpgradeElement {
    pub fn new(
        price_multiplier: f32,
        value_multiplier: f32,
        value_multiplier_unit: TowerUpgradeElementUnit,
    ) -> Self {
        Self {
            price_multiplier,
            value_multiplier,
            value_multiplier_unit,
            level: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TowerUpgrades {
    pub base_cost: u32,
    pub range: Option<TowerUpgradeElement>,
    pub damage: Option<TowerUpgradeElement>,
    pub attacks_speed: Option<TowerUpgradeElement>,
}

impl TowerUpgrades {
    pub fn new(
        base_cost: u32,
        range: Option<TowerUpgradeElement>,
        damage: Option<TowerUpgradeElement>,
        attacks_speed: Option<TowerUpgradeElement>,
    ) -> Self {
        Self {
            base_cost,
            range,
            damage,
            attacks_speed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TowerStatElement {
    pub base: f32,
    pub level: u32,
}

impl TowerStatElement {
    pub fn new(base: f32, level: u32) -> Self {
        Self { base, level }
    }
}

#[derive(Debug, Clone)]
pub struct TowerStatDamageElement {
    pub base: f32,
    pub level: u32,
    pub element: Element,
}

impl TowerStatDamageElement {
    pub fn new(base: f32, level: u32, element: Element) -> Self {
        Self {
            base,
            level,
            element,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TowerAoe {
    Radius(u32, f32),
    Count(u32, f32),
}

#[derive(Clone, PartialEq)]
pub enum TowerMeta {
    AoeEffect(TowerAoe),
    Behavior(TowerBehavior),
    TargetSelection(TargetSelection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerKind {
    Basic,
    Fire,
    Water,
    Earth,
    Air,
    Lightning,
    Ice,
    Poison,
    Sentinel,
}

/// Structure uniforme pour toutes les tourelles
#[derive(Clone)]
pub struct Tower {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub level: u32,
    pub cost: u32,
    pub stats: Vec<TowerStats>,
    pub meta: Option<Vec<TowerMeta>>,
    pub position: Position,
    pub last_attack: f32,
    pub on_action:
        Option<Rc<dyn Fn(Arc<MediatorService>, &mut Game, &mut Tower) -> Result<(), String>>>,
    pub highlight: Option<Color>,
}

impl Tower {
    pub fn new(
        name: String,
        symbol: String,
        level: u32,
        cost: u32,
        position: Position,
        stats: Vec<TowerStats>,
        meta: Option<Vec<TowerMeta>>,
        on_action: Option<
            Rc<dyn Fn(Arc<MediatorService>, &mut Game, &mut Tower) -> Result<(), String>>,
        >,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            symbol,
            level,
            cost,
            stats,
            meta,
            position,
            last_attack: 0.0,
            on_action,
            highlight: None,
        }
    }

    pub fn can_shoot(&self, current_time: f32) -> bool {
        let attack_speed = self
            .stats
            .iter()
            .find(|stat| stat.stat_type == TowerStatType::AttackSpeed);

        if let Some(attack_speed) = attack_speed {
            let time_since_last_attack = current_time - self.last_attack;
            time_since_last_attack >= 1.0 / attack_speed.base
        } else {
            false
        }
    }

    pub fn upgrade_level(&mut self) -> u32 {
        self.level += 1;
        self.level
    }

    pub fn upgrade(&mut self, upgrade_type: TowerStatType) -> Result<(), String> {
        let element = self
            .stats
            .iter_mut()
            .find(|stat| stat.stat_type == upgrade_type);

        if let Some(element) = element {
            if let Some(upgrade) = &element.upgrade {
                if element.level >= upgrade.max_level {
                    return Err(format!("La vitesse d'attaque est déjà au niveau maximum."));
                }

                element.level += 1;
                element.base = match upgrade.value_multiplier_unit {
                    TowerUpgradeElementUnit::Percent => element.base * upgrade.value_multiplier,
                    TowerUpgradeElementUnit::Unit => element.base + upgrade.value_multiplier,
                };
            }

            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn upgrade_cost_for_attribute(&self, stat: TowerStatType) -> Option<u32> {
        let stat = self.stats.iter().find(|s| s.stat_type == stat);
        if let Some(stat) = stat {
            if stat.level >= stat.upgrade.as_ref().unwrap().max_level {
                return None;
            }

            stat.get_next_price()
        } else {
            None
        }
    }

    pub fn shoot(
        &mut self,
        mediator: Arc<MediatorService>,
        game: &mut Game,
        current_time: f32,
    ) -> Vec<String> {
        let mut primary_targets = Vec::new();
        let logs = Vec::new();

        // Mettre à jour le temps du dernier tir
        self.last_attack = current_time;

        if let Some(on_action) = self.on_action.take() {
            on_action(mediator, game, self).unwrap();
            self.on_action = Some(on_action);
        }

        // Sélectionner les cibles primaires en fonction de la stratégie
        let current_wave = game.current_wave.as_mut().unwrap();
        if let Some(meta) = &self.meta {
            if let Some(_) = meta
                .iter()
                .find(|metadata| **metadata == TowerMeta::TargetSelection(TargetSelection::Nearest))
            {
                if let Some(target) = self.find_nearest_target(current_wave) {
                    primary_targets.push(target);
                }
            }

            if let Some(_) = meta
                .iter()
                .find(|metadata| **metadata == TowerMeta::TargetSelection(TargetSelection::All))
            {
                for (idx, monster) in current_wave.monsters.iter().enumerate() {
                    if monster.active && self.is_in_range(monster) {
                        primary_targets.push(idx);
                    }
                }
            }

            if let Some(_) = meta.iter().find(|metadata| {
                **metadata == TowerMeta::TargetSelection(TargetSelection::Strongest)
            }) {
                let strongest_monster = current_wave
                    .monsters
                    .iter()
                    .max_by_key(|m| m.hp as i32)
                    .map(|m| m.id);

                if let Some(strongest_monster) = strongest_monster {
                    let index = current_wave
                        .monsters
                        .iter()
                        .position(|m| m.id == strongest_monster)
                        .unwrap();

                    primary_targets.push(index);
                }
            }

            let damage = self
                .stats
                .iter()
                .find(|stat| stat.stat_type == TowerStatType::Damage);

            let behavior = meta.iter().find_map(|metadata| {
                if let TowerMeta::Behavior(behavior) = metadata {
                    Some(behavior)
                } else {
                    None
                }
            });

            if let Some(damage) = damage {
                for target_idx in primary_targets {
                    if let Some(monster) = current_wave.monsters.get_mut(target_idx) {
                        if !monster.active {
                            continue;
                        }

                        if let Some(behavior) = behavior {
                            let actual_damage = behavior.apply(monster, damage.base);
                            monster.hp -= actual_damage;
                        }
                    };

                    let aoe = meta.iter().find_map(|metadata| {
                        if let TowerMeta::AoeEffect(aoe) = metadata {
                            Some(aoe)
                        } else {
                            None
                        }
                    });

                    if let Some(aoe) = aoe {
                        let target_pos = current_wave.monsters[target_idx].position;
                        for (idx, monster) in current_wave.monsters.iter_mut().enumerate() {
                            if idx == target_idx || !monster.active {
                                continue;
                            }

                            let distance = target_pos.distance_to(&monster.position);
                            if let TowerAoe::Radius(radius, damage_multiplier) = aoe {
                                if distance <= *radius as f32 {
                                    let aoe_damage = damage.base * damage_multiplier; // 50% des dégâts pour l'AOE
                                    monster.hp -= behavior
                                        .expect("You should define a behavior when you use AOE")
                                        .apply(monster, aoe_damage);
                                }
                            }
                        }
                    }
                }
            }
        }

        logs
    }

    fn find_nearest_target(&self, wave: &Wave) -> Option<usize> {
        let range = self
            .stats
            .iter()
            .find(|stat| stat.stat_type == TowerStatType::Range);

        if let Some(range) = range {
            let tower_pos = self.position;
            let mut nearest_idx = None;
            let mut min_distance = f32::MAX;

            for (idx, monster) in wave.monsters.iter().enumerate() {
                if !monster.active {
                    continue;
                }

                let dx = (monster.position.x - tower_pos.x) as f32;
                let dy = (monster.position.y - tower_pos.y) as f32;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= range.base && distance < min_distance {
                    min_distance = distance;
                    nearest_idx = Some(idx);
                }
            }

            nearest_idx
        } else {
            None
        }
    }

    fn is_in_range(&self, monster: &Monster) -> bool {
        let range = self
            .stats
            .iter()
            .find(|stat| stat.stat_type == TowerStatType::Range);

        if let Some(range) = range {
            let tower_pos = self.position;
            let dx = (monster.position.x - tower_pos.x) as f32;
            let dy = (monster.position.y - tower_pos.y) as f32;
            let distance = (dx * dx + dy * dy).sqrt();

            distance <= range.base
        } else {
            false
        }
    }
}
