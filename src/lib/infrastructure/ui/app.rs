use crate::application::engine::{
    maps::forest::forest_map,
    towers::{BasicTower, FireBasicTower},
};
use crate::domain::entities::{game::Game, position::Position};
use crate::domain::entities::{tower::TowerType as GameTowerType, wave::Wave};
use color_eyre::Result;
use crossterm::event::KeyCode;
use rand::{rng, seq::IndexedRandom};

use super::{
    events::{
        event::{Event, EventConfig, Events},
        handlers::{
            handle_key_down, handle_key_esc, handle_key_left, handle_key_p, handle_key_q,
            handle_key_right, handle_key_up,
        },
    },
    tui::Tui,
    ui::render,
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

/// Types d'améliorations disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeType {
    AttackSpeed,
    Damage,
    Range,
}

/// Structure pour gérer le menu des améliorations
pub struct UpgradeMenu {
    pub tower_index: usize,
    pub selected_upgrade: usize,
    pub available_upgrades: Vec<(UpgradeType, &'static str)>,
}

impl UpgradeMenu {
    pub fn new(tower_index: usize) -> Self {
        Self {
            tower_index,
            selected_upgrade: 0,
            available_upgrades: vec![
                (UpgradeType::AttackSpeed, "Vitesse d'attaque"),
                (UpgradeType::Damage, "Dégâts"),
                (UpgradeType::Range, "Portée"),
            ],
        }
    }
}

/// Représente l'état global de l'application TUI
pub struct App {
    /// Indique si l'application doit continuer à s'exécuter
    pub running: bool,
    /// Le jeu sous-jacent
    pub game: Game,
    /// Vue courante de l'application
    pub current_view: View,
    /// Index sélectionné dans le menu actuel
    pub selected_index: usize,
    /// Liste des actions disponibles
    pub available_actions: Vec<GameAction>,
    /// Liste des types de tours disponibles
    pub available_towers: Vec<TowerType>,
    /// Mode d'interface actuel
    pub ui_mode: UiMode,
    /// Position du curseur sur la carte
    pub cursor_position: Position,
    /// Type de tour sélectionné
    pub selected_tower: Option<TowerType>,
    /// Menu d'amélioration des tours
    pub upgrade_menu: Option<UpgradeMenu>,
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
}

impl App {
    /// Crée une nouvelle instance de l'application avec le jeu fourni
    pub fn new(game: Game) -> Self {
        // Actions par défaut
        let actions = vec![
            GameAction::BuildTower,
            GameAction::RemoveTower,
            GameAction::UpgradeTower,
        ];

        // Tours disponibles
        let towers = vec![
            TowerType::Basic,
            TowerType::Fire,
            TowerType::Water,
            TowerType::Earth,
            TowerType::Air,
        ];

        Self {
            running: true,
            game,
            current_view: View::Game,
            selected_index: 0,
            available_actions: actions,
            available_towers: towers,
            ui_mode: UiMode::Normal,
            cursor_position: Position::new(5, 5),
            selected_tower: None,
            upgrade_menu: None,
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
            } else if self.game.waves.is_empty() && self.game.current_wave.is_none() {
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
            let new_x = (self.cursor_position.x + dx).max(0).min(20);
            let new_y = (self.cursor_position.y + dy).max(0).min(15);
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
            }
        }
    }

    /// Valide l'action sélectionnée
    pub fn confirm_selection(&mut self) {
        match self.current_view {
            View::Game => {
                match self.ui_mode {
                    UiMode::Normal => {
                        // En mode normal, sélectionner une action
                        let action = self.available_actions[self.selected_index];
                        match action {
                            GameAction::BuildTower => {
                                self.ui_mode = UiMode::TowerSelection;
                                self.selected_index = 0; // Réinitialiser l'index de sélection
                            }
                            GameAction::RemoveTower => {
                                self.ui_mode = UiMode::Placement;
                                self.selected_tower = None; // Pas de tour sélectionnée = mode suppression
                            }
                            GameAction::UpgradeTower => {
                                if !self.game.towers.is_empty() {
                                    self.ui_mode = UiMode::TowerUpgrade;
                                    self.selected_index = 0; // Réinitialiser l'index de sélection
                                } else {
                                    self.game.add_log("Aucune tour à améliorer.".to_string());
                                }
                            }
                        }
                    }
                    UiMode::TowerSelection => {
                        // En mode sélection de tour, choisir un type de tour
                        if self.selected_index < self.available_towers.len() {
                            let tower_type = self.available_towers[self.selected_index];
                            self.selected_tower = Some(tower_type);
                            self.ui_mode = UiMode::Placement; // Passer en mode placement
                        }
                    }
                    UiMode::Placement => {
                        // En mode placement, placer ou supprimer la tour
                        if let Some(tower_type) = self.selected_tower {
                            // Placer la tour selon son type
                            match tower_type {
                                TowerType::Basic => self.add_tower(self.cursor_position),
                                TowerType::Fire => self.add_fire_tower(self.cursor_position),
                                TowerType::Water => self.add_water_tower(self.cursor_position),
                                TowerType::Earth => self.add_earth_tower(self.cursor_position),
                                TowerType::Air => self.add_air_tower(self.cursor_position),
                            }
                        } else {
                            // Si pas de tour sélectionnée, supprimer la tour à cette position
                            self.remove_tower(self.cursor_position);
                        }

                        // Retourner au mode normal après le placement
                        self.ui_mode = UiMode::Normal;
                        self.selected_tower = None;
                    }
                    UiMode::TowerUpgrade => {
                        // Logique pour améliorer la tour sélectionnée
                        if let Some(upgrade_menu) = &self.upgrade_menu {
                            if upgrade_menu.selected_upgrade < upgrade_menu.available_upgrades.len()
                            {
                                // Appliquer l'amélioration sélectionnée
                                self.apply_upgrade();
                            } else {
                                // Annuler l'amélioration
                                self.cancel_upgrade();
                            }
                        } else {
                            // Si aucun menu d'amélioration n'est disponible, revenir au mode normal
                            self.ui_mode = UiMode::Normal;
                        }
                    }
                }
            }
            View::MainMenu => match self.selected_index {
                0 => self.set_view(View::Game),
                1 => self.quit(),
                _ => {}
            },
            View::Pause => match self.selected_index {
                0 => self.set_view(View::Game),
                1 => self.quit(),
                _ => {}
            },
            View::GameOver => {
                match self.selected_index {
                    0 => {
                        // Créer un nouveau jeu
                        self.reset_game();
                        self.set_view(View::Game);
                    }
                    1 => self.quit(),
                    _ => {}
                }
            }
        }
    }

