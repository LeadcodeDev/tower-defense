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
    pub stats: TowerStats,
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

    pub fn upgrade_attack_speed(&mut self) -> bool {
        // Limiter le niveau d'amélioration à 5
        if self.stats.upgrade_level >= 5 {
            return false;
        }

        // Facteurs d'amélioration pour chaque type de tour
        let factor = match self.stats.tower_type {
            TowerKind::Basic => 1.2,
            TowerKind::Fire => 1.15,
            TowerKind::Water => 1.18,
            TowerKind::Earth => 1.25,
            TowerKind::Air => 1.15,
            TowerKind::Lightning => 1.15,
            TowerKind::Ice => 1.15,
            TowerKind::Poison => 1.15,
        };

        self.stats.attacks_per_second *= factor;
        self.stats.upgrade_level += 1;
        true
    }

    pub fn upgrade_damage(&mut self) -> bool {
        // Limiter le niveau d'amélioration à 5
        if self.stats.upgrade_level >= 5 {
            return false;
        }

        // Facteurs d'amélioration pour chaque type de tour
        let factor = match self.stats.tower_type {
            TowerKind::Basic => 1.25,
            TowerKind::Fire => 1.3,
            TowerKind::Water => 1.22,
            TowerKind::Earth => 1.2,
            TowerKind::Air => 1.25,
            TowerKind::Lightning => 1.3,
            TowerKind::Ice => 1.25,
            TowerKind::Poison => 1.25,
        };

        self.stats.damage *= factor;
        self.stats.upgrade_level += 1;
        true
    }

    pub fn upgrade_range(&mut self) -> bool {
        // Limiter le niveau d'amélioration à 5
        if self.stats.upgrade_level >= 5 {
            return false;
        }

        // Bonus de portée pour chaque type de tour
        let bonus = match self.stats.tower_type {
            TowerKind::Basic => 0.5,
            TowerKind::Fire => 0.75,
            TowerKind::Water => 0.6,
            TowerKind::Earth => 0.4,
            TowerKind::Air => 0.5,
            TowerKind::Lightning => 0.75,
            TowerKind::Ice => 0.6,
            TowerKind::Poison => 0.5,
        };

        self.stats.range += bonus;
        self.stats.upgrade_level += 1;
        true
    }

    /// Retourne le coût d'amélioration en fonction du niveau actuel
    pub fn upgrade_cost(&self) -> u32 {
        let base_cost = match self.stats.tower_type {
            TowerKind::Basic => 25,
            TowerKind::Fire => 40,
            TowerKind::Water => 40,
            TowerKind::Earth => 50,
            TowerKind::Air => 50,
            TowerKind::Lightning => 50,
            TowerKind::Ice => 50,
            TowerKind::Poison => 50,
        };

        // Algorithme de croissance exponentielle avec base linéaire
        let level = self.upgrade_level();
        let exponential_factor = 1.5_f32.powi(level as i32);
        let linear_component = 20 * level;

        (base_cost as f32 * exponential_factor + linear_component as f32).round() as u32
    }

    /// Retourne le coût d'amélioration spécifique à une caractéristique
    pub fn upgrade_cost_for_attribute(&self, upgrade_type: UpgradeType) -> u32 {
        let base_cost = match self.stats.tower_type {
            TowerKind::Basic => 30,
            TowerKind::Fire => 45,
            TowerKind::Water => 40,
            TowerKind::Earth => 55,
            TowerKind::Air => 50,
            TowerKind::Lightning => 50,
            TowerKind::Ice => 50,
            TowerKind::Poison => 50,
        };

        let level = self.upgrade_level();

        // Coût de base avec croissance exponentielle
        let base = (base_cost as f32 * 1.3_f32.powi(level as i32)).round() as u32;

        // Facteur de synergie basé sur l'élément et la caractéristique
        let synergy_factor = match self.stats.tower_type {
            TowerKind::Basic => 1.0, // Pas de synergie spéciale
            TowerKind::Fire => match upgrade_type {
                UpgradeType::Damage => 1.2, // Les tours de feu sont meilleures en dégâts
                UpgradeType::AttackSpeed => 0.9, // Un peu moins chères pour la vitesse
                UpgradeType::Range => 1.1,  // Standard pour la portée
            },
            TowerKind::Water => match upgrade_type {
                UpgradeType::Damage => 0.9,      // Moins chères pour les dégâts
                UpgradeType::AttackSpeed => 1.0, // Standard pour la vitesse
                UpgradeType::Range => 1.2,       // Meilleures en portée
            },
            TowerKind::Earth => match upgrade_type {
                UpgradeType::Damage => 1.1,      // Un peu meilleures en dégâts
                UpgradeType::AttackSpeed => 1.2, // Meilleures en vitesse d'attaque
                UpgradeType::Range => 0.9,       // Moins chères pour la portée
            },
            TowerKind::Air => match upgrade_type {
                UpgradeType::Damage => 1.0,      // Standard pour les dégâts
                UpgradeType::AttackSpeed => 1.3, // Bien meilleures en vitesse
                UpgradeType::Range => 0.8,       // Moins chères pour la portée
            },
            TowerKind::Lightning => match upgrade_type {
                UpgradeType::Damage => 1.2,
                UpgradeType::AttackSpeed => 0.9,
                UpgradeType::Range => 1.1,
            },
            TowerKind::Ice => match upgrade_type {
                UpgradeType::Damage => 1.1,
                UpgradeType::AttackSpeed => 1.2,
                UpgradeType::Range => 0.9,
            },
            TowerKind::Poison => match upgrade_type {
                UpgradeType::Damage => 1.0,
                UpgradeType::AttackSpeed => 1.3,
                UpgradeType::Range => 0.8,
            },
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
                    monster.hp -= damage;

                    let log_message = format!(
                        "🏹 Tourelle {:?} attaque! -{:.1} HP sur {}. HP restants: {:.1}",
                        self.element(),
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
                            let aoe_damage = self.damage() * 0.5; // 50% des dégâts pour l'AOE
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
}

/// Types d'améliorations disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeType {
    AttackSpeed,
    Damage,
    Range,
}
