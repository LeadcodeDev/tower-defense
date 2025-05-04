use super::{monster::Monster, position::Position, wave::Wave};
use std::rc::Rc;

pub trait TerrainModifier: Fn() -> f32 + Clone {}
impl<T: Fn() -> f32 + Clone> TerrainModifier for T {}

#[derive(Clone)]
pub struct Map {
    pub name: String,
    pub description: String,
    pub start_symbol: String,
    pub end_symbol: String,
    pub waypoints: Vec<Position>,
    pub width: u32,
    pub height: u32,
    pub monsters: Vec<Monster>,
    pub apply_modifier: Option<Rc<dyn Fn() -> f32>>,
}

impl Map {
    pub fn new(
        name: String,
        description: String,
        start_symbol: String,
        end_symbol: String,
        waypoints: Vec<Position>,
        width: u32,
        height: u32,
        monsters: Vec<Monster>,
        apply_modifier: Option<Rc<dyn Fn() -> f32>>,
    ) -> Self {
        Self {
            name,
            description,
            start_symbol,
            end_symbol,
            waypoints,
            width,
            height,
            monsters,
            apply_modifier,
        }
    }

    /// Vérifie si une position est sur le chemin des monstres
    pub fn is_position_on_path(&self, position: &Position) -> bool {
        let is_waypoint_position = self
            .waypoints
            .iter()
            .any(|wp| wp.x == position.x && wp.y == position.y);

        if is_waypoint_position {
            return true;
        }

        for i in 0..self.waypoints.len() - 1 {
            let start = &self.waypoints[i];
            let end = &self.waypoints[i + 1];

            let location = position.x >= start.x.min(end.x)
                && position.x <= start.x.max(end.x)
                && position.y >= start.y.min(end.y)
                && position.y <= start.y.max(end.y);

            if location {
                let dx = end.x - start.x;
                let dy = end.y - start.y;

                if dx == 0 {
                    if position.x == start.x {
                        return true;
                    }
                } else if dy == 0 {
                    if position.y == start.y {
                        return true;
                    }
                } else {
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
