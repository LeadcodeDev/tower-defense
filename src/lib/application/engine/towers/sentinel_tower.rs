use std::rc::Rc;

use crate::domain::{
    entities::{
        position::Position,
        tower::{Tower, TowerStatType, TowerStats},
    },
    ports::notifier::Notifier,
};

pub struct SentinelTower;

impl SentinelTower {
    pub fn positionned(position: Position) -> Tower {
        Tower::new(
            "Sentinel Tower".to_string(),
            "🔭".to_string(),
            1,
            45,
            position,
            vec![
                TowerStats {
                    stat_type: TowerStatType::Range,
                    label: "Range".to_string(),
                    icon: "🔭".to_string(),
                    base: 2.0,
                    level: 1,
                    upgrade: None,
                },
                TowerStats {
                    stat_type: TowerStatType::AttackSpeed,
                    label: "Attack Speed".to_string(),
                    icon: "⏰".to_string(),
                    base: 1.0,
                    level: 1,
                    upgrade: None,
                },
            ],
            None,
            Some(Rc::new(|mediator, game, tower| {
                let mut monsters_detected = false;

                if let Some(wave) = &mut game.current_wave {
                    let monsters = wave
                        .monsters
                        .iter_mut()
                        .filter(|monster| !monster.detected.contains(&tower.id));

                    for monster in monsters {
                        let distance = tower.position.distance_to(&monster.position);
                        let range = tower
                            .stats
                            .iter()
                            .find(|stat| stat.stat_type == TowerStatType::Range)
                            .unwrap()
                            .base;

                        if distance.floor() <= range {
                            monsters_detected = true;
                            monster.detected.push(tower.id);

                            break;
                        }
                    }

                    if !monsters_detected {
                        return Ok(());
                    }

                    mediator.notifier.send_notification(
                        "⚠️ Monstre détecté",
                        &format!(
                            "Un monstre approche de la tour Sentinelle en ({}, {})",
                            tower.position.x, tower.position.y
                        ),
                    );

                    Ok(())
                } else {
                    Ok(())
                }
            })),
        )
    }
}
