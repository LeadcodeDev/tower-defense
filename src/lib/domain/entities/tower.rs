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
    /// Niveau d'amélioration
    pub upgrade_level: u32,
    /// Type de la tour
    pub tower_type: TowerKind,
    /// Statistiques de base (pour calculer les améliorations)
    pub base_stats: BaseStats,
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
    pub range: TowerUpgradeElement,
    pub damage: TowerUpgradeElement,
    pub attacks_speed: TowerUpgradeElement,
}

impl TowerUpgrades {
    pub fn new(
        base_cost: u32,
        range: TowerUpgradeElement,
        damage: TowerUpgradeElement,
        attacks_speed: TowerUpgradeElement,
    ) -> Self {
        Self {
            base_cost,
            range,
            damage,
            attacks_speed,
        }
    }
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
}

/// Structure uniforme pour toutes les tourelles
#[derive(Debug, Clone)]
pub struct Tower {
    pub name: String,
    pub stats: TowerStats,
    pub upgrades: TowerUpgrades,
}

impl Tower {
    pub fn position(&self) -> Position {
        self.stats.position
    }

    pub fn range(&self) -> f32 {
        self.stats.range
    }

    pub fn element(&self) -> Element {
        self.stats.element
    }

    pub fn damage(&self) -> f32 {
        self.stats.damage
    }

    pub fn attacks_per_second(&self) -> f32 {
        self.stats.attacks_per_second
    }

    pub fn is_aoe(&self) -> bool {
        self.stats.aoe
    }

    pub fn behavior(&self) -> &TowerBehavior {
        &self.stats.behavior
    }

    pub fn last_attack_time(&self) -> f32 {
        self.stats.last_attack
    }

    pub fn set_last_attack_time(&mut self, time: f32) {
        self.stats.last_attack = time;
    }

    pub fn target_selection(&self) -> TargetSelection {
        self.stats.target_selection
    }

    pub fn upgrade_level(&self) -> u32 {
        self.stats.upgrade_level
    }

    pub fn can_shoot(&self, current_time: f32) -> bool {
        let time_since_last_attack = current_time - self.last_attack_time();
        time_since_last_attack >= 1.0 / self.attacks_per_second()
    }

    pub fn upgrade_attack_speed(&mut self) -> Result<String, String> {
        // Niveau maximum selon le type de tour
        let max_level = if self.stats.tower_type == TowerKind::Earth {
            10
        } else {
            30
        };

        if self.upgrades.attacks_speed.level >= max_level {
            return Err(format!("La vitesse d'attaque est déjà au niveau maximum."));
        }

        let current = self.attacks_per_second();
        self.upgrades.attacks_speed.level += 1;
        self.stats.attacks_per_second = match self.upgrades.attacks_speed.value_multiplier_unit {
            TowerUpgradeElementUnit::Percent => {
                current * self.upgrades.attacks_speed.value_multiplier
            }

            TowerUpgradeElementUnit::Unit => current + self.upgrades.attacks_speed.value_multiplier,
        };

        Ok(format!(
            "🔧 Tour {} vitesse d'attaque améliorée ({} -> {})",
            self.tower_type_name(),
            current,
            self.attacks_per_second()
        ))
    }

    pub fn upgrade_damage(&mut self) -> Result<String, String> {
        // Niveau maximum selon le type de tour
        let max_level = if self.stats.tower_type == TowerKind::Earth {
            10
        } else {
            30
        };

        if self.upgrades.damage.level >= max_level {
            return Err(format!("Les dégâts sont déjà au niveau maximum."));
        }

        let current = self.damage();
        self.upgrades.damage.level += 1;
        self.stats.damage = match self.upgrades.damage.value_multiplier_unit {
            TowerUpgradeElementUnit::Percent => current * self.upgrades.damage.value_multiplier,
            TowerUpgradeElementUnit::Unit => current + self.upgrades.damage.value_multiplier,
        };

        Ok(format!(
            "🔧 Tour {} dégâts améliorés ({} -> {})",
            self.tower_type_name(),
            current,
            self.damage()
        ))
    }

