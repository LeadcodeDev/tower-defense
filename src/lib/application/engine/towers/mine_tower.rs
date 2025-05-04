use std::rc::Rc;

use ratatui::style::Color;
use uuid::Uuid;

use crate::domain::entities::{
    behavior::TowerBehavior,
    position::Position,
    tower::{TargetSelection, Tower, TowerKind, TowerMeta},
};

pub struct MineTower {
    pub detected_monsters: Vec<Uuid>,
}

impl MineTower {
    pub fn positionned(position: Position) -> Tower {
        Tower::new(
            "Mine".to_string(),
            "💰".to_string(),
            1,
            45,
            position,
            vec![],
            TowerMeta {
                aoe: None,
                behavior: TowerBehavior::Basic,
                target_selection: TargetSelection::Nearest,
                tower_type: TowerKind::Fire,
            },
            Some(Rc::new(|game, tower| {
                tower.highlight = if tower.highlight.is_some() {
                    None
                } else {
                    Some(Color::Yellow)
                };

                game.money += 100;
                Ok(())
            })),
        )
    }
}
