use crate::application::engine::towers::{BasicTower, FireBasicTower};

use super::{
    behavior::TowerBehavior, element::Element, monster::Monster, position::Position, wave::Wave,
};
use std::f32;
use std::fmt::Debug;

/// Stratégie de sélection de cible pour les tourelles
#[derive(Debug, Clone, Copy)]
pub enum TargetSelection {
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

/// Structure de base pour toutes les tourelles
#[derive(Debug, Clone)]
pub struct TowerBase {
    /// Position sur la carte
    pub position: Position,
    /// Portée de la tourelle
    pub range: f32,
    /// Élément de la tourelle
    pub element: Element,
    /// Dégâts infligés
    pub damage: f32,
    /// Attaques par seconde
    pub attacks_per_second: f32,
    /// Dernier moment d'attaque
    pub last_attack: f32,
    /// Est-ce que la tourelle attaque en zone (AOE)
    pub aoe: bool,
    /// Comportement de la tourelle
    pub behavior: TowerBehavior,
    /// Stratégie de sélection de cible
    pub target_selection: TargetSelection,
}

/// Types de tourelles disponibles
#[derive(Debug)]
pub enum TowerType {
    /// Tourelle de base
    Basic(crate::application::engine::towers::basic_tower::BasicTower),
    /// Tourelle de feu
    Fire(crate::application::engine::towers::fire_tower::FireBasicTower),
    /// Tourelle d'eau
    Water(crate::application::engine::towers::water_tower::WaterBasicTower),
    /// Tourelle de terre
    Earth(crate::application::engine::towers::earth_tower::EarthBasicTower),
    /// Tourelle d'air
    Air(crate::application::engine::towers::air_tower::AirBasicTower),
}

impl TowerType {
    pub fn position(&self) -> Position {
        match self {
            TowerType::Basic(t) => t.position(),
            TowerType::Fire(t) => t.position(),
            TowerType::Water(t) => t.position(),
            TowerType::Earth(t) => t.position(),
            TowerType::Air(t) => t.position(),
        }
    }

    pub fn range(&self) -> f32 {
        match self {
            TowerType::Basic(t) => t.range(),
            TowerType::Fire(t) => t.range(),
            TowerType::Water(t) => t.range(),
            TowerType::Earth(t) => t.range(),
            TowerType::Air(t) => t.range(),
        }
    }

    pub fn can_shoot(&self, current_time: f32) -> bool {
        match self {
            TowerType::Basic(t) => t.can_shoot(current_time),
            TowerType::Fire(t) => t.can_shoot(current_time),
            TowerType::Water(t) => t.can_shoot(current_time),
            TowerType::Earth(t) => t.can_shoot(current_time),
            TowerType::Air(t) => t.can_shoot(current_time),
        }
    }

    pub fn shoot(&mut self, wave: &mut Wave, current_time: f32) -> Vec<String> {
        match self {
            TowerType::Basic(t) => t.shoot(wave, current_time),
            TowerType::Fire(t) => t.shoot(wave, current_time),
            TowerType::Water(t) => t.shoot(wave, current_time),
            TowerType::Earth(t) => t.shoot(wave, current_time),
            TowerType::Air(t) => t.shoot(wave, current_time),
        }
    }

    pub fn set_last_attack_time(&mut self, time: f32) {
        match self {
            TowerType::Basic(t) => t.set_last_attack_time(time),
            TowerType::Fire(t) => t.set_last_attack_time(time),
            TowerType::Water(t) => t.set_last_attack_time(time),
            TowerType::Earth(t) => t.set_last_attack_time(time),
            TowerType::Air(t) => t.set_last_attack_time(time),
        }
    }

    /// Retourne le niveau d'amélioration de la tour
    pub fn upgrade_level(&self) -> u32 {
        match self {
            TowerType::Basic(t) => t.upgrade_level(),
            TowerType::Fire(t) => t.upgrade_level(),
            TowerType::Water(t) => t.upgrade_level(),
            TowerType::Earth(t) => t.upgrade_level(),
            TowerType::Air(t) => t.upgrade_level(),
        }
    }

    /// Améliore la vitesse d'attaque de la tour
    pub fn upgrade_attack_speed(&mut self) -> bool {
        match self {
            TowerType::Basic(t) => t.upgrade_attack_speed(),
            TowerType::Fire(t) => t.upgrade_attack_speed(),
            TowerType::Water(t) => t.upgrade_attack_speed(),
            TowerType::Earth(t) => t.upgrade_attack_speed(),
            TowerType::Air(t) => t.upgrade_attack_speed(),
        }
    }

    /// Améliore les dégâts de la tour
    pub fn upgrade_damage(&mut self) -> bool {
        match self {
            TowerType::Basic(t) => t.upgrade_damage(),
            TowerType::Fire(t) => t.upgrade_damage(),
            TowerType::Water(t) => t.upgrade_damage(),
            TowerType::Earth(t) => t.upgrade_damage(),
            TowerType::Air(t) => t.upgrade_damage(),
        }
    }

    /// Améliore la portée de la tour
    pub fn upgrade_range(&mut self) -> bool {
        match self {
            TowerType::Basic(t) => t.upgrade_range(),
            TowerType::Fire(t) => t.upgrade_range(),
            TowerType::Water(t) => t.upgrade_range(),
            TowerType::Earth(t) => t.upgrade_range(),
            TowerType::Air(t) => t.upgrade_range(),
        }
    }

