use ps2_mouse::{Mouse, MouseState};
use spinning_top::Spinlock;
use conquer_once::spin::Lazy;

use crate::serial_println; // or use `conquer_once` if you prefer

pub static MOUSE: Lazy<Spinlock<Mouse>> = Lazy::new(|| Spinlock::new(Mouse::new()));

pub fn init_mouse() {
    match MOUSE.lock().init() {
        Ok(_) => {
            serial_println!("Mouse initialized");
        }
        Err(e) => {
            serial_println!("Mouse initialization failed: {:?}", e);
        }
    }
    MOUSE.lock().set_on_complete(on_complete);
}

fn on_complete(mouse_state: MouseState) {
    serial_println!("Mouse initialized: {:?}", mouse_state);
    // You can also update your cursor, framebuffer, etc. here.
}