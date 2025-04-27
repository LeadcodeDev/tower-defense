use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{BaseStats, Tower, TowerKind, TowerStats},
};

pub struct IceTower;

impl IceTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            stats: TowerStats {
                position,
                range: 3.8,
                element: Element::Ice,
                damage: 10.0,
                attacks_per_second: 0.7,
                aoe: true,
                behavior: TowerBehavior::Frost { slow_factor: 0.3 },
                last_attack: 0.0,
                target_selection: crate::domain::entities::tower::TargetSelection::All,
                upgrade_level: 0,
                tower_type: TowerKind::Ice,
                base_stats: BaseStats {
                    range: 3.8,
                    damage: 10.0,
                    attacks_per_second: 0.7,
                },
            },
        }
    }
}

// Données spécifiques à la tour de glace
pub const ICE_TOWER_COST: u32 = 100;
pub const ICE_TOWER_UPGRADE_COST: u32 = 60;
pub const ICE_ATTACK_SPEED_FACTOR: f32 = 1.12;
pub const ICE_DAMAGE_FACTOR: f32 = 1.18;
pub const ICE_RANGE_BONUS: f32 = 0.45;

// Synergies pour les tours de glace
pub const ICE_DAMAGE_SYNERGY: f32 = 1.1; // Légèrement plus chères pour les dégâts
pub const ICE_SPEED_SYNERGY: f32 = 0.95; // Légèrement moins chères pour la vitesse
pub const ICE_RANGE_SYNERGY: f32 = 0.8; // Beaucoup moins chères pour la portée
