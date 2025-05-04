use std::sync::Arc;

use rust_tower::{
    application::engine::towers::{
        fire_tower::FireTower, mine_tower::MineTower, sentinel_tower::SentinelTower,
    },
    domain::{entities::position::Position, mediator::Mediator},
    infrastructure::ui::{app::App, notifications::NotifierAdapter, tui::Tui},
};

fn main() -> color_eyre::Result<()> {
    let notifier = NotifierAdapter::new();
    let mediator = Arc::new(Mediator::new(notifier));

    color_eyre::install()?;

    let mut app = App::new(
        mediator,
        vec![
            FireTower::positionned(Position::new(0, 0)),
            SentinelTower::positionned(Position::new(0, 0)),
            MineTower::positionned(Position::new(0, 0)),
        ],
    );

    let mut tui = Tui::new()?;
    tui.init()?;

    app.run(&mut tui)?;
    tui.exit()?;

    Ok(())
}