    /// Retourne le coût d'amélioration en fonction du niveau actuel
    pub fn upgrade_cost(&self) -> u32 {
        let base_cost = match self {
            TowerType::Basic(_) => 25,
            TowerType::Fire(_) => 40,
            TowerType::Water(_) => 40,
            TowerType::Earth(_) => 50,
            TowerType::Air(_) => 50,
        };

        // Algorithme de croissance exponentielle avec base linéaire
        let level = self.upgrade_level();
        let exponential_factor = 1.5_f32.powi(level as i32);
        let linear_component = 20 * level;

        (base_cost as f32 * exponential_factor + linear_component as f32).round() as u32
    }

    /// Retourne le type de la tour sous forme de chaîne
    pub fn tower_type_name(&self) -> &str {
        match self {
            TowerType::Basic(_) => "Basique",
            TowerType::Fire(_) => "Feu",
            TowerType::Water(_) => "Eau",
            TowerType::Earth(_) => "Terre",
            TowerType::Air(_) => "Air",
        }
    }
}

/// Interface pour toutes les tourelles
pub trait Tower {
    fn position(&self) -> Position;
    fn range(&self) -> f32;
    fn attack_damage(&self) -> f32;
    fn attack_speed(&self) -> f32;
    fn last_attack_time(&self) -> f32;
    fn set_last_attack_time(&mut self, time: f32);
    fn target_selection(&self) -> TargetSelection;

    // Méthodes additionnelles nécessaires pour l'implementation existante
    fn get_element(&self) -> Element;
    fn damage(&self) -> f32;
    fn attacks_per_second(&self) -> f32;
    fn is_aoe(&self) -> bool;
    fn behavior(&self) -> &TowerBehavior;

    fn can_shoot(&self, current_time: f32) -> bool {
        let time_since_last_attack = current_time - self.last_attack_time();
        time_since_last_attack >= 1.0 / self.attack_speed()
    }

    fn shoot(&mut self, wave: &mut Wave, current_time: f32) -> Vec<String> {
        let mut primary_targets = Vec::new();
        let mut logs = Vec::new();

        // Mettre à jour le temps du dernier tir
        self.set_last_attack_time(current_time);

        // Sélectionner les cibles primaires en fonction de la stratégie
        match self.target_selection() {
            TargetSelection::Nearest => {
                if let Some(target) = self.find_nearest_target(wave) {
                    primary_targets.push(target);
                }
            }
            TargetSelection::All => {
                // Cibler tous les monstres dans la portée
                for (idx, monster) in wave.monsters.iter().enumerate() {
                    if monster.active && self.is_in_range(monster) {
                        primary_targets.push(idx);
                    }
                }
            }
            _ => {
                // Par défaut, prendre le premier monstre dans la portée
                for (idx, monster) in wave.monsters.iter().enumerate() {
                    if monster.active && self.is_in_range(monster) {
                        primary_targets.push(idx);
                        break;
                    }
                }
            }
        }

        // Traiter chaque cible primaire et les cibles AOE si applicable
        for target_idx in primary_targets {
            if let Some(target_monster) = wave.monsters.get(target_idx) {
                if !target_monster.active {
                    continue;
                }

                // Appliquer les dégâts à la cible primaire d'abord
                if let Some(monster) = wave.monsters.get_mut(target_idx) {
                    let damage = self.attack_damage();
                    monster.hp -= damage;

                    let log_message = format!(
                        "🏹 Tourelle {:?} attaque! -{:.1} HP sur {}. HP restants: {:.1}",
                        self.get_element(),
                        damage,
                        monster.name,
                        monster.hp
                    );
                    logs.push(log_message);
                }

                // Si la tourelle fait des AOE, appliquer des dégâts aux monstres proches de la cible
                if self.is_aoe() {
                    let target_pos = wave.monsters[target_idx].position;
                    let aoe_radius = 1.0; // Rayon de 1 case autour de la cible

                    // Rechercher les monstres dans le rayon de l'AOE
                    for (idx, monster) in wave.monsters.iter_mut().enumerate() {
                        // Ne pas réappliquer les dégâts à la cible principale
                        if idx == target_idx || !monster.active {
                            continue;
                        }

                        // Calculer la distance entre le monstre et la cible principale
                        let distance = target_pos.distance_to(&monster.position);

                        // Si le monstre est dans le rayon de l'AOE
                        if distance <= aoe_radius {
                            let aoe_damage = self.attack_damage() * 0.5; // 50% des dégâts pour l'AOE
                            monster.hp -= aoe_damage;

                            let log_message = format!(
                                "🔥 Effet AOE! -{:.1} HP sur {}. HP restants: {:.1}",
                                aoe_damage, monster.name, monster.hp
                            );
                            logs.push(log_message);
                        }
                    }
                }
            }
        }

        logs
    }

    fn find_nearest_target(&self, wave: &Wave) -> Option<usize> {
        let tower_pos = self.position();
        let mut nearest_idx = None;
        let mut min_distance = f32::MAX;

        for (idx, monster) in wave.monsters.iter().enumerate() {
            if !monster.active {
                continue;
            }

            let dx = (monster.position.x - tower_pos.x) as f32;
            let dy = (monster.position.y - tower_pos.y) as f32;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= self.range() && distance < min_distance {
                min_distance = distance;
                nearest_idx = Some(idx);
            }
        }

        nearest_idx
    }

    fn is_in_range(&self, monster: &Monster) -> bool {
        let tower_pos = self.position();
        let dx = (monster.position.x - tower_pos.x) as f32;
        let dy = (monster.position.y - tower_pos.y) as f32;
        let distance = (dx * dx + dy * dy).sqrt();

        distance <= self.range()
    }

    // Nouvelles méthodes pour les améliorations
    fn upgrade_level(&self) -> u32;
    fn upgrade_attack_speed(&mut self) -> bool;
    fn upgrade_damage(&mut self) -> bool;
    fn upgrade_range(&mut self) -> bool;
}
