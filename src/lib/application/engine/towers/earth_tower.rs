use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{
        BaseStats, TargetSelection, Tower, TowerKind, TowerStats, TowerUpgradeElement,
        TowerUpgrades,
    },
};

pub struct EarthTower;

impl EarthTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            name: "Earth Tower".to_string(),
            upgrades: TowerUpgrades::new(
                55,
                TowerUpgradeElement::new(2.8, 0.5),
                TowerUpgradeElement::new(15.0, 1.25),
                TowerUpgradeElement::new(0.8, 1.25),
            ),
            stats: TowerStats {
                position,
                range: 2.8,
                element: Element::Earth,
                damage: 15.0,
                attacks_per_second: 0.8,
                aoe: false,
                behavior: TowerBehavior::Basic,
                last_attack: 0.0,
                target_selection: TargetSelection::Strongest,
                upgrade_level: 0,
                tower_type: TowerKind::Earth,
                base_stats: BaseStats {
                    range: 2.8,
                    damage: 15.0,
                    attacks_per_second: 0.8,
                },
            },
        }
    }
}

// Données spécifiques à la tour de terre
pub const EARTH_TOWER_COST: u32 = 90;
pub const EARTH_TOWER_UPGRADE_COST: u32 = 50;
pub const EARTH_ATTACK_SPEED_FACTOR: f32 = 1.1;
pub const EARTH_DAMAGE_FACTOR: f32 = 1.35;
pub const EARTH_RANGE_BONUS: f32 = 0.3;

// Synergies pour les tours de terre
pub const EARTH_DAMAGE_SYNERGY: f32 = 0.8; // Moins chères pour les dégâts
pub const EARTH_SPEED_SYNERGY: f32 = 1.3; // Plus chères pour la vitesse
pub const EARTH_RANGE_SYNERGY: f32 = 1.2; // Plus chères pour la portée
