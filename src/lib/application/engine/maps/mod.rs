pub mod cave;
pub mod desert;
pub mod forest;

use crate::domain::entities::map::Map;
use cave::CaveMap;
use desert::DesertMap;
use forest::ForestMap;

/// Enum pour représenter toutes les cartes disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapType {
    Forest,
    Desert,
    Cave,
}

impl MapType {
    /// Retourne le nom de la carte en français
    pub fn get_name(&self) -> &'static str {
        match self {
            MapType::Forest => "Forêt Enchantée",
            MapType::Desert => "Désert Aride",
            MapType::Cave => "Caverne Profonde",
        }
    }

    /// Retourne la description de la carte
    pub fn get_description(&self) -> &'static str {
        match self {
            MapType::Forest => "Une forêt dense avec des chemins sinueux. Difficulté: Facile",
            MapType::Desert => "Un désert hostile avec des routes directes. Difficulté: Moyen",
            MapType::Cave => {
                "Une caverne sombre aux nombreux passages étroits. Difficulté: Difficile"
            }
        }
    }

    /// Crée une instance de la carte
    pub fn create_map(&self) -> Map {
        match self {
            MapType::Forest => ForestMap::new(),
            MapType::Desert => DesertMap::new(),
            MapType::Cave => CaveMap::new(),
        }
    }

    /// Retourne toutes les cartes disponibles
    pub fn all_maps() -> Vec<MapType> {
        vec![MapType::Forest, MapType::Desert, MapType::Cave]
    }
}
