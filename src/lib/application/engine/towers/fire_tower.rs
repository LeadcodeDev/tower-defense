use crate::domain::entities::{
    behavior::TowerBehavior,
    position::Position,
    tower::{
        TargetSelection, Tower, TowerAoe, TowerMeta, TowerStatType, TowerStatUpgrade, TowerStats,
        TowerUpgradeElementUnit,
    },
};

pub struct FireTower;

impl FireTower {
    pub fn positionned(position: Position) -> Tower {
        Tower::new(
            "Fire Tower".to_string(),
            "🔥".to_string(),
            1,
            45,
            position,
            vec![
                TowerStats {
                    stat_type: TowerStatType::Range,
                    label: "Range".to_string(),
                    icon: "🔥".to_string(),
                    base: 5.0,
                    level: 1,
                    upgrade: Some(TowerStatUpgrade {
                        price_multiplier: 1.5,
                        value_multiplier: 1.2,
                        value_multiplier_unit: TowerUpgradeElementUnit::Unit,
                        max_level: 10,
                    }),
                },
                TowerStats {
                    stat_type: TowerStatType::Damage,
                    label: "Damage".to_string(),
                    icon: "🔥".to_string(),
                    base: 10.0,
                    level: 1,
                    upgrade: Some(TowerStatUpgrade {
                        price_multiplier: 1.8,
                        value_multiplier: 18.0,
                        value_multiplier_unit: TowerUpgradeElementUnit::Unit,
                        max_level: 10,
                    }),
                },
                TowerStats {
                    stat_type: TowerStatType::AttackSpeed,
                    label: "Attack Speed".to_string(),
                    icon: "🔥".to_string(),
                    base: 0.5,
                    level: 1,
                    upgrade: Some(TowerStatUpgrade {
                        price_multiplier: 1.8,
                        value_multiplier: 0.5,
                        value_multiplier_unit: TowerUpgradeElementUnit::Unit,
                        max_level: 10,
                    }),
                },
            ],
            Some(vec![
                TowerMeta::AoeEffect(TowerAoe::Radius(10, 50.0)),
                TowerMeta::Behavior(TowerBehavior::Basic),
                TowerMeta::TargetSelection(TargetSelection::Nearest),
            ]),
            None,
        )
    }
}