    /// Crée un nouveau jeu (réinitialise le jeu actuel)
    pub fn reset_game(&mut self) {
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

        let wave = Wave::new(Some(selected_monsters));

        let towers = vec![
            GameTowerType::Basic(BasicTower::positioned(Position::new(5, 2))),
            GameTowerType::Fire(FireBasicTower::positioned(Position::new(6, 6))),
            GameTowerType::Fire(FireBasicTower::positioned(Position::new(14, 6))),
        ];

        // Remplacer le jeu actuel par un nouveau
        self.game = Game::new(map, vec![wave], towers, 10, 1.0);

        // Réinitialiser les états d'interface
        self.ui_mode = UiMode::Normal;
        self.selected_index = 0;
        self.selected_tower = None;
    }

    // Méthodes pour ajouter différents types de tours
    pub fn add_tower(&mut self, position: Position) {
        if self.game.has_enough_money(TowerType::Basic.cost()) {
            if self.game.spend_money(TowerType::Basic.cost()) {
                self.game.add_tower(position);
            }
        }
    }

    pub fn add_fire_tower(&mut self, position: Position) {
        if self.game.has_enough_money(TowerType::Fire.cost()) {
            if self.game.spend_money(TowerType::Fire.cost()) {
                self.game.add_fire_tower(position);
            }
        }
    }

    pub fn add_water_tower(&mut self, position: Position) {
        if self.game.has_enough_money(TowerType::Water.cost()) {
            if self.game.spend_money(TowerType::Water.cost()) {
                // Implémenter la création de tour d'eau
                self.game.add_log(format!(
                    "Tour d'eau placée en ({},{})",
                    position.x, position.y
                ));
            }
        }
    }

    pub fn add_earth_tower(&mut self, position: Position) {
        if self.game.has_enough_money(TowerType::Earth.cost()) {
            if self.game.spend_money(TowerType::Earth.cost()) {
                // Implémenter la création de tour de terre
                self.game.add_log(format!(
                    "Tour de terre placée en ({},{})",
                    position.x, position.y
                ));
            }
        }
    }

    pub fn add_air_tower(&mut self, position: Position) {
        if self.game.has_enough_money(TowerType::Air.cost()) {
            if self.game.spend_money(TowerType::Air.cost()) {
                // Implémenter la création de tour d'air
                self.game.add_log(format!(
                    "Tour d'air placée en ({},{})",
                    position.x, position.y
                ));
            }
        }
    }

    pub fn remove_tower(&mut self, position: Position) {
        self.game.remove_tower(position);
    }

    pub fn upgrade_tower(&mut self, index: usize) {
        if index >= self.game.towers.len() {
            self.game
                .add_log("❌ Tour non trouvée pour l'amélioration.".to_string());
            return;
        }

        // Créer un menu d'amélioration pour cette tour
        self.upgrade_menu = Some(UpgradeMenu::new(index));

        // Afficher les informations sur la tour
        let tower_type = self.game.towers[index].tower_type_name();
        let tower_level = self.game.towers[index].upgrade_level();
        let upgrade_cost = self.game.towers[index].upgrade_cost();

        self.game
            .add_log(format!("🔍 Tour {} (Niveau {})", tower_type, tower_level));
        self.game
            .add_log(format!("💰 Coût d'amélioration: {} pièces", upgrade_cost));

        // Passer en mode amélioration spécifique
        self.ui_mode = UiMode::TowerUpgrade;
    }

    pub fn apply_upgrade(&mut self) {
        if let Some(upgrade_menu) = &self.upgrade_menu {
            let tower_index = upgrade_menu.tower_index;

            if upgrade_menu.selected_upgrade < upgrade_menu.available_upgrades.len() {
                let (upgrade_type, _) =
                    upgrade_menu.available_upgrades[upgrade_menu.selected_upgrade];

                // Appliquer l'amélioration choisie
                match upgrade_type {
                    UpgradeType::AttackSpeed => {
                        self.game.upgrade_tower_attack_speed(tower_index);
                    }
                    UpgradeType::Damage => {
                        self.game.upgrade_tower_damage(tower_index);
                    }
                    UpgradeType::Range => {
                        self.game.upgrade_tower_range(tower_index);
                    }
                }
            }
        }

        // Nettoyer après l'amélioration
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
}
