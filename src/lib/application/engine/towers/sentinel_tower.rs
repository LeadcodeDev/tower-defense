use uuid::Uuid;

use crate::{
    domain::entities::{
        behavior::TowerBehavior,
        element::Element,
        position::Position,
        tower::{
            TargetSelection, Tower, TowerKind, TowerMeta, TowerStatDamageElement, TowerStatElement,
            TowerStats, TowerUpgrades,
        },
    },
    infrastructure::ui::notifications::Notifier,
};

pub struct SentinelTower {
    pub detected_monsters: Vec<Uuid>,
}

impl SentinelTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            id: Uuid::new_v4(),
            name: "Tour Sentinelle".to_string(),
            level: 1,
            position,
            last_attack: 0.0,
            stats: TowerStats {
                range: TowerStatElement::new(1.0, 1), // Portée de 1 case
                damage: None,
                attacks_per_second: None, // Vérification toutes les 2 secondes
            },
            meta: TowerMeta {
                aoe: None,
                behavior: TowerBehavior::Basic,
                target_selection: TargetSelection::Nearest,
                tower_type: TowerKind::Sentinel,
            },
            upgrades: TowerUpgrades::new(50, None, None, None),
            on_action: Some(Box::new(|wave, tower| {
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
        }
    }
}
