use std::rc::Rc;

use uuid::Uuid;

use crate::{
    domain::entities::{
        behavior::TowerBehavior,
        element::Element,
        position::Position,
        tower::{
            TargetSelection, Tower, TowerKind, TowerMeta, TowerStatDamageElement, TowerStatElement,
            TowerStats, TowerUpgradeElement, TowerUpgradeElementUnit, TowerUpgrades,
        },
    },
    infrastructure::ui::notifications::Notifier,
};

pub struct SentinelTower {
    pub detected_monsters: Vec<Uuid>,
}

impl SentinelTower {
    pub fn positionned(position: Position) -> Tower {
        Tower::new(
            "Sentinel Tower".to_string(),
            "🔭".to_string(),
            1,
            45,
            position,
            TowerStats {
                range: TowerStatElement::new(1.0, 1),
                damage: None,
                attacks_per_second: None,
            },
            TowerUpgrades::new(45, None, None, None),
            TowerMeta {
                aoe: None,
                behavior: TowerBehavior::Basic,
                target_selection: TargetSelection::Nearest,
                tower_type: TowerKind::Fire,
            },
            Some(Rc::new(|wave, tower| {
                let mut monsters_detected = false;

                let monsters = wave
                    .monsters
                    .iter_mut()
                    .filter(|monster| !monster.detected.contains(&tower.id));

                for monster in monsters {
                    let distance = tower.position.distance_to(&monster.position);
                    if distance <= tower.stats.range.base {
                        monsters_detected = true;
                        monster.detected.push(tower.id);

                        break;
                    }
                }

                if !monsters_detected {
                    return Ok(());
                }

                Notifier::send_notification(
                    "⚠️ Monstre détecté",
                    &format!(
                        "Un monstre approche de la tour Sentinelle en ({}, {})",
                        tower.position.x, tower.position.y
                    ),
                );

                Ok(())
            })),
        )
    }
}
