use crate::domain::entities::{
    behavior::TowerBehavior,
    element::Element,
    position::Position,
    tower::{Tower, TowerBase},
};

/// Tour basique servant de modèle pour d'autres types de tours
#[derive(Debug, Clone)]
pub struct BasicTower {
    base: TowerBase,
}

impl BasicTower {
    pub fn positioned(position: Position) -> Self {
        Self {
            base: TowerBase {
                position,
                range: 1.0,
                element: Element::Fire, // Élément par défaut, peut être modifié
                damage: 10.0,
                attacks_per_second: 1.0,
                aoe: false,
                behavior: TowerBehavior::Basic,
            },
        }
    }

    pub fn new(
        position: Position,
        range: f32,
        element: Element,
        damage: f32,
        attacks_per_second: f32,
        aoe: bool,
        behavior: TowerBehavior,
    ) -> Self {
        Self {
            base: TowerBase {
                position,
                range,
                element,
                damage,
                attacks_per_second,
                aoe,
                behavior,
            },
        }
    }
}

impl Tower for BasicTower {
    fn position(&self) -> &Position {
        &self.base.position
    }

    fn range(&self) -> f32 {
        self.base.range
    }

    fn get_element(&self) -> Element {
        self.base.element
    }

    fn damage(&self) -> f32 {
        self.base.damage
    }

    fn attacks_per_second(&self) -> f32 {
        self.base.attacks_per_second
    }

    fn is_aoe(&self) -> bool {
        self.base.aoe
    }

    fn behavior(&self) -> &TowerBehavior {
        &self.base.behavior
    }
}
