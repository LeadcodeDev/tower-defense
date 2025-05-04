use crate::{
    application::engine::monsters::basics::{goblin, orc},
    domain::entities::{map::Map, position::Position},
};

pub struct DesertMap;

impl DesertMap {
    pub fn new() -> Map {
        let start_position = Position::new(0, 10);

        Map::new(
            "Désert Aride".to_string(),
            "Un désert aride et hostile".to_string(),
            "🌵".to_string(),
            "🚀".to_string(),
            vec![
                start_position,
                Position::new(5, 10),
                Position::new(5, 5),
                Position::new(10, 5),
                Position::new(10, 15),
                Position::new(15, 15),
                Position::new(15, 5),
                Position::new(20, 5),
            ],
            20, // largeur de la carte
            20, // hauteur de la carte
            vec![goblin(start_position), orc(start_position)],
            None,
        )
    }
}
