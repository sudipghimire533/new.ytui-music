use gadgets::ui::draw_all_ui;
use std::collections::HashMap;
use std::error::Error;
use types::state::GeometryData;
use user_config::reexports::compute_rect_for_item_tree as compute_rect;
use user_config::Config;

pub mod event;
pub mod gadgets;
pub mod init;
pub mod types;

use event::listen_for_event;
use event::EventSummary;
use types::{state::AppState, utils};
use user_config::action::KeyboardAction;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = init::config::get_config(init::default_config_source)
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
    let Config {
        layout,
        theme,
        keyboard,
    } = config;

    let mut appstate = AppState::default();
    let mut rect_map = HashMap::new();

    let mut recompute_layout =
        |geometrics: &mut GeometryData, terminal_rect| -> Result<(), String> {
            compute_rect(&layout.item_root, &mut rect_map, &terminal_rect);
            *geometrics = utils::consume_and_get_geometry(&mut rect_map)
                .map_err(|e| format!("While creating geometry from Rect map: {e:#?}"))?;
            Ok(())
        };

    let mut geometrics = GeometryData::default();
    recompute_layout(&mut geometrics, utils::into_my_rect(terminal.size()?))?;

    'ui_renderer: loop {
        terminal.draw(|frame| draw_all_ui(frame, &appstate, &theme, &geometrics))?;
        let event_summary = listen_for_event(&keyboard, &appstate);

        match event_summary {
            EventSummary::Nothing => (),
            EventSummary::Ignored => (),
            EventSummary::Resize => {
                recompute_layout(&mut geometrics, utils::into_my_rect(terminal.size()?))?
            }
            EventSummary::Execution(action) if action == KeyboardAction::Quit => break 'ui_renderer,
            EventSummary::Execution(action) => event::handle_action(action, &mut appstate),
        }
    }

    Ok(())
}
