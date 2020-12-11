
pub struct Client {
    name: String,
    monitor: Monitor,
    window: Window

    min_aspect_ratio: f64,
    max_aspect_ratio: f64,
    
    x: usize,
    y: usize,
    
    width: usize,
    height: usize,
    
    base_width: usize,
    base_height: usize,
    
    increment_width: usize,
    increment_height: usize,
    
    max_width: usize,
    min_width: usize,
    
    max_height: usize,
    min_height: usize,

    border_width: usize,
    old_border_width: usize,

    is_fixed: bool,
    is_floating: bool,
    is_urgent: bool,
    never_focus: bool,
    old_state: bool,
    is_fullscreen: bool,    
}