    pub fn upgrade_range(&mut self) -> Result<String, String> {
        // Niveau maximum selon le type de tour
        let max_level = if self.stats.tower_type == TowerKind::Earth {
            10
        } else {
            30
        };

        if self.upgrades.range.level >= max_level {
            return Err(format!("La portée est déjà au niveau maximum."));
        }

        let current = self.range();
        self.upgrades.range.level += 1;
        self.stats.range = match self.upgrades.range.value_multiplier_unit {
            TowerUpgradeElementUnit::Percent => current * self.upgrades.range.value_multiplier,
            TowerUpgradeElementUnit::Unit => current + self.upgrades.range.value_multiplier,
        };

        Ok(format!(
            "🔧 Tour {} portée améliorée ({} -> {})",
            self.tower_type_name(),
            current,
            self.range()
        ))
    }

    /// Retourne le coût d'amélioration en fonction du niveau actuel
    pub fn upgrade_cost(&self) -> u32 {
        let level = self.upgrade_level();
        let exponential_factor = 1.5_f32.powi(level as i32);
        let linear_component = 20 * level;

        (self.upgrades.base_cost as f32 * exponential_factor + linear_component as f32).round()
            as u32
    }

    pub fn upgrade_cost_for_attribute(&self, upgrade_type: UpgradeType) -> u32 {
        let level = match upgrade_type {
            UpgradeType::Damage => self.upgrades.damage.level,
            UpgradeType::AttackSpeed => self.upgrades.attacks_speed.level,
            UpgradeType::Range => self.upgrades.range.level,
        };

        // Vérifier si on a atteint le niveau maximum
        let max_level = if self.stats.tower_type == TowerKind::Earth {
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
            UpgradeType::Damage => self.upgrades.damage.price_multiplier,
            UpgradeType::AttackSpeed => self.upgrades.attacks_speed.price_multiplier,
            UpgradeType::Range => self.upgrades.range.price_multiplier,
        };

        (base as f32 * synergy_factor).round() as u32
    }

    /// Retourne le type de la tour sous forme de chaîne
    pub fn tower_type_name(&self) -> &str {
        match self.stats.tower_type {
            TowerKind::Basic => "Basique",
            TowerKind::Fire => "Feu",
            TowerKind::Water => "Eau",
            TowerKind::Earth => "Terre",
            TowerKind::Air => "Air",
            TowerKind::Lightning => "Éclair",
            TowerKind::Ice => "Glace",
            TowerKind::Poison => "Poison",
        }
    }

    pub fn shoot(&mut self, wave: &mut Wave, current_time: f32) -> Vec<String> {
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
                    let damage = self.damage();
                    // Appliquer les effets du comportement de la tour
                    let actual_damage = self.behavior().apply(monster, damage);
                    monster.hp -= actual_damage;

                    logs.push(format!(
                        "🏹 Tourelle {:?} attaque! -{:.1} HP sur {}. HP restants: {:.1}",
                        self.element(),
                        actual_damage,
                        monster.name,
                        monster.hp
                    ));
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
                            let aoe_damage = self.damage() * 0.5; // 50% des dégâts pour l'AOE
                            // Appliquer les effets du comportement de la tour
                            let actual_aoe_damage = self.behavior().apply(monster, aoe_damage);
                            monster.hp -= actual_aoe_damage;

                            let log_message = format!(
                                "🔥 Effet AOE! -{:.1} HP sur {}. HP restants: {:.1}",
                                actual_aoe_damage, monster.name, monster.hp
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

    /// Retourne true si toutes les améliorations de la tour sont au niveau maximum
    pub fn is_fully_upgraded(&self) -> bool {
        // Déterminer le niveau maximum selon le type de tour
        let max_level = if self.stats.tower_type == TowerKind::Earth {
            10
        } else {
            30
        };

        // Vérifier si toutes les améliorations sont au niveau maximum
        self.upgrades.attacks_speed.level >= max_level
            && self.upgrades.damage.level >= max_level
            && self.upgrades.range.level >= max_level
    }
}

/// Types d'améliorations disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeType {
    AttackSpeed,
    Damage,
    Range,
}
