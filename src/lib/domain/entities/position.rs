#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn initial() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Calcule la distance en blocs (distance de Manhattan)
    /// Cette distance est la somme des distances horizontales et verticales
    /// et est plus adaptée pour un jeu en grille
    pub fn distance_to(&self, other: &Position) -> f32 {
        if self.x == other.x && self.y == other.y {
            return 0.0;
        }

        // Si les positions sont adjacentes (horizontalement, verticalement ou en diagonale)
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        if dx <= 1 && dy <= 1 {
            return 1.0;
        }

        (dx + dy) as f32
    }

    /// Calcule la distance euclidienne classique (ligne droite)
    pub fn euclidean_distance_to(&self, other: &Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
}
