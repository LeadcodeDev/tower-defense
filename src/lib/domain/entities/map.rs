use super::{monster::Monster, position::Position};

#[derive(Debug, Clone)]
pub struct Map {
    pub name: String,
    pub waypoints: Vec<Position>,
    pub width: u32,
    pub height: u32,
    pub terrain_type: TerrainType,
    pub monsters: Vec<Monster>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerrainType {
    Plains,
    Forest,
    Desert,
    Mountain,
    Swamp,
    Snow,
    Cave,
}

impl TerrainType {
    // Retourne un coefficient de vitesse pour le terrain (plus petit = plus lent)
    pub fn speed_modifier(&self) -> f32 {
        match self {
            TerrainType::Plains => 1.0,   // Vitesse normale
            TerrainType::Forest => 0.8,   // -20% de vitesse
            TerrainType::Desert => 0.7,   // -30% de vitesse
            TerrainType::Mountain => 0.6, // -40% de vitesse
            TerrainType::Swamp => 0.5,    // -50% de vitesse
            TerrainType::Snow => 0.6,     // -40% de vitesse
            TerrainType::Cave => 0.65,    // -35% de vitesse
        }
    }

    // Retourne le nom du terrain en français
    pub fn get_name(&self) -> &'static str {
        match self {
            TerrainType::Plains => "Plaines",
            TerrainType::Forest => "Forêt",
            TerrainType::Desert => "Désert",
            TerrainType::Mountain => "Montagne",
            TerrainType::Swamp => "Marais",
            TerrainType::Snow => "Neige",
            TerrainType::Cave => "Caverne",
        }
    }
}

impl Map {
    pub fn new(
        name: String,
        waypoints: Vec<Position>,
        width: u32,
        height: u32,
        terrain_type: TerrainType,
        monsters: Vec<Monster>,
    ) -> Self {
        Self {
            name,
            waypoints,
            width,
            height,
            terrain_type,
            monsters,
        }
    }
}
