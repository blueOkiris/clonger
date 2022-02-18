/*
 * Author: Dylan Turner
 * Description: Basic version of the documentation plugin for clonger
 */

const NAME: &'static str = "Documentation";

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
        "Key press from Doc! Key: {}, Ctrl: {}, Alt: {}, Shift: {}, Sup: {}",
        key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed
    );
}

#[no_mangle]
pub extern "C" fn on_key_released(
        key: &String,
        ctrl_pressed: bool, alt_pressed: bool,
        shift_pressed: bool, super_pressed: bool) {
    println!(
        "Key release from Doc! Key: {}, Ctrl: {}, Alt: {}, Shift: {}, Sup: {}",
        key, ctrl_pressed, alt_pressed, shift_pressed, super_pressed
    );
}
