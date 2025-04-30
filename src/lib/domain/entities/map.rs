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

    /// Vérifie si une position est sur le chemin des monstres
    pub fn is_position_on_path(&self, position: &Position) -> bool {
        // Vérifier si la position est un waypoint
        if self
            .waypoints
            .iter()
            .any(|wp| wp.x == position.x && wp.y == position.y)
        {
            return true;
        }

        // Vérifier si la position est sur un segment entre deux waypoints
        for i in 0..self.waypoints.len() - 1 {
            let start = &self.waypoints[i];
            let end = &self.waypoints[i + 1];

            // Vérifier si la position est sur la ligne entre start et end
            if position.x >= start.x.min(end.x)
                && position.x <= start.x.max(end.x)
                && position.y >= start.y.min(end.y)
                && position.y <= start.y.max(end.y)
            {
                // Vérifier si la position est alignée avec la ligne
                let dx = end.x - start.x;
                let dy = end.y - start.y;

                if dx == 0 {
                    // Ligne verticale
                    if position.x == start.x {
                        return true;
                    }
                } else if dy == 0 {
                    // Ligne horizontale
                    if position.y == start.y {
                        return true;
                    }
                } else {
                    // Ligne diagonale
                    let slope = dy as f32 / dx as f32;
                    let expected_y = start.y as f32 + slope * (position.x - start.x) as f32;
                    if (expected_y - position.y as f32).abs() < 0.1 {
                        return true;
                    }
                }
            }
        }

        false
    }
}
