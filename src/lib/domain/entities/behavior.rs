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
    monster.movement_speed *= (1.0 - slow_factor);
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

fn apply_lightning(monster: &mut Monster, damage: f32, stun_chance: f32) -> f32 {
    if rand::random::<f32>() < stun_chance {
        println!("⚡ EFFET: Monstre étourdi brièvement!");
        // Dans un système plus avancé, on ajouterait un statut "stunned"
    }
    damage
}

fn apply_crusher(monster: &mut Monster, damage: f32, armor_reduction: f32) -> f32 {
    println!(
        "🔨 EFFET: Résistances réduites de {:.0}%",
        armor_reduction * 100.0
    );
    damage * (1.0 + armor_reduction)
}
