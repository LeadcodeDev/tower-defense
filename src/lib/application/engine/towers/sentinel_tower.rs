use crate::{
    domain::entities::{
        behavior::TowerBehavior,
        element::Element,
        position::Position,
        tower::{
            BaseStats, TargetSelection, Tower, TowerKind, TowerMeta, TowerStatDamageElement,
            TowerStatElement, TowerStats, TowerUpgradeElement, TowerUpgradeElementUnit,
            TowerUpgrades,
        },
        wave::Wave,
    },
    infrastructure::ui::notifications::Notifier,
};

pub struct SentinelTower;

impl SentinelTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            name: "Tour Sentinelle".to_string(),
            level: 1,
            position,
            last_attack: 0.0,
            stats: TowerStats {
                range: TowerStatElement::new(1.0, 1), // Portée de 1 case
                damage: TowerStatDamageElement::new(0.0, 1, Element::Neutral), // Pas de dégâts
                attacks_per_second: TowerStatElement::new(1.0, 1), // Vérification toutes les 2 secondes
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

                for monster in wave.monsters.iter().filter(|m| m.active) {
                    let distance = tower.position.distance_to(&monster.position);
                    if distance <= tower.stats.range.base {
                        monsters_detected = true;
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
