use std::sync::Arc;

use crate::application::engine::maps::forest::ForestMap;
use crate::domain::entities::map::Map;
use crate::domain::entities::tower::TowerKind;
use crate::domain::entities::tower::{Tower, TowerStatType};
use crate::domain::entities::{game::Game, position::Position};
use crate::domain::mediator::MediatorService;
use color_eyre::Result;
use crossterm::event::KeyCode;
use rand::{rng, seq::IndexedRandom};

use super::ui::render;
use super::{
    events::{
        event::{Event, EventConfig, Events},
        handlers::{
            handle_key_down, handle_key_esc, handle_key_left, handle_key_p, handle_key_q,
            handle_key_right, handle_key_up,
        },
    },
    tui::Tui,
};

/// Actions disponibles dans le menu d'action du jeu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    BuildTower,   // Action pour construire une tour (amène au sous-menu)
    RemoveTower,  // Action pour supprimer une tour
    UpgradeTower, // Action pour améliorer une tour existante
}

/// Types de tours disponibles pour la construction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerType {
    Basic,
    Fire,
    Water,
    Earth,
    Air,
    Lightning,
    Ice,
    Poison,
    Sentinel,
}

impl TowerType {
    /// Retourne le coût en pièces de la tour
    pub fn cost(&self) -> u32 {
        match *self {
            TowerType::Basic => 50,
            TowerType::Fire => 75,
            TowerType::Water => 75,
            TowerType::Earth => 100,
            TowerType::Air => 100,
            TowerType::Lightning => 110,
            TowerType::Ice => 95,
            TowerType::Poison => 90,
            TowerType::Sentinel => 100,
        }
    }

    /// Convertir en TowerKind
    pub fn to_tower_kind(&self) -> TowerKind {
        match *self {
            TowerType::Basic => TowerKind::Basic,
            TowerType::Fire => TowerKind::Fire,
            TowerType::Water => TowerKind::Water,
            TowerType::Earth => TowerKind::Earth,
            TowerType::Air => TowerKind::Air,
            TowerType::Lightning => TowerKind::Lightning,
            TowerType::Ice => TowerKind::Ice,
            TowerType::Poison => TowerKind::Poison,
            TowerType::Sentinel => TowerKind::Sentinel,
        }
    }
}

/// Modes d'interface utilisateur
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    Normal,         // Mode normal de jeu
    TowerSelection, // Mode de sélection du type de tour
    Placement,      // Mode de placement sur la carte
    TowerUpgrade,   // Mode d'amélioration de tour
}

/// Structure pour gérer le menu des améliorations
pub struct UpgradeMenu {
    pub tower_index: usize,
    pub selected_upgrade: usize,
    pub available_upgrades: Vec<(TowerStatType, String)>,
}

impl UpgradeMenu {
    pub fn new(tower_index: usize, available_upgrades: Vec<(TowerStatType, String)>) -> Self {
        Self {
            tower_index,
            selected_upgrade: 0,
            available_upgrades,
        }
    }
}

/// Représente l'état global de l'application TUI
pub struct App {
    pub mediator: Arc<MediatorService>,
    pub running: bool,
    pub game: Game,
    pub current_view: View,
    pub selected_index: usize,
    pub available_actions: Vec<GameAction>,
    pub available_towers: Vec<Tower>,
    pub ui_mode: UiMode,
    pub cursor_position: Position,
    pub selected_tower: Option<Tower>,
    pub upgrade_menu: Option<UpgradeMenu>,
    pub tower_selection_on_map: bool,
    pub selected_tower_index: Option<usize>,
    pub available_maps: Vec<Map>,
    pub selected_map: Option<Map>,
}

/// Les différentes vues disponibles dans l'application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Vue principale du jeu
    Game,
    /// Menu principal
    MainMenu,
    /// Écran de pause
    Pause,
    /// Écran de fin de jeu
    GameOver,
    /// Écran de sélection de carte
    MapSelection,
}

