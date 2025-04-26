use super::monster::Monster;

#[derive(Debug, Clone)]
pub enum TowerBehavior {
    Basic,
    Frost { slow_factor: f32 },
    Burning { dot_damage: f32 },
    Lightning { stun_chance: f32 },
    Crusher { armor_reduction: f32 },
}

impl TowerBehavior {
    pub fn apply(&self, monster: &mut Monster, damage: f32) -> f32 {
        match self {
            TowerBehavior::Basic => damage,
            TowerBehavior::Frost { slow_factor } => apply_frost(monster, damage, *slow_factor),
            TowerBehavior::Burning { dot_damage } => apply_burning(monster, damage, *dot_damage),
            TowerBehavior::Lightning { stun_chance } => {
                apply_lightning(monster, damage, *stun_chance)
            }
            TowerBehavior::Crusher { armor_reduction } => {
                apply_crusher(monster, damage, *armor_reduction)
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            TowerBehavior::Basic => "Basic",
            TowerBehavior::Frost { .. } => "Frost",
            TowerBehavior::Burning { .. } => "Burning",
            TowerBehavior::Lightning { .. } => "Lightning",
            TowerBehavior::Crusher { .. } => "Crusher",
        }
    }
}

fn apply_frost(monster: &mut Monster, damage: f32, slow_factor: f32) -> f32 {
    let original_speed = monster.movement_speed;
    monster.movement_speed *= 1.0 - slow_factor;
    println!(
        "❄️ EFFET: Monstre ralenti de {:.1} à {:.1} (facteur {:.0}%)",
        original_speed,
        monster.movement_speed,
        slow_factor * 100.0
    );
    damage
}

fn apply_burning(monster: &mut Monster, damage: f32, dot_damage: f32) -> f32 {
    monster.hp -= dot_damage;
    println!(
        "☠️ EFFET: Poison inflige {:.1} dégâts supplémentaires",
        dot_damage
    );
    damage
}

fn apply_lightning(_monster: &mut Monster, damage: f32, _stun_chance: f32) -> f32 {
    // TODO: Implement stun logic
    damage
}

fn apply_crusher(_monster: &mut Monster, damage: f32, _armor_reduction: f32) -> f32 {
    // TODO: Implement armor reduction
    damage
}

pub fn apply_slow(monster: &mut Monster, damage: f32, slow_factor: f32) -> f32 {
    monster.movement_speed *= 1.0 - slow_factor;
    damage
}
