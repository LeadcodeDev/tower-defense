use uuid::Uuid;

use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{
        BaseStats, TargetSelection, Tower, TowerKind, TowerMeta, TowerStatDamageElement,
        TowerStatElement, TowerStats, TowerUpgradeElement, TowerUpgradeElementUnit, TowerUpgrades,
    },
};

pub struct IceTower;

impl IceTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            id: Uuid::new_v4(),
            name: "Ice Tower".to_string(),
            level: 1,
            position,
            last_attack: 0.0,
            stats: TowerStats {
                range: TowerStatElement::new(5.0, 1),
                damage: Some(TowerStatDamageElement::new(25.0, 1, Element::Ice)),
                attacks_per_second: Some(TowerStatElement::new(5.0, 1)),
            },
            meta: TowerMeta {
                aoe: None,
                behavior: TowerBehavior::Basic,
                target_selection: TargetSelection::Nearest,
                tower_type: TowerKind::Fire,
            },
            upgrades: TowerUpgrades::new(
                45,
                Some(TowerUpgradeElement::new(
                    3.0,
                    0.5,
                    TowerUpgradeElementUnit::Unit,
                )),
                Some(TowerUpgradeElement::new(
                    12.0,
                    1.25,
                    TowerUpgradeElementUnit::Unit,
                )),
                Some(TowerUpgradeElement::new(
                    1.0,
                    0.2,
                    TowerUpgradeElementUnit::Unit,
                )),
            ),
            on_action: None,
        }
    }
}
