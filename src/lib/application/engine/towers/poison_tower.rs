use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{BaseStats, Tower, TowerKind, TowerStats, TowerUpgradeElement, TowerUpgrades},
};

pub struct PoisonTower;

impl PoisonTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            name: "Poison Tower".to_string(),
            upgrades: TowerUpgrades::new(
                50,
                TowerUpgradeElement::new(3.0, 0.5),
                TowerUpgradeElement::new(7.0, 1.25),
                TowerUpgradeElement::new(1.2, 1.2),
            ),
            stats: TowerStats {
                position,
                range: 3.0,
                element: Element::Poison,
                damage: 7.0,
                attacks_per_second: 1.2,
                aoe: true,
                behavior: TowerBehavior::Burning { dot_damage: 5.0 },
                last_attack: 0.0,
                target_selection: crate::domain::entities::tower::TargetSelection::Weakest,
                upgrade_level: 0,
                tower_type: TowerKind::Poison,
                base_stats: BaseStats {
                    range: 3.0,
                    damage: 7.0,
                    attacks_per_second: 1.2,
                },
            },
        }
    }
}

// Données spécifiques à la tour de poison
pub const POISON_TOWER_COST: u32 = 95;
pub const POISON_TOWER_UPGRADE_COST: u32 = 55;
pub const POISON_ATTACK_SPEED_FACTOR: f32 = 1.18;
pub const POISON_DAMAGE_FACTOR: f32 = 1.22;
pub const POISON_RANGE_BONUS: f32 = 0.35;

// Synergies pour les tours de poison
pub const POISON_DAMAGE_SYNERGY: f32 = 0.9; // Moins chères pour les dégâts
pub const POISON_SPEED_SYNERGY: f32 = 1.1; // Légèrement plus chères pour la vitesse
pub const POISON_RANGE_SYNERGY: f32 = 1.0; // Coût standard pour la portée
