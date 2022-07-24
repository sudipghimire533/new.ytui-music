pub mod extend_config;
pub mod rect;

use extend_config::ExtendSize;
use layout_config::direction::Direction;
use layout_config::item::ItemTree;
use layout_config::{identifier::Identifier, ui::UI};
use rect::Rect;
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
    compute_rect(&layout.item_root, &mut items_positions, &terminal_rect);

    println!("------------ Container rect -----------");
    println!("{:#?}", items_positions);
    println!("---------------------------------------");

    // TODO:
    // Only rect of containers have been computed so far
    // actual gadgets will fill their whole parent
}

fn i_can_start_from(
    me: &ItemTree,
    size_map: &mut HashMap<Identifier, Rect>,
    terminal_rect: &Rect,
) -> (u16, u16) {
    let previous_sibling_rect = me.parent.as_ref().map(|p| {
        size_map
            .get(&p.item.identifier)
            .expect("Expected that previous sibling rect have been computed")
    });

    match previous_sibling_rect {
        // Can start from where previous sibling started
        // but leaving sibling's area
        Some(sib_rect) => {
            let starting_height = sib_rect.y + sib_rect.height;
            let starting_width = sib_rect.x + sib_rect.width;
            (starting_height, starting_width)
        }

        // If no previous sibling rect is found
        // i.e this is the first child
        // it can start from where it's parent started
        None => {
            let parent_rect = me
                .parent
                .as_ref()
                .map(|p| {
                    size_map
                        .get(&p.item.identifier)
                        .expect("Expected parent rect to be computed already")
                })
                .unwrap_or(terminal_rect);

            (parent_rect.x, parent_rect.y)
        }
    }
}

fn compute_rect(me: &ItemTree, size_map: &mut HashMap<Identifier, Rect>, terminal_rect: &Rect) {
    let parent_rect: &Rect = match &me.parent {
        Some(parent) => {
            // if parent rect is not present yet,
            // we compute it
            if !size_map.contains_key(&parent.item.identifier) {
                compute_rect(parent, size_map, terminal_rect);
            }
            size_map
                .get(&parent.item.identifier)
                .expect("Just called compute_length. parent rect should be present")
        }

        // If this element do not have any parent
        // i.e this is root. We can treat terminal as parent
        None => terminal_rect,
    };

    let mut final_rect = parent_rect.clone();

    match me.item.split {
        Direction::Vertical => {
            final_rect.height = me.item.size.get_appliable_size(parent_rect.height);
        }
        Direction::Horizontal => {
            final_rect.width = me.item.size.get_appliable_size(parent_rect.width);
        }
    };

    let my_starting_point = i_can_start_from(me, size_map, terminal_rect);
    final_rect.y = my_starting_point.0;
    final_rect.x = my_starting_point.1;

    // Record this final rect
    size_map.insert(me.item.identifier.clone(), final_rect);

    // Now also compute for all child
    for child in me.childs.iter() {
        compute_rect(child, size_map, terminal_rect)
    }
}
