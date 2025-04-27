use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{BaseStats, TargetSelection, Tower, TowerKind, TowerStats},
};

pub struct WaterTower;

impl WaterTower {
    pub fn positionned(position: Position) -> Tower {
        Tower {
            stats: TowerStats {
                position,
                range: 3.5,
                element: Element::Water,
                damage: 12.0,
                attacks_per_second: 1.0,
                aoe: true,
                behavior: TowerBehavior::Basic,
                last_attack: 0.0,
                target_selection: TargetSelection::All,
                upgrade_level: 0,
                tower_type: TowerKind::Water,
                base_stats: BaseStats {
                    range: 3.5,
                    damage: 12.0,
                    attacks_per_second: 1.0,
                },
            },
        }
    }
}

// Données spécifiques à la tour d'eau
pub const WATER_TOWER_COST: u32 = 95;
pub const WATER_TOWER_UPGRADE_COST: u32 = 55;
pub const WATER_ATTACK_SPEED_FACTOR: f32 = 1.08;
pub const WATER_DAMAGE_FACTOR: f32 = 1.25;
pub const WATER_RANGE_BONUS: f32 = 0.35;

// Synergies pour les tours d'eau
pub const WATER_DAMAGE_SYNERGY: f32 = 1.15; // Plus chères pour les dégâts
pub const WATER_SPEED_SYNERGY: f32 = 1.0; // Coût standard pour la vitesse
pub const WATER_RANGE_SYNERGY: f32 = 0.95; // Légèrement moins chères pour la portée
