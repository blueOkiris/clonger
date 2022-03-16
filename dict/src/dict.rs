/*
 * Author: Dylan Turner
 * Description: Basic version of the documentation plugin for clonger
 */

use gtk::{
    Box, Orientation, Label
};

type TabBuildFunc = fn(&Label) -> Box;

const NAME: &'static str = "Dictionary";

#[no_mangle]
pub extern "C" fn name(name_ref: &mut String) {
    *name_ref = String::from(NAME);
}

// This returns a function to be called later, due to Gtk's creation restriction
#[no_mangle]
#[allow(improper_ctypes_definitions)] // to use GTK types across plugin to core
pub extern "C" fn build_tab() -> TabBuildFunc {
    |_label| {
        Box::builder()
            .hexpand(true).vexpand(true)
            .orientation(Orientation::Vertical)
            .build()
    }
}

#[no_mangle]
pub extern "C" fn win_on_key_pressed(
        _key: &String,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool,
        _clong_file: &mut String, _fname: &mut String) -> bool {
    // TODO: Implement key press event
    false
}

#[no_mangle]
pub extern "C" fn win_on_key_released(
        _key: &String,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool) {
    // TODO: Implement key release event
}
