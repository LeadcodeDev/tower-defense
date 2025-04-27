use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{
        BaseStats, Tower, TowerKind, TowerStats, TowerUpgradeElement, TowerUpgradeElementUnit,
        TowerUpgrades,
    },
};

pub struct LightningTower;

impl LightningTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            name: "Lightning Tower".to_string(),
            upgrades: TowerUpgrades::new(
                50,
                TowerUpgradeElement::new(3.2, 0.5, TowerUpgradeElementUnit::Unit),
                TowerUpgradeElement::new(18.0, 1.25, TowerUpgradeElementUnit::Percent),
                TowerUpgradeElement::new(0.6, 1.2, TowerUpgradeElementUnit::Percent),
            ),
            stats: TowerStats {
                position,
                range: 3.2,
                element: Element::Lightning,
                damage: 18.0,
                attacks_per_second: 0.6,
                aoe: false,
                behavior: TowerBehavior::Lightning { stun_chance: 0.25 },
                last_attack: 0.0,
                target_selection: crate::domain::entities::tower::TargetSelection::Farthest,
                upgrade_level: 0,
                tower_type: TowerKind::Lightning,
                base_stats: BaseStats {
                    range: 3.2,
                    damage: 18.0,
                    attacks_per_second: 0.6,
                },
            },
        }
    }
}

// Données spécifiques à la tour de foudre
pub const LIGHTNING_TOWER_COST: u32 = 110;
pub const LIGHTNING_TOWER_UPGRADE_COST: u32 = 65;
pub const LIGHTNING_ATTACK_SPEED_FACTOR: f32 = 1.15;
pub const LIGHTNING_DAMAGE_FACTOR: f32 = 1.4;
pub const LIGHTNING_RANGE_BONUS: f32 = 0.4;

// Synergies pour les tours de foudre
pub const LIGHTNING_DAMAGE_SYNERGY: f32 = 1.25; // Plus chères pour les dégâts
pub const LIGHTNING_SPEED_SYNERGY: f32 = 1.2; // Plus chères pour la vitesse
pub const LIGHTNING_RANGE_SYNERGY: f32 = 1.1; // Légèrement plus chères pour la portée
