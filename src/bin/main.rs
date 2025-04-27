use rust_tower::{
    application::engine::maps::MapType,
    domain::entities::{game::Game, wave::Wave},
    infrastructure::ui::{
        app::{App, View},
        tui::Tui,
    },
};

fn main() -> color_eyre::Result<()> {
    // Initialiser color-eyre pour une gestion d'erreur améliorée
    color_eyre::install()?;

    // Créer le jeu sous-jacent
    let game = create_game();

    // Créer notre application TUI
    let mut app = App::new(game);
    app.current_view = View::MainMenu; // Commencer par le menu principal

    // Initialiser TUI
    let mut tui = Tui::new()?;
    tui.init()?;

    // Configurer le gestionnaire d'événements
    app.run(&mut tui)?;

    tui.exit()?;
    Ok(())
}

fn create_game() -> Game {
    // Créer un jeu par défaut avec la carte de la forêt
    // Cette carte sera remplacée lorsque l'utilisateur en choisira une dans le menu
    let map = MapType::Forest.create_map();
    let wave = Wave::new(None);

    Game::new(map, vec![wave], vec![], 10, 1.0)
}
