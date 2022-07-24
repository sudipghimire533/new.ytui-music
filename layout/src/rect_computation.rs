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
    let previous_sibling_rect = me
        .parent
        .as_ref()
        .map(|p| {
            let previous_sibling = p
                .childs
                .iter()
                .take_while(|c| c.item.identifier != me.item.identifier)
                .last()?;
            let previous_siblig_identifer = &previous_sibling.item.identifier;

            if !size_map.contains_key(previous_siblig_identifer) {
                compute_rect_for_item_tree(previous_sibling, size_map, terminal_rect);
            }
            let rect = size_map
                .get(&p.item.identifier)
                .expect("Just inserted sibling rect. Should have been existed");

            Some(rect)
        })
        .flatten();

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
                    if !size_map.contains_key(&p.item.identifier) {
                        compute_rect_for_item_tree(p, size_map, terminal_rect);
                    }
                    size_map
                        .get(&p.item.identifier)
                        .expect("Just inserted sibling rect. Should have been existed")
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

#[cfg(test)]
mod tests {
    use super::*;
    use layout_config::direction::Direction;
    use layout_config::identifier::Identifier;
    use layout_config::item::Item;
    use layout_config::length::Length;
    use layout_config::size::Size;

    #[cfg(test)]
    const TERMINAL_RECT: Rect = Rect {
        x: 0,
        y: 0,
        height: 33,
        width: 150,
    };

    #[test]
    fn start_for_root() {
        let mut size_map = HashMap::new();
        let root_item = Item {
            identifier: Identifier::Custom("root".to_string()),
            childs: vec![Identifier::Reserved("gadget".into())],
            split: Direction::Vertical,
            size: Size {
                maximum: Length::Relative(100),
                minimum: Length::Absolute(0),
                preferred: Length::Relative(100),
            },
        };
        let first_child = Item {
            identifier: Identifier::Custom("first_child".into()),
            childs: vec![Identifier::Reserved("gadget2".into())],
            split: Direction::Vertical,
            size: Size {
                minimum: Length::Absolute(0),
                maximum: Length::Relative(100),
                preferred: Length::Relative(50),
            },
        };
        let second_child = Item {
            identifier: Identifier::Custom("second_child".into()),
            ..first_child.clone()
        };

        let item_tree: ItemTree = vec![
            Item {
                childs: vec![
                    Identifier::Custom("first_child".into()),
                    Identifier::Custom("second_child".into()),
                ],
                size: Size {
                    minimum: Length::Relative(50),
                    preferred: Length::Relative(50),
                    maximum: Length::Relative(50),
                },
                ..root_item.clone()
            },
            first_child.clone(),
            second_child.clone(),
        ]
        .try_into()
        .unwrap();
        assert_eq!(
            (16, 0),
            i_can_start_from(&item_tree.childs[1], &mut size_map, &TERMINAL_RECT),
        );
    }
}
