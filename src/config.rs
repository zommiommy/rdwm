
pub const BORDER_PX: usize = 1;
pub const SNAP_PX: usize = 32;
pub const SHOW_BAR: bool = True;

mod bar;
use bar::BarType;
pub const BAR: BarType = BarType::TopBar;

pub const FONTS: Vec<String> = vec!["monospace:size=10"];
pub const DMENU_FONT: String = "monospace:size=10";

mod colors;
use colors::Colors;
pub const COLORS: Vec<(Color, Color, Color)> =