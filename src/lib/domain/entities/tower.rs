use super::{
    behavior::TowerBehavior, element::Element, monster::Monster, position::Position, wave::Wave,
};
use std::f32;
use std::fmt::Debug;

/// Stratégie de sélection de cible pour les tourelles
#[derive(Debug, Clone, Copy)]
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

/// Structure de base pour toutes les tourelles
#[derive(Debug, Clone)]
pub struct TowerStats {
    /// Portée de la tourelle
    pub range: TowerStatElement,
    /// Élément de la tourelle
    pub damage: TowerStatDamageElement,
    /// Attaques par seconde
    pub attacks_per_second: TowerStatElement,
}

#[derive(Debug, Clone)]
pub struct BaseStats {
    pub range: f32,
    pub damage: f32,
    pub attacks_per_second: f32,
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

#[derive(Debug, Clone)]
pub enum TowerAoe {
    Radius(u32, f32),
    Count(u32, f32),
}

#[derive(Debug, Clone)]
pub struct TowerMeta {
    pub behavior: TowerBehavior,
    pub target_selection: TargetSelection,
    pub tower_type: TowerKind,
    pub aoe: Option<TowerAoe>,
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
pub struct Tower {
    pub name: String,
    pub level: u32,
    pub stats: TowerStats,
    pub upgrades: TowerUpgrades,
    pub meta: TowerMeta,
    pub position: Position,
    pub last_attack: f32,
    pub on_action: Option<Box<dyn Fn(&mut Wave, &mut Tower) -> Result<(), String>>>,
}

impl Tower {
    pub fn can_shoot(&self, current_time: f32) -> bool {
        let time_since_last_attack = current_time - self.last_attack;
        time_since_last_attack >= 1.0 / self.stats.attacks_per_second.base
    }

    pub fn upgrade_level(&mut self) -> u32 {
        self.level += 1;
        self.level
    }

    pub fn upgrade_attack_speed(&mut self) -> Result<String, String> {
        // Niveau maximum selon le type de tour
        let max_level = if self.meta.tower_type == TowerKind::Earth {
            10
        } else {
            30
        };

        if let Some(upgrades) = &mut self.upgrades.attacks_speed {
            if upgrades.level >= max_level {
                return Err(format!("La vitesse d'attaque est déjà au niveau maximum."));
            }
        }

        let current = self.stats.attacks_per_second.base;

        if let Some(upgrades) = &mut self.upgrades.attacks_speed {
            upgrades.level += 1;
            self.stats.attacks_per_second.base = match upgrades.value_multiplier_unit {
                TowerUpgradeElementUnit::Percent => current * upgrades.value_multiplier,
                TowerUpgradeElementUnit::Unit => current + upgrades.value_multiplier,
            };

            Ok(format!(
                "🔧 Tour {} vitesse d'attaque améliorée ({} -> {})",
                self.tower_type_name(),
                current,
                self.stats.attacks_per_second.base
            ))
        } else {
            Err(format!("La vitesse d'attaque ne peut pas être améliorée."))
        }
    }

    pub fn upgrade_damage(&mut self) -> Result<String, String> {
        // Niveau maximum selon le type de tour
        let max_level = if self.meta.tower_type == TowerKind::Earth {
            10
        } else {
            30
        };

        if self.upgrades.damage.as_ref().unwrap().level >= max_level {
            return Err(format!("Les dégâts sont déjà au niveau maximum."));
        }

        let current = self.stats.damage.base;

        if let Some(upgrades) = &mut self.upgrades.damage {
            upgrades.level += 1;
            self.stats.damage.base = match upgrades.value_multiplier_unit {
                TowerUpgradeElementUnit::Percent => current * upgrades.value_multiplier,
                TowerUpgradeElementUnit::Unit => current + upgrades.value_multiplier,
            };

            Ok(format!(
                "🔧 Tour {} dégâts améliorés ({} -> {})",
                self.tower_type_name(),
                current,
                self.stats.damage.base
            ))
        } else {
            Err(format!("Les dégâts ne peuvent pas être améliorés."))
        }
    }

    pub fn upgrade_range(&mut self) -> Result<String, String> {
        // Niveau maximum selon le type de tour
        let max_level = if self.meta.tower_type == TowerKind::Earth {
            10
        } else {
            30
        };

        if let Some(upgrades) = &mut self.upgrades.range {
            if upgrades.level >= max_level {
                return Err(format!("La portée est déjà au niveau maximum."));
            }
        }

        let current = self.stats.range.base;

        if let Some(upgrades) = &mut self.upgrades.range {
            upgrades.level += 1;
            self.stats.range.base = match upgrades.value_multiplier_unit {
                TowerUpgradeElementUnit::Percent => current * upgrades.value_multiplier,
                TowerUpgradeElementUnit::Unit => current + upgrades.value_multiplier,
            };

            Ok(format!(
                "🔧 Tour {} portée améliorée ({} -> {})",
                self.tower_type_name(),
                current,
                self.stats.range.base
            ))
        } else {
            Err(format!("La portée ne peut pas être améliorée."))
        }
    }

    /// Retourne le coût d'amélioration en fonction du niveau actuel
    pub fn upgrade_cost(&self, level: u32) -> u32 {
        let exponential_factor = 1.5_f32.powi(level as i32);
        let linear_component = 20 * level;

        (self.upgrades.base_cost as f32 * exponential_factor + linear_component as f32).round()
            as u32
    }

