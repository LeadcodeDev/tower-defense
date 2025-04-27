use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{
        BaseStats, TargetSelection, Tower, TowerKind, TowerStats, TowerUpgradeElement,
        TowerUpgradeElementUnit, TowerUpgrades,
    },
};

pub struct BasicTower;

impl BasicTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            name: "Basic Tower".to_string(),
            upgrades: TowerUpgrades::new(
                30,
                TowerUpgradeElement::new(2.0, 0.5, TowerUpgradeElementUnit::Unit),
                TowerUpgradeElement::new(10.0, 1.25, TowerUpgradeElementUnit::Percent),
                TowerUpgradeElement::new(1.0, 1.2, TowerUpgradeElementUnit::Percent),
            ),
            stats: TowerStats {
                position,
                range: 2.0,
                element: Element::Neutral,
                damage: 10.0,
                attacks_per_second: 1.0,
                aoe: false,
                behavior: TowerBehavior::Basic,
                last_attack: 0.0,
                target_selection: TargetSelection::Nearest,
                upgrade_level: 0,
                tower_type: TowerKind::Basic,
                base_stats: BaseStats {
                    range: 2.0,
                    damage: 10.0,
                    attacks_per_second: 1.0,
                },
            },
        }
    }
}

pub const BASIC_RANGE_BONUS: f32 = 0.5;
