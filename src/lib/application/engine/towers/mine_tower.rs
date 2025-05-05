use std::rc::Rc;

use ratatui::style::Color;

use crate::domain::entities::{
    position::Position,
    tower::{Tower, TowerStatType, TowerStatUpgrade, TowerStats, TowerUpgradeElementUnit},
};

pub struct MineTower;

impl MineTower {
    pub fn positionned(position: Position) -> Tower {
        Tower::new(
            "Mine".to_string(),
            "💰".to_string(),
            1,
            45,
            position,
            vec![TowerStats {
                stat_type: TowerStatType::Money,
                label: "Money".to_string(),
                icon: "💰".to_string(),
                base: 10.0,
                level: 1,
                upgrade: Some(TowerStatUpgrade {
                    price_multiplier: 1.5,
                    value_multiplier: 1.2,
                    value_multiplier_unit: TowerUpgradeElementUnit::Unit,
                    max_level: 10,
                }),
            }],
            None,
            Some(Rc::new(|_, game, tower| {
                tower.highlight = if tower.highlight.is_some() {
                    None
                } else {
                    Some(Color::Yellow)
                };

                let money = tower
                    .stats
                    .iter()
                    .find(|s| s.stat_type == TowerStatType::Money)
                    .unwrap();

                game.money += money.base as u32;
                Ok(())
            })),
        )
    }
}
