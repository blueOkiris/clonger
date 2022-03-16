/*
 * Author: Dylan Turner
 * Description: Basic version of the ipa typing window plugin for clonger
 */

use gtk::gdk::keys::Key;

const NAME: &'static str = "w_IPA";

#[no_mangle]
pub extern "C" fn name(name_ref: &mut String) {
    *name_ref = String::from(NAME);
}

#[no_mangle]
pub extern "C" fn on_key_pressed(
        _key: &Key,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool,
        _clong_file: &mut String, _fname: &mut String) -> bool {
    // TODO: Implement key press event
    false
}

#[no_mangle]
pub extern "C" fn on_key_released(
        _key: &Key,
        _ctrl_pressed: bool, _alt_pressed: bool,
        _shift_pressed: bool, _super_pressed: bool) {
    // TODO: Implement key release event
}