    pub fn upgrade_cost_for_attribute(&self, upgrade_type: UpgradeType) -> u32 {
        let level = match upgrade_type {
            UpgradeType::Damage => self.upgrades.damage.as_ref().unwrap().level,
            UpgradeType::AttackSpeed => self.upgrades.attacks_speed.as_ref().unwrap().level,
            UpgradeType::Range => self.upgrades.range.as_ref().unwrap().level,
        };

        // Vérifier si on a atteint le niveau maximum
        let max_level = if self.meta.tower_type == TowerKind::Earth {
            // Niveau maximum spécifique pour la tour de terre
            10
        } else {
            // Niveau maximum par défaut pour les autres tours
            30
        };

        if level >= max_level {
            return 0; // Coût de 0 indique que l'amélioration est au maximum
        }

        let base = (self.upgrades.base_cost as f32 * 1.3_f32.powi(level as i32)).round() as u32;

        let synergy_factor = match upgrade_type {
            UpgradeType::Damage => self.upgrades.damage.as_ref().unwrap().price_multiplier,
            UpgradeType::AttackSpeed => {
                self.upgrades
                    .attacks_speed
                    .as_ref()
                    .unwrap()
                    .price_multiplier
            }
            UpgradeType::Range => self.upgrades.range.as_ref().unwrap().price_multiplier,
        };

        (base as f32 * synergy_factor).round() as u32
    }

    /// Retourne le type de la tour sous forme de chaîne
    pub fn tower_type_name(&self) -> &str {
        match self.meta.tower_type {
            TowerKind::Basic => "Basique",
            TowerKind::Fire => "Feu",
            TowerKind::Water => "Eau",
            TowerKind::Earth => "Terre",
            TowerKind::Air => "Air",
            TowerKind::Lightning => "Éclair",
            TowerKind::Ice => "Glace",
            TowerKind::Poison => "Poison",
            TowerKind::Sentinel => "Sentinelle",
        }
    }

    pub fn shoot(&mut self, wave: &mut Wave, current_time: f32) -> Vec<String> {
        let mut primary_targets = Vec::new();
        let mut logs = Vec::new();

        // Mettre à jour le temps du dernier tir
        self.last_attack = current_time;

        if let Some(on_action) = self.on_action.take() {
            on_action(wave, self).unwrap();
            self.on_action = Some(on_action);
        }

        // Sélectionner les cibles primaires en fonction de la stratégie
        match self.meta.target_selection {
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
                    let damage = self.stats.damage.base;
                    // Appliquer les effets du comportement de la tour
                    let actual_damage = self.meta.behavior.apply(monster, damage);
                    monster.hp -= actual_damage;

                    logs.push(format!(
                        "🏹 Tourelle {:?} attaque! -{:.1} HP sur {}. HP restants: {:.1}",
                        self.stats.damage.element, actual_damage, monster.name, monster.hp
                    ));
                }

                // Si la tourelle fait des AOE, appliquer des dégâts aux monstres proches de la cible
                if let Some(aoe) = &self.meta.aoe {
                    let target_pos = wave.monsters[target_idx].position;
                    // Rechercher les monstres dans le rayon de l'AOE
                    for (idx, monster) in wave.monsters.iter_mut().enumerate() {
                        // Ne pas réappliquer les dégâts à la cible principale
                        if idx == target_idx || !monster.active {
                            continue;
                        }

                        // Calculer la distance entre le monstre et la cible principale
                        let distance = target_pos.distance_to(&monster.position);

                        // Si le monstre est dans le rayon de l'AOE
                        if let TowerAoe::Radius(radius, damage_multiplier) = aoe {
                            if distance <= *radius as f32 {
                                let aoe_damage = self.stats.damage.base * damage_multiplier; // 50% des dégâts pour l'AOE
                                // Appliquer les effets du comportement de la tour
                                let actual_aoe_damage =
                                    self.meta.behavior.apply(monster, aoe_damage);
                                monster.hp -= actual_aoe_damage;
                            }
                        }

                        if let TowerAoe::Count(count, damage_multiplier) = aoe {}
                    }
                }
            }
        }

        logs
    }

    fn find_nearest_target(&self, wave: &Wave) -> Option<usize> {
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

            if distance <= self.stats.range.base && distance < min_distance {
                min_distance = distance;
                nearest_idx = Some(idx);
            }
        }

        nearest_idx
    }

    fn is_in_range(&self, monster: &Monster) -> bool {
        let tower_pos = self.position;
        let dx = (monster.position.x - tower_pos.x) as f32;
        let dy = (monster.position.y - tower_pos.y) as f32;
        let distance = (dx * dx + dy * dy).sqrt();

        distance <= self.stats.range.base
    }

    /// Retourne true si toutes les améliorations de la tour sont au niveau maximum
    pub fn is_fully_upgraded(&self) -> bool {
        // Vérifier si toutes les améliorations sont au niveau maximum
        self.upgrades.attacks_speed.as_ref().unwrap().level >= 30
            && self.upgrades.damage.as_ref().unwrap().level >= 30
            && self.upgrades.range.as_ref().unwrap().level >= 30
    }
}

/// Types d'améliorations disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeType {
    AttackSpeed,
    Damage,
    Range,
}
