/*
 * Author: Dylan Turner
 * Description: Define an interface for creating plugins
 */

use std::fs::read_dir;
use gtk4::{ Box, Orientation };
use libloading::{ Library, Symbol, Error };

type Name = unsafe fn(&mut String);
type KeyPressedHandler = unsafe fn(
    &String, bool, bool, bool, bool, &mut String, &mut String
) -> bool;
type KeyReleasedHandler = unsafe fn(&String, bool, bool, bool, bool);
pub type TabBuildFunc = unsafe fn() -> Box;
type TabBuildFuncLoader = unsafe fn() -> TabBuildFunc;

// Note that name isn't stored here so a vector of names can be copied
pub struct Plugin {
    lib: Library
}

/*
 * This is the main thing right here!
 * If you want to make a plugin, it has to implement all of these methods
 * even if it chooses to do nothing in them
 * 
 * If it's missing a plugin, default behavior (usually nothing) is used
 */
impl Plugin {
    pub fn new(fname: String) -> Self {
        Self {
            lib: unsafe { Library::new(fname).unwrap() }
        }
    }

    pub fn name(&self) -> String {
        let mut ret = String::new();
        let name_func: Result<Symbol<Name>, Error> = unsafe {
            self.lib.get(b"name")
        };
        match name_func {
            Err(err) => { // Just ignore if missing the function
                println!("Error in plugin's build function: {}", err);
            }, Ok(n) => unsafe { n(&mut ret) }
        }
        ret
    }

    pub fn build_tab(&self) -> TabBuildFunc {
        let handler: Result<Symbol<TabBuildFuncLoader>, Error> = unsafe {
            self.lib.get(b"build_tab")
        };
        match handler {
            Err(err) => {
                println!("c!");
                println!("Error in plugin's build_tab function: {}", err);
                || {
                    Box::builder()
                        .hexpand(true).vexpand(true)
                        .orientation(Orientation::Vertical)
                        .build()
                }
            }, Ok(build_func_loader) => unsafe {
                build_func_loader()
            }
        }
    }

    pub fn on_key_pressed(
            &self,
            key: &String,
            ctrl_pressed: bool, alt_pressed: bool,
            shift_pressed: bool, super_pressed: bool,
            file: &mut String, fname: &mut String) -> bool {
        let handler: Result<Symbol<KeyPressedHandler>, Error> = unsafe {
            self.lib.get(b"on_key_pressed")
        };
        match handler {
            Err(err) => {
                println!("Error in plugin's build function: {}", err);
                false
            }, Ok(key_pressed_handler) => unsafe {
                key_pressed_handler(
                    key,
                    ctrl_pressed, alt_pressed, shift_pressed, super_pressed,
                    file, fname
                )
            }
        }
    }

    pub fn on_key_released(
            &self,
            key: &String,
            ctrl_pressed: bool, alt_pressed: bool,
            shift_pressed: bool, super_pressed: bool) {
        let handler: Result<Symbol<KeyReleasedHandler>, Error> = unsafe {
            self.lib.get(b"on_key_released")
        };
        match handler {
            Err(err) => {
                println!("Error in plugin's build function: {}", err);
            }, Ok(key_released_handler) => unsafe {
                key_released_handler(
                    key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed
                );
            }
        }
    }
}


/*
 * These need to be const so we can call it statically for the App struct
 * The App struct has to be static to be able to share data between event clsrs
 * Also, const can't use for loops, so we have to use recursion
 */
pub fn load_plugins() -> (Vec<Plugin>, Vec<String>) {
    let mut libs = Vec::new();
    let mut lib_names = Vec::new();
    let paths = read_dir("target/release/").unwrap();
    for path in paths {
        let fname = path.unwrap().path().display().to_string();
        if fname.ends_with(".so") {
            println!("Found plugin: {}", fname);
            
            let plugin = Plugin::new(fname);
            let name = plugin.name();

            lib_names.push(name);
            libs.push(plugin);
        }
    }
    (libs, lib_names)
}
