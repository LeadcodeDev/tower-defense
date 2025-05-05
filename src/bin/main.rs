use std::sync::Arc;

use rust_tower::{
    application::engine::{
        maps::{cave::CaveMap, desert::DesertMap, forest::ForestMap},
        towers::{fire_tower::FireTower, mine_tower::MineTower, sentinel_tower::SentinelTower},
    },
    domain::{
        entities::position::Position, mediator::Mediator, services::notifications::NotifierAdapter,
    },
    infrastructure::ui::{app::App, tui::Tui},
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
        vec![ForestMap::new(), DesertMap::new(), CaveMap::new()],
    );

    let mut tui = Tui::new()?;
    tui.init()?;

    app.run(&mut tui)?;
    tui.exit()?;

    Ok(())
}
