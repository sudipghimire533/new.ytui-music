use crate::styles::Style;

pub struct PanePreferences {
    /// How many number of list item to show
    /// when we are fetching result from local source
    pub local_result_count: u8,
    /// How many number of item in list to show
    /// when fetching result from remote source
    pub remote_result_count: u8,
    /// base style for list item
    pub base_style: Style,
    /// highlight style for list item
    pub highlight_style: Style,
}
