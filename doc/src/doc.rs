/*
 * Author: Dylan Turner
 * Description: Basic version of the documentation plugin for clonger
 */

use gtk4::{
    Box, Orientation, ScrolledWindow, TextView,
    prelude::BoxExt
};

type TabBuildFunc = fn() -> Box;

const NAME: &'static str = "Documentation";
const DEF_MARGIN: i32 = 10;

#[no_mangle]
pub extern "C" fn name(name_ref: &mut String) {
    *name_ref = String::from(NAME);
}

// This returns a function to be called later, due to Gtk's creation restriction
#[no_mangle]
#[allow(improper_ctypes_definitions)] // to use GTK types across plugin to core
pub extern "C" fn build_tab() -> TabBuildFunc {
    || {
        let content = Box::builder()
            .hexpand(true).vexpand(true)
            .orientation(Orientation::Vertical)
            .build();
        let scrollview = ScrolledWindow::builder()
            .hexpand(true).vexpand(true)
            .margin_top(DEF_MARGIN).margin_bottom(DEF_MARGIN)
            .margin_start(DEF_MARGIN).margin_end(DEF_MARGIN)
            .build();
        let text = TextView::builder()
            .accepts_tab(true)
            .build();

        scrollview.set_child(Some(&text));
        content.append(&scrollview);
        content
    }
}

#[no_mangle]
pub extern "C" fn on_key_pressed(
        _key: &String,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool,
        _file: &mut String, _fname: &mut String) -> bool {
    // TODO: Implement key press event
    false
}

#[no_mangle]
pub extern "C" fn on_key_released(
        _key: &String,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool) {
    // TODO: Implement key release event
}
