#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Neutral,
    Fire,
    Water,
    Earth,
    Air,
    Lightning,
    Ice,
    Poison,
}

impl Element {
    pub fn get_name(&self) -> &str {
        match self {
            Element::Neutral => "neutral",
            Element::Fire => "fire",
            Element::Water => "water",
            Element::Earth => "earth",
            Element::Air => "air",
            Element::Lightning => "lightning",
            Element::Ice => "ice",
            Element::Poison => "poison",
        }
    }

    // Bonus d'élément (feu > terre, eau > feu, etc.)
    pub fn effectiveness_against(&self, target_element: &Element) -> f32 {
        match (self, target_element) {
            // Éléments de base
            (Element::Fire, Element::Earth) => 1.5, // Le feu est efficace contre la terre
            (Element::Water, Element::Fire) => 1.5, // L'eau est efficace contre le feu
            (Element::Earth, Element::Air) => 1.5,  // La terre est efficace contre l'air
            (Element::Air, Element::Water) => 1.5,  // L'air est efficace contre l'eau

            // Nouveaux éléments
            (Element::Lightning, Element::Water) => 2.0, // La foudre est très efficace contre l'eau
            (Element::Lightning, Element::Air) => 0.75, // La foudre est moins efficace contre l'air
            (Element::Ice, Element::Fire) => 0.5,       // La glace est inefficace contre le feu
            (Element::Ice, Element::Water) => 1.25,     // La glace est efficace contre l'eau
            (Element::Poison, Element::Earth) => 0.75, // Le poison est moins efficace contre la terre
            (Element::Poison, Element::Air) => 1.25,   // Le poison est efficace contre l'air

            (Element::Neutral, _) => 1.0, // Élément neutre : pas de bonus ou malus
            (a, b) if a == b => 0.5,      // Même élément : résistance
            _ => 1.0,                     // Pas de bonus ou malus
        }
    }
}
