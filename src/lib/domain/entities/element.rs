#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Fire,
    Water,
    Earth,
    Air,
}

impl Element {
    pub fn get_name(&self) -> &str {
        match self {
            Element::Fire => "fire",
            Element::Water => "water",
            Element::Earth => "earth",
            Element::Air => "air",
        }
    }

    // Bonus d'élément (feu > terre, eau > feu, etc.)
    pub fn effectiveness_against(&self, target_element: &Element) -> f32 {
        match (self, target_element) {
            (Element::Fire, Element::Earth) => 1.5, // Le feu est efficace contre la terre
            (Element::Water, Element::Fire) => 1.5, // L'eau est efficace contre le feu
            (Element::Earth, Element::Air) => 1.5,  // La terre est efficace contre l'air
            (Element::Air, Element::Water) => 1.5,  // L'air est efficace contre l'eau
            (a, b) if a == b => 0.5,                // Même élément : résistance
            _ => 1.0,                               // Pas de bonus ou malus
        }
    }
}
