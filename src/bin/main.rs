use rand::rng;
use rand::seq::IndexedRandom;
use rust_tower::application::engine::maps::forest::forest_map;
use rust_tower::application::engine::towers::{BasicTower, FireBasicTower};
use rust_tower::domain::entities::game::Game;
use rust_tower::domain::entities::map::Map;
use rust_tower::domain::entities::monster::{Monster, Resistances};
use rust_tower::domain::entities::position::Position;
use rust_tower::domain::entities::tower::TowerType;
use rust_tower::domain::entities::wave::Wave;

fn main() {
    let map = forest_map();

    // Nombre de monstres à générer aléatoirement
    let n = 10;

    // Sélection aléatoire de N monstres parmi le vecteur
    let mut selected_monsters = Vec::new();
    for _ in 0..n {
        if let Some(monster) = map.monsters.choose(&mut rng()) {
            selected_monsters.push(monster.clone());
        }
    }

    let wave = Wave::new(selected_monsters);

    let towers = vec![
        TowerType::Basic(BasicTower::positioned(Position::new(5, 0))),
        TowerType::Fire(FireBasicTower::positioned(Position::new(10, 0))),
    ];

    let mut game = Game::new(vec![map], vec![wave], towers, 10, 1.0);
    game.run(0.1, None); // Augmenter le fps pour une simulation plus fluide
}
