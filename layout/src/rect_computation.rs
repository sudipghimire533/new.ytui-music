use crate::rect::Rect;
use layout_config::direction::Direction;
use layout_config::identifier::Identifier;
use layout_config::item::ItemTree;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum NoSiblingRect {
    IsOrphan,
    IsEldestChild,
}

fn get_sibling_rect<'a, 'b, 's>(
    me: &'b ItemTree,
    size_map: &'s mut HashMap<Identifier, Rect>,
    terminal_rect: &'a Rect,
) -> Result<&'s Rect, NoSiblingRect> {
    let parent = me.parent.as_ref().ok_or(NoSiblingRect::IsOrphan)?;

    let immidiate_elder_sibling = parent
        .childs
        .iter()
        .take_while(|c| c.item.identifier.ne(&me.item.identifier))
        .last()
        .ok_or(NoSiblingRect::IsEldestChild)?;

    if !size_map.contains_key(&immidiate_elder_sibling.item.identifier) {
        compute_rect(immidiate_elder_sibling, size_map, terminal_rect, false)
    }

    let sibling_rect = size_map
        .get(&immidiate_elder_sibling.item.identifier)
        .expect("Just computed sibling rect, must be there");

    Ok(sibling_rect)
}

fn i_can_start_from(
    me: &ItemTree,
    size_map: &mut HashMap<Identifier, Rect>,
    terminal_rect: &Rect,
) -> (u16, u16) {
    match get_sibling_rect(me, size_map, terminal_rect) {
        Err(NoSiblingRect::IsOrphan) => (terminal_rect.y, terminal_rect.x),

        Ok(sibling_rect) => (
            sibling_rect.y + sibling_rect.height,
            sibling_rect.x + sibling_rect.width,
        ),

        Err(NoSiblingRect::IsEldestChild) => {
            let parent = me
                .parent
                .as_ref()
                .expect("Eldest child always have a parent");
            if !size_map.contains_key(&parent.item.identifier) {
                compute_rect(parent, size_map, terminal_rect, false);
            }
            let parent_rect = size_map
                .get(&parent.item.identifier)
                .expect("Just computed rect for parent. Must be there");

            (parent_rect.y, parent_rect.x)
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

                    let net_sibling_height = get_sibling_rect(me, size_map, terminal_rect)
                        .map(|s| s.height + s.y - parent_rect.y)
                        .unwrap_or_default();
                    final_rect.height = me
                        .item
                        .size
                        .get_absolute(parent_rect.height, net_sibling_height);
                }
                Direction::Horizontal => {
                    final_rect.x = my_starting.1;

                    let net_sibling_width = get_sibling_rect(me, size_map, terminal_rect)
                        .map(|s| s.width + s.x - parent_rect.x)
                        .unwrap_or_default();
                    final_rect.width = me
                        .item
                        .size
                        .get_absolute(parent_rect.width, net_sibling_width);
                }
            }

            final_rect
        }
    };

    // Record this final rect
    size_map.insert(me.item.identifier.clone(), final_rect);

    me.childs
        .iter()
        .take_while(|_| compute_for_child)
        .for_each(|child| {
            compute_rect(child, size_map, terminal_rect, compute_for_child);
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use layout_config::direction::Direction;
    use layout_config::identifier::Identifier;
    use layout_config::item::Item;
    use layout_config::length::Length;
    use layout_config::ui::UI;

    fn ensure_boundry_check(item_root: ItemTree, filled_size_map: &HashMap<Identifier, Rect>) {
        let tree_as_vec: Vec<_> = item_root.try_into().unwrap();
        tree_as_vec.iter().for_each(|item| {
            let (my_height, my_width) = filled_size_map
                .get(&item.identifier)
                .map(|r| (r.height, r.width))
                .unwrap();
            let (net_child_height, net_child_width) = item
                .childs
                .iter()
                .filter(|c| matches!(c, Identifier::Custom(..)))
                .fold((0, 0), |acc, c| {
                    let child_rect = filled_size_map.get(c).unwrap();
                    (acc.0 + child_rect.height, acc.1 + child_rect.width)
                });
            match item.split {
                Direction::Vertical => {
                    assert!(my_height >= net_child_height);
                }
                Direction::Horizontal => {
                    assert!(my_width >= net_child_width);
                }
            }
        })
    }

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
        let red_el_custom = Rect { x: 0, y: 0, height: 5, width: 150 };
        assert_eq!(
            Some(&red_el_custom),
            size_map.get(&Identifier::Custom("red_element_custom".into())),
        );

        #[rustfmt::skip]
        let top_area = Rect { x: 0, y: 5, height: 28, width: 150 };
        assert_eq!(Some(&top_area), size_map.get(&Custom("bottom_area".into())));

        #[rustfmt::skip]
        let top_left = Rect { x: 0, y: 5, height: 28, width: 75 };
        assert_eq!(
            Some(&top_left),
            size_map.get(&Identifier::Custom("bottom_left".into()))
        );

        #[rustfmt::skip]
        let top_right = Rect { x: 75, y: 5, height: 28, width: 75 };
        assert_eq!(
            Some(&top_right),
            size_map.get(&Identifier::Custom("bottom_right".into()))
        );

        #[rustfmt::skip]
        let green_container = Rect { x: 75, y: 5, height: 9, width: 75 };
        assert_eq!(
            Some(&green_container),
            size_map.get(&Identifier::Custom("green_container".into()))
        );

        #[rustfmt::skip]
        let yellow_container = Rect { y: 14, ..green_container };
        assert_eq!(
            Some(&yellow_container),
            size_map.get(&Identifier::Custom("yellow_container".into()))
        );

        #[rustfmt::skip]
        let blue_container = Rect { y: 23, height: 10, ..yellow_container };
        assert_eq!(
            Some(&blue_container),
            size_map.get(&Identifier::Custom("blue_container".into()))
        );

        ensure_boundry_check(ui.item_root.clone(), &size_map);
    }

    #[test]
    fn start_for_root() {
        let mut size_map = HashMap::new();

        let root_item = Item {
            identifier: Identifier::Custom("root".to_string()),
            childs: vec![Identifier::Reserved("gadget".into())],
            split: Direction::Vertical,
            size: Length::Relative(100),
        };
        let first_child = Item {
            identifier: Identifier::Custom("first_child".into()),
            childs: vec![Identifier::Reserved("gadget2".into())],
            split: Direction::Vertical,
            size: Length::Relative(50),
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
                // root size will always be overriden as it was Length::Relative(100)
                size: Length::Relative(50),
                ..root_item
            },
            first_child,
            second_child,
        ]
        .try_into()
        .unwrap();

        assert_eq!(
            (16, 150),
            i_can_start_from(&item_tree.childs[1], &mut size_map, &TERMINAL_RECT),
        );
    }
}
