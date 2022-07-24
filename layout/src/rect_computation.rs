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
    match me.parent.as_ref() {
        None => (terminal_rect.x, terminal_rect.y),

        Some(parent) => {
            if !size_map.contains_key(&parent.item.identifier) {
                compute_rect(parent, size_map, terminal_rect, false);
            }
            let parent_rect = size_map
                .get(&parent.item.identifier)
                .expect("Just computed parent rect, must be there")
                .clone();

            parent
                .childs
                .iter()
                .take_while(|c| c.item.identifier != me.item.identifier)
                .last()
                .map(|s| {
                    if !size_map.contains_key(&s.item.identifier) {
                        compute_rect(s, size_map, terminal_rect, false);
                    }
                    let sibling_rect = size_map
                        .get(&s.item.identifier)
                        .expect("Just computed sibling rect, must be there");

                    (
                        sibling_rect.y + sibling_rect.height,
                        sibling_rect.x + sibling_rect.width,
                    )
                })
                .unwrap_or((parent_rect.y, parent_rect.x))
        }
    }
}

pub fn compute_rect_for_item_tree(
    me: &ItemTree,
    size_map: &mut HashMap<Identifier, Rect>,
    terminal_rect: &Rect,
) {
    compute_rect(me, size_map, terminal_rect, true)
}

fn compute_rect(
    me: &ItemTree,
    size_map: &mut HashMap<Identifier, Rect>,
    terminal_rect: &Rect,
    compute_for_child: bool,
) {
    let final_rect: Rect = match me.parent.as_ref() {
        // for root element it always fill the terminal_rect
        // this means that
        // for root element, provided size is ignored
        //
        // if it is intended to limit the total layout
        // use window's height & width property instead
        None => terminal_rect.clone(),

        Some(parent) => {
            if !size_map.contains_key(&parent.item.identifier) {
                compute_rect(parent, size_map, terminal_rect, false);
            }
            let parent_rect = size_map
                .get(&parent.item.identifier)
                .expect("Parent rect was computed just here..")
                .clone();

            let mut final_rect = parent_rect.clone();
            let my_starting = i_can_start_from(me, size_map, terminal_rect);
            match parent.item.split {
                Direction::Vertical => {
                    final_rect.y = my_starting.0;
                    final_rect.height = me.item.size.get_appliable_size(parent_rect.height);
                }
                Direction::Horizontal => {
                    final_rect.x = my_starting.1;
                    final_rect.width = me.item.size.get_appliable_size(parent_rect.width);
                }
            }

            final_rect
        }
    };

    // Record this final rect
    size_map.insert(me.item.identifier.clone(), final_rect);

    // Now also compute for all child
    if compute_for_child {
        for child in me.childs.iter() {
            compute_rect(child, size_map, terminal_rect, true)
        }
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
    use layout_config::ui::UI;

    #[cfg(test)]
    const TERMINAL_RECT: Rect = Rect {
        x: 0,
        y: 0,
        height: 33,
        width: 150,
    };

    #[test]
    fn test_rect_for_layout_file() {
        use Identifier::Custom;
        let config_file_str = include_str!("../../layout-config/layout.json");
        let ui: UI = serde_json::from_str(config_file_str).unwrap();
        let mut size_map = HashMap::new();

        compute_rect_for_item_tree(&ui.item_root, &mut size_map, &TERMINAL_RECT);

        #[rustfmt::skip]
        let root = Rect { x: 0, y: 0, height: 33, width: 150 };
        assert_eq!(
            Some(&root),
            size_map.get(&Custom("things_starts_from_me".into()))
        );

        #[rustfmt::skip]
        let top_area = Rect { x: 0, y: 0, height: 16, width: 150 };
        assert_eq!(Some(&top_area), size_map.get(&Custom("top_area".into())));

        #[rustfmt::skip]
        let red_el_custom = Rect { x: 0, y: 16, height: 5, width: 150 };
        assert_eq!(
            Some(&red_el_custom),
            size_map.get(&Identifier::Custom("red_element_custom".into())),
        );

        #[rustfmt::skip]
        let top_left = Rect { x: 0, y: 0, height: 16, width: 75 };
        assert_eq!(
            Some(&top_left),
            size_map.get(&Identifier::Custom("top_left".into()))
        );

        #[rustfmt::skip]
        let top_right = Rect { x: 75, y: 0, height: 16, width: 75 };
        assert_eq!(
            Some(&top_right),
            size_map.get(&Identifier::Custom("top_right".into()))
        );

        #[rustfmt::skip]
        let green_container = Rect { x: 75, y: 0, height: 5, width: 75 };
        assert_eq!(
            Some(&green_container),
            size_map.get(&Identifier::Custom("green_container".into()))
        );

        #[rustfmt::skip]
        let yellow_container = Rect { y: 5, ..green_container };
        assert_eq!(
            Some(&yellow_container),
            size_map.get(&Identifier::Custom("yellow_container".into()))
        );

        #[rustfmt::skip]
        let blue_container = Rect { y: 10, ..yellow_container };
        assert_eq!(
            Some(&blue_container),
            size_map.get(&Identifier::Custom("blue_container".into()))
        );
    }

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
            (16, 150),
            i_can_start_from(&item_tree.childs[1], &mut size_map, &TERMINAL_RECT),
        );
    }
}
