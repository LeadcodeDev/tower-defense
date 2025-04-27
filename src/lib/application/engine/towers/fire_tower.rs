use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{
        BaseStats, TargetSelection, Tower, TowerKind, TowerStats, TowerUpgradeElement,
        TowerUpgradeElementUnit, TowerUpgrades,
    },
};

pub struct FireTower;

impl FireTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            name: "Fire Tower".to_string(),
            upgrades: TowerUpgrades::new(
                45,
                TowerUpgradeElement::new(3.0, 0.5, TowerUpgradeElementUnit::Unit),
                TowerUpgradeElement::new(12.0, 1.25, TowerUpgradeElementUnit::Unit),
                TowerUpgradeElement::new(1.0, 0.2, TowerUpgradeElementUnit::Unit),
            ),
            stats: TowerStats {
                position,
                range: 5.0,
                element: Element::Fire,
                damage: 12.0,
                attacks_per_second: 1.0,
                aoe: true,
                behavior: TowerBehavior::Basic,
                last_attack: 0.0,
                target_selection: TargetSelection::Nearest,
                upgrade_level: 0,
                tower_type: TowerKind::Fire,
                base_stats: BaseStats {
                    range: 5.0,
                    damage: 25.0,
                    attacks_per_second: 5.0,
                },
            },
        }
    }
}

// Données spécifiques à la tour de feu
pub const FIRE_TOWER_COST: u32 = 90;
pub const FIRE_TOWER_UPGRADE_COST: u32 = 50;
pub const FIRE_ATTACK_SPEED_FACTOR: f32 = 1.25;
pub const FIRE_DAMAGE_FACTOR: f32 = 1.2;
pub const FIRE_RANGE_BONUS: f32 = 0.3;

// Synergies pour les tours de feu
pub const FIRE_DAMAGE_SYNERGY: f32 = 1.3; // Plus chères pour les dégâts
pub const FIRE_SPEED_SYNERGY: f32 = 1.15; // Légèrement plus chères pour la vitesse
pub const FIRE_RANGE_SYNERGY: f32 = 1.0; // Coût standard pour la portée
