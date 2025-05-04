use crate::domain::entities::monster::{Monster, Resistances};
use crate::domain::entities::position::Position;

pub fn goblin(position: Position) -> Monster {
    Monster::new(
        "Gobelin".to_string(),
        "😈".to_string(),
        50.0,
        position,
        5.0,
        Resistances::default(),
        1,
    )
}

pub fn orc(position: Position) -> Monster {
    Monster::new(
        "Orc".to_string(),
        "👹".to_string(),
        100.0,
        position,
        0.8,
        Resistances::default(),
        2,
    )
}
