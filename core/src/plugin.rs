/*
 * Author: Dylan Turner
 * Description: Define an interface for creating plugins
 */

use std::fs::read_dir;
use libloading::{ Library, Symbol };

type KeyEventHandler = unsafe fn(&String, bool, bool, bool, bool) -> isize;

pub struct Plugin {
    lib: Library
}

/*
 * This is the main thing right here!
 * If you want to make a plugin, it has to implement all of these methods
 * even if it chooses to do nothing in them
 */
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
        let key_pressed_handler: Symbol<KeyEventHandler> = unsafe {
            self.lib.get(b"on_key_pressed").unwrap()
        };
        unsafe {
            key_pressed_handler(
                key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed
            );
        }
    }
    
    pub fn on_key_released(
            &self,
            key: &String,
            ctrl_pressed: bool, alt_pressed: bool,
            shift_pressed: bool, super_pressed: bool) {
        let key_released_handler: Symbol<KeyEventHandler> = unsafe {
            self.lib.get(b"on_key_pressed").unwrap()
        };
        unsafe {
            key_released_handler(
                key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed
            );
        }
    }
}


/*
 * These need to be const so we can call it statically for the App struct
 * The App struct has to be static to be able to share data between event clsrs
 * Also, const can't use for loops, so we have to use recursion
 */

pub fn load_plugins() -> Vec<Plugin> {
    let mut libs = Vec::new();
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
