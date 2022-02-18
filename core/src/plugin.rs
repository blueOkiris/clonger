/*
 * Author: Dylan Turner
 * Description: Define an interface for creating plugins
 */

use std::fs::read_dir;
use libloading::{ Library, Symbol };

type KeyPressedEvent = unsafe fn(&String, bool, bool, bool, bool) -> isize;

pub struct Plugin {
    lib: Library
}

impl Plugin {
    pub fn new(fname: String) -> Self {
        Self {
            lib: unsafe { Library::new(fname).unwrap() }
        }
    }
    
    pub fn on_key_pressed(
            &self,
            key: &String,
            ctrl_pressed: bool, alt_pressed: bool,
            shift_pressed: bool, super_pressed: bool) {
        let key_pressed_handler: Symbol<KeyPressedEvent> = unsafe {
            self.lib.get(b"on_key_pressed").unwrap()
        };
        unsafe {
            key_pressed_handler(
                key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed
            );
        }
    }
}

pub fn load_plugins() -> Vec<Plugin> {
    let mut libs: Vec<Plugin> = Vec::new();
    let paths = read_dir("target/release/").unwrap();
    for path in paths {
        let fname = path.unwrap().path().display().to_string();
        if fname.ends_with(".so") {
            println!("Found plugin: {}", fname);
            libs.push(Plugin::new(fname));
        }
    }
    libs
}
