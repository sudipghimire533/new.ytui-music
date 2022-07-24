use crate::extend_config::ExtendSize;
use crate::rect::Rect;
use layout_config::direction::Direction;
use layout_config::identifier::Identifier;
use layout_config::item::ItemTree;
use std::collections::HashMap;

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

pub fn compute_rect_for_item_tree(
    me: &ItemTree,
    size_map: &mut HashMap<Identifier, Rect>,
    terminal_rect: &Rect,
) {
    let parent_rect: &Rect = match &me.parent {
        Some(parent) => {
            // if parent rect is not present yet,
            // we compute it
            if !size_map.contains_key(&parent.item.identifier) {
                compute_rect_for_item_tree(parent, size_map, terminal_rect);
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
        compute_rect_for_item_tree(child, size_map, terminal_rect)
    }
}
