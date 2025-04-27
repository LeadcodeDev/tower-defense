use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{
        BaseStats, TargetSelection, Tower, TowerKind, TowerStats, TowerUpgradeElement,
        TowerUpgradeElementUnit, TowerUpgrades,
    },
};

pub struct AirTower;

impl AirTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            name: "Air Tower".to_string(),
            upgrades: TowerUpgrades::new(
                50,
                TowerUpgradeElement::new(4.0, 1.0, TowerUpgradeElementUnit::Unit),
                TowerUpgradeElement::new(8.0, 1.0, TowerUpgradeElementUnit::Unit),
                TowerUpgradeElement::new(1.5, 1.0, TowerUpgradeElementUnit::Unit),
            ),
            stats: TowerStats {
                position,
                range: 4.0,
                element: Element::Air,
                damage: 8.0,
                attacks_per_second: 1.5,
                aoe: false,
                behavior: TowerBehavior::Basic,
                last_attack: 0.0,
                target_selection: TargetSelection::Flying,
                upgrade_level: 0,
                tower_type: TowerKind::Air,
                base_stats: BaseStats {
                    range: 4.0,
                    damage: 8.0,
                    attacks_per_second: 1.5,
                },
            },
        }
    }
}

// Données spécifiques à la tour d'air
pub const AIR_TOWER_COST: u32 = 85;
pub const AIR_TOWER_UPGRADE_COST: u32 = 45;
pub const AIR_ATTACK_SPEED_FACTOR: f32 = 1.25;
pub const AIR_DAMAGE_FACTOR: f32 = 1.15;
pub const AIR_RANGE_BONUS: f32 = 0.5;

// Synergies pour les tours d'air
pub const AIR_DAMAGE_SYNERGY: f32 = 1.2; // Plus chères pour les dégâts
pub const AIR_SPEED_SYNERGY: f32 = 0.8; // Moins chères pour la vitesse
pub const AIR_RANGE_SYNERGY: f32 = 0.9; // Moins chères pour la portée
