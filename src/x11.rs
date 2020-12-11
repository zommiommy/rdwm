use x11::xlib;

struct SafeDisplay {
    display: xlib::Display
}

impl Drop for SafeDisplay {
    fn drop(&mut self) {
        xlib::XCloseDisplay(self.display)
    }
}

impl SafeDisplay {
    fn new(name: Option<String>) -> SafeDisplay {
        MyDisplay{
            display: match name{
                Some(name_string) => xlib::XOpenDisplay(&name_string as *const str),
                None => xlib::XOpenDisplay(std::ptr::null),
            }
        }
    }


}