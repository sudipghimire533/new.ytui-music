pub mod extend_config;
pub mod rect;
pub mod rect_computation;

use layout_config::ui::UI;
use rect::Rect;
use rect_computation::compute_rect_for_item_tree;
use std::collections::HashMap;

fn get_terminal_height_width() -> (u16, u16) {
    let stty_output = std::process::Command::new("/usr/bin/stty")
        .arg("size")
        .stdin(std::process::Stdio::inherit())
        .output()
        .unwrap()
        .stdout;

    let stty_size_output = String::from_utf8(stty_output).unwrap();
    let mut division = stty_size_output.split_ascii_whitespace();

    let height = division.next().unwrap().parse::<u16>().unwrap();
    let width = division.next().unwrap().parse::<u16>().unwrap();

    (height, width)
}

fn main() {
    let (terminal_height, terminal_width) = get_terminal_height_width();
    let terminal_rect = Rect {
        x: 0,
        y: 0,
        height: terminal_height,
        width: terminal_width,
    };

    println!("Working on terminal: {terminal_rect:?}");

    let layout_config_file = include_str!("../../layout-config/layout.json");
    let layout = serde_json::from_str::<UI>(layout_config_file).unwrap();

    println!("----------- Layout Tree -------------");
    println!("{}", layout.item_root);
    println!("--------------------------------------");

    let mut items_positions = HashMap::new();
    compute_rect_for_item_tree(&layout.item_root, &mut items_positions, &terminal_rect);

    println!("------------ Container rect -----------");
    println!("{:#?}", items_positions);
    println!("---------------------------------------");

    // TODO:
    // Only rect of containers have been computed so far
    // actual gadgets will fill their whole parent
}
