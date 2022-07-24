use layout_config::ui::UI;
pub mod extend_config;
pub mod rect;


fn main() {
    let layout_config_file = include_str!("../../layout-config/layout.json");
    let layout = serde_json::from_str::<UI>(layout_config_file).unwrap();

    println!("{}", layout.items);
}
