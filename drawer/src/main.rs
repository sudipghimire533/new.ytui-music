use gadgets::ui::draw_all_ui;
use std::collections::HashMap;
use std::error::Error;
use user_config::reexports::compute_rect_for_item_tree as compute_rect;
use user_config::Config;

pub mod gadgets;
pub mod init;
pub mod types;

use types::{state::AppState, utils};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = init::config::get_config()
        .map_err(|e| format!("Unable to get user configuration: {e:?}"))?;

    // read config, show option for user to generate new if not previously exists
    // have sub command to check config and to upate the binary
    // check screen size and show feedback
    // after eveything is correct then only proceed
    let (mut terminal, rollback_terminal) = init::terminal::setup_terminal()
        .map_err(|e| format!("While setting up terminal: {e:#?}"))?;

    // Wait if terminal can be rolled back
    // before reporting app error
    let app_res =
        run_app(&mut terminal, config).map_err(|e| format!("Application exit with error: {e:#?}"));

    let rollback_res = rollback_terminal(&mut terminal)
        .map_err(|e| format!("While doing rollback terminal: {e:#?}"));

    // Prioritiy of app_res.error > rollback_res.error
    app_res?;
    rollback_res?;

    Ok(())
}

fn run_app<B: tui::backend::Backend>(
    terminal: &mut tui::terminal::Terminal<B>,
    config: Config,
) -> Result<(), Box<dyn Error>> {
    let terminal_rect = utils::into_my_rect(terminal.size()?);
    let Config { layout, theme, .. } = config;

    let appstate = AppState::default();
    let mut rect_map = HashMap::new();
    compute_rect(&layout.item_root, &mut rect_map, &terminal_rect);

    let geometrics = utils::consume_and_get_geometry(&mut rect_map)
        .map_err(|e| format!("While creating geometry from Rect map: {e:#?}"))?;

    draw_all_ui(&mut terminal.get_frame(), &appstate, &theme, geometrics);

    Ok(())
}
