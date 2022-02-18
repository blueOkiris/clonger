/*
 * Author: Dylan Turner
 * Description: Basic version of the ipa typing window plugin for clonger
 */

const NAME: &'static str = "w_IPA";

#[no_mangle]
pub extern "C" fn name(name_ref: &mut String) {
    *name_ref = String::from(NAME);
}

#[no_mangle]
pub extern "C" fn on_key_pressed(
        key: &String,
        ctrl_pressed: bool, alt_pressed: bool,
        shift_pressed: bool, super_pressed: bool) {
    println!(
        "Key press from IPA! Key: {}, Ctrl: {}, Alt: {}, Shift: {}, Sup: {}",
        key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed
    );
}

#[no_mangle]
pub extern "C" fn on_key_released(
        key: &String,
        ctrl_pressed: bool, alt_pressed: bool,
        shift_pressed: bool, super_pressed: bool) {
    println!(
        "Key release from IPA! Key: {}, Ctrl: {}, Alt: {}, Shift: {}, Sup: {}",
        key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed
    );
}
