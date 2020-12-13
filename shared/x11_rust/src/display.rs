use super::*;

pub struct Display {
    display: *mut xlib::Display,
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe{
            xlib::XCloseDisplay(self.display);
        }
    }
}

impl Display {
    pub fn new(name: Option<String>) -> Display {
        let display = unsafe{
            match name {
                Some(name_string) => xlib::XOpenDisplay(name_string.as_str().as_ptr() as *const i8),
                None => xlib::XOpenDisplay(std::ptr::null()),
            }
        };

        Display {
            display,
        }
    }

    pub fn width(&self, screen_int: i32) -> i32 {
        unsafe { xlib::XDisplayWidth(self.display, screen_int) }
    }

    pub fn height(&self, screen_int: i32) -> i32 {
        unsafe { xlib::XDisplayHeight(self.display, screen_int) }
    }
    
    pub fn get_wm_hints(&self, window: Window) -> WMHints{
        WMHints{
            ptr: unsafe{
                xlib::XGetWMHints()
            }
        }
    }
}