impl App {
    /// Crée une nouvelle instance de l'application avec le jeu fourni
    pub fn new(mediator: Arc<MediatorService>, towers: Vec<Tower>, maps: Vec<Map>) -> Self {
        // Actions par défaut
        let actions = vec![
            GameAction::BuildTower,
            GameAction::UpgradeTower,
            GameAction::RemoveTower,
        ];

        Self {
            mediator: mediator.clone(),
            running: true,
            game: Game::new(mediator.clone(), towers.clone(), 10, 1.0),
            current_view: View::MainMenu,
            selected_index: 0,
            available_actions: actions,
            available_towers: towers,
            ui_mode: UiMode::Normal,
            cursor_position: Position::new(5, 5),
            selected_tower: None,
            upgrade_menu: None,
            tower_selection_on_map: false,
            selected_tower_index: None,
            available_maps: maps,
            selected_map: None,
        }
    }

    pub fn run(&mut self, tui: &mut Tui) -> Result<()> {
        let events = Events::new(EventConfig::default());
        while self.running {
            tui.draw(|frame| render(self, frame))?;

            match events.next()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => handle_key_q(self),
                    KeyCode::Up => handle_key_up(self),
                    KeyCode::Down => handle_key_down(self),
                    KeyCode::Left => handle_key_left(self),
                    KeyCode::Right => handle_key_right(self),
                    KeyCode::Enter => self.confirm_selection(),
                    KeyCode::Esc => handle_key_esc(self),
                    KeyCode::Char('p') => handle_key_p(self),
                    _ => {}
                },
                Event::Tick => self.tick(0.1),
                _ => {}
            }
        }

        Ok(())
    }

    /// Met à jour l'état du jeu
    pub fn tick(&mut self, delta_time: f32) {
        // Ne met à jour le jeu que si nous sommes dans la vue de jeu active
        if matches!(self.current_view, View::Game) {
            // Déplacer la logique d'update ici pour être contrôlée par l'UI
            self.game.update(delta_time);

            // Vérifier l'état du jeu pour les transitions
            if self.game.player_life <= 0 {
                self.current_view = View::GameOver;
            } else if self.game.waves.is_none() && self.game.current_wave.is_none() {
                self.current_view = View::GameOver;
            }
        }
    }

    /// Quitte l'application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Change la vue courante
    pub fn set_view(&mut self, view: View) {
        self.current_view = view;
        self.selected_index = 0; // Réinitialiser la sélection
        self.ui_mode = UiMode::Normal; // Réinitialiser le mode
    }

    /// Retourne le nombre d'éléments dans le menu actuel
    fn current_menu_length(&self) -> usize {
        match self.current_view {
            View::Game => {
                match self.ui_mode {
                    UiMode::Normal => self.available_actions.len(),
                    UiMode::TowerSelection => self.available_towers.len(),
                    UiMode::Placement => 0, // En mode placement, pas de menu
                    UiMode::TowerUpgrade => self.game.towers.len(), // Nombre de tours sur le terrain
                }
            }
            View::MainMenu => 2, // Nombre d'options dans le menu principal
            View::MapSelection => self.available_maps.len(), // Nombre de cartes disponibles
            View::Pause => 2,    // Nombre d'options dans le menu de pause
            View::GameOver => 2, // Nombre d'options dans le menu de game over
        }
    }

    /// Sélectionne l'élément suivant dans le menu
    pub fn next_item(&mut self) {
        // Ne pas changer de sélection en mode placement
        if self.ui_mode == UiMode::Placement {
            return;
        }

        if self.ui_mode == UiMode::TowerUpgrade {
            if let Some(upgrade_menu) = &mut self.upgrade_menu {
                let options_count = upgrade_menu.available_upgrades.len() + 1; // +1 pour l'option "Annuler"
                upgrade_menu.selected_upgrade = (upgrade_menu.selected_upgrade + 1) % options_count;
            }
            return;
        }

        let menu_len = self.current_menu_length();
        if menu_len > 0 {
            self.selected_index = (self.selected_index + 1) % menu_len;
        }
    }

    /// Sélectionne l'élément précédent dans le menu
    pub fn previous_item(&mut self) {
        // Ne pas changer de sélection en mode placement
        if self.ui_mode == UiMode::Placement {
            return;
        }

        if self.ui_mode == UiMode::TowerUpgrade {
            if let Some(upgrade_menu) = &mut self.upgrade_menu {
                let options_count = upgrade_menu.available_upgrades.len() + 1; // +1 pour l'option "Annuler"
                upgrade_menu.selected_upgrade =
                    (upgrade_menu.selected_upgrade + options_count - 1) % options_count;
            }
            return;
        }

        let menu_len = self.current_menu_length();
        if menu_len > 0 {
            self.selected_index = (self.selected_index + menu_len - 1) % menu_len;
        }
    }

    /// Déplace le curseur sur la carte
    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        if self.ui_mode == UiMode::Placement {
            let current_map = self.game.current_map.as_ref().unwrap();
            let new_x = (self.cursor_position.x + dx)
                .max(0)
                .min(current_map.width.try_into().unwrap());
            let new_y = (self.cursor_position.y + dy)
                .max(0)
                .min(current_map.height.try_into().unwrap());
            self.cursor_position = Position::new(new_x, new_y);
        }
    }

    /// Annule l'action en cours
    pub fn cancel_action(&mut self) {
        match self.ui_mode {
            UiMode::Normal => {
                // En mode normal, ne rien faire ou mettre en pause
            }
            UiMode::TowerSelection => {
                // Retourner au mode normal
                self.ui_mode = UiMode::Normal;
                self.selected_index = 0; // Réinitialiser la sélection
                self.tower_selection_on_map = false;
                self.selected_tower_index = None;
            }
            UiMode::Placement => {
                // Si on est en mode placement, retourner à la sélection de tour
                if let Some(_) = self.selected_tower {
                    self.ui_mode = UiMode::TowerSelection;
                } else {
                    // Si pas de tour sélectionnée, retourner au mode normal
                    self.ui_mode = UiMode::Normal;
                }
                self.selected_tower = None;
            }
            UiMode::TowerUpgrade => {
                // Retourner au mode normal depuis l'amélioration
                self.ui_mode = UiMode::Normal;
                self.selected_index = 0;
                self.upgrade_menu = None;
                self.tower_selection_on_map = false;
                self.selected_tower_index = None;
            }
        }
    }

    /// Valide l'action sélectionnée en fonction de la vue courante
    pub fn confirm_selection(&mut self) {
        match self.current_view {
            View::Game => {
                match self.ui_mode {
                    UiMode::Normal => {
                        let action = self.available_actions[self.selected_index];
                        match action {
                            GameAction::BuildTower => {
                                self.ui_mode = UiMode::TowerSelection;
                                self.selected_index = 0;
                                self.tower_selection_on_map = false;
                            }
                            GameAction::RemoveTower => {
                                self.ui_mode = UiMode::Placement;
                                self.selected_tower = None;
                                self.tower_selection_on_map = false;
                            }
                            GameAction::UpgradeTower => {
                                if !self.game.towers.is_empty() {
                                    self.ui_mode = UiMode::Placement;
                                    self.selected_tower = None;
                                }
                            }
                        }
                    }
                    UiMode::TowerSelection => {
                        if self.tower_selection_on_map {
                            if let Some(tower_index) = self.selected_tower_index {
                                self.upgrade_tower(tower_index, None);

                                self.ui_mode = UiMode::TowerUpgrade;
                                self.tower_selection_on_map = false;
                            }
                        } else {
                            if self.selected_index < self.available_towers.len() {
                                let tower = self.available_towers[self.selected_index].clone();

                                self.selected_tower = Some(tower);
                                self.ui_mode = UiMode::Placement; // Passer en mode placement
                            }
                        }
                    }
                    UiMode::Placement => {
                        let is_upgrade_mode = self.selected_index < self.available_actions.len()
                            && self.selected_tower.is_none()
                            && self.available_actions[self.selected_index]
                                == GameAction::UpgradeTower;

                        if is_upgrade_mode {
                            let mut found_tower = false;
                            for (idx, tower) in self.game.towers.iter().enumerate() {
                                if tower.position.x == self.cursor_position.x
                                    && tower.position.y == self.cursor_position.y
                                {
                                    self.upgrade_tower(idx, None);
                                    found_tower = true;
                                    break;
                                }
                            }

                            if !found_tower {
                                self.game.add_log(
                                    "Aucune tour à cette position pour amélioration.".to_string(),
                                );
                            }
                        } else if let Some(tower) = self.selected_tower.clone() {
                            self.add_tower(tower, self.cursor_position).unwrap();

                            self.ui_mode = UiMode::Normal;
                            self.selected_tower = None;
                        } else {
                            self.remove_tower(self.cursor_position);
                            self.ui_mode = UiMode::Normal;
                        }
                    }
                    UiMode::TowerUpgrade => {
                        if let Some(upgrade_menu) = &self.upgrade_menu {
                            // Vérifier si l'option Annuler est sélectionnée (dernière option)
                            if upgrade_menu.selected_upgrade
                                >= upgrade_menu.available_upgrades.len()
                            {
                                // Annuler et retourner au mode normal
                                self.upgrade_menu = None;
                                self.ui_mode = UiMode::Normal;
                                return;
                            }

                            // Appliquer l'amélioration choisie
                            self.apply_upgrade();
                        }
                    }
                }
            }
            View::MainMenu => {
                match self.selected_index {
                    0 => {
                        // Passer à la sélection de la carte
                        self.set_view(View::MapSelection);
                    }
                    1 => {
                        self.quit();
                    }
                    _ => {}
                }
            }
            View::MapSelection => {
                if self.selected_index < self.available_maps.len() {
                    let selected_map = self.available_maps[self.selected_index].clone();

                    self.selected_map = Some(selected_map.clone());
                    self.game = Game::new(self.mediator.clone(), vec![], 10, 1.0);
                    self.game.current_map = Some(selected_map);

                    self.set_view(View::Game);
                }
            }
            View::Pause => match self.selected_index {
                0 => self.set_view(View::Game),
                1 => self.quit(),
                _ => {}
            },
            View::GameOver => match self.selected_index {
                0 => self.set_view(View::MapSelection),
                1 => self.quit(),
                _ => {}
            },
        }
    }

    pub fn reset_game(&mut self) {
        let map = ForestMap::new();
        let n = 10;

        let mut selected_monsters = Vec::new();
        for _ in 0..n {
            if let Some(monster) = map.monsters.choose(&mut rng()) {
                selected_monsters.push(monster.clone());
            }
        }

        self.game = Game::new(self.mediator.clone(), vec![], 10, 1.0);
        self.game.current_map = Some(map);

        self.ui_mode = UiMode::Normal;
        self.selected_index = 0;
        self.selected_tower = None;
    }

    pub fn add_tower(&mut self, tower: Tower, position: Position) -> Result<(), String> {
        if !self.game.has_enough_money(tower.cost) {
            return Err("Pas assez d'argent".to_string());
        }

        if !self.is_position_valid(&position) {
            return Err("Position invalide".to_string());
        }

        if self.game.spend_money(tower.cost) {
            let mut new_tower = tower.clone();
            new_tower.position = position;
            self.game.towers.push(new_tower);

            self.game.add_log(format!(
                "{} placed at [{}, {}]",
                tower.name, position.x, position.y
            ));

            Ok(())
        } else {
            Err("Erreur lors de la construction".to_string())
        }
    }

    pub fn remove_tower(&mut self, position: Position) {
        self.game.remove_tower(position);
    }

    pub fn upgrade_tower(&mut self, index: usize, keep_selection: Option<usize>) {
        if index >= self.game.towers.len() {
            self.game
                .add_log("❌ Tour non trouvée pour l'amélioration.".to_string());
            return;
        }

        let selected_upgrade = if let Some(current_menu) = &self.upgrade_menu {
            keep_selection.unwrap_or(current_menu.selected_upgrade)
        } else {
            keep_selection.unwrap_or(0)
        };

        let tower = &self.game.towers[index].clone();
        self.game
            .add_log(format!("🔍 Tour {} (Niveau {})", tower.name, tower.level));

        let tower = &self.game.towers[index];
        let mut upgrades = vec![];

        let upgradeable_stats = tower
            .stats
            .iter()
            .filter(|stat| stat.upgrade.is_some())
            .collect::<Vec<_>>();

        for element in upgradeable_stats {
            if let Ok(format) = element.upgrade.as_ref().unwrap().format(element) {
                upgrades.push((element.stat_type.clone(), format));
            }
        }

        self.upgrade_menu = Some(UpgradeMenu {
            tower_index: index,
            selected_upgrade,
            available_upgrades: upgrades,
        });

        self.ui_mode = UiMode::TowerUpgrade;
    }

    pub fn apply_upgrade(&mut self) {
        if let Some(upgrade_menu) = &self.upgrade_menu {
            let tower_index = upgrade_menu.tower_index;
            let current_selection = upgrade_menu.selected_upgrade;

            if current_selection < upgrade_menu.available_upgrades.len() {
                let (upgrade_type, _) = &upgrade_menu.available_upgrades[current_selection];

                let tower = &self.game.towers[tower_index];
                let cost = tower.upgrade_cost_for_attribute(upgrade_type.clone());

                if cost.is_none() {
                    self.game
                        .add_log(format!("❌ Cette amélioration est déjà au niveau maximum."));
                    return;
                }

                if let Ok(_) = self.game.upgrade_tower(tower_index, upgrade_type.clone()) {
                    if tower_index < self.game.towers.len() {
                        self.upgrade_tower(tower_index, Some(current_selection));
                        return;
                    }
                }
            }
        }

        self.upgrade_menu = None;
        self.ui_mode = UiMode::Normal;
    }

    pub fn next_upgrade_option(&mut self) {
        if let Some(upgrade_menu) = &mut self.upgrade_menu {
            let options_count = upgrade_menu.available_upgrades.len();
            if options_count > 0 {
                upgrade_menu.selected_upgrade = (upgrade_menu.selected_upgrade + 1) % options_count;
            }
        }
    }

    pub fn previous_upgrade_option(&mut self) {
        if let Some(upgrade_menu) = &mut self.upgrade_menu {
            let options_count = upgrade_menu.available_upgrades.len();
            if options_count > 0 {
                upgrade_menu.selected_upgrade =
                    (upgrade_menu.selected_upgrade + options_count - 1) % options_count;
            }
        }
    }

    pub fn cancel_upgrade(&mut self) {
        self.upgrade_menu = None;
        self.ui_mode = UiMode::Normal;
    }

    pub fn is_tower_selection_on_map(&self) -> bool {
        self.tower_selection_on_map
    }

    pub fn start_tower_selection_on_map(&mut self) {
        if self.game.towers.is_empty() {
            self.game
                .add_log("Aucune tour à sélectionner sur la carte.".to_string());
            return;
        }

        self.ui_mode = UiMode::TowerSelection;
        self.tower_selection_on_map = true;

        self.selected_tower_index = Some(0);
        if let Some(index) = self.selected_tower_index {
            if index < self.game.towers.len() {
                let tower = &self.game.towers[index];
                self.cursor_position = tower.position;

                let tower_type = tower.name.clone();
                self.game.add_log(format!(
                    "🔍 Tour {} (Niveau {}) sélectionnée",
                    tower_type, tower.level
                ));
            }
        }
    }

    pub fn select_tower_on_map_up(&mut self) {
        if let Some(current_index) = self.selected_tower_index {
            let current_pos = self.game.towers[current_index].position;

            let mut closest_tower_index = None;
            let mut min_distance = f32::MAX;

            for (i, tower) in self.game.towers.iter().enumerate() {
                let pos = tower.position;
                if pos.y < current_pos.y {
                    let dx = (pos.x - current_pos.x) as f32;
                    let dy = (pos.y - current_pos.y) as f32;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < min_distance {
                        min_distance = distance;
                        closest_tower_index = Some(i);
                    }
                }
            }

            if let Some(index) = closest_tower_index {
                self.selected_tower_index = Some(index);
                self.cursor_position = self.game.towers[index].position;

                let tower = &self.game.towers[index];
                let tower_type = tower.name.clone();

                self.game.add_log(format!(
                    "🔍 Tour {} (Niveau {}) sélectionnée",
                    tower_type, tower.level
                ));
            }
        }
    }

    pub fn select_tower_on_map_down(&mut self) {
        if let Some(current_index) = self.selected_tower_index {
            let current_pos = self.game.towers[current_index].position;

            let mut closest_tower_index = None;
            let mut min_distance = f32::MAX;

            for (i, tower) in self.game.towers.iter().enumerate() {
                let pos = tower.position;
                if pos.y > current_pos.y {
                    let dx = (pos.x - current_pos.x) as f32;
                    let dy = (pos.y - current_pos.y) as f32;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < min_distance {
                        min_distance = distance;
                        closest_tower_index = Some(i);
                    }
                }
            }

            if let Some(index) = closest_tower_index {
                self.selected_tower_index = Some(index);
                self.cursor_position = self.game.towers[index].position;

                let tower = &self.game.towers[index];
                let tower_type = tower.name.clone();

                self.game.add_log(format!(
                    "🔍 Tour {} (Niveau {}) sélectionnée",
                    tower_type, tower.level
                ));
            }
        }
    }

    pub fn select_tower_on_map_left(&mut self) {
        if let Some(current_index) = self.selected_tower_index {
            let current_pos = self.game.towers[current_index].position;

            let mut closest_tower_index = None;
            let mut min_distance = f32::MAX;

            for (i, tower) in self.game.towers.iter().enumerate() {
                let pos = tower.position;
                if pos.x < current_pos.x {
                    let dx = (pos.x - current_pos.x) as f32;
                    let dy = (pos.y - current_pos.y) as f32;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < min_distance {
                        min_distance = distance;
                        closest_tower_index = Some(i);
                    }
                }
            }

            if let Some(index) = closest_tower_index {
                self.selected_tower_index = Some(index);
                self.cursor_position = self.game.towers[index].position;

                let tower = &self.game.towers[index];
                let tower_type = tower.name.clone();

                self.game.add_log(format!(
                    "🔍 Tour {} (Niveau {}) sélectionnée",
                    tower_type, tower.level
                ));
            }
        }
    }

    pub fn select_tower_on_map_right(&mut self) {
        if let Some(current_index) = self.selected_tower_index {
            let current_pos = self.game.towers[current_index].position;

            let mut closest_tower_index = None;
            let mut min_distance = f32::MAX;

            for (i, tower) in self.game.towers.iter().enumerate() {
                let pos = tower.position;
                if pos.x > current_pos.x {
                    let dx = (pos.x - current_pos.x) as f32;
                    let dy = (pos.y - current_pos.y) as f32;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < min_distance {
                        min_distance = distance;
                        closest_tower_index = Some(i);
                    }
                }
            }

            if let Some(index) = closest_tower_index {
                self.selected_tower_index = Some(index);
                self.cursor_position = self.game.towers[index].position;

                let tower = &self.game.towers[index];
                let tower_type = tower.name.clone();

                self.game.add_log(format!(
                    "🔍 Tour {} (Niveau {}) sélectionnée",
                    tower_type, tower.level
                ));
            }
        }
    }

    fn is_position_valid(&self, position: &Position) -> bool {
        if let Some(map) = &self.game.current_map {
            if map.is_position_on_path(position) {
                return false;
            }
        }

        let tower = self
            .game
            .towers
            .iter()
            .any(|t| t.position.x == position.x && t.position.y == position.y);

        if tower {
            return false;
        }

        true
    }
}
