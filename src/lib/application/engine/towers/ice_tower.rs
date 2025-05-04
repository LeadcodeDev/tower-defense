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
        Tower::new(
            "Ice Tower".to_string(),
            "❄️".to_string(),
            1,
            45,
            position,
            TowerStats {
                range: TowerStatElement::new(5.0, 1),
                damage: Some(TowerStatDamageElement::new(25.0, 1, Element::Air)),
                attacks_per_second: Some(TowerStatElement::new(5.0, 1)),
            },
            TowerUpgrades::new(
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
            TowerMeta {
                aoe: None,
                behavior: TowerBehavior::Basic,
                target_selection: TargetSelection::Nearest,
                tower_type: TowerKind::Fire,
            },
            None,
        )
    }
}
